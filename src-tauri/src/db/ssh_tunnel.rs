use std::net::SocketAddr;
use std::sync::Arc;
use russh::client::{self, Handle, Handler};
use russh::{ChannelId, CryptoVec};
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::task::{self, JoinHandle};
use tokio::sync::Mutex;

#[derive(Debug, thiserror::Error)]
pub enum SshTunnelError {
    #[error("SSH connection failed: {0}")]
    Connection(String),
    #[error("Authentication failed: {0}")]
    Auth(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("SSH error: {0}")]
    Ssh(String),
}

struct ClientHandler;

impl Handler for ClientHandler {
    type Error = russh::Error;

    fn check_server_key(
        &mut self,
        _server_public_key: &russh::keys::PublicKey,
    ) -> impl std::future::Future<Output = Result<bool, Self::Error>> + Send {
        async { Ok(true) }
    }

    fn channel_open_confirmation(
        &mut self,
        _id: ChannelId,
        _max_packet_size: u32,
        _window_size: u32,
        _session: &mut client::Session,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async { Ok(()) }
    }

    fn channel_success(
        &mut self,
        _channel: ChannelId,
        _session: &mut client::Session,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async { Ok(()) }
    }

    fn channel_failure(
        &mut self,
        _channel: ChannelId,
        _session: &mut client::Session,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async { Ok(()) }
    }

    fn channel_close(
        &mut self,
        _channel: ChannelId,
        _session: &mut client::Session,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async { Ok(()) }
    }

    fn channel_eof(
        &mut self,
        _channel: ChannelId,
        _session: &mut client::Session,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async { Ok(()) }
    }

    fn data(
        &mut self,
        _channel: ChannelId,
        _data: &[u8],
        _session: &mut client::Session,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async { Ok(()) }
    }
}

pub struct SshTunnel {
    pub local_port: u16,
    _accept_task: JoinHandle<()>,
    _session: Arc<Mutex<Handle<ClientHandler>>>,
}

impl SshTunnel {
    pub async fn connect(
        ssh_host: &str,
        ssh_port: u16,
        ssh_username: &str,
        ssh_password: &str,
        ssh_private_key: &str,
        ssh_passphrase: &str,
        target_host: &str,
        target_port: u16,
    ) -> Result<Self, SshTunnelError> {
        let config = client::Config {
            inactivity_timeout: Some(std::time::Duration::from_secs(30)),
            ..Default::default()
        };
        let config = Arc::new(config);
        let sh = ClientHandler;

        let mut session = client::connect(config, (ssh_host, ssh_port), sh)
            .await
            .map_err(|e| SshTunnelError::Connection(e.to_string()))?;

        // Authenticate
        let auth_result = if !ssh_private_key.is_empty() {
            let key_pair = if ssh_passphrase.is_empty() {
                russh::keys::load_secret_key(ssh_private_key, None)
            } else {
                russh::keys::load_secret_key(ssh_private_key, Some(ssh_passphrase))
            };
            match key_pair {
                Ok(key) => {
                    let key_with_hash = russh::keys::PrivateKeyWithHashAlg::new(
                        Arc::new(key),
                        None,
                    );
                    session
                        .authenticate_publickey(ssh_username.to_string(), key_with_hash)
                        .await
                        .map_err(|e| SshTunnelError::Auth(e.to_string()))?
                }
                Err(e) => return Err(SshTunnelError::Auth(format!("Key load failed: {e}"))),
            }
        } else if !ssh_password.is_empty() {
            session
                .authenticate_password(ssh_username, ssh_password)
                .await
                .map_err(|e| SshTunnelError::Auth(e.to_string()))?
        } else {
            return Err(SshTunnelError::Auth("No authentication method provided".to_string()));
        };

        match auth_result {
            russh::client::AuthResult::Success => {}
            russh::client::AuthResult::Failure { .. } => {
                return Err(SshTunnelError::Auth("Authentication rejected by server".to_string()));
            }
        }

        let session = Arc::new(Mutex::new(session));

        // Bind local port
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let local_addr = listener.local_addr()?;
        let local_port = local_addr.port();

        let target_host = target_host.to_string();
        let target_port = target_port;
        let session_clone = Arc::clone(&session);

        let accept_task = task::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((local_stream, _)) => {
                        let sess = Arc::clone(&session_clone);
                        let host = target_host.clone();
                        let port = target_port;
                        task::spawn(async move {
                            let _ = forward_connection(sess, local_stream, host, port).await;
                        });
                    }
                    Err(_) => break,
                }
            }
        });

        Ok(SshTunnel {
            local_port,
            _accept_task: accept_task,
            _session: session,
        })
    }
}

async fn forward_connection(
    session: Arc<Mutex<Handle<ClientHandler>>>,
    mut local_stream: TcpStream,
    target_host: String,
    target_port: u16,
) -> Result<(), SshTunnelError> {
    let channel = {
        let mut sess = session.lock().await;
        sess.channel_open_direct_tcpip(target_host, target_port as u32, "127.0.0.1", 0)
            .await
            .map_err(|e| SshTunnelError::Ssh(e.to_string()))?
    };

    let stream = channel.into_stream();
    let (local_read, local_write) = local_stream.into_split();
    let (ssh_read, ssh_write) = tokio::io::split(stream);

    let to_local = task::spawn(async move {
        let mut read = ssh_read;
        let mut write = local_write;
        let _ = tokio::io::copy(&mut read, &mut write).await;
        let _ = write.shutdown().await;
    });

    let to_remote = task::spawn(async move {
        let mut read = local_read;
        let mut write = ssh_write;
        let _ = tokio::io::copy(&mut read, &mut write).await;
        let _ = write.shutdown().await;
    });

    let _ = tokio::join!(to_local, to_remote);
    Ok(())
}

use std::sync::Mutex;
#[cfg(not(test))]
use tauri::Manager;

mod commands;
mod db;
mod keyring;
mod plugin;
mod schema;
mod state;

pub struct AppState(Mutex<state::AppState>);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[cfg(not(test))]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(commands::ConnectionPools::new())
        .manage(commands::SshTunnels::new())
        .manage(commands::ActiveQueries::new())
        .setup(|app| {
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");
            std::fs::create_dir_all(&app_dir).ok();
            let app_state = state::AppState::new(&app_dir).expect("failed to init state");
            app.manage(AppState(Mutex::new(app_state)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::test_connection,
            commands::save_connection,
            commands::get_connections,
            commands::delete_connection,
            commands::connect_to_database,
            commands::execute_sql,
            commands::execute_multi_sql,
            commands::cancel_query,
            commands::get_schema,
            commands::save_query_history,
            commands::get_query_history,
            commands::save_bookmark,
            commands::get_bookmarks,
            commands::delete_bookmark,
            commands::export_csv,
            commands::export_json,
            commands::import_csv,
            commands::update_cell,
            commands::explain_query,
            commands::profile_query,
            commands::get_foreign_keys,
            commands::diff_schemas,
            commands::list_plugins,
            commands::read_plugin,
            commands::save_setting,
            commands::get_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
#[allow(dead_code)]
pub fn run() {}

#[cfg(test)]
mod test_entry {
    use super::*;
    #[test]
    fn touch_handlers() {
        let _ = commands::ConnectionPools::new;
        let _ = commands::ActiveQueries::new;
        let _ = commands::SshTunnels::new;
        let _ = commands::test_connection;
        let _ = commands::save_connection;
        let _ = commands::get_connections;
        let _ = commands::delete_connection;
        let _ = commands::connect_to_database;
        let _ = commands::execute_sql;
        let _ = commands::execute_multi_sql;
        let _ = commands::cancel_query;
        let _ = commands::get_schema;
        let _ = commands::save_query_history;
        let _ = commands::get_query_history;
        let _ = commands::save_bookmark;
        let _ = commands::get_bookmarks;
        let _ = commands::delete_bookmark;
        let _ = commands::export_csv;
        let _ = commands::export_json;
        let _ = commands::import_csv;
        let _ = commands::update_cell;
        let _ = commands::explain_query;
        let _ = commands::profile_query;
        let _ = commands::get_foreign_keys;
        let _ = commands::diff_schemas;
        let _ = commands::list_plugins;
        let _ = commands::read_plugin;
        let _ = commands::save_setting;
        let _ = commands::get_settings;
        let _ = db::connection::ConnectionConfig::new;
        let _ = db::pool::create_pool;
        let _ = db::pool::resolve_ssl_policy;
        let _ = db::pool::create_mysql_pool;
        let _ = db::query::execute_query;
        let _ = db::error::humanize_error;
        let _ = keyring::set_password;
        let _ = plugin::list_plugins;
        let _ = schema::get_schemas;
        let _ = AppState;
    }
}

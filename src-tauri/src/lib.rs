use tauri::Manager;
use std::sync::Mutex;

mod commands;
mod db;
mod schema;
mod state;

pub struct AppState(Mutex<state::AppState>);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(commands::ConnectionPools::new())
        .manage(commands::ActiveQueries::new())
        .setup(|app| {
            let app_dir = app.path().app_data_dir().expect("failed to get app data dir");
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
            commands::save_setting,
            commands::get_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

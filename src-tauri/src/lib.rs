mod auth;
mod hackatime;

use std::sync::Arc;
use tokio::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenvy::dotenv().ok();
    tauri::Builder::default()
        .plugin(tauri_plugin_oauth::init())
        .plugin(tauri_plugin_opener::init())
        .manage(Arc::new(Mutex::new(auth::AuthState::default())))
        .invoke_handler(tauri::generate_handler![
            auth::start_auth,
            auth::get_auth_state,
            auth::logout,
            hackatime::get_user_info,
            hackatime::get_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
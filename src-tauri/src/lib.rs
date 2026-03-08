mod auth;
mod hackatime;

use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenvy::dotenv().ok();
    tauri::Builder::default()
        .plugin(tauri_plugin_oauth::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(Arc::new(Mutex::new(auth::AuthState::default())))
        .setup(|app| {
            if let Some(token) = auth::load_token(app.handle()) {
                let auth_state = app.handle().state::<Arc<Mutex<auth::AuthState>>>();
                let auth_state_clone = auth_state.inner().clone();
                tauri::async_runtime::block_on(async move {
                    let mut state = auth_state_clone.lock().await;
                    state.is_authenticated = true;
                    state.access_token = Some(token);
                });
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            auth::start_auth,
            auth::get_auth_state,
            auth::logout,
            hackatime::get_user_info,
            hackatime::get_streak,
            hackatime::get_hours,
            hackatime::get_projects,
            hackatime::get_project_details,
            hackatime::get_latest_heartbeat,
            hackatime::get_api_key,
            hackatime::get_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
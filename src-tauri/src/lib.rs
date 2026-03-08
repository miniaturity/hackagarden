// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[derive(serde::Deserialize)]
struct TokenResponse {
    access_token: String,
}

#[tauri::command]
async fn exchange_code(code: String) -> Result<String, String> {
    let client = reqwest::Client::new();

    let params = [
        ("client_id",     "YOUR_CLIENT_ID"),
        ("client_secret", "YOUR_CLIENT_SECRET"),  // safe — runs in Rust, not JS
        ("code",          &code),
    ];

    let res = client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .form(&params)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<TokenResponse>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(res.access_token)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_oauth::init())
        .invoke_handler(tauri::generate_handler![exchange_code])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



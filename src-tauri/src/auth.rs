use base64::{engine::general_purpose, Engine as _};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

pub const BASE_URL: &str = "https://hackatime.hackclub.com";
const CLIENT_ID: &str = "euy1nsAGCGwK28I_IdgILVfq6rqqpW-ltIobjfZOhBQ";

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AuthState {
    pub is_authenticated: bool,
    pub access_token: Option<String>,
    pub username: Option<String>,
}

// --- PKCE helpers ---

fn generate_verifier() -> String {
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    general_purpose::URL_SAFE_NO_PAD.encode(&bytes)
}

fn generate_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    general_purpose::URL_SAFE_NO_PAD.encode(hasher.finalize())
}

fn generate_state() -> String {
    let mut rng = rand::thread_rng();
    const CHARSET: &[u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    (0..32)
        .map(|_| CHARSET[rng.gen::<usize>() % CHARSET.len()] as char)
        .collect()
}

fn parse_query(url: &str) -> std::collections::HashMap<String, String> {
    url.split('?')
        .nth(1)
        .unwrap_or("")
        .split('&')
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            let key = parts.next()?.to_string();
            let value = urlencoding::decode(parts.next().unwrap_or(""))
                .unwrap_or_default()
                .to_string();
            Some((key, value))
        })
        .collect()
}

// --- Commands ---

#[tauri::command]
pub async fn start_auth(
    auth_state: State<'_, Arc<Mutex<AuthState>>>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let (tx, rx) = tokio::sync::oneshot::channel::<String>();
    let tx = std::sync::Mutex::new(Some(tx));

    let port = tauri_plugin_oauth::start_with_config(
        tauri_plugin_oauth::OauthConfig {
            ports: Some(vec![8080]),
            ..Default::default()
        },
        move |url| {
            if let Some(sender) = tx.lock().unwrap().take() {
                let _ = sender.send(url);
            }
        },
    )
    .map_err(|e| format!("Failed to start OAuth listener: {e}"))?;

    let redirect_uri = format!("http://localhost:{port}");

    let verifier = generate_verifier();
    let challenge = generate_challenge(&verifier);
    let csrf_state = generate_state();

    let auth_url = format!(
        "{BASE_URL}/oauth/authorize\
        ?client_id={CLIENT_ID}\
        &redirect_uri={}\
        &response_type=code\
        &scope=profile\
        &state={}\
        &code_challenge={}\
        &code_challenge_method=S256",
        urlencoding::encode(&redirect_uri),
        urlencoding::encode(&csrf_state),
        urlencoding::encode(&challenge),
    );

    use tauri_plugin_opener::OpenerExt;
    app.opener()
        .open_url(&auth_url, None::<&str>)
        .map_err(|e| format!("Failed to open browser: {e}"))?;

    let callback_url = rx
        .await
        .map_err(|_| "OAuth flow cancelled or timed out".to_string())?;

    let _ = tauri_plugin_oauth::cancel(port);

    let params = parse_query(&callback_url);

    if let Some(error) = params.get("error") {
        return Err(format!("Auth error: {error}"));
    }

    let returned_state = params.get("state").ok_or("Missing state in callback")?;
    if returned_state != &csrf_state {
        return Err("State mismatch — possible CSRF attack".to_string());
    }

    let code = params
        .get("code")
        .ok_or("No authorization code in callback")?
        .clone();

    let client = reqwest::Client::new();

    let client_secret = std::env::var("CLIENT_SECRET")
    .map_err(|_| "CLIENT_SECRET not set".to_string())?;

    let token_resp = client
        .post(format!("{BASE_URL}/oauth/token"))
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", &code),
            ("client_id", CLIENT_ID),
            ("client_secret", &client_secret),
            ("redirect_uri", &redirect_uri),
            ("code_verifier", &verifier),
        ])
        .send()
        .await
        .map_err(|e| format!("Token exchange request failed: {e}"))?;

    let status = token_resp.status();
    let body = token_resp.text().await.unwrap_or_default();
    eprintln!("Token exchange status: {status}");
    eprintln!("Token exchange body: {body}");

    if !status.is_success() {
        return Err(format!("Token exchange failed ({status}): {body}"));
    }

    let token_json: serde_json::Value = serde_json::from_str(&body)
        .map_err(|e| format!("Failed to parse token response: {e}"))?;

    let access_token = token_json["access_token"]
        .as_str()
        .ok_or("No access_token in response")?
        .to_string();

    let user_resp = client
        .get(format!("{BASE_URL}/api/v1/authenticated/me"))
        .bearer_auth(&access_token)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch user info: {e}"))?;

    let username = if user_resp.status().is_success() {
        user_resp
            .json::<serde_json::Value>()
            .await
            .ok()
            .and_then(|v| v["username"].as_str().map(|s| s.to_string()))
    } else {
        None
    };

    let mut state = auth_state.lock().await;
    state.is_authenticated = true;
    state.access_token = Some(access_token);
    state.username = username;

    Ok(())
}

#[tauri::command]
pub async fn get_auth_state(
    auth_state: State<'_, Arc<Mutex<AuthState>>>,
) -> Result<AuthState, String> {
    Ok(auth_state.lock().await.clone())
}

#[tauri::command]
pub async fn logout(auth_state: State<'_, Arc<Mutex<AuthState>>>) -> Result<(), String> {
    *auth_state.lock().await = AuthState::default();
    Ok(())
}
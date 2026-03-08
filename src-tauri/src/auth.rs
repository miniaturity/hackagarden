use base64::{engine::general_purpose, Engine as _};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

pub const BASE_URL: &str = "https://hackatime.hackclub.com";
const CLIENT_ID: &str = "75fdeca1f06154cc711f05855655f76f";

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

/// Opens the browser to Hack Club's OAuth page, waits for the redirect,
/// exchanges the code for a token, and stores it in shared state.
/// The frontend just calls this and awaits — it blocks until auth completes.
#[tauri::command]
pub async fn start_auth(
    auth_state: State<'_, Arc<Mutex<AuthState>>>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let verifier = generate_verifier();
    let challenge = generate_challenge(&verifier);
    let csrf_state = generate_state();

    // oneshot channel to receive the callback URL from the plugin
    let (tx, rx) = tokio::sync::oneshot::channel::<String>();
    let tx = std::sync::Mutex::new(Some(tx));

    // Start a local HTTP server; when the browser redirects to it,
    // the plugin calls our closure with the full callback URL.
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

    // PKCE
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

    // Block until the browser redirects back (user logs in)
    let callback_url = rx
        .await
        .map_err(|_| "OAuth flow cancelled or timed out".to_string())?;

    // Clean up the local server
    let _ = tauri_plugin_oauth::cancel(port);

    let params = parse_query(&callback_url);

    if let Some(error) = params.get("error") {
        return Err(format!("Auth error: {error}"));
    }

    // CSRF check
    let returned_state = params
        .get("state")
        .ok_or("Missing state in callback")?;
    if returned_state != &csrf_state {
        return Err("State mismatch — possible CSRF attack".to_string());
    }

    let code = params
        .get("code")
        .ok_or("No authorization code in callback")?
        .clone();

    // Exchange the code for an access token
    let client = reqwest::Client::new();
    let client_secret = std::env::var("CLIENT_SECRET")
    .map_err(|_| "CLIENT_SECRET not set".to_string())?;

    let token_resp = client
        .post(format!("{BASE_URL}/oauth/token"))
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", &code),
            ("client_id", CLIENT_ID),
            ("redirect_uri", &redirect_uri),
            ("code_verifier", &verifier),
        ])
        .send()
        .await
        .map_err(|e| format!("Token exchange request failed: {e}"))?;

    if !token_resp.status().is_success() {
        let err = token_resp.text().await.unwrap_or_default();
        return Err(format!("Token exchange failed: {err}"));
    }

    let token_json: serde_json::Value = token_resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse token response: {e}"))?;

    let access_token = token_json["access_token"]
        .as_str()
        .ok_or("No access_token in response")?
        .to_string();

    // Grab username while we're at it
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
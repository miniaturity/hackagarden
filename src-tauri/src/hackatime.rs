use crate::auth::{AuthState, BASE_URL};
use chrono::Utc;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

async fn fetch_json(
    client: &reqwest::Client,
    url: &str,
    token: &str,
) -> Result<serde_json::Value, String> {
    let resp = client
        .get(url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Request failed ({status}): {body}"));
    }

    resp.json()
        .await
        .map_err(|e| format!("Failed to parse response: {e}"))
}

/// github_username, display_name, email, slack_id
#[tauri::command]
pub async fn get_user_info(
    auth_state: State<'_, Arc<Mutex<AuthState>>>,
) -> Result<serde_json::Value, String> {
    let token = auth_state.lock().await.access_token.clone()
        .ok_or("Not authenticated")?;
    fetch_json(&reqwest::Client::new(), &format!("{BASE_URL}/api/v1/authenticated/me"), &token).await
}

/// current coding streak
#[tauri::command]
pub async fn get_streak(
    auth_state: State<'_, Arc<Mutex<AuthState>>>,
) -> Result<serde_json::Value, String> {
    let token = auth_state.lock().await.access_token.clone()
        .ok_or("Not authenticated")?;
    fetch_json(&reqwest::Client::new(), &format!("{BASE_URL}/api/v1/authenticated/streak"), &token).await
}

/// total coding hours between two dates (YYYY-MM-DD)
#[tauri::command]
pub async fn get_hours(
    start_date: String,
    end_date: String,
    auth_state: State<'_, Arc<Mutex<AuthState>>>,
) -> Result<serde_json::Value, String> {
    let token = auth_state.lock().await.access_token.clone()
        .ok_or("Not authenticated")?;
    let url = format!("{BASE_URL}/api/v1/authenticated/hours?start_date={start_date}&end_date={end_date}");
    fetch_json(&reqwest::Client::new(), &url, &token).await
}

/// all projects the user has logged time on
#[tauri::command]
pub async fn get_projects(
    auth_state: State<'_, Arc<Mutex<AuthState>>>,
) -> Result<serde_json::Value, String> {
    let token = auth_state.lock().await.access_token.clone()
        .ok_or("Not authenticated")?;
    fetch_json(&reqwest::Client::new(), &format!("{BASE_URL}/api/v1/authenticated/projects"), &token).await
}

/// details for a single project by name
#[tauri::command]
pub async fn get_project_details(
    project_name: String,
    auth_state: State<'_, Arc<Mutex<AuthState>>>,
) -> Result<serde_json::Value, String> {
    let token = auth_state.lock().await.access_token.clone()
        .ok_or("Not authenticated")?;
    let url = format!("{BASE_URL}/api/v1/authenticated/projects/{}", urlencoding::encode(&project_name));
    fetch_json(&reqwest::Client::new(), &url, &token).await
}

/// most recent heartbeat
#[tauri::command]
pub async fn get_latest_heartbeat(
    auth_state: State<'_, Arc<Mutex<AuthState>>>,
) -> Result<serde_json::Value, String> {
    let token = auth_state.lock().await.access_token.clone()
        .ok_or("Not authenticated")?;
    fetch_json(&reqwest::Client::new(), &format!("{BASE_URL}/api/v1/authenticated/heartbeats/latest"), &token).await
}

/// API key for wakatime-compatible editors
#[tauri::command]
pub async fn get_api_key(
    auth_state: State<'_, Arc<Mutex<AuthState>>>,
) -> Result<String, String> {
    let token = auth_state.lock().await.access_token.clone()
        .ok_or("Not authenticated")?;
    let data = fetch_json(&reqwest::Client::new(), &format!("{BASE_URL}/api/v1/authenticated/api_keys"), &token).await?;
    data["token"].as_str().map(|s| s.to_string()).ok_or("No token in response".to_string())
}

/// streak + last 7 days of hours in one call
#[tauri::command]
pub async fn get_stats(
    auth_state: State<'_, Arc<Mutex<AuthState>>>,
) -> Result<serde_json::Value, String> {
    let token = auth_state.lock().await.access_token.clone()
        .ok_or("Not authenticated")?;

    let client = reqwest::Client::new();
    let today = Utc::now().date_naive();
    let week_ago = today - chrono::Duration::days(7);

    let streak_url = format!("{BASE_URL}/api/v1/authenticated/streak");
    let hours_url = format!(
        "{BASE_URL}/api/v1/authenticated/hours?start_date={}&end_date={}",
        week_ago.format("%Y-%m-%d"),
        today.format("%Y-%m-%d"),
    );

    let (streak, hours) = tokio::join!(
        fetch_json(&client, &streak_url, &token),
        fetch_json(&client, &hours_url, &token)
    );

    Ok(serde_json::json!({
        "streak": streak.unwrap_or_default(),
        "weekly_hours": hours.unwrap_or_default(),
    }))
}
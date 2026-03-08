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
        return Err(format!("Request to {url} returned {}", resp.status()));
    }

    resp.json()
        .await
        .map_err(|e| format!("Failed to parse response: {e}"))
}

/// Returns the raw /me response: username, display_name, email, slack_id, etc.
#[tauri::command]
pub async fn get_user_info(
    auth_state: State<'_, Arc<Mutex<AuthState>>>,
) -> Result<serde_json::Value, String> {
    let token = auth_state
        .lock()
        .await
        .access_token
        .clone()
        .ok_or("Not authenticated")?;

    fetch_json(
        &reqwest::Client::new(),
        &format!("{BASE_URL}/api/v1/authenticated/me"),
        &token,
    )
    .await
}

/// Returns streak days + total coding seconds for the past 7 days.
#[tauri::command]
pub async fn get_stats(
    auth_state: State<'_, Arc<Mutex<AuthState>>>,
) -> Result<serde_json::Value, String> {
    let token = auth_state
        .lock()
        .await
        .access_token
        .clone()
        .ok_or("Not authenticated")?;

    let client = reqwest::Client::new();
    let today = Utc::now().date_naive();
    let week_ago = today - chrono::Duration::days(7);

    // Fetch streak and weekly hours in parallel
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
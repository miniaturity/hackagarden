use crate::auth::{AuthState, BASE_URL};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::State;
use tauri_plugin_store::StoreExt;
use tokio::sync::Mutex;

const SECONDS_PER_COIN: u64 = 3600;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CurrencyState {
    pub balance: u64,
    /// Total hours already converted
    pub total_hours_minted: u64,
    /// Cumulative hours per language (in hours)
    pub language_hours: HashMap<String, f64>,
    /// Cumulative total hours 
    pub total_hours: f64,
}



fn get_store(
    app: &tauri::AppHandle,
) -> Arc<tauri_plugin_store::Store<tauri::Wry>> {
    app.store("currency.json").unwrap()
}

pub fn load_currency(app: &tauri::AppHandle) -> CurrencyState {
    let store = get_store(app);
    store
        .get("currency_state")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default()
}

fn save_currency(app: &tauri::AppHandle, state: &CurrencyState) {
    let store = get_store(app);
    store.set(
        "currency_state",
        serde_json::to_value(state).expect("serializable"),
    );
    let _ = store.save();
}

// api helpers

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
        return Err(format!("API error ({status}): {body}"));
    }

    resp.json()
        .await
        .map_err(|e| format!("Parse error: {e}"))
}

// --- Tauri Commands ---

#[tauri::command]
pub async fn get_currency_state(app: tauri::AppHandle) -> Result<CurrencyState, String> {
    Ok(load_currency(&app))
}

#[tauri::command]
pub async fn sync_currency(
    auth_state: State<'_, Arc<Mutex<AuthState>>>,
    app: tauri::AppHandle,
) -> Result<CurrencyState, String> {
    let token = auth_state
        .lock()
        .await
        .access_token
        .clone()
        .ok_or("Not authenticated")?;

    let client = reqwest::Client::new();
    let today = Utc::now().date_naive().format("%Y-%m-%d").to_string();

    let hours_url = format!(
        "{BASE_URL}/api/v1/authenticated/hours?start_date=2020-01-01&end_date={today}"
    );
    let projects_url = format!("{BASE_URL}/api/v1/authenticated/projects");

    let (hours_result, projects_result) = tokio::join!(
        fetch_json(&client, &hours_url, &token),
        fetch_json(&client, &projects_url, &token),
    );

    let hours_json = hours_result?;
    let projects_json = projects_result?;

    let total_seconds = hours_json
        .get("total_seconds")
        .or_else(|| hours_json.pointer("/data/total_seconds"))
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0) as u64;

    let total_hours_now = total_seconds / SECONDS_PER_COIN; // whole hours

    let mut language_hours: HashMap<String, f64> = HashMap::new();

    if let Some(projects) = projects_json
        .get("projects")
        .and_then(|v| v.as_array())
    {
        for project in projects {
            if let Some(langs) = project.get("languages").and_then(|v| v.as_array()) {
                for lang in langs {
                    let name = lang
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown")
                        .to_string();
                    let secs = lang
                        .get("total_seconds")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0);
                    *language_hours.entry(name).or_insert(0.0) += secs / 3600.0;
                }
            }
        }
    }

    let mut state = load_currency(&app);

    let new_whole_hours = total_hours_now.saturating_sub(state.total_hours_minted);

    if new_whole_hours > 0 {
        state.balance += new_whole_hours;
        state.total_hours_minted += new_whole_hours;
    }

    state.language_hours = language_hours;
    state.total_hours = total_seconds as f64 / 3600.0;

    save_currency(&app, &state);
    Ok(state)
}

#[tauri::command]
pub async fn spend_coins(
    amount: u64,
    app: tauri::AppHandle,
) -> Result<CurrencyState, String> {
    let mut state = load_currency(&app);
    if amount > state.balance {
        return Err(format!(
            "Insufficient coins: have {}, need {amount}",
            state.balance
        ));
    }
    state.balance -= amount;
    save_currency(&app, &state);
    Ok(state)
}
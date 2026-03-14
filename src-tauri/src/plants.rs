use crate::currency::{load_currency, save_currency};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri_plugin_store::StoreExt;

pub struct Plant {
  pub id: String,
  pub name: String,
  pub desc: String,
  pub cost: u64,
  pub texture: String
}



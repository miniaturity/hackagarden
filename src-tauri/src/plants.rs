use crate::currency::{load_currency, save_currency};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri_plugin_store::StoreExt;

pub struct Plant {
  pub id: String,
  pub name: String,
  pub desc: String,
  pub rarity: String,
  pub cost: u64,
  pub texture: String,

  pub hoursWhenBought: u64,
  
  pub growth: u64, // Growth in hours
  pub growthStage: u64, // Stage of growth ()
  pub growthMult: u64, // Multiplier for growth
}



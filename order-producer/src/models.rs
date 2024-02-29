use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::securities::random_security;

#[derive(Serialize, Deserialize)]
pub enum AssetClass {
    FX,
    FI,
    COMMODITY,
    EQUITY,
}

#[derive(Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub created_date: DateTime<Utc>,
    pub client_id: String,
    pub instrument: String,
    pub asset_class: AssetClass,
    pub quantity: i32,
}

impl Order {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            created_date: Utc::now(),
            client_id: Uuid::new_v4().to_string(),
            instrument: random_security(),
            asset_class: AssetClass::FX,
            quantity: rand::thread_rng().gen(),
        }
    }
}

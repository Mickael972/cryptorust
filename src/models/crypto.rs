use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Crypto {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub image: String,
    pub usd: f64,
}
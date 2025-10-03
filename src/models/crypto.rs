use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Crypto {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub image: Option<String>,
    #[serde(rename = "current_price")]
    pub usd: f64,
}
use crate::models::crypto::Crypto;
use reqwest::{header, Error};

pub async fn fetch_crypto_data() -> Result<Vec<Crypto>, Error> {
    let client = reqwest::Client::new();
    let cryptos = client
        .get("https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd")
        .header(header::USER_AGENT, "Mozilla/5.0 (compatible; MyRustApp/1.0; +http://mywebsite.com)")
            .send()
            .await?
            .json::<Vec<Crypto>>()
            .await?;

    Ok(cryptos)
}
use crate ::service::coingecko_service;
use actix_web::{get, HttpResponse, Responder};
use  serde::Deserialize;

#[derive(Deserialize)]
pub struct CryptoQuery {
    #[serde(default = "default_ids")]
    id: String,
}

fn default_ids() -> String {
    "bitcoin,ethereum".to_string()
}

#[get("/crypto")]
pub async fn get_crypto(query: actix_web::web::Query<CryptoQuery>) -> impl Responder {
    match coingecko_service::fetch_crypto_data(&query.id).await {
        Ok(crypto) => HttpResponse::Ok().json(crypto),
        Err(_) => HttpResponse::InternalServerError().body("Erreur lors de la récupération des données."),
    }
}
use crate ::service::coingecko_service;
use actix_web::{get, HttpResponse, Responder};

#[get("/crypto")]
pub async fn get_crypto() -> impl Responder {
    match  coingecko_service::fetch_crypto_data().await {
        Ok(crypto) => HttpResponse::Ok().json(crypto),
        Err(_) => HttpResponse::InternalServerError().body("Erreur lors de la récupération des données."),
    }
}
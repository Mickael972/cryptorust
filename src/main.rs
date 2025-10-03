mod models; 
mod handler;
mod service;
use actix_web::{get, App, HttpServer, Responder};



#[get("/")]
async fn index() -> impl Responder {
    "Bonjour ! Le serveur est en ligne."
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Le serveur démarre sur http://127.0.0.1:8080");

    
    HttpServer::new(|| {
        
        App::new().service(handler::handler::get_crypto)
    })
    .bind(("127.0.0.1", 8080))? 
    .run()
    .await
}
use actix_web::{get, App, HttpServer, Responder};

// C'est une fonction qui répondra quand on visite la page d'accueil.
// L'étiquette #[get("/")] dit : "Hé, si quelqu'un arrive sur l'adresse de base, c'est toi qui travailles !"
#[get("/")]
async fn index() -> impl Responder {
    "Bonjour ! Le serveur est en ligne."
}

// C'est la fonction principale qui lance tout.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Le serveur démarre sur http://127.0.0.1:8080");

    // On crée un nouveau serveur HTTP.
    HttpServer::new(|| {
        // On enregistre notre fonction "index" pour qu'elle réponde à la racine "/".
        App::new().service(index)
    })
    .bind(("127.0.0.1", 8080))? // On dit au serveur d'écouter sur le port 8080.
    .run()
    .await
}
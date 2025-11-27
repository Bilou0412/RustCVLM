use axum::{
    routing::get,
    routing::post,
    Json,
    Router,
};

use serde::Deserialize;

#[derive(Deserialize,Debug)]
struct CreateVehicle{
    brand: String,
    model: String,
    year: u32,
}

#[tokio::main]
async fn main() {
    // 1. METS UN POINT D'ARRÊT ICI (Ligne A)
    println!("Configuration du routeur..."); 
    
    // On appelle une fonction nommée 'handler_hello' au lieu de le faire inline
    let app = Router::new()
    .route("/", get(handler_hello))
    .route("/vehicle", post(create_vehicle));

    println!("Le serveur va démarrer sur le port 3000...");
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    // Le debugger va 'attendre' ici que des requêtes arrivent
    axum::serve(listener, app).await.unwrap();
}

async fn handler_hello() -> &'static str {
    let message = "Hello, World!";   

    // 🔴 METS LE POINT ROUGE ICI (Ligne 24 par exemple)
    println!("Debug value: {}", message); 

    println!("J'ai reçu une visite !");
    message 
}

async fn create_vehicle(Json(payload): Json<CreateVehicle>) -> String {
    
    // 🔴 3. METS TON POINT D'ARRÊT (BREAKPOINT) ICI (Ligne 31)
    println!("Nouvelle voiture reçue !");
    
    // Ici, tu pourras inspecter 'payload' dans le debugger
    let reponse = format!("Voiture créée : {} {} ({})", payload.brand, payload.model, payload.year);
    
    reponse
}
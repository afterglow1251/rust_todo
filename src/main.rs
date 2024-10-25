mod routes;  // Імпортуємо модулі маршрутів
mod models;  // Імпортуємо моделі

use actix_web::{web, App, HttpServer};
use mongodb::{Client, options::ClientOptions};
use std::env;
use dotenv::dotenv;
use crate::routes::auth::{signup, login, logout}; // Імпортуємо маршрути

async fn init_mongo() -> mongodb::error::Result<Client> {
    dotenv().ok();
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client_options = ClientOptions::parse(uri).await?;

    Client::with_options(client_options)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongo_client = init_mongo().await.expect("Failed to initialize MongoDB client");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(mongo_client.clone())) // Додаємо клієнт MongoDB до App
            .service(signup) // Реєструємо маршрут для signup
            .service(login)  // Реєструємо маршрут для login
            .service(logout) // Реєструємо маршрут для logout
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

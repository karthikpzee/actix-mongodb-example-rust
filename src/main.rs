use actix_cors::Cors;
use actix_web::{App, http, HttpServer, middleware, web};
use mongodb::Client;

mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_env_variables();
    let port = std::env::var("PORT").unwrap();
    let mongodb_connection_string = std::env::var("MONGODB_CONNECTION_STRING").unwrap();
    println!("Starting server on port {}!", port);
    let client = Client::with_uri_str(mongodb_connection_string).await.expect("Failed to connect to MongoDB!");
    let db_data = web::Data::new(client);
    HttpServer::new(move || {
        let cors = Cors::default()
              .allow_any_origin()
              .allow_any_method()
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT, http::header::CONTENT_TYPE])
              .max_age(3600);
        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(db_data.clone())
            .service(controllers::default_controller::index)
            .service(controllers::webhook_controller::index)
    })
    .bind(("127.0.0.1", port.parse().unwrap()))?
    .run()
    .await
}

fn set_env_variables() {
    set_default_env_var("PORT", "3000");
    set_default_env_var("MONGODB_CONNECTION_STRING", "mongodb://root:password@localhost/ContentData?authSource=admin&w=1");
}

fn set_default_env_var(key: &str, value: &str) {
    if std::env::var(key).is_err() {
        std::env::set_var(key, value);
    }
}

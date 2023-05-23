use actix_web::{get, HttpResponse};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    name: String,
    version: String
}

#[get("/")]
pub async fn index() -> HttpResponse {
    let info = Info { name: String::from("cdm-generator-rust"), version: String::from("1.0.0") };
    HttpResponse::Ok()
        .content_type("application/json")
        .json(info)
}
use actix_web::{get, HttpResponse, web::Data};
use mongodb::{Client, Collection, bson};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    count: u64
}

#[get("/webhook")]
pub async fn index(client: Data<Client>) -> HttpResponse {
    let data: Collection<bson::Document> = client.database("ContentData").collection("canonicaldatas");
    let response = Response { count: data.count_documents(bson::doc! {}, None).await.unwrap()};
    HttpResponse::Ok()
        .content_type("application/json")
        .json(response)
}
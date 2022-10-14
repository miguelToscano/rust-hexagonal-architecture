use actix_web::{web, App, HttpResponse, HttpServer, Responder};

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

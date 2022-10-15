use actix_web::{
    web::{self, Json},
    App, HttpResponse, HttpServer, Responder,
};

#[derive(serde::Serialize)]
pub struct HealthCheckResponse {
    pub status: String,
}

pub async fn health_check() -> impl Responder {
    let response = HealthCheckResponse {
        status: String::from("Ok"),
    };

    return Json(response);
}

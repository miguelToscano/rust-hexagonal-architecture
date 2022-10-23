use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::PgPool;
use std::net::TcpListener;

use crate::server::routes;

pub async fn run(listener: TcpListener) -> Result<()> {
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(routes::health_check))
            .route("/sign_up", web::post().to(routes::sign_up))
            .route("/users", web::get().to(routes::get_users))
    })
    .bind("lolcahost:8080")?
    .await;
}

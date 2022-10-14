use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::PgPool;
use std::net::TcpListener;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);

    let server =
        HttpServer::new(move || App::new().route("/health_check", web::get().to(health_check)))
            .listen(listener)?
            .run();

    Ok(server)
}

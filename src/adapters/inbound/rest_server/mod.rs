use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use mongodb::{bson::doc, options::ClientOptions, Client};

use crate::adapters::outbound::users_repositories::{UsersRepository, };
use crate::adapters::outbound::users_repositories::mongo_db::MongoDBUsersRepository;
pub mod routes;
pub mod types;

pub async fn run() -> std::io::Result<()> {
    let users_repository = MongoDBUsersRepository::init().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(users_repository.clone()))
            .route("/health_check", web::get().to(routes::health_check))
            .route("/sign_up", web::post().to(routes::sign_up))
            .service(routes::get_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

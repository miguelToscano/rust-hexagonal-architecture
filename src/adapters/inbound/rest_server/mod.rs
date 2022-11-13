use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use mongodb::{bson::doc, options::ClientOptions, Client};

use crate::adapters::outbound::users_repositories::mongo_db::MongoDBUsersRepository;
use crate::adapters::outbound::users_repositories::UsersRepository;
pub mod routes;
pub mod types;

pub async fn run() -> std::io::Result<()> {
    let users_repository = MongoDBUsersRepository::init().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(users_repository.clone()))
            .service(routes::get_users)
            .service(routes::sign_up)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

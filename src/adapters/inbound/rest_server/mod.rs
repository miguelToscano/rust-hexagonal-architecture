extern crate env_logger;
use actix_web::{middleware::Logger, web, App, HttpServer};

use crate::adapters::outbound::users_repositories::mongo_db::MongoDBUsersRepository;
pub mod routes;
pub mod types;

pub async fn run() -> Result<(), std::io::Error> {
    let users_repository = MongoDBUsersRepository::init().await;
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new(
                "Started at: %t - Status code: %s - Response size: %b bytes - Response time: %T milliseconds",
            ))
            .app_data(web::Data::new(users_repository.clone()))
            .service(routes::get_users)
            .service(routes::sign_up)
            .service(routes::get_user_by_emai)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

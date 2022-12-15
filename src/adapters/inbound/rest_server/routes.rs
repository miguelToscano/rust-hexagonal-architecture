use actix_web::{
    get, post,
    web::{self, Data, Json},
    HttpResponse,
};
use serde::Deserialize;

use crate::adapters::outbound::users_repositories::mongo_db::MongoDBUsersRepository;
use crate::domain::users::types::{CreateUserInput, User};
use crate::ports;

#[derive(serde::Serialize)]
pub struct HealthCheckResponse {
    pub status: String,
}

pub async fn health_check() -> Json<HealthCheckResponse> {
    let response = HealthCheckResponse {
        status: String::from("Ok"),
    };

    Json(response)
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct SignUpRequestBody {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct SignUpResponse {
    pub token: String,
}

#[post("/sign_up")]
pub async fn sign_up(
    users_repository: Data<MongoDBUsersRepository>,
    sign_up_input: Json<SignUpRequestBody>,
) -> HttpResponse {
    println!("Sign up request body: {:?}", sign_up_input);

    let create_user_input = CreateUserInput {
        email: sign_up_input.email.clone(),
        password: sign_up_input.password.clone(),
        username: sign_up_input.username.clone(),
    };

    match ports::inbound::create_user(users_repository.get_ref(), &create_user_input).await {
        Ok(token) => HttpResponse::Ok().json(SignUpResponse { token }),
        Err(()) => HttpResponse::InternalServerError().body(String::from("Internal server error")),
    }
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct GetUsersResponse {
    pub count: usize,
    pub users: Vec<User>,
}

#[derive(serde::Serialize, Deserialize)]
pub struct GetUserByEmailResponse {
    pub user: User,
}

#[get("/users")]
pub async fn get_users(users_repository: Data<MongoDBUsersRepository>) -> HttpResponse {
    println!("Getting users");

    match ports::inbound::get_users(users_repository.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(GetUsersResponse {
            count: users.len(),
            users,
        }),
        Err(()) => HttpResponse::InternalServerError().body(String::from("Internal server error")),
    }
}

#[get("/users/{email}")]
pub async fn get_user_by_emai(
    users_repository: Data<MongoDBUsersRepository>,
    info: web::Path<(String,)>,
) -> HttpResponse {
    println!("Getting user by email");

    match ports::inbound::get_user_by_email(users_repository.get_ref(), info.into_inner().0).await
    {
        Ok(user) => HttpResponse::Ok().json(GetUserByEmailResponse { user }),
        Err(()) => HttpResponse::InternalServerError().body(String::from("Internal server error")),
    }
}

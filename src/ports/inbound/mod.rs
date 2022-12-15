use crate::domain::users::services as users_service;
use crate::domain::users::types::{CreateUserInput, User};
use crate::ports::outbound::databases::mongo_db::UsersRepository;

pub async fn get_users(users_repository: &impl UsersRepository) -> Result<Vec<User>, ()> {
    let users = users_service::get_all(users_repository).await?;
    Ok(users)
}

pub async fn create_user(
    users_repository: &impl UsersRepository,
    user: &CreateUserInput,
) -> Result<String, ()> {
    users_service::create(users_repository, user).await?;
    Ok(String::from("Computed token"))
}

pub async fn get_user_by_email(
    users_repository: &impl UsersRepository,
    email: String,
) -> Result<User, ()> {
    let user = users_service::get_by_email(users_repository, email).await?;
    Ok(user)
}

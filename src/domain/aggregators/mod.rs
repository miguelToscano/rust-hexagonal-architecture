use crate::adapters::outbound::users_repositories::UsersRepository;
use crate::domain::users::types::{User, CreateUserInput};
use crate::domain::users::services as users_service;

pub async fn get_users(users_repository: &impl UsersRepository) -> Result<Vec<User>, ()> {
    let users = users_service::get_all(users_repository).await?;
    return Ok(users);
}

pub async fn create_user(users_repository: &impl UsersRepository, user: &CreateUserInput ) -> Result<String, ()> {
    let created_user = users_service::create(users_repository, user).await?;
    return Ok(String::from("Computed token"));
}
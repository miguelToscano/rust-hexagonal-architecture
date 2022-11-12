use crate::adapters::outbound::users_repositories::UsersRepository;
use crate::domain::users::types::User;
use crate::domain::users::services as users_service;

pub async fn get_users(users_repository: &impl UsersRepository) -> Result<Vec<User>, ()> {
    let users = users_service::get_users(users_repository).await?;
    return Ok(users);
}
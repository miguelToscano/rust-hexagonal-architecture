use crate::domain::users::types::{CreateUserInput, User};
use crate::adapters::outbound::users_repositories::UsersRepository;

pub async fn get_by_email(email: String) -> Result<User, ()> {
    todo!();
}

pub async fn create(user_input: CreateUserInput) -> Result<User, ()> {
    let user = User::from(user_input);
    return Ok(user);
}

pub async fn get_users(users_repository: &impl UsersRepository) -> Result<Vec<User>, ()> {
    let users = users_repository.get_users().await?;
    return Ok(users);
}

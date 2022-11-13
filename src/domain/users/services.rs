use crate::domain::users::types::{CreateUserInput, User};
use crate::adapters::outbound::users_repositories::{UsersRepository, self};

pub async fn get_by_email(email: String) -> Result<User, ()> {
    todo!();
}

pub async fn create(users_repository: &impl UsersRepository, user_input: &CreateUserInput) -> Result<User, ()> {
    let user = User::from(user_input.clone());
    users_repository.create_user(user_input).await;
    return Ok(user);
}

pub async fn get_all(users_repository: &impl UsersRepository) -> Result<Vec<User>, ()> {
    let users = users_repository.get_users().await?;
    return Ok(users);
}

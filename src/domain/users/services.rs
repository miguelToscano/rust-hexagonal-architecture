use crate::adapters::outbound::users_repositories::{UsersRepository};
use crate::domain::users::types::{CreateUserInput, User};

pub async fn create(
    users_repository: &impl UsersRepository,
    user_input: &CreateUserInput,
) -> Result<User, ()> {
    let user = User::from(user_input.clone());
    users_repository.create_user(user_input).await?;
    return Ok(user);
}

pub async fn get_all(users_repository: &impl UsersRepository) -> Result<Vec<User>, ()> {
    let users = users_repository.get_users().await?;
    return Ok(users);
}

pub async fn get_by_email(users_repository: &impl UsersRepository, email: &String) -> Result<User, ()> {
    return Ok(User { email: String::from("Test email"), password_hash: String::from("password"), username: String::from("username"), created_at: String::from("date") });
}

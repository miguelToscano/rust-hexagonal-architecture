use crate::domain::users::types::{CreateUserInput, User};

pub async fn get_by_email(email: String) -> Result<User, ()> {
    todo!();
}

pub async fn create(user_input: CreateUserInput) -> Result<User, ()> {
    let user = User::from(user_input);
    return Ok(user);
}

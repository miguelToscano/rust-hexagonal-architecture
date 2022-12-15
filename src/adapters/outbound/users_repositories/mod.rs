use crate::domain::users::types::{CreateUserInput, User};
use async_trait::async_trait;

pub mod mongo_db;

#[async_trait]
pub trait UsersRepository {
    async fn get_users(&self) -> Result<Vec<User>, ()>;

    async fn create_user(&self, user: &CreateUserInput) -> Result<(), ()>;

    async fn get_user_by_emai(&self, email: String) -> Result<User, ()>;
}

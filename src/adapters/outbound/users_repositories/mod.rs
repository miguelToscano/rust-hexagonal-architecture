use crate::domain::users::types::User;
use async_trait::async_trait;

pub mod mongo_db;

#[async_trait]
pub trait UsersRepository {
    async fn get_users(&self) -> Result<Vec<User>, ()>;
}
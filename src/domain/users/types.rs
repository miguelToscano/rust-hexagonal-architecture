use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use argon2::{self};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateUserInput {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub email: String,
    pub password_hash: String,
    pub username: String,
    pub created_at: String,
}

impl User {
    pub fn new(&self, create_user_input: CreateUserInput) -> User {
        return User::from(create_user_input);
    }

    pub fn hash_password(password: String) -> String {
        return password;
    }
}

impl From<CreateUserInput> for User {
    fn from(create_user_input: CreateUserInput) -> User {
        User {
            email: create_user_input.email,
            password_hash: User::hash_password(create_user_input.password),
            username: create_user_input.username,
            created_at: Utc::now().to_string(), // e.g. `2014-11-28T12:45:59.324310806Z`
        }
    }
}

#[async_trait]
pub trait UsersRepository {
    async fn get_users(&self) -> Result<Vec<User>, ()>;

    async fn create_user(&self, user: &CreateUserInput) -> Result<(), ()>;

    async fn get_user_by_emai(&self, email: &String) -> Result<User, ()>;
}


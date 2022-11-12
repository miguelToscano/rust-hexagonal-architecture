use mongodb::Collection;
use mongodb::bson::doc;
use futures_util::TryStreamExt;
use super::UsersRepository;
use crate::domain::users::types::User;
use dotenv::dotenv;
use mongodb::Client;
use async_trait::async_trait;

#[derive(Clone)]
pub struct MongoDBUsersRepository {
    collection: Collection<User>
}

impl MongoDBUsersRepository {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match std::env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("newsletter");
        let collection: Collection<User> = db.collection("users");
        MongoDBUsersRepository { collection }
    }
}

#[async_trait]
impl UsersRepository for MongoDBUsersRepository {

    async fn get_users(&self) -> Result<Vec<User>, ()> {
        let mut cursors = self
            .collection
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of users");

        let mut users: Vec<User> = Vec::new();

        while let Some(user) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }
        
        return Ok(users);
    }
}
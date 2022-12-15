use crate::domain::users::types::{CreateUserInput, User};
use crate::ports::outbound::databases::mongo_db::UsersRepository;
use async_trait::async_trait;
use dotenv::dotenv;
use futures_util::TryStreamExt;
use mongodb::bson::doc;
use mongodb::Client;
use mongodb::Collection;

#[derive(Clone)]
pub struct MongoDBUsersRepository {
    collection: Collection<User>,
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

    async fn create_user(&self, user: &CreateUserInput) -> Result<(), ()> {
        let new_doc = User::from(user.clone());
        self.collection
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating user");

        Ok(())
    }

    async fn get_user_by_emai(&self, email: &String) -> Result<User, ()> {
        let filter = doc! {"email": format!("{email}")};
        let user = self
            .collection
            .find_one(filter, None)
            .await
            .expect("Error fetching user");

        return Ok(user.unwrap());
    }
}

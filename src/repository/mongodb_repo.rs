use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::extjson::de::Error,
    results::InsertOneResult,
    Client, Collection,
};

use crate::models::user_model::User;

#[derive(Debug,Clone)]
pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {

    pub async fn init() -> Self {

        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("test_cart");
        let col: Collection<User> = db.collection("User");

        MongoRepo { 
            col
        }

    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {

        let doc = User {
            id: None,
            name: new_user.name,
            email: new_user.email,
            pwd: new_user.pwd,
        };

        let user = self
            .col
            .insert_one(doc, None)
            .await.ok()
            .expect("Error creating user");

        Ok(user)
    }

}
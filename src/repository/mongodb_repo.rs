use std::env;
extern crate dotenv;
use actix_web::dev::Response;
use dotenv::dotenv;

use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use mongodb::{
    bson::{extjson::de::Error, doc, self},
    results::{InsertOneResult, CollectionType},
    Client, Collection,
};

use crate::models::user_model::{User, TokenClaims, LoginUserSchema};

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

    pub async fn find_by_email(&self, data: LoginUserSchema) -> Result<Option<User>, Error> {

        let user = self
            .col
            .find_one(doc! {"email": data.email}, None)
            .await.ok()
            .expect("Error finding");

        Ok(user)
    }

    fn user_informations(&self, token: &str) -> Result<Option<User>, Error> {
        
        let secret_key = "secret";

        let _var = secret_key;
        let key = _var.as_bytes();
        let _decode = decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(key),
            &Validation::new(Algorithm::HS256),
        );

        match _decode {
            Ok(decoded) => {
                match self.find_by_email(LoginUserSchema { email: decoded.claims.sub.to_string(), password: todo!() }) {
                    Ok(user) => Ok(user),
                    Err(_) => Err(())
                }
            }
            Err(_) => Err(())
        }
    }


}
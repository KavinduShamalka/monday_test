use std::env;
extern crate dotenv;
use dotenv::dotenv;

use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use mongodb::{
    bson::{extjson::de::Error, doc},
    results::InsertOneResult,
    Client, Collection,
};

use crate::models::user_model::{User, TokenClaims, Response};

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

    // pub async fn find_by_email(&self, data: String) -> Result<Option<User>, Error> {

    //     let id = data;

    //     let user = self
    //         .col
    //         .find_one( doc! {"_id" : id }, None)
    //         .await.ok()
    //         .expect("Error finding");

    //     Ok(user)
    // }

    pub async fn user_informations(&self, token: &str) -> Result<Option<User>, Response> {
        
        let secret_key = "secret".to_owned();

        let var = secret_key;
        let key = var.as_bytes();
        let decode = decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(key),
            &Validation::new(Algorithm::HS256),
        ); 

        match decode {
            Ok(decoded) => {

                println!("{:?}", decoded.claims.sub.to_owned());

                let id = decoded.claims.sub.to_string();

                let user = self
                    .col
                    .find_one( doc! {"_id" : id }, None)
                    .await.ok()
                    .expect("Error finding");

                println!("{:?}", user);
        
                Ok(user)

                // match self.find_by_email((decoded.claims.sub.to_owned()).parse().unwrap()).await {
                //     Ok(user) => {
                //         println!("{:?}", user);
                //         Ok(user)
                //     },
                //     Err(_) => Err(Response {
                //         status: false,
                //         message: "Something Wrong".to_string(),
                //     }),
                // }


            }
            Err(_) => Err(Response {
                status: false,
                message: "Invalid Token".to_string(),
            }),
        }
    }


}
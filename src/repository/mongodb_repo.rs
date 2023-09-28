use std::env;
extern crate dotenv;
use dotenv::dotenv;

use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use mongodb::{
    bson::{doc, oid::ObjectId, extjson::de::Error},
    results::{InsertOneResult, UpdateResult, DeleteResult},
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

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, String> {

        let email = self.existing_user(new_user.email.clone()).await;

        let new_email = new_user.email.clone();

        if email == new_email {
   
            Err("Email already exists".to_owned())

        } else {
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

    pub async fn existing_user(&self, user: String ) -> String {

        let email = user;

        let filter = doc! {"email": email};

        let check_email = self
            .col
            .find_one(filter, None)
            .await.ok()
            .expect("Error finding email");
        
        match check_email {
            Some(user) => user.email,
            None => "No user found".to_owned()
        }

    }


    pub async fn find_by_email(&self, email: &String, password: &String) -> Result<Option<User>, Error> {


        let user = self
            .col
            .find_one( doc! {
                "email" : email,
                "pwd": password 
            }, None)
            .await.ok()
            .expect("dadadad")
            .expect("Error finding");
        
        Ok(Some(user))
        
    }

    pub async fn user_informations(&self, token: &str) -> Result<Option<User>, Response> {
        
        let secret_key = "secret".to_owned();

        let var = secret_key;
        let key = var.as_bytes();
        let decode = decode::<TokenClaims> (
            token,
            &DecodingKey::from_secret(key),
            &Validation::new(Algorithm::HS256),
        ); 

        match decode {

            Ok(decoded) => {

                println!("object_id{:?}", decoded.claims.sub.to_owned());

                let id = decoded.claims.sub;

                let bson_id = ObjectId::parse_str(id).unwrap(); //used to convert <String> to <bjectId>

                let user = self
                    .col
                    .find_one( doc! {"_id" : bson_id }, None)
                    .await.ok()
                    .expect("Error finding");

                println!("{:?}", user);
        
                Ok(user)

            }
            Err(_) => Err(Response {
                status: false,
                message: "Invalid Token".to_string(),
            }),
        }
    }

    
    pub async fn update_user(&self, token: &str, user: User) -> Result<UpdateResult, Response> {

        let secret_key = "secret".to_owned();

        let var = secret_key;
        let key = var.as_bytes();
        let decode = decode::<TokenClaims> (
            token,
            &DecodingKey::from_secret(key),
            &Validation::new(Algorithm::HS256),
        ); 

        match decode {

            Ok(decoded) => {

                println!("object_id{:?}", decoded.claims.sub.to_owned());

                let id = decoded.claims.sub;

                let bson_id = ObjectId::parse_str(id).unwrap();

                let filter = doc! {"_id": bson_id};

                let new_doc = doc! {
                    "$set":
                        {
                            "email": user.email,
                            "name": user.name,
                            "pwd": user.pwd
                        },
                };
                let updated_doc = self
                    .col
                    .update_one(filter, new_doc, None)
                    .await
                    .ok()
                    .expect("Error updating user");
        
                Ok(updated_doc)

            }
            Err(_) => Err(Response {
                status: false,
                message: "Invalid Token".to_string(),
            }),
    }


    }



    pub async fn delete_user(&self, token: &str) -> Result<DeleteResult, Response> {

        let secret_key = "secret".to_owned();

        let var = secret_key;
        let key = var.as_bytes();
        let decode = decode::<TokenClaims> (
            token,
            &DecodingKey::from_secret(key),
            &Validation::new(Algorithm::HS256),
        ); 


        match decode {

            Ok(decoded) => {

                println!("object_id{:?}", decoded.claims.sub.to_owned());

                let id = decoded.claims.sub;

                let bson_id = ObjectId::parse_str(id).unwrap();

                let filter = doc! {"_id": bson_id};

                let delete = self
                    .col
                    .delete_one(filter, None)
                    .await
                    .ok()
                    .expect("Error deleting todos");
                
                Ok(delete)

            },
            Err(_) => Err(Response {
                status: false,
                message: "Invalid Token".to_string(),
            }),
    }


    }
}
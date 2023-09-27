use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub pwd: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: ObjectId,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub message: String,
    pub status: bool,
}

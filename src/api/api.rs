use crate::{repository::mongodb_repo::MongoRepo,
    models::user_model::{
        User,
        TokenClaims, LoginUserSchema
    } 
};


use actix_web::{
    post, 
    web::{Data, Json, self},
    HttpResponse, Responder, cookie::Cookie,
    cookie::time::Duration as ActixWebDuration, get, HttpRequest,
};

use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde_json::json;




//register user
#[post("/user")]
pub async fn create_user(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {

    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        pwd: new_user.pwd.to_owned(),
        email: new_user.email.to_owned(),
    };

    let user_detail = db.create_user(data).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }

}


#[post("/login/{id}")]
async fn login_user_handler(
    data: Json<LoginUserSchema>,
    path: web::Path<String>,
    db: Data<MongoRepo>
) -> impl Responder {

    let check = db.find_by_email(data.email);
    

    let jwt_secret = "secret";

    let id = path.into_inner();

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: id,
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(ActixWebDuration::new(60 * 60, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success", "token": token}))
}


#[get("/userInformations")]
async fn user_informations_get(_req: HttpRequest, db: Data<MongoRepo>) -> HttpResponse {
    let _auth = _req.headers().get("Authorization");
    let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
    let token = _split[1].trim();
   
    match db.user_informations(token) {
        Ok(result) => HttpResponse::Ok().json(result.unwrap()),
        Err(err) => HttpResponse::Ok().json(err),
    }
}




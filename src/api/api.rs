use crate::{repository::mongodb_repo::MongoRepo,
    models::user_model::{
        User,
        TokenClaims, LoginUserSchema
    } 
};


use actix_web::{
    post, 
    web::{Data, Json},
    HttpResponse, Responder, cookie::Cookie,
    cookie::time::Duration as ActixWebDuration, get, HttpRequest, put, delete,
};

use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde_json::json;


//User registration
#[post("/register")]
pub async fn register_user_handler(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {

    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        pwd: new_user.pwd.to_owned(),
        email: new_user.email.to_owned(),
    };

    let user_detail = db.create_user(data).await;
    match user_detail {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success", "message": "Registration Successfull"})),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }

}

//User Login
#[post("/login")]
async fn login_user_handler(data: Json<LoginUserSchema>,db: Data<MongoRepo>) -> impl Responder {

    let email = &data.email;
    let password = &data.password;

    match db.find_by_email(email, password).await.unwrap() {

        Some(user) => {  let jwt_secret = "secret".to_owned();

                        let now = Utc::now();
                        let iat = now.timestamp() as usize;

                        let id = user.id.unwrap();
                        
                        let exp = (now + Duration::minutes(60)).timestamp() as usize;
                        let claims: TokenClaims = TokenClaims {
                            sub: id.to_string(),
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
                    },
            None => {
                        return HttpResponse::BadRequest()
                        .json(json!({"status": "fail", "message": "Invalid email or password"}));
                    }
        }

}


#[get("/view")]
async fn user_informations_get(_req: HttpRequest, db: Data<MongoRepo>) -> HttpResponse {
    let _auth = _req.headers().get("Authorization");
    let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
    let token = _split[1].trim();
   
    match db.user_informations(token).await {
        Ok(result) => HttpResponse::Ok().json(json!({"result": result})),
        Err(err) => HttpResponse::Ok().json(err),
    }
}

#[put("/update")]
async fn update_user(db: Data<MongoRepo>, _req: HttpRequest, user: Json<User>) -> HttpResponse {
    let _auth = _req.headers().get("Authorization");
    let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();    
    let token = _split[1].trim();

    let user = User {
        id: None,
        name: user.name.clone(),
        email: user.email.clone(),
        pwd: user.pwd.clone()
    };

    match db.update_user(token, user).await {
        Ok(result) => HttpResponse::Ok().json(json!({"result": result})),
        Err(err) => HttpResponse::Ok().json(err),
    }
}

//Delete collection
#[delete("/delete")]
async fn delete_user(db: Data<MongoRepo>, _req: HttpRequest) -> HttpResponse {
    let _auth = _req.headers().get("Authorization");
    let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();    
    let token = _split[1].trim();

    match db.delete_user(token).await {
        Ok(result) => HttpResponse::Ok().json(json!({"result": result})),
        Err(err) => HttpResponse::Ok().json(err),
    }
}




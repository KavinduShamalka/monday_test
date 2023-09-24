use crate::{auth, repository::mongodb_repo::MongoRepo,
    models::user_model::{
        User,
        TokenClaims
    } 
};


use actix_web::{
    post, 
    web::{Data, Json, self},
    HttpResponse, Responder, cookie::Cookie,
    cookie::time::Duration as ActixWebDuration, get, HttpRequest, HttpMessage,
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
    path: web::Path<String>
) -> impl Responder {

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

#[get("/auth/logout")]
async fn logout_handler(_: auth::JwtMiddleware) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success"}))
}

// #[get("/users/me")]
// async fn get_me_handler(
//     req: HttpRequest,
//     data: web::Data<AppState>,
//     _: auth::JwtMiddleware,
// ) -> impl Responder {
//     let ext = req.extensions();
//     let user_id = ext.get::<uuid::Uuid>().unwrap();

//     let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
//         .fetch_one(&data.db)
//         .await
//         .unwrap();

//     let json_response = serde_json::json!({
//         "status":  "success",
//         "data": serde_json::json!({
//             "user": filter_user_record(&user)
//         })
//     });

//     HttpResponse::Ok().json(json_response)
// }
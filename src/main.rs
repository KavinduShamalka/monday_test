mod repository;
mod models;
mod api;
mod auth;

use actix_web::{ web::Data, middleware::Logger, get, App, HttpResponse, HttpServer, Responder};
use repository::mongodb_repo::MongoRepo;
use api::api::{login_user_handler, user_informations_get, register_user_handler};

use crate::api::api::{update_user, delete_user};


#[get("/test")]
async fn test() -> impl Responder {
    const MESSAGE: &str = "JWT Authentication in Rust using Actix-web and MongoDB";
    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    println!("Server started successfully");

    HttpServer::new(move || {

        App::new()
            .app_data(db_data.clone())//Db connection
            .wrap(Logger::default())
            .service(test)
            .service(register_user_handler)
            .service(login_user_handler)
            .service(user_informations_get)
            .service(update_user)
            .service(delete_user)
            
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}

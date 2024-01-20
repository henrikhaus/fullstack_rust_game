use std::env;

use actix_web::{web, App, HttpServer};
use serde::{Deserialize};
use web::Data;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use crate::route::Controller;
use crate::route::user::UserController;


pub mod service;
pub mod state;
pub mod route;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // Import declarations from .env file
  dotenv().ok();
  env_logger::init();

  let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let db_pool = PgPoolOptions::new()
      .max_connections(5)
      .connect(&db_url)
      .await
      .expect("FATAL: Failed to create pool");

  // Create a server, and listen on the speicified port
  HttpServer::new(move || {
    App::new()
        .app_data(Data::new(db_pool.clone())) // adds connection for each thread
        .configure(UserController::configure)
  })
      .bind(get_url_env_vars())?
      .run()
      .await
}

/// Gets env variables for the host and port property, required to run the server
fn get_url_env_vars() -> (String, u16) {
  let host = env::var("HOST").expect("FATAL: The HOST env. var must be set!");
  let port = env::var("PORT").expect("FATAL: The PORT env. var must be set!").parse::<u16>()
      .expect("FATAL: The PORT env variable must be parsable into a u16!");
  (host, port)
}

// #[get("/")]
// async fn hello() -> impl Responder {
//   HttpResponse::Ok().body("Hello world!")
// }
//
// #[get("/user/{user_id}")]
// async fn get_user_by_id(path: web::Path<u32>, data: Data<AppState>) -> impl Responder {
//   let result = data.user_repo.get(Id(path.into_inner())).await;
//
//   match result {
//     None => HttpResponse::NotFound().body("User was not found"),
//     Some(user) => HttpResponse::Ok().json(user),
//   }
// }
//
// #[post("/user/{user_id}")]
// async fn create_user(path: web::Path<u32>, user_create: web::Json<UserCreate>, data:
// Data<AppState>) -> impl Responder {
//   let user_create = user_create.into_inner();
//
//   let user = User::new(path.into_inner(), &user_create.name, user_create.age);
//
//   let result = data.user_repo.create(user).await;
//
//   match result {
//     Ok(user_resp) => return HttpResponse::Created().json(user_resp),
//     Err(err) => {
//       return match err {
//         CreateRecordError::AlreadyExists => HttpResponse::BadRequest().into(),
//         CreateRecordError::IoError => HttpResponse::InternalServerError().into(),
//       };
//     }
//   };
// }

#[derive(Deserialize, Debug)]
pub struct UserCreate {
  name: String,
  age: u16,
}

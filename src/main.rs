use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use web::Data;
use crate::db::{AppState, Repository, UserJsonRepo, User, Id, CreateRecordError};

pub mod db;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "debug");
  env_logger::init();

  HttpServer::new(|| {
    App::new()
        .service(hello)
        .service(get_user_by_id)
        .service(create_user)
        .app_data(Data::new(AppState::new()))
  })
      .bind(("127.0.0.1", 8080))?
      .run()
      .await
}

#[get("/")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

#[get("/user/{user_id}")]
async fn get_user_by_id(path: web::Path<u32>, data: Data<AppState>) -> impl Responder {
  let result = data.user_repo.get(Id(path.into_inner())).await;

  match result {
    None => HttpResponse::NotFound().body("User was not found"),
    Some(user) => HttpResponse::Ok().json(user),
  }
}

#[post("/user/{user_id}")]
async fn create_user(path: web::Path<u32>, user_create: web::Json<UserCreate>, data:
Data<AppState>) -> impl Responder {
  let user_create = user_create.into_inner();

  let user = User::new(path.into_inner(), &user_create.name, user_create.age);

  let result = data.user_repo.create(user).await;

   match result {
    Ok(user_resp) => return HttpResponse::Created().json(user_resp),
    Err(err) => {
      return match err {
        CreateRecordError::AlreadyExists => HttpResponse::BadRequest().into(),
        CreateRecordError::IoError => HttpResponse::InternalServerError().into(),
      }
    }
  };
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserCreate {
  name: String,
  age: u16,
}

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use web::Data;
use crate::db::{AppState, Repository, JsonDb, User, Id, CreateRecordError};

pub mod db;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
        .service(hello)
        .service(get_user_by_id)
        .app_data(JsonDb::new("users.json"))
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
async fn get_user_by_id<'a>(path: web::Path<u32>, data: Data<AppState<'a>>) -> impl Responder {
  let result = data.user_repo.get(Id(path.into_inner())).await;

  match result {
    None => HttpResponse::NotFound().body("User was not found"),
    Some(user) => HttpResponse::Ok().json(user),
  }
}

#[post("/user/{user_id}")]
async fn create_user<'a>(path: web::Path<u32>, user_create: web::Json<UserCreate>, data:
Data<AppState<'a>>) -> impl Responder {
  let user_create = user_create.into_inner();

  let user = User::new(path.into_inner(), &user_create.name, user_create.age);

  let result = data.user_repo.create(user).await;

  return match result {
    Ok(_) => HttpResponse::Created(),
    Err(err) => {
      match err {
        CreateRecordError::AlreadyExists => HttpResponse::BadRequest(),
        CreateRecordError::IoError => HttpResponse::InternalServerError(),
      }
    }
  };
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserCreate {
  name: String,
  age: u16,
}

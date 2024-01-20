use std::str::FromStr;
use actix_web::{HttpResponse, Responder, web};
use actix_web::web::ServiceConfig;
use sqlx::{Error, PgPool};
use uuid::Uuid;
use crate::route::Controller;
use crate::service::db::postgres::user::UserPgRepo;
use crate::service::db::repo::{NotFound, Repository, RepositoryError};


pub struct UserController;

const PATH: &str = "/user";
const PATH_ID: &str = "/user/{user_id}";

impl Controller for UserController {
  fn configure(cfg: &mut ServiceConfig) {
    cfg.route(PATH, web::get().to(list_users))
        .route(PATH, web::post().to(create_user))
        .route(PATH_ID, web::get().to(get_user))
        .route(PATH_ID, web::put().to(update_user))
        .route(PATH_ID, web::delete().to(delete_user));
  }
}

async fn list_users(pool: web::Data<PgPool>) -> impl Responder {
  let user_repo = UserPgRepo::new(pool.get_ref());

  let users = user_repo.get_all().await;

  if let Ok(users) = users {
    return HttpResponse::Ok().json(users);
  } else if let Err(_err) = users {
    return HttpResponse::InternalServerError().finish();
  }

  unreachable!()
}

async fn get_user(pool: web::Data<PgPool>, user_id: web::Path<String>) -> impl Responder {
  let user_id = Uuid::from_str(&user_id.into_inner());
  if let Err(_) = user_id {
    return HttpResponse::BadRequest().body("Invalid user_id format. Should be a UUID v4 string.");
  }
  let user_id = user_id.unwrap();

  let user_repo = UserPgRepo::new(pool.get_ref());

  let user = user_repo.get_by_id(user_id).await;

  match user {
    Ok(user) => {
      HttpResponse::Ok().json(user)
    }
    Err(err) => {
      match err {
        RepositoryError::Action(a) => {
          HttpResponse::NotFound().finish()
        }
        RepositoryError::Client(c) => {
          HttpResponse::InternalServerError().finish()
        }
      }
    }
  }
}

async fn create_user() -> impl Responder {
  HttpResponse::Ok()
}

async fn update_user() -> impl Responder {
  HttpResponse::Ok()
}

async fn delete_user() -> impl Responder {
  HttpResponse::Ok()
}

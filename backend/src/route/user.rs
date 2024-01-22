use std::str::FromStr;
use actix_web::{HttpResponse, Resource, Responder, Scope, web};
use actix_web::web::ServiceConfig;
use serde::Deserialize;
use sqlx::{Error, PgPool};
use uuid::Uuid;
use crate::route::Controller;
use crate::service::db::models::user::User;
use crate::service::db::postgres::user::{UserPgRepo};
use crate::service::db::repo::{AlreadyExists, Repository, RepositoryError};


pub struct UserController;

const PATH: &str = "/user";
const PATH_ID: &str = "/user/{user_id}";

/// Creates a scoped resource for all user endpoints,
/// to be used in routing
///
/// #### users
/// - GET /user
/// - POST /user
///
/// #### users_detail
/// - GET /user/{user_id}
/// - PUT /user/{user_id}
/// - DELETE /user/{user_id}
pub fn user_scope() -> Scope {
  web::scope("/user")
      .service(web::resource("")
          .name("users")
          .get(list_users)
          .post(create_user))
      .service(web::resource("/{user_id}")
          .name("user_detail")
          .get(get_user)
          .put(update_user)
          .delete(delete_user))
}

async fn list_users(pool: web::Data<PgPool>) -> impl Responder {
  let user_repo = UserPgRepo::new(pool.get_ref());

  let users = user_repo.get_all().await;

  match users {
    Ok(users) => HttpResponse::Ok().json(users),
    Err(_err) => HttpResponse::InternalServerError().finish(),
  }
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
        RepositoryError::Action(_a) => {
          HttpResponse::NotFound().finish()
        }
        RepositoryError::Client(_c) => {
          HttpResponse::InternalServerError().finish()
        }
      }
    }
  }
}


#[derive(Deserialize)]
struct PostUserReqBody {
  username: String,
}

async fn create_user(body: web::Json<PostUserReqBody>, pool: web::Data<PgPool>) -> impl Responder {
  let username = &body.username;
  let user = User::new(username, 0);

  let result = UserPgRepo::new(pool.get_ref()).create(user).await;

  match result {
    Ok(user) => {
      HttpResponse::Created().json(user)
    }
    Err(err) => {
      match err {
        RepositoryError::Action(_) => {
          HttpResponse::Conflict().body("User with that username already exists")
        }
        RepositoryError::Client(_) => {
          HttpResponse::InternalServerError().finish()
        }
      }
    }
  }
}

async fn update_user() -> impl Responder {
  HttpResponse::Ok()
}

async fn delete_user() -> impl Responder {
  HttpResponse::Ok()
}

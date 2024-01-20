use actix_web::{HttpResponse, Responder, web};
use actix_web::web::ServiceConfig;
use crate::route::Controller;
use crate::service::db::repo::Repository;

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

async fn list_users() -> impl Responder {
  // let repo = UserRedisRepo::new(redis.into_inner());
  // let users = repo.get_all().await;
  //
  // if let Some(u) = users {
  //   return HttpResponse::Ok().json(u);
  // }
  //
  // return HttpResponse::NoContent().body("");
  HttpResponse::Ok()
}

async fn get_user() -> impl Responder {
  HttpResponse::Ok()
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

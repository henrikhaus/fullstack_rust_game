use std::fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
        .service(hello)
        .service(echo)
        .service(get_user_by_id)
        .service(create_user)
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
async fn get_user_by_id(path: web::Path<String>) -> impl Responder {
  let user_id = path.into_inner();
  let users = read_db();
  dbg!(&users);
  let user = users.iter().find(|u| {
    u.name == user_id
  });

  return match user {
    None => { HttpResponse::NotFound().body("The user was not found") }
    Some(u) => { HttpResponse::Ok().json(u) }
  };
}

#[post("/user/{user_id}")]
async fn create_user(path: web::Path<u32>, user_create: web::Json<UserCreate>) -> impl
Responder {
  let mut users = read_db();
  let user_create = user_create.into_inner();
  let user = User {
    id: path.into_inner(),
    age: user_create.age.to_owned(),
    name: user_create.name.to_owned(),
  };

  users.push(user);
  write_db(users);
  HttpResponse::Created()
}

pub fn write_db(users: Vec<User>) {
  let value = serde_json::to_string(&users).expect("Should be fine");
  fs::write("users.json", value);
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}


pub fn read_db() -> Vec<User> {
  let contents = fs::read_to_string("users.json")
      .expect("Should have been able to read the file");
  serde_json::from_str(&contents).expect("Should be valid JSON")
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
  id: u32,
  name: String,
  age: u16,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserCreate {
  name: String,
  age: u16,
}

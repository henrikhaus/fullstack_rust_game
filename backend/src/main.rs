use std::env;

use actix_web::{web, App, HttpServer};

use crate::route::user::{user_scope, UserController};
use crate::route::Controller;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use web::Data;

pub mod route;
pub mod service;

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

    // Create a server, and listen on the specified port
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db_pool.clone())) // adds connection for each thread
            .service(user_scope())
    })
    .bind(get_url_env_vars())?
    .run()
    .await
}

/// Gets env variables for the host and port property, required to run the server
fn get_url_env_vars() -> (String, u16) {
    let host = env::var("HOST").expect("FATAL: The HOST env. var must be set!");
    let port = env::var("PORT")
        .expect("FATAL: The PORT env. var must be set!")
        .parse::<u16>()
        .expect("FATAL: The PORT env variable must be parsable into a u16!");
    (host, port)
}

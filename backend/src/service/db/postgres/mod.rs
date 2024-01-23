use crate::service::db::repo::Repository;
use serde::Serialize;
use sqlx::postgres::PgRow;
use sqlx::FromRow;

pub mod user;

pub trait PgRepo<'pool>: Repository {
   fn new(pool: &'pool sqlx::PgPool) -> Self;
}

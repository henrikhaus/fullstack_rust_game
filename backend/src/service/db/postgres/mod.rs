use crate::service::db::repo::Repository;
use serde::Serialize;
use sqlx::{FromRow, Row};
use sqlx::postgres::PgRow;

pub mod user;
pub mod poker_game;


pub trait PgRepo<'a, T>: Repository<'a, T, sqlx::Error>
where
    T: FromRow<'a, PgRow> + Serialize,
{
    fn new(pool: &'a sqlx::PgPool) -> Self;
}

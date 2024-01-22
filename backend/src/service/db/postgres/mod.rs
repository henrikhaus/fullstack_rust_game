use crate::service::db::repo::Repository;
use serde::Serialize;
use sqlx::FromRow;

pub mod user;

pub trait PgRepo<'a, T>: Repository<'a, T, sqlx::Error>
where
    T: FromRow<'a, T> + Serialize,
{
    fn new(pool: &'a sqlx::PgPool) -> Self;
}

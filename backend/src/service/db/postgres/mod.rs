use crate::service::db::repo::Repository;




pub mod user;

pub trait PgRepo<'pool>: Repository {
   fn new(pool: &'pool sqlx::PgPool) -> Self;
}

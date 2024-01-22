use serde::Serialize;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;
use crate::service::db::postgres::PgRepo;
use crate::service::db::repo::{AlreadyExists, NotFound, Repository, RepositoryError};

#[derive(Serialize, FromRow)]
pub struct PokerGame {
  pub(crate) player: Uuid,
  pub(crate) value: i32,
}


pub struct PokerGamePgRepo<'a> {
  pool: &'a PgPool,
}


impl<'a> Repository<'a, PokerGame, sqlx::Error> for PokerGamePgRepo<'a> {
  async fn get_all(&self) -> Result<Vec<PokerGame>, RepositoryError<(), sqlx::Error>> {
    todo!()
  }

  async fn get_by_id(&self, id: Uuid) -> Result<PokerGame, RepositoryError<NotFound, sqlx::Error>> {
    todo!()
  }

  async fn create(&self, value: PokerGame) -> Result<PokerGame, RepositoryError<AlreadyExists,
    sqlx::Error>> {
    todo!()
  }

  async fn update(&self, value: PokerGame) -> Result<PokerGame, RepositoryError<NotFound,
    sqlx::Error>> {
    todo!()
  }

  async fn delete(&self, id: Uuid) -> Result<(), RepositoryError<NotFound, sqlx::Error>> {
    todo!()
  }
}

impl<'a> PgRepo<'a, PokerGame> for PokerGamePgRepo<'a> {
  fn new(pool: &'a PgPool) -> Self {
    Self {
      pool
    }
  }
}

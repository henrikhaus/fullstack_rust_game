use sqlx::types::Uuid;
use crate::service::db::models::user::User;
use crate::service::db::postgres::PgRepo;
use crate::service::db::repo::{AlreadyExists, NotFound, Repository, RepositoryError};

pub struct UserPgRepo<'a> {
  pool: &'a sqlx::PgPool,
}

impl<'a> PgRepo<'a, User> for UserPgRepo<'a> {
  fn new(pool: &'a sqlx::PgPool) -> Self {
    Self {
      pool
    }
  }
}

impl<'a> Repository<'a, User, sqlx::Error> for UserPgRepo<'a> {
  async fn get_all(&self) -> Result<Vec<User>, RepositoryError<(), sqlx::Error>> {
    let users = sqlx::query_as!(User, "SELECT * FROM \"user\"")
        .fetch_all(self.pool).await;

    match users {
      Ok(users) => Ok(users),
      Err(err) => Err(RepositoryError::Client(err.into()))
    }
  }

  async fn get_by_id(&self, _id: Uuid) -> Result<User, RepositoryError<NotFound, sqlx::Error>> {
    let user = sqlx::query_as!(
      User, "SELECT * FROM \"user\" WHERE id = $1", _id)
        .fetch_optional(self.pool).await;

    match user {
      Ok(maybe_user) => {
        match maybe_user {
          None => Err(RepositoryError::Action(NotFound)),
          Some(user) => Ok(user),
        }
      }
      Err(err) => Err(RepositoryError::Client(err)),
    }
  }

  async fn create(&self, value: User) -> Result<User, RepositoryError<AlreadyExists, sqlx::Error>> {
    let username = value.username;
    let coins = value.coins;

    let result = sqlx::query_as!(User,
      "INSERT INTO \"user\" (username, coins)\
      VALUES ($1, $2)\
      RETURNING id, username, coins",
      username, coins).fetch_one(self.pool).await;

    match result {
      Ok(user) => Ok(user),
      Err(err) => Err(RepositoryError::Client(err)),
    }
  }

  async fn update(&self, _value: User) -> Result<User, RepositoryError<NotFound, sqlx::Error>> {
    todo!()
  }

  async fn delete(&self, _id: Uuid) -> Result<(), RepositoryError<NotFound, sqlx::Error>> {
    todo!()
  }
}

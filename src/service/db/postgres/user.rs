
use sqlx::{PgPool};
use sqlx::types::Uuid;
use crate::service::db::models::user::User;
use crate::service::db::repo::{AlreadyExists, NotFound, Repository, RepositoryError};

struct UserPgRepo<'a> {
  pool: &'a PgPool,
}

const TABLE_NAME: &str = "user";


impl<'a> Repository<'a, User, sqlx::Error> for UserPgRepo<'a> {
  async fn get_all(&self) -> Result<Vec<User>, RepositoryError<(), sqlx::Error>> {
    let users = sqlx::query_as!(User, "SELECT * FROM \"user\"")
        .fetch_all(self.pool).await;

    match users {
      Ok(users) => Ok(users),
      Err(err) => {
        return Err(RepositoryError::Client(err.into()));
      }
    }
  }

  async fn get_by_id(&self, _id: Uuid) -> Result<User, RepositoryError<NotFound, sqlx::Error>> {
    todo!()
  }

  async fn create(&self, _value: User) -> Result<User, RepositoryError<AlreadyExists, sqlx::Error>> {
    todo!()
  }

  async fn update(&self, _value: User) -> Result<User, RepositoryError<NotFound, sqlx::Error>> {
    todo!()
  }

  async fn delete(&self, _id: Uuid) -> Result<(), RepositoryError<NotFound, sqlx::Error>> {
    todo!()
  }
}

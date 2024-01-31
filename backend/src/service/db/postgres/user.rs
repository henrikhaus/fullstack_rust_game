use crate::service::db::models::user::User;
use crate::service::db::postgres::{IntoRepoResult, PgRepo};
use crate::service::db::repo::{AlreadyExists, NotFound, Repository, RepositoryError};
use sqlx::types::Uuid;
use sqlx::Postgres;

pub struct UserPgRepo<'pool> {
    pool: &'pool sqlx::PgPool,
}

impl<'pool> PgRepo<'pool> for UserPgRepo<'pool> {
    fn new(pool: &'pool sqlx::PgPool) -> Self {
        Self { pool }
    }
}

impl<'pool> Repository for UserPgRepo<'pool> {
    type Data = User;
    type ClientError = sqlx::Error;

    async fn get_all(&self) -> Result<Vec<User>, RepositoryError<(), sqlx::Error>> {
        let users = sqlx::query_as::<Postgres, User>("SELECT * FROM \"user\"")
            .fetch_all(self.pool)
            .await;

        match users {
            Ok(users) => Ok(users),
            Err(err) => Err(RepositoryError::Client(err.into())),
        }
    }

    async fn get_by_id(&self, hello: Uuid) -> Result<User, RepositoryError<NotFound, sqlx::Error>> {
        let user = sqlx::query_as::<Postgres, User>("SELECT * FROM \"user\" WHERE id = $1")
            .bind(hello)
            .fetch_optional(self.pool)
            .await;

        user.into_repo_result()
    }

    async fn create(
        &self,
        value: User,
    ) -> Result<User, RepositoryError<AlreadyExists, sqlx::Error>> {
        let result = sqlx::query_as::<Postgres, User>(
            "INSERT INTO \"user\" (username, coins)\
                VALUES ($1, $2)\
                RETURNING id, username, coins",
        )
        .bind(value.username())
        .bind(value.chips())
        .fetch_one(self.pool)
        .await;

        match result {
            Ok(user) => Ok(user),
            Err(err) => Err(RepositoryError::Client(err)),
        }
    }

    async fn update(&self, value: User) -> Result<User, RepositoryError<NotFound, sqlx::Error>> {
        let user = sqlx::query_as::<Postgres, User>(
            "UPDATE \"user\"
                SET username = $1, coins = $2
                WHERE id = $3
                ",
        )
        .bind(value.username())
        .bind(value.chips() as i64)
        .bind(value.id())
        .fetch_optional(self.pool)
        .await;

        match user {
            Ok(maybe_user) => match maybe_user {
                Some(user) => Ok(user),
                None => Err(RepositoryError::Action(NotFound)),
            },
            Err(err) => Err(RepositoryError::Client(err)),
        }
    }

    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError<NotFound, sqlx::Error>> {
        let result = sqlx::query(
            "DELETE \"user\"
            WHERE id = $1
            ",
        )
        .bind(id)
        .execute(self.pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(RepositoryError::Client(err)),
        }
    }
}

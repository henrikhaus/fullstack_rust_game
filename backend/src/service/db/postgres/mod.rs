use sqlx::Error;
use crate::service::db::models::user::User;
use crate::service::db::repo::{NotFound, Repository, RepositoryError};

pub mod user;

pub trait PgRepo<'pool>: Repository<ClientError = sqlx::Error> {
    fn new(pool: &'pool sqlx::PgPool) -> Self;
}

pub trait IntoRepoResult<T> {
    fn into_repo_result (self) -> Result<T, RepositoryError<NotFound, sqlx::Error>>;
}

impl<T> IntoRepoResult<T> for Result<Option<T>, sqlx::Error> {
    fn into_repo_result (self) -> Result<T, RepositoryError<NotFound, sqlx::Error>> {
        match self {
            Ok(maybe_data) => match maybe_data {
                None => Err(RepositoryError::Action(NotFound)),
                Some(data) => Ok(data),
            },
            Err(err) => Err(RepositoryError::Client(err)),
        }
    }
}
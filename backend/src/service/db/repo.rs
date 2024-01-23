use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::body::BoxBody;
use serde::Serialize;
use sqlx::types::Uuid;

/// The record was not found, making the action invalid
pub struct NotFound;

/// The record already existed, making the action invalid
pub struct AlreadyExists;

pub enum RepositoryError<T, C> {
    Action(T),
    Client(C),
}

#[derive(Serialize)]
struct ErrorResponse {
  message: &'static str
}

impl<T, C> Responder for RepositoryError<T, C> {
  type Body = BoxBody;

  fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
     match self {
      RepositoryError::Action(_) => {
        HttpResponse::BadRequest().json(ErrorResponse {
          message: "Bad Request"
        })
      }
      RepositoryError::Client(_) => {
        HttpResponse::InternalServerError().json(ErrorResponse {
          message: "Internal Server Error"
        })
      }
    }
  }
}


pub trait Repository {
    type Data;
    type ClientError;

    /// Retrieves all records
    async fn get_all(&self) -> Result<Vec<Self::Data>, RepositoryError<(), Self::ClientError>>;

    /// Retrieves a record
    async fn get_by_id(
        &self,
        id: Uuid,
    ) -> Result<Self::Data, RepositoryError<NotFound, Self::ClientError>>;

    /// Creates a new record
    async fn create(
        &self,
        value: Self::Data,
    ) -> Result<Self::Data, RepositoryError<AlreadyExists, Self::ClientError>>;

    /// Updates a record and returns the updated record
    async fn update(
        &self,
        value: Self::Data,
    ) -> Result<Self::Data, RepositoryError<NotFound, Self::ClientError>>;

    /// Deletes a record
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError<NotFound, Self::ClientError>>;
}

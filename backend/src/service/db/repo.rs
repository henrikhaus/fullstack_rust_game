
use sqlx::types::Uuid;

/// The record was not found, making the action invalid
pub struct NotFound;

/// The record already existed, making the action invalid
pub struct AlreadyExists;

pub enum RepositoryError<T, C> {
  Action(T),
  Client(C),
}


pub trait Repository<'a, T, C> {
  /// Retrieves all records
  async fn get_all(&self) -> Result<Vec<T>, RepositoryError<(), C>>;

  /// Retrieves a record
  async fn get_by_id(&self, id: Uuid) -> Result<T, RepositoryError<NotFound, C>>;

  /// Creates a new record
  async fn create(&self, value: T) -> Result<T, RepositoryError<AlreadyExists, C>>;

  /// Updates a record and returns the updated record
  async fn update(&self, value: T) -> Result<T, RepositoryError<NotFound, C>>;

  /// Deletes a record
  async fn delete(&self, id: Uuid) -> Result<(), RepositoryError<NotFound, C>>;
}

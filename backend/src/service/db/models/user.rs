use serde::Serialize;
use sqlx::{FromRow};
use sqlx::types::Uuid;

#[derive(FromRow, Debug, Clone, Serialize)]
pub struct User {
  pub id: Uuid,
  pub username: String,
  pub coins: i64,
}

impl User {
  pub fn new(username: &str, coins: i64) -> Self {
    Self {
      id: Uuid::new_v4(),
      username: username.to_string(),
      coins,
    }
  }

  pub fn id(&self) -> Uuid { self.id }
  pub fn name(&self) -> &str { &self.username }
  pub fn coins(&self) -> i64 { self.coins }
}

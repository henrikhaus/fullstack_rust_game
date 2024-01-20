use serde::{Deserialize, Serialize};
use sqlx::{FromRow};
use sqlx::types::Uuid;

#[derive(FromRow, Debug, Clone)]
pub struct User {
  pub id: Uuid,
  pub username: String,
  pub coins: i64,
}

impl User {
  pub fn new(id: Uuid, name: &str, coins: i64) -> Self {
    Self {
      id,
      username: name.to_string(),
      coins,
    }
  }

  pub fn id(&self) -> Uuid { self.id }
  pub fn name(&self) -> &str { &self.username }
  pub fn coins(&self) -> i64 { self.coins }
}

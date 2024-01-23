use serde::Serialize;
use sqlx::types::Uuid;
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub coins: i64,
}

impl User {
    pub fn new(username: String, coins: u64) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            coins: coins.try_into().unwrap_or(i64::MAX),
        }
    }

    /// The row id in the database
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// A unique username for this profile
    pub fn username(&self) -> &str {
        &self.username
    }

    /// The number of coins this user has
    pub fn coins(&self) -> u64 {
        self.coins as u64
    }
}

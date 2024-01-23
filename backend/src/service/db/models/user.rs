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
    pub fn new(username: String, coins: i64) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            coins,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.username
    }
    pub fn coins(&self) -> i64 {
        self.coins
    }
}

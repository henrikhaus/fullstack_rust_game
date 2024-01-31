use serde::Serialize;
use sqlx::types::Uuid;
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone, Serialize)]
pub struct User {
    id: Uuid,
    username: String,
    chips: i64,
    premium_chips: i64,
    xp: i64,
    rank: i32,
}

impl User {
    pub fn new(username: String, chips: i64, premium_chips: i64, xp: i64, rank: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            chips,
            premium_chips,
            xp,
            rank,
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
    pub fn chips(&self) -> i64 {
        self.chips
    }

    /// The number of premium chips this user has
    pub fn premium_chips(&self) -> i64 {
        self.premium_chips
    }

    /// The amount of xp this user has
    pub fn xp(&self) -> i64 {
        self.xp
    }

    /// The rank this user has
    pub fn rank(&self) -> i32 {
        self.rank
    }
}

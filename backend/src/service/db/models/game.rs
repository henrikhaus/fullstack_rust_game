use serde::Serialize;
use sqlx::types::Uuid;
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone, Serialize)]
pub struct Game {
    id: Uuid,
    user_id: Uuid,
    duration: i32,
    folds: i32,
    raises: i32,
    wins: i32,
    losses: i32,
}

impl Game {
    pub fn new(user_id: Uuid, duration: i32, folds: i32, raises: i32, wins: i32, losses: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            duration,
            folds,
            raises,
            wins,
            losses
        }
    }

    /// The row id in the database
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// The user id for this game
    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }

    /// The length of the game in minutes
    pub fn duration(&self) -> i32 {
        self.duration
    }

    /// The number of folds the user had that game
    pub fn folds(&self) -> i32 {
        self.folds
    }

    /// The number of raises the user had that game
    pub fn raises(&self) -> i32 {
        self.raises
    }

    /// The number of wins the user had that game
    pub fn wins(&self) -> i32 {
        self.wins
    }

    /// The number of losses the user had that game
    pub fn losses(&self) -> i32 {
        self.losses
    }
}

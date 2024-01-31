use serde::Serialize;
use sqlx::types::Uuid;
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone, Serialize)]
pub struct Friendship {
    id: Uuid,
    requester: Uuid,
    responder: Uuid,
    status: String,
}

pub enum Status {
    Active,
    Pending,
    Blocked,
}

impl Friendship {
    pub fn new(requester: Uuid, responder: Uuid, status: Status) -> Self {
        Self {
            id: Uuid::new_v4(),
            requester,
            responder,
            status: status.into()
        }
    }

    /// The row id in the database
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// The requesting user
    pub fn requester(&self) -> Uuid {
        self.requester
    }

    /// The responding user
    pub fn responder(&self) -> Uuid {
        self.responder
    }

    /// The status of this friendship
    pub fn status(&self) -> Status {
        self.status.as_ref().try_into().expect("Invalid enum value")
    }
}

struct InvalidVariant;

impl TryFrom<&str> for Status {
    type Error = InvalidVariant;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "active" => Ok(Status::Active),
            "pending" => Ok(Status::Pending),
            "blocked" => Ok(Status::Blocked),
            _ => Err(Self::Error)
        }
    }
}

impl From<Status> for &str {
    fn from(value: Status) -> &'static str {
        match value {
            Status::Active => "active",
            Status::Pending => "pending",
            Status::Blocked => "blocked"
        }
    }
}
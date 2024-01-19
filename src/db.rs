use std::fs;
use serde::{Deserialize, Serialize};

pub struct UserId(pub u32);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
  id: u32,
  name: String,
  age: u16,
}

pub enum SetRecordError {
  InvalidInput,
  IoError,
}

pub enum SetRecordResponse {
  Created,
  Updated,
}

pub enum DeleteRecordError {
  DoesNotExist
}

pub trait Db {
  /// Retrieves a user from the database
  async fn get_user(&self, user_id: UserId) -> Option<User>;

  /// Sets a user in the database
  async fn set_user(&self, user: User) -> Result<SetRecordResponse, SetRecordError>;

  /// Deletes a user from the database
  async fn delete_user(&self, user_id: UserId) -> Result<(), DeleteRecordError>;

  /// Retrieves all users from the database
  async fn get_all_users(&self) -> Vec<User>;
}


pub struct JsonDb {
  file_name: String,
}


impl JsonDb {
  pub fn new(file_name: &str) -> Self {
    if !file_name.ends_with(".json") {
      panic!("Should be a JSON file!");
    }

    Self {
      file_name: file_name.to_string()
    }
  }
}



pub struct AppState<T> where T : Db {
  pub db: T,
}



impl Db for JsonDb {
  async fn get_user(&self, user_id: UserId) -> Option<User> {
    let users = self.get_all_users().await;

    match users.iter().find(|&u| u.id == user_id.0) {
      None => None,
      Some(u) => Some(u.clone()),
    }
  }

  async fn set_user(&self, user: User) -> Result<SetRecordResponse, SetRecordError> {
    let mut users: Vec<User> = self.get_all_users().await;
    let response: SetRecordResponse;

    if let Some(u) = users.iter_mut().find(|u| u.id == user.id) {
      *u = user;
      response = SetRecordResponse::Updated
    } else {
      users.push(user);
      response = SetRecordResponse::Created
    }


    let value = serde_json::to_string(&users).expect("Should be fine");
    match fs::write(&self.file_name, value) {
      Ok(_) => Ok(response),
      Err(_) => Err(SetRecordError::IoError)
    }
  }

  async fn delete_user(&self, user_id: UserId) -> Result<(), DeleteRecordError> {
    todo!()
  }

  async fn get_all_users(&self) -> Vec<User> {
    todo!()
  }
}

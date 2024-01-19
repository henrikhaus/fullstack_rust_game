use std::fmt::Debug;
use std::fs;


use serde::{Deserialize, Serialize};

pub struct Id(pub u32);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
  id: u32,
  name: String,
  age: u16,
}

impl User {
  pub fn new(id: u32, name: &str, age: u16) -> Self {
    Self {
      id,
      name: name.to_string(),
      age,
    }
  }

  pub fn id(&self) -> u32 { self.id }
  pub fn name(&self) -> String { self.name.to_string() }
  pub fn age(&self) -> u16 { self.age }
}

pub enum UpdateRecordError {
  NotFound,
  IoError,
}

pub enum DeleteRecordError {
  DoesNotExist
}

pub enum CreateRecordError {
  AlreadyExists,
  IoError,
}

pub trait Repository<'a, T> where T: Deserialize<'a> + Serialize + Debug + Clone {
  /// Retrieves a user from the database
  async fn get(&self, id: Id) -> Option<T>;

  /// Sets a user in the database
  async fn create(&self, user: T) -> Result<T, CreateRecordError>;
  async fn update(&self, user: T) -> Result<(), UpdateRecordError>;

  /// Deletes a user from the database
  async fn delete(&self, id: Id) -> Result<(), DeleteRecordError>;

  /// Retrieves all users from the database
  async fn get_all(&self) -> Vec<T>;
}


pub struct UserJsonRepo {
  file_name: String,
}


impl UserJsonRepo {
  pub fn new(file_name: &str) -> Self {
    if !file_name.ends_with(".json") {
      panic!("Should be a JSON file!");
    }

    Self {
      file_name: file_name.to_string()
    }
  }
}


pub struct AppState {
  pub user_repo: UserJsonRepo,
}

impl AppState {
  pub fn new() -> Self {
    AppState {
      user_repo: UserJsonRepo::new("users.json")
    }
  }
}


impl<'a> Repository<'a, User> for UserJsonRepo {
  async fn get(&self, user_id: Id) -> Option<User> {
    let users = self.get_all().await;

    match users.iter().find(|&u| u.id == user_id.0) {
      None => None,
      Some(u) => Some(u.clone()),
    }
  }

  async fn create(&self, user: User) -> Result<User, CreateRecordError> {
    let mut users: Vec<User> = self.get_all().await;

    if let None = users.iter_mut().find(|u| u.id == user.id) {
      users.push(user.clone());
    } else {
      return Err(CreateRecordError::AlreadyExists);
    }


    let value = serde_json::to_string(&users).expect("Should be fine");
    if let Err(_) = fs::write(&self.file_name, value) {
      return Err(CreateRecordError::IoError);
    }

    Ok(user)
  }

  async fn update(&self, user: User) -> Result<(), UpdateRecordError> {
    let mut users: Vec<User> = self.get_all().await;

    if let Some(u) = users.iter_mut().find(|u| u.id == user.id) {
      *u = user;
    } else {
      return Err(UpdateRecordError::NotFound);
    }


    let value = serde_json::to_string(&users).expect("Should be fine");
    if let Err(_) = fs::write(&self.file_name, value) {
      return Err(UpdateRecordError::IoError);
    }

    Ok(())
  }

  async fn delete(&self, _user_id: Id) -> Result<(), DeleteRecordError> {
    todo!()
  }

  async fn get_all(&self) -> Vec<User> {
    let file = fs::read_to_string(&self.file_name).expect("Should be good :)");
    let json: Vec<User> = serde_json::from_str(&file).expect("Should be fine again :)");
    json
  }
}

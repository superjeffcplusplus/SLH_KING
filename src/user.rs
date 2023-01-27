use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum Role {
  STUDENT,
  PROF,
  NONE,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum Resource {
  GRADES,
  USERS,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum Action {
  RW,
  R,
  NONE,
}

impl Display for Action {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Action::RW => write!(f, "RW"),
      Action::R =>  write!(f, "R"),
      Action::NONE =>  write!(f, "NONE"),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
  pub name: String,
  pub pwd_hash: String,
  pub role: Role
}

impl User {
  pub fn get_authorized_resource_descriptor(&self) -> String {
    let prefix = "grades_".to_string();
    let descriptor = match self.role {
      Role::STUDENT => prefix + self.name.as_str(),
      Role::PROF => prefix + "*",
      Role::NONE => "".to_string(),
    };
    descriptor
  }
}
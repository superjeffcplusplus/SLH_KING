use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum Role {
  STUDENT,
  PROF,
  NONE,
}

impl Display for Role {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Role::STUDENT => write!(f, "Student"),
      Role::PROF => write!(f, "Prof"),
      Role::NONE => write!(f, "NONE"),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum Resource {
  GRADES,
  USERS,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum Action {
  Write,
  Read,
  NONE,
}

impl Display for Action {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Action::Write => write!(f, "Write"),
      Action::Read =>  write!(f, "Read"),
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

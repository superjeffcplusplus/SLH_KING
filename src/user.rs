use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum Role {
  STUDENT,
  PROF,
  NONE
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
  pub name: String,
  pub pwd_hash: String,
  pub role: Role
}
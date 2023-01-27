use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Role {
  STUDENT,
  PROF,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
  pub name: String,
  pub pwd_hash: String,
  pub role: Role
}
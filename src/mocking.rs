use crate::hashing::new_hash_from_pwd;
use crate::user::{Role, User};
use crate::USERS_DATABASE;

pub fn add_users(user_db: &USERS_DATABASE) {
  let mut map = user_db.lock().unwrap();
  let users = ["prof1", "prof2", "prof3"];
  for u in users {
    let pwd_hash = new_hash_from_pwd("1234")
      .expect("Unable to create mock data");
    let usr_obj = User {
      name: u.to_string(),
      pwd_hash: pwd_hash.to_string(),
      role: Role::PROF,
    };
    map.insert(u.to_string(), usr_obj);
  }
  let users = ["alice", "bob", "charlie", "jeff", "student1","student2"];
  for u in users {
    let pwd_hash = new_hash_from_pwd("1234")
      .expect("Unable to create mock data");
    let usr_obj = User {
      name: u.to_string(),
      pwd_hash: pwd_hash.to_string(),
      role: Role::STUDENT,
    };
    map.insert(u.to_string(), usr_obj);
  }
}
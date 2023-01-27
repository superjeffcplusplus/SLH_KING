use std::sync::Mutex;
use lazy_static::lazy_static;


pub struct State {
  pub authenticated: bool,
  pub grades_res_prefix: String,
}

lazy_static! {
  pub static ref STATE: Mutex<State> = Mutex::new(State {
    authenticated: false,
    grades_res_prefix: "grades_".to_string(),
  });
}
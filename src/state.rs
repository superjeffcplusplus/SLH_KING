use crate::user::User;

pub struct State {
  pub user: Option<User>,
  pub authenticated: bool,
}
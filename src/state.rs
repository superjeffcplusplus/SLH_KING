use crate::access_control::AccessControl;
use crate::user::User;

pub struct State {
  pub user: Option<User>,
  pub authenticated: bool,
  pub access_control: Option<AccessControl>,
}
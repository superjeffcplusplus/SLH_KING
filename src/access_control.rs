use casbin::prelude::*;
use lazy_static::lazy_static;
use std::sync::Mutex;
use log::error;
use serde::de::Error;

const CONFIG: &str = "accessControl/access_control.conf";
const POLICY: &str = "accessControl/policies.csv";


pub struct AccessControl {
  enforcer: Enforcer,
}

// lazy_static! {
//   static ref ENFORCER: Mutex<Enforcer> = Mutex::new(
//     Enforcer::new(CONFIG, OBJ_POLICY)
//   );
// }

impl AccessControl {
  pub async fn new() -> Result<AccessControl> {
    let enforcer = Enforcer::new(CONFIG, POLICY).await?;
    Ok(AccessControl { enforcer })
  }

  /// Centralized access control mechanism
  pub fn auth(&self, subject: &str, resource: &str, action: &str) -> bool {
    if let Ok(authorized) = self.enforcer.enforce((subject, resource, action)) {
      authorized
    } else {
      error!("Casbin model does not map request.");
      false
    }
  }
}


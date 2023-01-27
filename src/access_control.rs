use casbin::prelude::*;
use log::error;

const CONFIG: &str = "accessControl/policies.conf";
const POLICY: &str = "accessControl/policies.csv";


pub struct AccessControl {
  enforcer: Enforcer,
}

impl AccessControl {
  pub async fn new() -> Result<AccessControl> {
    let enforcer = Enforcer::new(CONFIG, POLICY).await?;
    Ok(AccessControl { enforcer })
  }

  /// Centralized access control mechanism
  pub fn check_authorization(&self, subject: &str, resource: &str, action: &str) -> bool {
    if let Ok(authorized) = self.enforcer.enforce((subject, resource, action)) {
      authorized
    } else {
      error!("Casbin model does not map request.");
      false
    }
  }
}


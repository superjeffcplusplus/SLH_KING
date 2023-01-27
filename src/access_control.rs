use casbin::prelude::*;
use lazy_static::lazy_static;
use futures::executor::block_on;
use log::{debug, error};

const CONFIG: &str = "accessControl/policies.conf";
const POLICY: &str = "accessControl/policies.csv";

lazy_static! {
  pub static ref ACCESS_CTRL: AccessControl = {
    let access_ctrl = match block_on(AccessControl::new()) {
      Ok(val) => val,
      Err(e) => {
        debug!("{}", e);
        error!("Create AccessControl failed.");
        println!("Unexpected end of program.");
        std::process::exit(1);
      }
    };
    access_ctrl
  };
}

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


use std::collections::HashMap;
use std::error::Error;
use serde::Serialize;
use crate::user::{Action, Resource, Role, User};

#[derive(Serialize)]
pub struct CasbinPolicy {
  pub p: String,
  pub subject: Role,
  pub object: Resource,
  pub actions: Action,
}

#[derive(Serialize)]
struct CasbinGroupingPolicy {
  pub g: String,
  pub subject: String,
  pub group: Role,
}

impl CasbinPolicy {
  pub fn write_to_csv(user_db: &HashMap<String, User>) -> Result<(), Box<dyn Error>> {
    { // Write policies for groups
      let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_path("accessControl/objectPolicies.csv")?;
      wtr.serialize(
        CasbinPolicy {
          p: "p".to_string(),
          subject: Role::STUDENT,
          object: Resource::GRADES,
          actions: Action::R,
        }
      )?;
      wtr.serialize(
        CasbinPolicy {
          p: "p".to_string(),
          subject: Role::PROF,
          object: Resource::GRADES,
          actions: Action::RW,
        }
      )?;
      wtr.flush()?;
    }


    {// Define users group
      let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_path("accessControl/groupingPolicies.csv")?;
      for u in user_db.values() {
        let row = CasbinGroupingPolicy {
          g: "g".to_string(),
          subject: u.name.to_string(),
          group: u.role,
        };
        wtr.serialize(row)?;
      }
      wtr.flush()?;
    }

    Ok(())
  }
}
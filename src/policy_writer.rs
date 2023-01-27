use std::collections::HashMap;
use std::error::Error;
use std::{fs, io};
use serde::Serialize;
use crate::user::{Action, Role, User};

#[derive(Serialize)]
pub struct CasbinPolicy {
  pub p: String,
  pub subject: String,
  pub object: String,
  pub actions: Action,
}

#[derive(Serialize)]
struct CasbinGroupingPolicy {
  pub g: String,
  pub subject: String,
  pub group: String,
}

impl CasbinPolicy {
  pub fn write_to_csv(user_db: &HashMap<String, User>) -> Result<(), Box<dyn Error>> {
    let students: Vec<&User> = user_db.values()
      .filter(|&u| u.role == Role::STUDENT).collect();
    let profs: Vec<&User> = user_db.values()
      .filter(|&u| u.role == Role::PROF).collect();
    let mut wtr_g = csv::WriterBuilder::new()
      .has_headers(false)
      .from_path("accessControl/groupingPolicies.csv")?;

      let mut wtr_p = csv::WriterBuilder::new()
        .has_headers(false)
        .from_path("accessControl/objectPolicies.csv")?;

      // let mut wtr = csv::WriterBuilder::new()
      //   .has_headers(false)
      //   .from_path("accessControl/objectPolicies.csv")?;
      wtr_p.serialize(
        CasbinPolicy {
          p: "p".to_string(),
          subject: Role::PROF.to_string(),
          object: "grades/all".to_string(),
          actions: Action::Read,
        }
      )?;
      wtr_p.serialize(
        CasbinPolicy {
          p: "p".to_string(),
          subject: Role::PROF.to_string(),
          object: "grades/all".to_string(),
          actions: Action::Write,
        }
      )?;
      for student in students {
        wtr_p.serialize(
          CasbinPolicy{
            p: "p".to_string(),
            subject: student.name.clone(),
            object: format!("grades/{}", student.name),
            actions: Action::Read,
          }
        )?;
        wtr_g.serialize(
          CasbinGroupingPolicy {
            g: "g2".to_string(),
            subject: format!("grades/{}",student.name),
            group: "grades/all".to_string(),
          }
        )?;
      }
      wtr_p.flush()?;


      for u in profs {
        let row = CasbinGroupingPolicy {
          g: "g".to_string(),
          subject: u.name.to_string(),
          group: u.role.to_string(),
        };
        wtr_g.serialize(row)?;
      }
      wtr_g.flush()?;

    CasbinPolicy::merge_policy_files()?;
    Ok(())
  }

  /// Juste a trick because csv writer cannot write lines with
  /// different line numbers
  fn merge_policy_files() -> Result<(), Box<dyn Error>> {
    let mut out = fs::OpenOptions::new()
      .append(true)
      .create(true)
      .open("accessControl/policies.csv")?;

    let mut obj_pol = fs::OpenOptions::new()
      .read(true)
      .open("accessControl/objectPolicies.csv")?;

    let mut gr_pol = fs::OpenOptions::new()
      .read(true)
      .open("accessControl/groupingPolicies.csv")?;

    io::copy(&mut obj_pol, &mut out)?;
    io::copy(&mut gr_pol, &mut out)?;

    Ok(())
  }
}
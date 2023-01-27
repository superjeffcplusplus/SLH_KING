use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::sync::Mutex;

use lazy_static::{__Deref, lazy_static};
use log::{error, trace, warn};
use crate::access_control::ACCESS_CTRL;

use crate::user::{Action, User};

const DATABASE_FILE: &str = "db/grades_db.json";
const USERS_DATABASE_FILE: &str = "db/usr_db.json";


lazy_static! {
    pub static ref GRADE_DATABASE: Mutex<HashMap<String, Vec<f32>>> = {
      let map = read_grades_db(DATABASE_FILE).unwrap_or(HashMap::new());
      Mutex::new(map)
    };
    pub static ref USERS_DATABASE: Mutex<HashMap<String, User>> = {
        let map = read_usr_db(USERS_DATABASE_FILE).unwrap_or(HashMap::new());
        Mutex::new(map)
    };
}

pub fn save_db() -> Result<(), Box<dyn Error>> {
  {
    // Declare the value to instantiate the lazy variable in case of quitting
    // directly after start
    let value = GRADE_DATABASE.lock().unwrap();
    let file = File::create(DATABASE_FILE)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, value.deref())?;
  }

  {
    let value = USERS_DATABASE.deref();
    let file = File::create(USERS_DATABASE_FILE)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, value)?;
  }
  trace!("Database successfully saved.");
  Ok(())
}

pub fn user_exits(username: &str) -> bool {
  let db = GRADE_DATABASE.deref().lock().unwrap();
  match db.get(username) {
    None => false,
    Some(_) => true,
  }
}

pub fn get_student_grades(student_name: &str, requester: &User, resource: &str) -> Option<Vec<f32>> {
  let db = GRADE_DATABASE.deref().lock().unwrap();
  let is_authorized = ACCESS_CTRL.check_authorization(requester.name.as_str(), resource, Action::Read.to_string().as_str());
  if is_authorized {
    let out = match db.get(student_name) {
      None => None,
      Some(val) => Some(val.clone())
    };
    out
  } else {
    warn!("Unauthorized attempt to access notes of {} by {}", student_name, requester.name);
    None
  }
}

fn read_grades_db<P: AsRef<Path>>(
  path: P,
) -> Result<HashMap<String, Vec<f32>>, Box<dyn Error>> {
  let file = File::open(path)?;
  let reader = BufReader::new(file);
  let map = serde_json::from_reader(reader).map_err(|e| {
    error!("Unable to read grades : {}",e);
    e
  })?;
  Ok(map)
}

fn read_usr_db<P: AsRef<Path>>(
  path: P
) -> Result<HashMap<String, User>, Box<dyn Error>> {
  let file = File::open(path)?;
  let reader = BufReader::new(file);
  let map = match serde_json::from_reader(reader) {
    Ok(val) => val,
    Err(e) => {
      error!("Unable to read users db : {}",e);
      panic!();
    }
  };
  Ok(map)
}
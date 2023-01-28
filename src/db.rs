use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use std::sync::Mutex;
use dryoc::classic::crypto_box::Nonce;
use dryoc::classic::crypto_secretbox::{crypto_secretbox_keygen, Key};
use dryoc::dryocsecretbox::NewByteArray;
use lazy_static::{__Deref, lazy_static};
use log::{error, info, trace, warn};
use crate::access_control::{ACCESS_CTRL};
use crate::encryption::{create_encryption_key, create_nonce, decrypt_to_string, encrypt_string, read_b64_from_file, vec_to_key, vec_to_nonce};

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
    let mut file = File::create(DATABASE_FILE)?;
    let db_str = serde_json::to_string(value.deref())?;
    let key = match read_b64_from_file("secret/key.txt") {
      Ok(val) => vec_to_key(val).unwrap(),
      Err(_) => create_encryption_key("secret/key.txt"),
    };
    let nonce = create_nonce("secret/nonce.txt");
    let cipher = encrypt_string(&db_str, &key, &nonce)?;
    file.write(&cipher)?;
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
  let db = USERS_DATABASE.deref().lock().unwrap();

  match db.get(username) {
    None => false,
    Some(_) => true,
  }
}

pub fn get_student_grades(student_name: &str, requester: &User) -> Option<Vec<f32>> {
  let db = GRADE_DATABASE.deref().lock().unwrap();
  let resource = format!("grades/{}", student_name);
  let is_authorized = ACCESS_CTRL.check_authorization(requester.name.as_str(), resource.as_str(), Action::Read.to_string().as_str());
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

pub fn add_grade(student_name: &str, requester: &User, grade: f32) -> Option<()>{
  let mut db = GRADE_DATABASE.deref().lock().unwrap();
  let resource= format!("grades/{}", student_name);
  let is_authorized = ACCESS_CTRL.check_authorization(requester.name.as_str(), resource.as_str(), Action::Write.to_string().as_str());
  if is_authorized {
    let mut notes = match db.get(student_name) {
      None => vec![],
      Some(val) => val.clone(),
    };
    notes.push(grade);
    db.insert(student_name.to_string(), notes);
    info!("{} add a new note to {}.", requester.name, student_name);
    Some(())
  } else {
    warn!("Unauthorized attempt to add note to {} by {}.", student_name, requester.name);
    None
  }

}

fn read_grades_db(path: &str) -> Result<HashMap<String, Vec<f32>>, Box<dyn Error>> {
  let file = File::open(&path)?;
  let reader = BufReader::new(file);
  let map_res = serde_json::from_reader::<BufReader<File>,HashMap<String, Vec<f32>>>(reader);
  let map = match map_res {
    Ok(map) => map,
    Err(e) => {
      info!("Cannot deserialize DB, Trying decrypt file first ...");
      let key = read_b64_from_file("secret/key.txt")
        .map_err(|e| {
          error!("Cannot decrypt grades db, key not found.");
          e
        })?;
      let nonce = read_b64_from_file("secret/nonce.txt")
        .map_err(|e| {
          error!("Cannot decrypt grades db, nonce not found.");
          e
        })?;
      // TODO handle conversion error
      let key = vec_to_key(key).unwrap();
      let nonce = vec_to_nonce(nonce).unwrap();
      let mut cipher = vec![];
      let file = File::open(&path)?;
      let mut reader = BufReader::new(file);
      reader.read_to_end(&mut cipher)?;
      let clear = decrypt_to_string(&cipher, &key, &nonce)?;
      let map = serde_json::from_str(clear.as_str()).map_err(|e| {
        error!("{} : Cannot decrypt grades db.", e);
        e
      })?;
      map
    }
  };
  Ok(map)
}

fn read_usr_db<P: AsRef<Path>>(
  path: P
) -> Result<HashMap<String, User>, Box<dyn Error>> {
  let file = File::open(path)?;
  let reader = BufReader::new(file);
  let map = serde_json::from_reader(reader).map_err(|e| {
    error!("Unable to read grades : {}",e);
    e
  })?;
  Ok(map)
}


use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::sync::Mutex;

use lazy_static::{__Deref, lazy_static};
use log::{debug, error, trace, warn};
use read_input::prelude::*;
use simplelog::{ColorChoice, Config, LevelFilter, TerminalMode, TermLogger};
use crate::hashing::compare_pwd_with_hash;
use crate::policy_writer::CasbinPolicy;
use crate::state::State;
use crate::user::{Role, User};

mod hashing;
mod mocking;
mod user;
mod state;
mod policy_writer;

const DATABASE_FILE: &str = "grades_db.json";
const USERS_DATABASE_FILE: &str = "usr_db.json";

lazy_static! {
  static ref STATE: Mutex<State> = Mutex::new(State {
    user: None,
    authenticated: false,
  });
}

lazy_static! {
    static ref GRADE_DATABASE: Mutex<HashMap<String, Vec<f32>>> = {
      let map = read_grades_db(DATABASE_FILE).unwrap_or(HashMap::new());
      Mutex::new(map)
    };
    pub static ref USERS_DATABASE: Mutex<HashMap<String, User>> = {
        let map = read_usr_db(USERS_DATABASE_FILE).unwrap_or(HashMap::new());
        Mutex::new(map)
    };
}

// static mut GRADE_DATABASE: Lazy<HashMap<String, Vec<f32>>> = Lazy::new(|| {
//     read_database_from_file(DATABASE_FILE).unwrap_or(HashMap::new())
// });

// static PROF_CREDENTIALS: Lazy<HashMap<String, String>> = Lazy::new(|| {
//     read_prof_creds(PROF_CREDS_FILE).unwrap()
// });

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

fn welcome() {
  println!("Welcome to KING: KING Is Not GAPS");
}

fn student_action() {
  println!("*****\n1: See your grades\n2: About\n0: Quit");
  let choice = input().inside(0..=1).msg("Enter Your choice: ").get();
  match choice {
    1 => show_grades("Enter your name. Do NOT lie!"),
    0 => quit(),
    _ => panic!("impossible choice"),
  }
}

fn teacher_action() {
  println!("*****\n1: See grades of student\n2: Enter grades\n3 About\n0: Quit");
  let choice = input().inside(0..=2).msg("Enter Your choice: ").get();
  match choice {
    1 => show_grades("Enter the name of the user of which you want to see the grades:"),
    2 => enter_grade(),
    0 => quit(),
    _ => panic!("impossible choice"),
  }
}

fn show_grades(message: &str) {
  println!("{}", message);
  let name: String = input().get();
  println!("Here are the grades of user {}", name);
  match GRADE_DATABASE.lock().unwrap().get(&name) {
    Some(grades) => {
      println!("{:?}", grades);
      println!(
        "The average is {}",
        (grades.iter().sum::<f32>()) / ((*grades).len() as f32)
      );
    }
    None => panic!("User not in system"),
  };
}

fn become_teacher(teacher: &mut bool) {
  let username: String = input::<String>().msg("Enter your username: ").get();
  let password: String = input().msg("Enter your password: ").get();
  match USERS_DATABASE.lock().unwrap().get(&username) {
    Some(usr) => {
      if compare_pwd_with_hash(password.as_str(), usr.pwd_hash.as_str()) {
        *teacher = true
      }
    }
    None => *teacher = false,
  }
  if !*teacher {
    error!(
             "Failed teacher login with username {} and password {}",
             username, password
         );
  }
  // {
  //     *teacher = true;
  // } else {
  //     *teacher = false;
  //     error!(
  //         "Failed teacher login with username {} and password {}",
  //         username, password
  //     );
  // }
}

fn enter_grade() {
  println!("What is the name of the student?");
  let name: String = input().get();
  println!("What is the new grade of the student?");
  let grade: f32 = input().add_test(|x| *x >= 0.0 && *x <= 6.0).get();
  let mut db = GRADE_DATABASE.lock().unwrap();
  match db.get_mut(&name) {
    Some(v) => v.push(grade),
    None => {
      db.insert(name, vec![grade]);
    }
  };
}

fn quit() {
  {
    trace!("Saving grades!");
    // Declare the value to instantiate the lazy variable in case of quitting
    // directly after start
    let value = GRADE_DATABASE.lock().unwrap();
    let file = File::create(DATABASE_FILE).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, value.deref()).unwrap();
  }

  {
    trace!("Saving grades!");
    let value = USERS_DATABASE.deref();
    let file = File::create(USERS_DATABASE_FILE).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, value).map_err(|e| {
      error!("Cannot save prof cresds : {}",e);
    }).unwrap();
  }

  std::process::exit(0);
}

fn login() {
  println!("Login");
  let username: String = input::<String>().msg("Enter your username: ").get();
  let password: String = input().msg("Enter your password: ").get();
  let def_usr = User {
    name: "".to_string(),
    pwd_hash: "".to_string(),
    role: Role::NONE,
  };
  let tmp = USERS_DATABASE.lock().unwrap();
  let db_rec = tmp.get(&username).unwrap_or(&def_usr);
  if compare_pwd_with_hash(password.as_str(), db_rec.pwd_hash.as_str()) {
    let mut s = STATE.lock().unwrap();
    s.authenticated = true;
    s.user = Some(db_rec.clone());
  } else {
    warn!("Authentication failure with username {}", username);
  }
}

fn main() {
  TermLogger::init(
    LevelFilter::Trace,
    Config::default(),
    TerminalMode::Stderr,
    ColorChoice::Auto,
  )
    .unwrap();
  mocking::add_users(&USERS_DATABASE);
  {
    let usr_db = USERS_DATABASE.deref().lock().unwrap();
    match CasbinPolicy::write_to_csv(&usr_db) {
      Ok(_) => {}
      Err(e) => {
        debug!("{}", e);
        error!("Cannot write policies file.");
        println!("An error occurred. Quitting...");
        std::process::exit(1);
      }
    };
    // Unlock mutex
  }
  welcome();
  login();
  let s = STATE.deref().lock().unwrap();
  if s.authenticated {
    match &s.user {
      Some(u) => {
        match u.role {
          Role::STUDENT => loop {
            student_action()
          },
          Role::PROF => loop {
            teacher_action()
          },
          Role::NONE => error!("User with NONE role."),
        }
      },
      None => {
        error!("No user defined after login.")
      },
    }
  }
  println!("Unexpected end of program.");
  std::process::exit(1);
}
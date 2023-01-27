use std::sync::Mutex;

use lazy_static::{__Deref, lazy_static};
use log::{debug, error, warn};
use read_input::prelude::*;
use simplelog::{ColorChoice, Config, LevelFilter, TerminalMode, TermLogger};
use crate::db::{GRADE_DATABASE, USERS_DATABASE};
use crate::hashing::compare_pwd_with_hash;
use crate::policy_writer::CasbinPolicy;
use crate::state::State;
use crate::user::{Role, User};

mod hashing;
mod mocking;
mod user;
mod state;
mod policy_writer;
mod db;


lazy_static! {
  static ref STATE: Mutex<State> = Mutex::new(State {
    user: None,
    authenticated: false,
  });
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
  match db::save_db() {
    Ok(_) => {}
    Err(e) => {
      debug!("{}", e);
      error!("Cannot write database.");
      println!("An error occurred while saving data.");
      std::process::exit(1);
    }
  };
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

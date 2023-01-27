
use lazy_static::{__Deref};
use log::{debug, error, info, warn};
use read_input::prelude::*;
use simplelog::{ColorChoice, Config, LevelFilter, TerminalMode, TermLogger};
use crate::db::{GRADE_DATABASE, USERS_DATABASE};
use crate::hashing::compare_pwd_with_hash;
use crate::policy_writer::CasbinPolicy;
use crate::user::{Role, User};

mod hashing;
mod mocking;
mod user;
mod state;
mod policy_writer;
mod db;
mod access_control;

fn welcome() {
  println!("Welcome to KING: KING Is Not GAPS");
}

fn student_action(current_user: User) {
  println!("*****\n1: See your grades\n2: About\n0: Quit");
  let choice = input().inside(0..=1).msg("Enter Your choice: ").get();
  match choice {
    1 => show_grades(current_user.name.as_str(), &current_user),
    0 => quit(),
    _ => panic!("impossible choice"),
  }
}

fn teacher_action(current_user: User) {
  println!("*****\n1: See grades of student\n2: Enter grades\n3 About\n0: Quit");
  let choice = input().inside(0..=2).msg("Enter Your choice: ").get();

  match choice {
    1 => {
      print!("Enter the name of the user of which you want to see the grades:");
      let name: String = input().get();
      show_grades(name.as_str(), &current_user);
    },
    2 => enter_grade(),
    0 => quit(),
    _ => panic!("impossible choice"),
  }
}

fn show_grades(student_name: &str, current_user: &User) {
  let resource = current_user.get_authorized_resource_descriptor();
  if db::user_exits(student_name) {
    match db::get_student_grades(student_name, current_user, resource.as_str()) {
      Some(grades) => {
        println!("Here are the grades of user {}", student_name);
        println!("{:?}", grades);
        println!(
          "The average is {}",
          (grades.iter().sum::<f32>()) / ((*grades).len() as f32)
        );
      }
      None => println!("No grades to show."),
    };
  } else {
    println!("User not in system");
  }
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

fn login() -> Option<User> {
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
    info!("Successful user authentication {}.", db_rec.name);
    Some(db_rec.clone())
  } else {
    warn!("Authentication failure with username {}", username);
    None
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
  if let Some(user) = login() {
    match user.role {
      Role::STUDENT => loop {
        student_action(user.clone())
      },
      Role::PROF => loop {
        teacher_action(user.clone())
      },
      Role::NONE => error!("User with NONE role."),
    }
  } else {
    println!("Authentication failure.");
  };
}

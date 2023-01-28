
use lazy_static::{__Deref};
use log::{debug, error, info, warn};
use read_input::prelude::*;
use simplelog::{ColorChoice, Config, LevelFilter, TerminalMode, TermLogger};
use crate::db::{USERS_DATABASE};
use crate::hashing::compare_pwd_with_hash;
use crate::input_validation::is_usr_n_valid;
use crate::policy_writer::CasbinPolicy;
use crate::user::{Role, User};

mod hashing;
mod mocking;
mod user;
mod state;
mod policy_writer;
mod db;
mod access_control;
mod input_validation;
mod encryption;

fn usr_name_input() -> String {
  input().add_test(|i:&String| is_usr_n_valid(i)).msg("Enter username (^(A-Za-z){3,12}$) : ").get()
}

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
  let choice = input().inside(0..=2).msg("Enter Your choice : ").get();
  match choice {
    1 => {
      println!("Enter the name of the user of which you want to see the grades:");
      let name: String = input().get();
      show_grades(name.as_str(), &current_user);
    },
    2 => enter_grade(&current_user),
    0 => quit(),
    _ => panic!("impossible choice"),
  }
}

fn show_grades(student_name: &str, current_user: &User) {
  if db::user_exits(student_name) {
    match db::get_student_grades(student_name, current_user) {
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

fn enter_grade(current_user: &User) {
  print!("What is the name of the student?");
  let name: String = usr_name_input();
  if db::user_exits(&name) {
    print!("What is the new grade of the student?");
    let grade: f32 = input().add_test(|x| *x >= 0.0 && *x <= 6.0).get();
    match db::add_grade(name.as_str(),&current_user, grade) {
      None => {
        error!("Adding note failed.");
        println!("Operation failed");
      },
      Some(_) => println!("Note successfully added."),
    };
  } else {
    println!("User not in system");
  }
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
  let username: String = usr_name_input();
  let password: String = input().add_test(|i: &String| i.len() < 32).msg("Enter your password (max 32 char): ").get();
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
    LevelFilter::Info,
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

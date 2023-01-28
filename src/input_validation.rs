use lazy_static::lazy_static;
use regex::Regex;

static USR_NAME: &str = r"^[A-Za-z]{3,12}$";

lazy_static! {
  static ref USR_NAME_RE: Regex = Regex::new(USR_NAME).unwrap();
}

pub fn is_usr_n_valid(input: &String) -> bool {
  validate_input(&USR_NAME_RE, input.as_str())
}

// Input validator, uses the provided regex to check a given input validity
fn validate_input(regex: &Regex, input: &str) -> bool {
  return regex.is_match(&input);
}
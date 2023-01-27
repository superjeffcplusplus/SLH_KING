use std::fmt;
use argon2::{Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use once_cell::sync::Lazy;
use rand_core::OsRng;

#[derive(Debug)]
pub struct PwdHasherError;

impl fmt::Display for PwdHasherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Password hashing error")
    }
}

static PWD_HASHER: Lazy<Argon2> = Lazy::new(|| {
    Argon2::from(&Params::default())
});

/// Hash the provided password and compare with pwd_and_hash.
/// If pwd_and_hash eq. "", then a default hash is created in the aim of
/// preserving constant-time code execution.
pub fn compare_pwd_with_hash(password: &str, pwd_and_hash: &str) -> bool {
    // to maintain a constant execution time, we calculate the hash in all the case
    let def_hash = new_hash_from_pwd("1234").unwrap();
    let hash_to_test_str = if pwd_and_hash == "" {
        def_hash.as_str()
    } else {
        pwd_and_hash
    };
    let pwd_to_test = if pwd_and_hash == "" {
        "5678"
    } else {
        password
    };
    let hash_obj_res = PasswordHash::new(&hash_to_test_str);
    let hash_to_test = match hash_obj_res {
        Ok(val) => val,
        Err(e) => {
            println!("{}", e);
            return false;
        },
    };
    PWD_HASHER.verify_password(pwd_to_test.as_bytes(), &hash_to_test).is_ok()
}

/// Create a new hash from a new password
pub fn new_hash_from_pwd(password: &str) -> Result<String, PwdHasherError> {
    let salt = SaltString::generate(&mut OsRng);
    match PWD_HASHER.hash_password(password.as_bytes(), &salt) {
        Ok(val) => Ok(val.to_string()),
        Err(e) => {
            println!("{}", e);
            Err(PwdHasherError)
        }
    }
}


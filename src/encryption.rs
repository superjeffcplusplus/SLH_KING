use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use dryoc::classic::crypto_secretbox::{crypto_secretbox_easy, crypto_secretbox_keygen, crypto_secretbox_open_easy, Key, Nonce};
use base64::{Engine as _, engine::general_purpose};
use dryoc::constants::{CRYPTO_SECRETBOX_MACBYTES, CRYPTO_SECRETBOX_NONCEBYTES};
use std::{error, fmt, str};
use dryoc::dryocsecretbox::NewByteArray;

#[derive(Debug, Clone)]
pub struct ConversionError;

impl fmt::Display for ConversionError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Cannot convert object to key or nonce.")
  }
}



pub fn encrypt_string(to_encrypt: &String, key: &Key, nonce: &Nonce) -> Result<Vec<u8>, Box<dyn Error>> {
  let mut message = to_encrypt.as_bytes();
  let mut ciphertext  = vec![0u8; message.len() + CRYPTO_SECRETBOX_MACBYTES];
  crypto_secretbox_easy(&mut ciphertext, &mut message, nonce, key)?;
  Ok(ciphertext)
}

pub fn decrypt_to_string(to_decrypt: &Vec<u8>, key: &Key, nonce: &Nonce) -> Result<String, Box<dyn Error>>{
  let mut decrypted = vec![0u8; to_decrypt.len() - CRYPTO_SECRETBOX_MACBYTES];
  crypto_secretbox_open_easy(&mut decrypted, &to_decrypt, &nonce, &key)?;
  let str = str::from_utf8(&decrypted)?;
  Ok(str.to_string())
}

pub fn create_encryption_key(path: &str) -> Key {
  let secret_key: Key = crypto_secretbox_keygen();
  let encoded: String = general_purpose::STANDARD_NO_PAD.encode(secret_key);
  let mut file = File::create(path).unwrap();
  file.write(encoded.as_bytes()).unwrap();
  secret_key
}

pub fn create_nonce(path: &str) -> Nonce {
  let nonce = Nonce::gen();
  let encoded: String = general_purpose::STANDARD_NO_PAD.encode(nonce);
  let mut file = File::create(path).unwrap();
  file.write(encoded.as_bytes()).unwrap();
  nonce
}

pub fn read_b64_from_file(path: &str) -> Result<Vec<u8>, Box<dyn Error>>{
  let mut f = File::open(path)?;
  let mut key_str = String::new();
  f.read_to_string(&mut key_str)?;
  let key_bytes = general_purpose::STANDARD_NO_PAD.decode(&key_str)?;
  Ok(key_bytes)
}

pub fn vec_to_key(key: Vec<u8>) -> Option<Key> {
  let len = key.len();
  if len != 32 {
    return None
  }
  let mut _key = [0u8; 32];
  for i in 0..len {
    _key[i] = key[i];
  };
  _key = _key as Key;
  Some(_key)
}

pub fn vec_to_nonce(nonce: Vec<u8>) -> Option<Nonce> {
  let len = nonce.len();
  if len != 24 {
    return None
  }
  let mut _nonce = [0u8; 24];
  for i in 0..len {
    _nonce[i] = nonce[i];
  };
  _nonce = _nonce as Nonce;
  Some(_nonce)
}

#[cfg(test)]
mod test_encryption {
  use std::fs;
  use dryoc::dryocsecretbox::NewByteArray;
  use super::*;

  #[test]
  fn key_gen_must_create_key_file() {
    let path = "db/test_key.txt";
    create_encryption_key(path);
    let f = File::open(path);
    assert!(f.is_ok());
    let res = fs::remove_file(path);
    assert!(res.is_ok());
  }

  #[test]
  fn read_key_should_return_bytes_vec() {
    let path = "db/test_key.txt";
    create_encryption_key(path);
    let key = read_b64_from_file(path);
    assert!(key.is_ok());
    assert_eq!(key.unwrap().len(), 32);
    let res = fs::remove_file(path);
    assert!(res.is_ok());
  }

  #[test]
  fn encryption_should_not_fail() {
    let to_encrypt = "Tityre tu patulae recubans sub tegmine fagi ... (Il n'a pas d'examens...)";
    let key: Key = crypto_secretbox_keygen();
    let nonce = Nonce::gen();
    let cipher = encrypt_string(&to_encrypt.to_string(), &key, &nonce);
    assert!(cipher.is_ok());
  }

  #[test]
  fn decryption_should_not_fail() {
    let to_encrypt = "Tityre tu patulae recubans sub tegmine fagi ... (Il n'a pas d'examens... utf-8: ä ü Ç)";
    let key: Key = crypto_secretbox_keygen();
    let nonce = Nonce::gen();
    let cipher = encrypt_string(&to_encrypt.to_string(), &key, &nonce);
    assert!(cipher.is_ok());
    let dec_str = decrypt_to_string(&cipher.unwrap(), &key, &nonce);
    assert!(dec_str.is_ok());
    assert_eq!(dec_str.unwrap().as_str(),to_encrypt);
  }

}
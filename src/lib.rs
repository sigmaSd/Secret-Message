//! # Secret Message
//!
//! Simple way to encrypt a message (No security whatsoever!!!)
//!
//! This crate exposes 3 functions:
//!
//! Encrypt and decrypt a messge:
//!
//! `encode` and `decode`:
//!
//! ```rust
//! use secret_msg::{encode, decode};
//!
//! let (secret, key) = encode("my_secret!");
//! assert_eq!(decode(&secret, key), "my_secret!");
//! let (secret, key) = encode(1234);
//! assert_eq!(decode(&secret, key), "1234");
//! ```
//!
//! Encrypt a message with no easy way to retrieve it back
//!
//! `one_way_encode`:
//!
//! ```rust
//! use secret_msg::one_way_encode;
//!
//! let sipher = one_way_encode("my_secret!");
//! assert_eq!(sipher, "1537");
//! let sipher = one_way_encode(158721);
//! assert_eq!(sipher, "2361");
//! ```

use rand::seq::SliceRandom;
use rand::thread_rng;

enum EncMethod {
    INC,
    DEC,
    TIME,
}

impl EncMethod {
    //XXX Check fns when adding methods
    fn choose() -> Self {
        let mut rng = thread_rng();
        let choices = [0, 1, 2];
        match choices.choose(&mut rng) {
            Some(0) => EncMethod::INC,
            Some(1) => EncMethod::DEC,
            Some(2) => EncMethod::TIME,
            _ => unreachable!(),
        }
    }
    fn key(&self) -> usize {
        match self {
            EncMethod::INC => 0,
            EncMethod::DEC => 1,
            EncMethod::TIME => time_to_key(),
        }
    }
    fn from_key(key: usize) -> Self {
        match key {
            0 => EncMethod::INC,
            1 => EncMethod::DEC,
            _ => EncMethod::TIME,
        }
    }
}

/// encrypt a msg with no easy way to get the original back from it
pub fn one_way_encode<T: ToString>(msg: T) -> String {
    let hash: [u8; 16] = md5::compute(msg.to_string()).into();
    hash.iter().fold(0, |acc, x| acc + *x as usize).to_string()
}

/// encrypt a msg -> returns an encrytped msg and a decode key
pub fn encode<T: ToString>(msg: T) -> (String, usize) {
    let method = EncMethod::choose();
    let key = method.key();

    let enc = msg
        .to_string()
        .chars()
        .map(|c| encrypt(c, &method, key))
        .collect::<String>();

    (enc, key)
}

/// decrypt a msg using decode key
pub fn decode(enc_msg: &str, key: usize) -> String {
    let method = EncMethod::from_key(key);
    enc_msg
        .chars()
        .map(|c| decrypt(c, &method, key))
        .collect::<String>()
}

fn encrypt(c: char, method: &EncMethod, key: usize) -> char {
    match method {
        EncMethod::INC => char_move(c, 1),
        EncMethod::DEC => char_move(c, -1),
        EncMethod::TIME => enc_time(c, key),
    }
}

fn decrypt(c: char, method: &EncMethod, key: usize) -> char {
    match method {
        EncMethod::INC => char_move(c, -1),
        EncMethod::DEC => char_move(c, 1),
        EncMethod::TIME => decrypt_time(c, key),
    }
}

// Char Move Crypto
fn char_move(c: char, add: i32) -> char {
    let mut enc_c = (c as i32 + add) % 255;
    if enc_c <= 0 {
        enc_c += 255;
    }
    enc_c as u8 as char
}

// Time Crypto
fn enc_time(c: char, key: usize) -> char {
    // handle add with overflow
    let enc_c = key + c as usize;
    (enc_c % 255) as u8 as char
}
fn decrypt_time(c: char, key: usize) -> char {
    // handle subtract with overflow
    let mut c = c as usize;
    let key = key % 255;
    if c <= key {
        c += 255;
    }
    (c - key) as u8 as char
}

// helper fns
fn time_to_key() -> usize {
    use std::process::Command;
    let date = Command::new("date").output().unwrap().stdout;
    let mut code = 0;
    String::from_utf8_lossy(&date).chars().for_each(|c| {
        if c.is_digit(10) {
            code += c.to_digit(10).unwrap();
        }
    });
    // hmm
    assert!(code != 0 && code != 1);
    code as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let (secret, key) = encode("my very secret msg ÿ0");
        assert_eq!(decode(&secret, key), "my very secret msg ÿ0");

        let (secret, key) = encode(56516510);
        assert_eq!(decode(&secret, key), "56516510");
    }
}

//! # Secret Message
//!
//! Simple way to encrypt a message (No security whatsoever!!!)
//!
//! This crate exposes 2 functions:
//!
//! `encode` and `decode`:
//!
//! ```rust
//! use secret_msg::{encode, decode};
//!
//! let (secret, key) = encode("my_secret!");
//! assert_eq!(decode(&secret, key), "my_secret!");
//! ```

use rand::seq::SliceRandom;
use rand::thread_rng;

enum EncMethod {
    INC,
    DEC,
}

impl EncMethod {
    //XXX Check fns when adding methods
    fn choose() -> Self {
        let mut rng = thread_rng();
        let choices = [0, 1];
        match choices.choose(&mut rng) {
            Some(0) => EncMethod::INC,
            Some(1) => EncMethod::DEC,
            _ => unreachable!(),
        }
    }
    fn key(&self) -> usize {
        match self {
            EncMethod::INC => 0,
            EncMethod::DEC => 1,
        }
    }
    fn from_key(key: usize) -> Self {
        match key {
            0 => EncMethod::INC,
            1 => EncMethod::DEC,
            _ => unreachable!(),
        }
    }
}

/// encrypt a msg -> returns an encrytped msg and a decode key
pub fn encode(msg: &str) -> (String, usize) {
    let method = EncMethod::choose();

    let enc = msg.chars().map(|c| encrypt(c, &method)).collect::<String>();
    let key = method.key();

    (enc, key)
}

/// decrypt a msg using decode key
pub fn decode(enc_msg: &str, key: usize) -> String {
    let method = EncMethod::from_key(key);
    match method {
        EncMethod::INC => enc_msg
            .chars()
            .map(|c| decrypt(c, &method))
            .collect::<String>(),
        EncMethod::DEC => enc_msg
            .chars()
            .map(|c| decrypt(c, &method))
            .collect::<String>(),
    }
}

fn encrypt(c: char, method: &EncMethod) -> char {
    // handle ascii limits
    if c == '0' || c == 'ÿ' {
        return c;
    }

    match method {
        EncMethod::INC => (c.to_string().as_bytes()[0] + 1) as char,
        EncMethod::DEC => (c.to_string().as_bytes()[0] - 1) as char,
    }
}

fn decrypt(c: char, method: &EncMethod) -> char {
    // handle ascii limits
    if c == '0' || c == 'ÿ' {
        return c;
    }

    match method {
        EncMethod::INC => (c.to_string().as_bytes()[0] - 1) as char,
        EncMethod::DEC => (c.to_string().as_bytes()[0] + 1) as char,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let (secret, key) = encode("my very secret msg ÿ0");
        assert_eq!(decode(&secret, key), "my very secret msg ÿ0");
    }
}

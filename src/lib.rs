//! # Secret Message
//!
//! Simple way to encrypt a message (No security whatsoever!!!)
//!
//! This crate exposes **SecretMessage** trait:
//!
//! encrypt and decrypt a messge:
//!
//! **encrypt** and **decrypt**:
//!
//! ```rust
//! use secret_msg::SecretMessage;
//!
//! let (secret, key) = "my_secret!".encrypt();
//! assert_eq!(secret.decrypt(key), "my_secret!");
//! let (secret, key) = 1234.encrypt();
//! assert_eq!(secret.decrypt(key), "1234");
//! ```
//!
//! or use a custom key:
//!
//! ```rust
//! use secret_msg::SecretMessage;
//! let secret = "cool secret".encrypt_with_key(58794);
//! assert_eq!(secret.decrypt(58794), "cool secret");
//! ```
//! encrypt a message with no easy way to retrieve it back:
//!
//! **one_way_encrypt**:
//!
//! ```rust
//! use secret_msg::SecretMessage;
//!
//! let cipher = "my_secret!".one_way_encrypt();
//! assert_eq!(cipher, "1537");
//! let cipher = 158721.one_way_encrypt();
//! assert_eq!(cipher, "2361");
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

pub trait SecretMessage {
    /// encrypt a msg with no easy way to get the original back
    fn one_way_encrypt(&self) -> String;

    /// encrypt a msg with a given key
    fn encrypt_with_key(&self, key: usize) -> String;

    /// encrypt a msg -> returns an encrytped msg and a decrypt key
    fn encrypt(&self) -> (String, usize);

    /// decrypt a msg using decrypt key
    fn decrypt(&self, key: usize) -> String;
}

impl<T: ToString> SecretMessage for T {
    fn one_way_encrypt(&self) -> String {
        let hash: [u8; 16] = md5::compute(self.to_string()).into();
        hash.iter().fold(0, |acc, x| acc + *x as usize).to_string()
    }

    fn encrypt_with_key(&self, key: usize) -> String {
        let method = EncMethod::from_key(key);

        self.to_string()
            .chars()
            .map(|c| encrypt_character(c, &method, key))
            .collect::<String>()
    }

    fn encrypt(&self) -> (String, usize) {
        let method = EncMethod::choose();

        let key = method.key();
        let enc = self.encrypt_with_key(key);

        (enc, key)
    }

    fn decrypt(&self, key: usize) -> String {
        let method = EncMethod::from_key(key);
        self.to_string()
            .chars()
            .map(|c| decrypt_character(c, &method, key))
            .collect::<String>()
    }
}

fn encrypt_character(c: char, method: &EncMethod, key: usize) -> char {
    match method {
        EncMethod::INC => char_move(c, 1),
        EncMethod::DEC => char_move(c, -1),
        EncMethod::TIME => enc_time(c, key),
    }
}

fn decrypt_character(c: char, method: &EncMethod, key: usize) -> char {
    match method {
        EncMethod::INC => char_move(c, -1),
        EncMethod::DEC => char_move(c, 1),
        EncMethod::TIME => decrypt_character_time(c, key),
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
fn decrypt_character_time(c: char, key: usize) -> char {
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
    use std::time::Instant;
    let mut code = 0;
    Instant::now().elapsed().as_nanos().to_string().chars().for_each(|c| {
        if c.is_digit(10) {
            code += c.to_digit(10).unwrap();
        }
    });
    // hmm
    if code == 0 || code == 1 {
        code = 2;
    }
    code as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let (secret, key) = "my very secret msg ÿ0".encrypt();
        assert_eq!(secret.decrypt(key), "my very secret msg ÿ0");

        let (secret, key) = 56516510.encrypt();
        assert_eq!(secret.decrypt(key), "56516510");

        let secret = "cool secret".encrypt_with_key(35413613);
        assert_eq!(secret.decrypt(35413613), "cool secret");
    }
}

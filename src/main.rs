use secret_msg::{decode, encode};
use std::env::args;
use std::io::{self, Read};

fn main() {
    match args().nth(1).unwrap().to_lowercase().as_str().trim() {
        "enc" => enc(),
        "dec" => dec(),
        _ => eprintln!("Uknown command"),
    }
}

fn enc() {
    let data = match args().nth(2) {
        Some(data) => data,
        None => {
            let mut buffer = String::new();
            let stdin = io::stdin();
            let mut handle = stdin.lock();

            handle.read_to_string(&mut buffer).unwrap();
            buffer
        }
    };
    //TODO: Find a fix for Time Crypto Unicode
    let enc_data = loop {
        let enc_data = encode(&data);
        if enc_data.1 == 0 || enc_data.1 == 1 {
            break enc_data;
        }
    };
    enc_data.0.lines().for_each(|l| {
        println!("{}", l);
    });
    println!("--------------------\nkey:{}", &enc_data.1);
}

fn dec() {
    let key = args().nth(2).unwrap().parse::<usize>().unwrap();

    let data = match args().nth(3) {
        Some(data) => data,
        None => {
            let mut buffer = String::new();
            let stdin = io::stdin();
            let mut handle = stdin.lock();

            handle.read_to_string(&mut buffer).unwrap();
            buffer
        }
    };

    data.lines().for_each(|l| {
        println!("{}", decode(l, key));
    });
}

use secret_msg::{decode, encode};
use std::env::args;
use std::fs::File;
use std::io::{self, Read, Write};

fn main() {
    match args().nth(1).unwrap().to_lowercase().as_str().trim() {
        "enc" => enc(),
        "dec" => dec(),
        _ => eprintln!("Uknown command"),
    }
}

fn enc() {
    let data = match args().nth(2) {
        Some(data) => {
            let mut buffer = String::new();
            let mut file = File::open(data).unwrap();
            file.read_to_string(&mut buffer).unwrap();
            buffer
        }
        None if args().len() == 2 => {
            let mut buffer = String::new();
            let stdin = io::stdin();
            let mut handle = stdin.lock();

            handle.read_to_string(&mut buffer).unwrap();
            buffer
        }
        None => {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            buffer
        }
    };

    let enc_data = encode(&data);
    let mut enc_file = File::create("./enc").unwrap();
    write!(enc_file, "{}", enc_data.0).unwrap();
    println!("Key: {}", enc_data.1);
}

fn dec() {
    let key = args().nth(2).unwrap().parse::<usize>().unwrap();

    let data = match args().nth(3) {
        Some(data) => {
            let mut buffer = String::new();
            let mut file = File::open(data).unwrap();
            file.read_to_string(&mut buffer).unwrap();
            buffer
        }
        None if args().len() == 2 => {
            let mut buffer = String::new();
            let stdin = io::stdin();
            let mut handle = stdin.lock();

            handle.read_to_string(&mut buffer).unwrap();
            buffer
        }
        None => {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            buffer
        }
    };

    data.lines().for_each(|l| {
        println!("{}", decode(l, key));
    });
}

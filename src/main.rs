use secret_msg::{decode, encode};
use std::env::args;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

fn main() {
    if args().len() < 2 {
        print_usage();
        std::process::exit(0);
    }
    match args().nth(1).unwrap().to_lowercase().as_str().trim() {
        "enc" => enc(),
        "dec" => dec(),
        _ => print_usage(),
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
    let mut enc_file = {
        let f = args().last().unwrap();
        if Path::exists(Path::new(&f)) {
            panic!("cant write output file already exists");
        }
        File::create(f).unwrap()
    };
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

fn print_usage() {
    println!(
        "sm: Secret Message

    Usage: sm enc $file_to_encrypt $encryption_out

    A key will be printed, you can use it to decrypt that message

            sm dec $key $encryption_out

    Example:

            In: sm enc hello_world.txt hello_world.enc
            Out: Key: 1
            In: sm dec 1 hello_world.enc

    Also you can you stdin:

            echo 'hello' | sm enc
            echo 'ifmmp' | sm dec 0"
    );
}

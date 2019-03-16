use secret_msg::SecretMessage;
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

fn encb() {
    let data = {
        let mut data = vec![];
        let d = args().nth(2).unwrap();
        let mut d = File::open(d).unwrap();
        d.read_to_end(&mut data).unwrap();
        data
    };
    let enc: Vec<u8> = data
        .iter()
        .map(|v| {
            let v = *v;
            if v == 0 {
                255
            } else {
                v - 1
            }
        })
        .collect();

    let mut enc_file = {
        let f = args().last().unwrap();
        if Path::exists(Path::new(&f)) {
            panic!("cant write output file already exists");
        }
        File::create(f).unwrap()
    };

    enc_file.write_all(&enc).unwrap();
}

fn decb() {
    let data = {
        let mut data = vec![];
        let d = args().nth(2).unwrap();
        let mut d = File::open(d).unwrap();
        d.read_to_end(&mut data).unwrap();
        data
    };
    let dec: Vec<u8> = data
        .iter()
        .map(|v| {
            let v = *v;
            if v == 255 {
                0
            } else {
                v + 1
            }
        })
        .collect();

    let mut dec_file = {
        let f = args().last().unwrap();
        if Path::exists(Path::new(&f)) {
            panic!("cant write output file already exists");
        }
        File::create(f).unwrap()
    };

    dec_file.write_all(&dec).unwrap();
}

fn enc() {
    let data = match args().nth(2) {
        Some(data) => read_from_file(&data),
        None if args().len() == 2 => read_from_stdin(),
        None => read_interactively(),
    };

    if data.is_err() {
        encb();
        return;
    }

    let enc_data = data.unwrap().encrypt();

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
    let key = match args().nth(2).unwrap().parse::<usize>() {
        Ok(k) => k,
        Err(_) => {
            decb();
            return;
        }
    };

    let data = match args().nth(3) {
        Some(data) => read_from_file(&data),
        None if args().len() == 2 => read_from_stdin(),
        None => read_interactively(),
    };

    if data.is_err() {
        decb();
        return;
    }

    data.unwrap().lines().for_each(|l| {
        println!("{}", l.decrypt(key));
    });
}

fn read_from_file(data: &str) -> Result<String, io::Error> {
    let mut buffer = String::new();
    match File::open(data) {
        Ok(mut file) => {
            file.read_to_string(&mut buffer)?;
            Ok(buffer)
        }
        Err(_) => read_from_stdin(),
    }
}
fn read_from_stdin() -> Result<String, io::Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn read_interactively() -> Result<String, io::Error> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
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

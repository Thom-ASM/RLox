use std::{
    env,
    fs::{self},
    io,
};

use scanner::Scanner;
use tokens::Token;

mod error;
mod scanner;
mod tokens;

fn run(bytes: &Vec<u8>) {
    let mut scan = Scanner::new(bytes);
    let tokens: Vec<Token> = scan.scan_tokens();

    tokens
        .iter()
        .for_each(|token| println!("{}", token.to_string()));
}

fn run_file(file_path: &String) {
    let bytes = fs::read(file_path);
    match bytes {
        Ok(file) => run(&file),
        Err(_) => panic!("file not found"),
    }
}

fn run_prompt() {
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(_) => run(&buf.into_bytes()),
        Err(_) => panic!("Could not read from stdin"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("{:?}", args[1]);
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

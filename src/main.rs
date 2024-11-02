#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let res = eval(input.trim());

        println!("{}", res);
    }
}

fn eval(input: &str) -> String {
    match input {
        input if input.starts_with("exit") => exit(0),
        _ => format!("{}: command not found", input),
    }
}

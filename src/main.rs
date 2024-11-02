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
    let words: Vec<&str> = input.split_whitespace().collect();
    let (cmd, args) = words.split_first().unwrap();

    match *cmd {
        "exit" => exit(0),
        "echo" => args.join(" "),
        _ => format!("{}: command not found", input),
    }
}

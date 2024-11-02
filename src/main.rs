use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;
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
        "type" => match args[0] {
            "exit" | "echo" | "type" => format!("{} is a shell builtin", args[0]),
            _ => {
                let filename = args[0];
                match env::var("PATH")
                    .unwrap()
                    .split(":")
                    .map(|dir| Path::new(dir).join(filename))
                    .find(|fullpath| fullpath.exists())
                {
                    Some(fullpath) => format!("{} is {}", filename, fullpath.display()),
                    None => format!("{}: not found", args[0]),
                }
            }
        },
        _ => format!("{}: command not found", cmd),
    }
}

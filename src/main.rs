use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

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
            "exit" | "echo" | "type" | "pwd" => format!("{} is a shell builtin", args[0]),
            _ => {
                let filename = args[0];
                match find(filename) {
                    Some(fullpath) => format!("{} is {}", filename, fullpath.display()),
                    None => format!("{}: not found", filename),
                }
            }
        },
        "pwd" => env::current_dir().unwrap().to_string_lossy().to_string(),
        _ => {
            if find(cmd).is_some() {
                let output = Command::new(cmd).args(args).output().unwrap();

                if output.status.success() {
                    String::from_utf8_lossy(&output.stdout)
                        .trim_end()
                        .to_string()
                } else {
                    String::from_utf8_lossy(&output.stderr)
                        .trim_end()
                        .to_string()
                }
            } else {
                format!("{}: command not found", cmd)
            }
        }
    }
}

fn find(filename: &str) -> Option<PathBuf> {
    env::var("PATH")
        .unwrap()
        .split(":")
        .map(|dir| Path::new(dir).join(filename))
        .find(|fullpath| fullpath.exists())
}

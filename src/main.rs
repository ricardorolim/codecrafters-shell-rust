use std::env;
use std::fmt::format;
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

        if let Some(output) = eval(input.trim()) {
            println!("{}", output);
        }
    }
}

fn eval(input: &str) -> Option<String> {
    let words: Vec<&str> = input.split_whitespace().collect();
    let (cmd, args) = words.split_first().unwrap();

    match *cmd {
        "exit" => exit(0),
        "echo" => Some(args.join(" ")),
        "type" => match args[0] {
            "exit" | "echo" | "type" | "pwd" => Some(format!("{} is a shell builtin", args[0])),
            _ => {
                let filename = args[0];
                match find(filename) {
                    Some(fullpath) => Some(format!("{} is {}", filename, fullpath.display())),
                    None => Some(format!("{}: not found", filename)),
                }
            }
        },
        "pwd" => Some(env::current_dir().unwrap().to_string_lossy().to_string()),
        "cd" => match env::set_current_dir(args[0]) {
            Ok(..) => None,
            Err(..) => Some(format!("cd: {}: No such file or directory", args[0])),
        },
        _ => run_program(cmd, args),
    }
}

fn run_program(cmd: &str, args: &[&str]) -> Option<String> {
    if find(cmd).is_some() {
        let output = Command::new(cmd).args(args).output().unwrap();

        if output.status.success() {
            Some(
                String::from_utf8_lossy(&output.stdout)
                    .trim_end()
                    .to_string(),
            )
        } else {
            Some(
                String::from_utf8_lossy(&output.stderr)
                    .trim_end()
                    .to_string(),
            )
        }
    } else {
        Some(format!("{}: command not found", cmd))
    }
}

fn find(filename: &str) -> Option<PathBuf> {
    env::var("PATH")
        .unwrap()
        .split(":")
        .map(|dir| Path::new(dir).join(filename))
        .find(|fullpath| fullpath.exists())
}

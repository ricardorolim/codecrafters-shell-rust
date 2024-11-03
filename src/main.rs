use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

enum Builtin {
    Exit,
    Echo,
    Type,
    Pwd,
    Cd,
    Unknown,
}

impl From<&str> for Builtin {
    fn from(cmd: &str) -> Self {
        match cmd {
            "exit" => Builtin::Exit,
            "echo" => Builtin::Echo,
            "type" => Builtin::Type,
            "pwd" => Builtin::Pwd,
            "cd" => Builtin::Cd,
            _ => Builtin::Unknown,
        }
    }
}

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

    match Builtin::from(*cmd) {
        Builtin::Exit => exit(0),
        Builtin::Echo => Some(args.join(" ")),
        Builtin::Type => handle_type(args),
        Builtin::Pwd => Some(env::current_dir().unwrap().to_string_lossy().to_string()),
        Builtin::Cd => handle_cd(args),
        Builtin::Unknown => run_program(cmd, args),
    }
}

fn handle_type(args: &[&str]) -> Option<String> {
    if let Builtin::Unknown = Builtin::from(args[0]) {
        let filename = args[0];
        match find(filename) {
            Some(fullpath) => Some(format!("{} is {}", filename, fullpath.display())),
            None => Some(format!("{}: not found", filename)),
        }
    } else {
        Some(format!("{} is a shell builtin", args[0]))
    }
}

fn handle_cd(args: &[&str]) -> Option<String> {
    let dir = if args[0].contains("~") {
        &env::var("HOME").unwrap()
    } else {
        args[0]
    };

    match env::set_current_dir(dir) {
        Ok(..) => None,
        Err(..) => Some(format!("cd: {}: No such file or directory", args[0])),
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

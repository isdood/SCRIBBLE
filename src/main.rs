use std::env;
use std::process::{Command, Stdio};

fn process_string(input: &str) -> String {
    input.replace("__", "**")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <command>", args[0]);
        std::process::exit(1);
    }

    let mut cmd = Command::new("./bin/seed.orig")
        .args(&args[1..])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    let output = cmd.wait_with_output()
        .expect("Failed to wait on command");

    if let Ok(stdout) = String::from_utf8(output.stdout) {
        print!("{}", process_string(&stdout));
    }

    std::process::exit(cmd.wait().unwrap().code().unwrap_or(1));
}

use std::env;
use std::io::{self, Write};
use std::process::Command;
use std::str;

fn main() {
    loop {
        let current_dir = env::current_dir().expect("Failed to get current directory");
        print!("{}> ", current_dir.display());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let command = input.trim();
        if command.eq_ignore_ascii_case("exit") {
            break;
        }

        let output = Command::new("cmd")
            .args(&["/C", command])
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8 in output");
            println!("{}", stdout);
        } else {
            let stderr = str::from_utf8(&output.stderr).expect("Invalid UTF-8 in error output");
            eprintln!("Command failed with error: {}", stderr);
        }
    }
}

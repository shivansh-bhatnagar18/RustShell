#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::process::Command;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");
    let builtin_commands = vec!["echo", "exit", "type"];
    loop {
    // Uncomment this block to pass the first stage
        print!("$ ");
        io::stdout().flush().unwrap();

    // Wait for user input  
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        if input.trim() == "exit 0" {
            break;
        } else {
            let mut args = input.trim().split_whitespace();
            let command = args.next().unwrap();
            let args = args.skip(0);
            if command == "echo" {
                println!("{}", args.collect::<Vec<&str>>().join(" "));
            } else if command == "type" {
                if builtin_commands.contains(&args.clone().next().unwrap()) {
                    println!("{} is a shell builtin", args.clone().next().unwrap());
                } else {
                    match env::var("PATH")
                    .unwrap().split(":")
                    .map(|path| format!("{}/{}", path, args.clone().next().unwrap()))
                    .find(|path| std::fs::metadata(path).is_ok()) {
                        Some(path) => println!("{} is {}", args.clone().next().unwrap(), path),
                        None => println!("{} not found", args.clone().next().unwrap())
                    }
                }
            } else {
                match env::var("PATH")
                .unwrap().split(":")
                .map(|path| format!("{}/{}", path, command))
                .find(|path| std::fs::metadata(path).is_ok()) {
                    Some(path) => {
                        let output = Command::new(path)
                        .args(args)
                        .output()
                        .expect("Failed to execute command");
                        io::stdout().write_all(&output.stdout).unwrap();
                        io::stderr().write_all(&output.stderr).unwrap();
                    },
                    None => println!("{}: command not found", command)
                }
            }
        }
    }
}

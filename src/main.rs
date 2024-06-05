#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");
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
            } else {
                println!("{}: command not found", command);
            }
        }
    }
}

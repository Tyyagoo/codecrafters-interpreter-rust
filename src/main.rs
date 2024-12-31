mod token;

use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let mut had_error = false;

            if !file_contents.is_empty() {
                let mut scanner = token::Scanner::new(&file_contents);

                while let Some(maybe_token) = scanner.next() {
                    match maybe_token {
                        Ok(token) => println!("{}", token),
                        Err(reason) => {
                            had_error = true;
                            writeln!(io::stderr(), "{}", reason).unwrap();
                        },
                    }
                    
                }
            }
            println!("EOF  null");

            if had_error {
                std::process::exit(65);
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

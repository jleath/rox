#![allow(dead_code)]

use clap::Parser;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process;

mod scanner;
mod token;

use crate::scanner::{Scanner, ScannerError};

struct Interpreter {
    had_error: bool,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter { had_error: false }
    }
    fn run_file(&mut self, path: &Path) {
        match fs::read_to_string(path) {
            Ok(source) => {
                self.run(source);
                if self.had_error {
                    process::exit(65);
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                process::exit(64);
            }
        }
    }

    fn run_prompt(&mut self) {
        loop {
            let mut line = String::from("");
            print!("> ");
            io::stdout().flush().unwrap();

            // If user input is invalid (ex: non utf8), panic
            io::stdin()
                .read_line(&mut line)
                .expect("Error reading input");

            self.run(String::from(line.trim()));
            self.had_error = false;
        }
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source);

        match scanner.scan_tokens() {
            Ok(tokens) => {
                for token in tokens {
                    println!("{:?}", token);
                }
            }
            Err(e) => match e {
                ScannerError::UnterminatedString(line) => self.error(line, "Unterminated String"),
                ScannerError::UnexpectedChar(line) => self.error(line, "Unexpected character"),
            },
        }
    }

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, location: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, location, message);
        self.had_error = true;
    }
}

#[derive(Parser)]
#[clap(author, version, about)]
struct InterpreterArgs {
    /// Path to the lox script to run
    #[clap(value_parser)]
    script_path: Option<String>,
}

fn main() {
    let mut interpreter = Interpreter::new();
    let cli = InterpreterArgs::parse();
    match cli.script_path {
        Some(path) => interpreter.run_file(Path::new(&path)),
        None => interpreter.run_prompt(),
    }
}

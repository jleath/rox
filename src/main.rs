#![allow(dead_code)]

use clap::Parser;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process;

mod astprinter;
mod expr;
mod parser;
mod scanner;
mod token;

use crate::astprinter::AstPrinter;
use crate::parser::ParseError;
use crate::parser::Parser as LoxParser;
use crate::scanner::{Scanner, ScannerError};
use crate::token::{Token, TokenType};

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
                let mut parser = LoxParser::new(tokens.to_owned());
                let result = parser.parse();
                if result.is_err() {
                    match result.unwrap_err() {
                        ParseError::UnbalancedParens(token, message)
                        | ParseError::UnknownPrimary(token, message) => {
                            self.parse_error(token, &message);
                        }
                    }
                } else {
                    println!("{}", AstPrinter::new().print(*(result.unwrap())));
                }
            }
            Err(e) => match e {
                ScannerError::UnterminatedString(line) => {
                    self.scan_error(line, "Unterminated String")
                }
                ScannerError::UnexpectedChar(line) => self.scan_error(line, "Unexpected character"),
            },
        }
    }

    fn scan_error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn parse_error(&mut self, token: Token, message: &str) {
        if token.token_type() == TokenType::Eof {
            self.report(token.line(), " at end", message);
        } else {
            self.report(
                token.line(),
                format!(" at '{}'", token.lexeme()).as_str(),
                message,
            );
        }
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

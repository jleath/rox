use clap::Parser;
use std::fs;
use std::path::Path;
use std::process;

struct Interpreter {}

impl Interpreter {
    fn run_file(&mut self, path: &Path) {
        match fs::read_to_string(path) {
            Ok(source) => self.run(&source),
            Err(e) => {
                eprintln!("{}", e);
                process::exit(64);
            }
        }
    }

    fn run_prompt(&mut self) {
        todo!()
    }

    fn run(&mut self, _source: &str) {
        todo!();
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
    let mut interpreter = Interpreter {};
    let cli = InterpreterArgs::parse();
    match cli.script_path {
        Some(path) => interpreter.run_file(Path::new(&path)),
        None => interpreter.run_prompt(),
    }
}

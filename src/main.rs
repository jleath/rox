use clap::Parser;
use std::path::Path;

struct Interpreter {}

impl Interpreter {
    fn run_file(&mut self, _path: &Path) {
        todo!()
    }

    fn run_prompt(&mut self) {
        todo!()
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

mod scanner;
mod token;
mod token_kind;

use scanner::{Scanner, SyntaxError};
use std::{
    env, fs,
    io::{self, Write},
};

fn main() {
    CommandLine::new().start();
}

struct CommandLine {
    args: Vec<String>,
}

impl CommandLine {
    fn new() -> Self {
        let args: Vec<String> = env::args().skip(1).collect();

        Self { args }
    }

    fn start(self) {
        match self.args.len() {
            0 => run_prompt(),
            1 => run_file(self.args[0].as_str()),
            _ => panic!("Usage: rlox [script]"),
        }
    }
}

fn run_file(path: &str) {
    let source_code = fs::read_to_string(path).unwrap();
    let result = run(source_code.as_str());
    if let Err(errors) = result {
        report(errors);
    }
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let result = run(line.as_str());
        if let Err(errors) = result {
            report(errors);
        }
    }
}

fn run(source: &str) -> Result<(), Vec<SyntaxError>> {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        if let Some(literal) = &token.literal {
            if let Some(number) = literal.downcast_ref::<f64>() {
                println!("{:?}, literal: {:?}", token, number);
            } else if let Some(text) = literal.downcast_ref::<String>() {
                println!("{:?}, literal: {:?}", token, text);
            }
        } else {
            println!("{:?}", token);
        }
    }

    Ok(())
}

fn report(errors: Vec<SyntaxError>) {
    for error in errors {
        eprintln!("{:?}", error);
    }
}

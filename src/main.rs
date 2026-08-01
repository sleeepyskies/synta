mod lexer;
mod parser;

use crate::{
    lexer::{Lexer, TokenKind},
    parser::Parser,
};
use clap::Parser as ClapParser;
use std::fs;

#[derive(ClapParser, Debug)]
#[command(version, about)]
struct Args {
    /// Path to the script file, if not present, synta will run in interactive mode
    #[arg(short, long)]
    script_file: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(file) = args.script_file {
        let Ok(source) = fs::read_to_string(&file) else {
            println!("Could not read from file {file:?}");
            return;
        };
        run_script(&source);
    } else {
        run_interactive();
    }
}

#[allow(clippy::unwrap_used)]
fn run_script(script: &str) {
    let tokens: Vec<TokenKind> = Lexer::new(script)
        .map(|token| token.unwrap().kind)
        .collect();

    let mut parser = Parser::new(tokens);
    let expressions = parser.parse();
    println!("{expressions:?}");
}

fn run_interactive() {
    println!("Interactive mode is not yet supported");
}

mod lexer;

use crate::lexer::Lexer;
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Path to the script file, if not present, synta will run in interactive mode
    #[arg(short, long)]
    script_file: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(script_file) = args.script_file {
        let source = fs::read_to_string(script_file).expect("Could not open script file.");
        run_script(&source);
    } else {
        run_interactive();
    }
}

fn run_script(script: &str) {
    let _ = Lexer::new(script);
}

fn run_interactive() {
    unimplemented!("Not done yet :D");
}

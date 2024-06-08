#[macro_use]
mod scanner;
mod loxi;
mod repl;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl::init(),
        v if v > 1 => {
            println!("Usage: loxi [script]")
        }
        _ => {
            // TODO: Read the file
        }
    }
}

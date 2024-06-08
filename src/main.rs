#[macro_use]
mod scanner;
mod loxi;
mod repl;

use std::{env, fs::File, io::Read, process};

use loxi::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            println!("{:?}", args);
            run_file(&args[1]);
        }
        v if v > 2 => {
            println!("Usage: loxi [script]");
            process::exit(64);
        }
        _ => {
            repl::init();
        }
    }
}

fn run_file(f: &str) {
    let mut f = File::open(f).expect("error in opening file");
    let mut contents = Vec::new();
    f.read_to_end(&mut contents).expect("error in reading file");

    let contents = String::from_utf8(contents).expect("file is not encoded in utf8");

    run(&contents)
}

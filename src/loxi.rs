use crate::scanner::{Scanner, ScannerError};
use std::io::{Result as IoResult, Write};

pub fn run(program: &str) {
    let tokens = Scanner::new(program);

    for token in tokens {
        println!("{:?}", token);
    }
}

fn print_parser_errors<W: Write>(mut out: W, errors: &[ScannerError]) -> IoResult<()> {
    for error in errors {
        out.write_fmt(format_args!(
            "\tline: {} | error: {}\n",
            error.line, error.message
        ))
        .unwrap();
    }
    out.flush()
}

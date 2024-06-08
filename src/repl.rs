use crate::loxi::run;
use std::{
    io::{self, BufRead, Write},
    sync::mpsc,
    time::Duration,
};

const PROMPT: &[u8] = b">> ";

pub fn init() {
    let stdin = io::stdin();
    let read_handle = stdin.lock();
    let stdout = io::stdout();
    let write_handle = stdout.lock();

    start(read_handle, write_handle);
}

fn start<R: BufRead, W: Write>(mut ip: R, mut out: W) {
    let (send, recv) = mpsc::channel();

    let mut should_quit = false;
    ctrlc::set_handler(move || {
        send.send(()).expect("error in sending signal to channel");
    })
    .expect("error in setting Ctrl+C handler");

    loop {
        out.write_all(PROMPT).unwrap();
        out.flush().unwrap();
        if recv.recv_timeout(Duration::from_millis(5)).is_ok() {
            if should_quit {
                std::process::exit(0);
            }
            should_quit = true;
            out.write_all(b"\r                                                          \r")
                .unwrap();
            continue;
        } else {
            should_quit = false;
        }

        let mut s = String::new();
        ip.read_line(&mut s).unwrap();

        run(&s);
    }
}

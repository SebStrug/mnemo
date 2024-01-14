use std::io::Stdout;
use std::process;

use termion::raw::RawTerminal;

pub fn exit_gracefully(stdout: &mut RawTerminal<Stdout>) {
    stdout.suspend_raw_mode().unwrap();
    // Newline required otherwise zsh prints a '%' to denote a missing newline
    println!("");
    process::exit(1);
}

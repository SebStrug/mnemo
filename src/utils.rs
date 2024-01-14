use std::io::{Stdout, Write};
use std::process;

use termion::cursor;
use termion::raw::RawTerminal;

pub fn exit_gracefully(stdout: &mut RawTerminal<Stdout>) {
    write!(stdout, "{}", cursor::Show).unwrap();
    stdout.suspend_raw_mode().unwrap();
    // Newline required otherwise zsh prints a '%' to denote a missing newline
    println!("");
    process::exit(1);
}

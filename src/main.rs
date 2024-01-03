use std::io::{stdin, stdout, Write};
use std::{env, fs, process::exit};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Requires one argument: the name of the text. See `-l` for options")
    } else if args.len() > 2 {
        panic!("Should only pass one argument: the name of the text")
    }
    let arg: &String = &args[1];

    let mut text_index: usize = 0;
    match arg.as_str() {
        "-h" | "--help" => help(),
        "-l" | "--list" => list_texts(),
        _ => ()
    }
    let text: Vec<String> = collect_text(arg);

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        "{}{}Mnemo!\nText: {}\nq to exit.{}",
        // Clear screen, go to start, hide cursor
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        arg,
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::CurrentLine,
        )
        .unwrap();

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('c') => {
                write!(stdout, "{}", &text[text_index]).unwrap();
                text_index += 1;
            },
            _ => write!(stdout, "{}", "Unhandled char").unwrap(),
        }

        stdout.flush().unwrap();
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn help() {
    println!("Mnemo is a tiny app to help you memorise texts like poetry or book openings");
    println!("Save the text into 'texts/' and then run it");
    exit(1);
}

fn list_texts() {
    println!("Available texts:");
    let paths = fs::read_dir("texts/").unwrap();
    for path in paths {
        println!("Text: {:?}", path.unwrap().path().file_stem().unwrap())
    }
    exit(1);
}

fn collect_text(query: &str) -> Vec<String> {
    let mut text_fpath = "texts/".to_owned();

    text_fpath.push_str(query);
    text_fpath.push_str(".txt");

    let contents = fs::read_to_string(text_fpath).expect("Should have been able to read the file");
    // Each string has to be owned
    contents.split('\n').map(|s| s.to_owned()).collect()
}

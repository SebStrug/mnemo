use std::fs;
use std::io::{stdin, stdout, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

enum NavCommand {
    FromBeginning,
    PrevLine,
    NextLine,
    ToEnd,
    // General controls
    Quit,
    Help,
    ListTexts,
}

impl NavCommand {
    fn from_event(char: &Key) -> Option<Self> {
        match char {
            Key::Char('z') => Some(Self::FromBeginning),
            Key::Char('x') => Some(Self::PrevLine),
            Key::Char('c') => Some(Self::NextLine),
            Key::Char('v') => Some(Self::ToEnd),
            Key::Char('q') => Some(Self::Quit),
            Key::Char('h') => Some(Self::Help),
            Key::Char('l') => Some(Self::ListTexts),
            _ => None,
        }
    }
}

fn main() {
    // let text: Vec<String> = collect_text(arg);

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        "{}{}Mnemo!{}q to exit.{}l to list texts.{}",
        // Clear screen, go to start, hide cursor
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Goto(1, 2),
        termion::cursor::Goto(1, 3),
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

        match NavCommand::from_event(&c.unwrap()) {
            Some(NavCommand::Quit) => break,
            Some(NavCommand::Help) =>  write!(
                stdout,
                "{}{}Mnemo is a tiny app to help you memorise short texts like poems, book openings, or quotes.{}Save the text into 'texts/' and then run Mnemo",
                termion::clear::All,
                termion::cursor::Goto(1, 1),
                termion::cursor::Goto(1, 3),
            ).unwrap(),
            Some(NavCommand::ListTexts) => write!(
                stdout,
                "{}{}Available texts: {:?}",
                termion::clear::All,
                termion::cursor::Goto(1, 1),
                collect_all_texts()
            ).unwrap(),
            // Some(NavCommand::NextLine) => {
            //     write!(stdout, "{}", &text[text_index]).unwrap();
            //     text_index += 1;
            // }
            _ => write!(stdout, "Unhandled char").unwrap(),
        }

        stdout.flush().unwrap();
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn collect_all_texts() -> Vec<String> {
    let dir_entries = fs::read_dir("texts/").unwrap();
    let mut text_paths: Vec<String> = Vec::new();
    for dir_entry in dir_entries {
        let path = dir_entry.unwrap().path();
        let text_path = path.file_stem().unwrap().to_str().unwrap();
        text_paths.push(text_path.to_string());
    }
    text_paths
}

fn collect_text(query: &str) -> Vec<String> {
    let mut text_fpath = "texts/".to_owned();

    text_fpath.push_str(query);
    text_fpath.push_str(".txt");

    let contents = fs::read_to_string(text_fpath).expect("Should have been able to read the file");
    // Each string has to be owned
    contents.split('\n').map(|s| s.to_owned()).collect()
}

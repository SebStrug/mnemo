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
    GetText,
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
            Key::Char('e') => Some(Self::GetText),
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
        "{}{}Mnemo!{}q to exit.{}l to list texts.{}e to enter a text.{}",
        // Clear screen, go to start, hide cursor
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Goto(1, 2),
        termion::cursor::Goto(1, 3),
        termion::cursor::Goto(1, 4),
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();

    // Vars to help with entering text
    let mut text_input_mode = false;
    let mut text_from_keys = String::new();
    let mut stdout_index: u16 = 1;

    // Vars to help with iterating through text
    let mut text: Option<Vec<String>> = None;
    let mut curr_line: usize = 0;

    for c in stdin.keys() {
        let key = &c.as_ref().unwrap();

        if text_input_mode {
            match key {
                Key::Char('\n') => {
                    text_input_mode = false;
                    text = Some(collect_text(&text_from_keys));
                    write!(
                        stdout,
                        "{}{}Entered text: {}{}Get the next line with 'c'",
                        termion::clear::All,
                        termion::cursor::Goto(1, 1),
                        &text_from_keys,
                        termion::cursor::Goto(1, 3),
                    )
                    .unwrap();
                }
                Key::Char(ch) => {
                    let writer =
                        write!(stdout, "{}{}", termion::cursor::Goto(stdout_index, 1), &ch);
                    text_from_keys.push(*ch);
                    stdout_index += 1;
                    writer.unwrap();
                }
                _ => {
                    write!(stdout, "Unrecognised character").unwrap();
                    break;
                }
            }
        } else {
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
                Some(NavCommand::GetText) => {
                    text_input_mode = true;
                },
                Some(NavCommand::NextLine) => {
                    if let Some(t) = &text {
                        curr_line += 1;
                        let line = &t[curr_line];
                        write!(
                            stdout,
                            "{}{}",
                            termion::cursor::Goto(1, (curr_line+1) as u16),
                            line
                        ).unwrap();
                    }
                }
                _ => write!(stdout, "Unhandled key").unwrap(),
            }
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
    // Artisanal hand-crafted path
    let mut text_fpath = "texts/".to_owned();
    text_fpath.push_str(query);
    text_fpath.push_str(".txt");

    let contents = fs::read_to_string(text_fpath).expect("No such text found");
    // Each string has to be owned
    contents.split('\n').map(|s| s.to_owned()).collect()
}

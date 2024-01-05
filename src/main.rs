use std::fs;
use std::io::{stdin, stdout, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::style;

enum NavCommand {
    FromBeginning,
    PrevLine,
    NextLine,
    ToEnd,
    // General controls
    Quit,
    Help,
    ListTexts,
    EnterText,
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
            Key::Char('e') => Some(Self::EnterText),
            _ => None,
        }
    }
}

struct Text {
    lines: Vec<String>,
    curr_line_ind: usize,
    curr_word_ind: usize,
    length: usize,
}

impl Text {
    pub fn new(text: Vec<String>) -> Text {
        let len = &text.len();
        Text {
            lines: text,
            curr_line_ind: 0,
            curr_word_ind: 0,
            length: *len,
        }
    }

    pub fn get_curr_line(&self) -> Option<&str> {
        if self.curr_line_ind < self.length {
            Some(&self.lines[self.curr_line_ind])
        } else {
            None
        }
    }
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        "{}{}{bold}{italic}Mnemo!{style_reset}
        {goto2}{bold}q{style_reset} to exit.
        {goto3}{bold}h{style_reset} for help.
        {goto4}{bold}l{style_reset} to list texts.
        {goto5}{bold}e{style_reset} to enter a text.{hide_cursor}",
        // Clear screen, go to start, hide cursor
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        bold=style::Bold,
        italic=style::Italic,
        style_reset=style::Reset,
        goto2=termion::cursor::Goto(1, 2),
        goto3=termion::cursor::Goto(1, 3),
        goto4=termion::cursor::Goto(1, 4),
        goto5=termion::cursor::Goto(1, 5),
        hide_cursor=termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();

    // Vars to help with entering text
    let mut text_input_mode = false;
    let mut text_from_keys = String::new();
    let mut stdout_index: u16 = 1;

    // Vars to help with iterating through text
    let mut text: Option<Text> = None;

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
                        write!(stdout, "{}{}", termion::cursor::Goto(stdout_index, 2), &ch);
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
                termion::clear::All,
            )
            .unwrap();

            match NavCommand::from_event(&c.unwrap()) {
                Some(NavCommand::Quit) => break,
                Some(NavCommand::Help) =>  write!(
                    stdout,
                    "{}{}{bold}{italic}Mnemo{reset} is a tiny app to help you memorise short texts like poems, book openings, or quotes.{}Save the text into {italic}'texts/'{reset} and then run {bold}{italic}Mnemo{reset}",
                    termion::clear::All,
                    termion::cursor::Goto(1, 1),
                    termion::cursor::Goto(1, 3),
                    bold=style::Bold,
                    italic=style::Italic,
                    reset=style::Reset,
                ).unwrap(),
                Some(NavCommand::ListTexts) => write!(
                    stdout,
                    "{}{}Available texts: {:?}",
                    termion::clear::All,
                    termion::cursor::Goto(1, 1),
                    collect_all_texts()
                ).unwrap(),
                Some(NavCommand::EnterText) => {
                    text_input_mode = true;
                    write!(stdout, "{}{}Entering text:", 
                    termion::clear::All,
                    termion::cursor::Goto(1, 1)
                ).unwrap();
                },
                Some(NavCommand::NextLine) => {
                    if let Some(t) = &mut text {
                        t.curr_line_ind += 1;
                        if let Some(l) = t.get_curr_line() {
                            write!(
                                stdout,
                                "{}{}",
                                termion::cursor::Goto(1, (t.curr_line_ind+1) as u16),
                                l
                            ).unwrap();
                        } else {
                            t.curr_line_ind -= 1;
                        }
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

fn collect_text(query: &str) -> Text {
    // Artisanal hand-crafted path
    let mut text_fpath = "texts/".to_owned();
    text_fpath.push_str(query);
    text_fpath.push_str(".txt");

    let contents = fs::read_to_string(text_fpath).expect("No such text found");
    // Each string has to be owned
    let text = contents.split('\n').map(|s| s.to_owned()).collect();
    Text::new(text)
}

use std::fs;
use std::io::{stdin, stdout, Stdout, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::style;

enum NavMenu {
    Quit,
    Help,
    ListTexts,
    EnterText,
}

impl NavMenu {
    fn from_event(char: &Key) -> Option<Self> {
        match char {
            Key::Char('q') => Some(Self::Quit),
            Key::Char('h') => Some(Self::Help),
            Key::Char('l') => Some(Self::ListTexts),
            Key::Char('e') => Some(Self::EnterText),
            _ => None,
        }
    }
}

enum NavCommand {
    FromBeginning,
    PrevLine,
    NextLine,
    ToEnd,
}

impl NavCommand {
    fn from_event(char: &Key) -> Option<Self> {
        match char {
            Key::Char('z') => Some(Self::FromBeginning),
            Key::Char('x') => Some(Self::PrevLine),
            Key::Char('c') => Some(Self::NextLine),
            Key::Char('v') => Some(Self::ToEnd),
            _ => None,
        }
    }
}

struct MnemoState {
    entering_text: bool,
    navigating_text: bool,
    requested_text: String,
    stdout_index: u16,
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
    intro_message(&mut stdout);
    stdout.flush().unwrap();

    let mut state = MnemoState {
        entering_text: false,
        navigating_text: false,
        requested_text: String::new(),
        stdout_index: 1,
    };
    let mut text = Text {
        lines: Vec::new(),
        curr_line_ind: 0,
        curr_word_ind: 0,
        length: 0,
    };

    for c in stdin.keys() {
        let key = &c.as_ref().unwrap();

        // Main menu
        match (state.entering_text, state.navigating_text, NavMenu::from_event(key)) {
            (false, _, Some(NavMenu::Quit)) => break,
            (false, false, Some(NavMenu::Help)) =>  write!(
                stdout,
                "{}{}{bold}{italic}Mnemo{reset} is a tiny app to help you memorise short texts like poems, book openings, or quotes.{}Save the text into {italic}'texts/'{reset} and then run {bold}{italic}Mnemo{reset}",
                termion::clear::All,
                termion::cursor::Goto(1, 1),
                termion::cursor::Goto(1, 3),
                bold=style::Bold,
                italic=style::Italic,
                reset=style::Reset,
            ).unwrap(),
            (false, false, Some(NavMenu::ListTexts)) => write!(
                stdout,
                "{}{}Available texts: {:?}",
                termion::clear::All,
                termion::cursor::Goto(1, 1),
                collect_all_texts()
            ).unwrap(),
            (false, false, Some(NavMenu::EnterText)) => {
                state.entering_text = true;
                write!(stdout, "{}{}Entering text:", 
                    termion::clear::All,
                    termion::cursor::Goto(1, 1)
                ).unwrap();
                // Flush and continue otherwise first character of entered text will always be 'e'
                stdout.flush().unwrap();
                continue;
            },
            (false, false, _) => {
                write!(stdout, "Unhandled character").unwrap();
            },
            _ => (),
        }

        // Entering a text to mnemorise
        match (state.entering_text, key) {
            (true, Key::Char('\n')) => {
                state.entering_text = false;
                state.navigating_text = true;
                text = collect_text(&state.requested_text);
                write!(
                    stdout,
                    "{}{}Entered text: {}
                    {}Get the next line with 'c'
                    {}Get the previous line with 'x'
                    ",
                    termion::clear::All,
                    termion::cursor::Goto(1, 1),
                    &state.requested_text,
                    termion::cursor::Goto(1, 3),
                    termion::cursor::Goto(1, 4),
                )
                .unwrap();
            }
            (true, Key::Char(ch)) => {
                let writer = write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Goto(state.stdout_index, 2),
                    &ch
                );
                state.requested_text.push(*ch);
                state.stdout_index += 1;
                writer.unwrap();
            }
            (true, Key::Backspace) => {
                state.stdout_index -= 1;
                state.requested_text.pop();
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Goto(state.stdout_index, 2),
                    termion::clear::AfterCursor
                )
                .unwrap();
            }
            _ => (),
        }

        // Navigating the text
        match (state.navigating_text, NavCommand::from_event(&c.unwrap())) {
            (true, Some(NavCommand::NextLine)) => {
                // Clear the leftover helping info about how to navigate a text, when we state navigating
                if text.curr_line_ind == 0 {
                    write!(stdout, "{}", termion::clear::All).unwrap();
                }

                // We may have no more lines to print!
                if let Some(l) = text.get_curr_line() {
                    write!(
                        stdout,
                        "{}{}",
                        termion::cursor::Goto(1, (text.curr_line_ind + 1) as u16),
                        l
                    )
                    .unwrap();
                    text.curr_line_ind += 1;
                }
            }
            (true, Some(NavCommand::PrevLine)) => {
                if text.curr_line_ind == 0 {
                    write!(stdout, "{}", termion::clear::All).unwrap();
                } else if text.curr_line_ind > 0 {
                    write!(
                        stdout,
                        "{}{}",
                        termion::clear::CurrentLine,
                        termion::cursor::Goto(1, text.curr_line_ind as u16)
                    )
                    .unwrap();
                    text.curr_line_ind -= 1;
                }
            }
            _ => (),
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
    let text_fpath = &text_fpath[..];

    let contents = match fs::read_to_string(text_fpath) {
        Ok(c) => c,
        Err(_) => {
            println!("\nNo such text found {:?}", &text_fpath);
            panic!("No text found");
        }
    };
    // Each string has to be owned
    let text = contents.split('\n').map(|s| s.to_owned()).collect();
    Text::new(text)
}

fn intro_message(stdout: &mut RawTerminal<Stdout>) {
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
        bold = style::Bold,
        italic = style::Italic,
        style_reset = style::Reset,
        goto2 = termion::cursor::Goto(1, 2),
        goto3 = termion::cursor::Goto(1, 3),
        goto4 = termion::cursor::Goto(1, 4),
        goto5 = termion::cursor::Goto(1, 5),
        hide_cursor = termion::cursor::Hide
    )
    .unwrap();
}

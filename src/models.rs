use std::io::{Stdout, Write};
use termion::clear;
use termion::color;
use termion::cursor;
use termion::event::Key;
use termion::raw::RawTerminal;

pub enum NavMenu {
    Quit,
    Help,
    ListTexts,
    EnterText,
    MainMenu,
}

impl NavMenu {
    pub fn from_event(char: &Key) -> Option<Self> {
        match char {
            Key::Char('q') => Some(Self::Quit),
            Key::Char('h') => Some(Self::Help),
            Key::Char('l') => Some(Self::ListTexts),
            Key::Char('e') => Some(Self::EnterText),
            Key::Char('m') => Some(Self::MainMenu),
            _ => None,
        }
    }
}

pub enum NavCommand {
    FromBeginning,
    PrevLine,
    NextLine,
    NextWord,
}

impl NavCommand {
    pub fn from_event(char: &Key) -> Option<Self> {
        match char {
            Key::Char('z') => Some(Self::FromBeginning),
            Key::Char('x') => Some(Self::PrevLine),
            Key::Char('c') => Some(Self::NextLine),
            Key::Char('v') => Some(Self::NextWord),
            _ => None,
        }
    }
}

pub struct MnemoState {
    pub entering_text: bool,
    pub navigating_text: bool,
    pub requested_text: String,
    pub stdout_index: u16,
}

pub struct StdoutState {
    pub curr_row: u16,
    pub curr_col: u16,
}

impl StdoutState {
    pub fn clear_all(&mut self, stdout: &mut RawTerminal<Stdout>) {
        write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    }

    pub fn move_to_next_line(&mut self, stdout: &mut RawTerminal<Stdout>) {
        self.curr_row += 1;
        write!(stdout, "{}", cursor::Goto(1, self.curr_row)).unwrap();
    }

    pub fn move_to_prev_line(&mut self, stdout: &mut RawTerminal<Stdout>) {
        self.curr_row -= 1;
        write!(stdout, "{}", cursor::Goto(1, self.curr_row)).unwrap();
    }

    // Clear current line & go to start of line
    pub fn reset_curr_line(&mut self, stdout: &mut RawTerminal<Stdout>) {
        write!(
            stdout,
            "{}{}",
            cursor::Goto(1, self.curr_row),
            clear::CurrentLine,
        )
        .unwrap();
    }
}

#[derive(Clone)]
pub struct Line {
    pub words: Vec<String>,
    pub length: usize,
}

pub struct Text {
    pub lines: Vec<Line>,
    // The current line is actually the line *to be shown*
    pub curr_line_ind: usize,
    pub curr_word_ind: usize,
    pub length: usize,
    pub prev_key: Option<Key>,
}

impl Text {
    pub fn new(text: Vec<Line>) -> Text {
        let len = &text.len();
        Text {
            lines: text,
            curr_line_ind: 0,
            curr_word_ind: 0,
            length: *len,
            prev_key: None,
        }
    }

    pub fn get_line_by_ind(&self, ind: &usize) -> Option<Line> {
        if (ind < &0) || (ind > &(&self.length - 1)) {
            None
        } else {
            Some(self.lines[*ind].clone())
        }
    }

    pub fn get_line(&self, line_ind: &usize) -> Option<String> {
        if let Some(l) = self.get_line_by_ind(line_ind) {
            Some(l.words.join(" "))
        } else {
            None
        }
    }

    pub fn get_line_up_to_word(&self, line_ind: &usize, up_to_word_ind: &usize) -> Option<String> {
        if let Some(l) = self.get_line_by_ind(line_ind) {
            if *up_to_word_ind < l.words.len() {
                Some(l.words[..*up_to_word_ind].join(" "))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_word(&self, line_ind: &usize, word_ind: &usize) -> Option<String> {
        if let Some(l) = self.get_line_by_ind(line_ind) {
            if let Some(w) = l.words.get(*word_ind) {
                Some(w.to_owned())
            } else {
                None
            }
        } else {
            None
        }
    }

    // Text struct and stdout state are linked
    pub fn redisplay_current_line(
        &mut self,
        stdout: &mut RawTerminal<Stdout>,
        stdout_state: &mut StdoutState,
    ) {
        write!(
            stdout,
            "{}{}{}",
            clear::CurrentLine,
            cursor::Goto(1, stdout_state.curr_row),
            self.get_line(&self.curr_line_ind).unwrap(),
        )
        .unwrap();
    }

    pub fn show_curr_line(&mut self, stdout: &mut RawTerminal<Stdout>) {
        write!(
            stdout,
            "{}{}{}",
            color::Fg(color::LightCyan),
            self.get_line(&self.curr_line_ind).unwrap(),
            color::Fg(color::Reset),
        )
        .unwrap();
    }

    pub fn show_line(&mut self, stdout: &mut RawTerminal<Stdout>, l: &String) {
        write!(
            stdout,
            "{}{}{}",
            color::Fg(color::LightCyan),
            l,
            color::Fg(color::Reset),
        )
        .unwrap();
    }
}

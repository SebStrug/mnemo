use std::fs;
use std::io::{stdin, stdout, Stdout, Write};
use std::process;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::style;
use termion::{clear, color, cursor};

enum NavMenu {
    Quit,
    Help,
    ListTexts,
    EnterText,
    MainMenu,
}

impl NavMenu {
    fn from_event(char: &Key) -> Option<Self> {
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

enum NavCommand {
    FromBeginning,
    PrevLine,
    NextLine,
    NextWord,
}

impl NavCommand {
    fn from_event(char: &Key) -> Option<Self> {
        match char {
            Key::Char('z') => Some(Self::FromBeginning),
            Key::Char('x') => Some(Self::PrevLine),
            Key::Char('c') => Some(Self::NextLine),
            Key::Char('v') => Some(Self::NextWord),
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

#[derive(Clone)]
struct Line {
    words: Vec<String>,
}

struct Text {
    lines: Vec<Line>,
    curr_line_ind: usize,
    curr_word_ind: usize,
    length: usize,
}

impl Text {
    pub fn new(text: Vec<Line>) -> Text {
        let len = &text.len();
        Text {
            lines: text,
            curr_line_ind: 0,
            curr_word_ind: 0,
            length: *len,
        }
    }

    fn get_line_by_ind(&self, ind: &usize) -> Option<Line> {
        if (ind < &0) | (ind > &(&self.length - 1)) {
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
        let key = &c.unwrap();

        // Classic developer panic
        if key == &Key::Ctrl('c') {
            break;
        };

        // Main menu
        match (state.entering_text, state.navigating_text, NavMenu::from_event(key)) {
            (false, _, Some(NavMenu::Quit)) => break,
            (false, false, Some(NavMenu::Help)) =>  write!(
                stdout,
                "{}{}{bold}{italic}Mnemo{reset} is a tiny app to help you memorise short texts like poems, book openings, or quotes.{}Save the text into {italic}'texts/'{reset} and then run {bold}{italic}Mnemo{reset}
                {}Press 'm' to go back to the main menu",
                termion::clear::All,
                termion::cursor::Goto(1, 1),
                termion::cursor::Goto(1, 3),
                termion::cursor::Goto(1, 5),
                bold=style::Bold,
                italic=style::Italic,
                reset=style::Reset,
            ).unwrap(),
            (false, _, Some(NavMenu::MainMenu)) => intro_message(&mut stdout),
            (false, false, Some(NavMenu::ListTexts)) => write!(
                stdout,
                "{}{}Available texts: {:?}{}Press 'm' to go back to the main menu",
                termion::clear::All,
                termion::cursor::Goto(1, 1),
                collect_all_texts(),
                termion::cursor::Goto(1, 3),
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
                write!(stdout, "{}Unhandled character, press 'm' to go back to the main manu", termion::clear::All).unwrap();
            },
            _ => (),
        }

        // Entering a text to mnemorise
        match (state.entering_text, key) {
            (true, Key::Char('\n')) => {
                state.entering_text = false;
                state.navigating_text = true;
                text = collect_text(&state.requested_text, &mut stdout);
                if text.length == 0 {
                    break;
                }
                write!(
                    stdout,
                    "{}{}Entered text: {}
                    {}Get the next line with 'c'
                    {}Get the previous line with 'x'
                    {}Get the next word with 'v'
                    ",
                    termion::clear::All,
                    termion::cursor::Goto(1, 1),
                    &state.requested_text,
                    termion::cursor::Goto(1, 3),
                    termion::cursor::Goto(1, 4),
                    termion::cursor::Goto(1, 5),
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
        match (state.navigating_text, NavCommand::from_event(key)) {
            (true, Some(NavCommand::NextLine)) => {
                // When entering navigation mode, clear the leftover helping info about how to navigate a text
                // if (text.curr_word_ind == 0) & (text.curr_line_ind == 0) {
                //     write!(stdout, "{}", termion::clear::All).unwrap();
                // }

                // // Print the whole current line if we've already revealed some words
                // if text.curr_word_ind > 0 {
                //     write!(
                //         stdout,
                //         "{}{}{}",
                //         clear::CurrentLine,
                //         cursor::Goto(1, text.curr_line_ind as u16),
                //         text.get_line(&text.curr_line_ind).unwrap()
                //     )
                //     .unwrap();
                // } else if text.curr_word_ind == text.length {
                //     text.curr_line_ind += 1;
                // }

                // We may have no more lines to print
                println!("here 1");
                if let Some(l) = text.get_line(&text.curr_line_ind) {
                    // If there's a previous line, remove its color
                    // if text.curr_line_ind > 0 {
                    //     let prev_line = text.get_line(&(text.curr_line_ind - 1)).unwrap();
                    //     write!(
                    //         stdout,
                    //         "{}{}{}",
                    //         cursor::Goto(1, (text.curr_line_ind - 1) as u16),
                    //         color::Fg(color::Reset),
                    //         prev_line
                    //     ).unwrap();
                    // }
                    let writer = write!(
                        stdout,
                        "{}{}{}",
                        color::Fg(color::LightCyan),
                        termion::cursor::Goto(1, (text.curr_line_ind) as u16),
                        l
                    );
                    text.curr_line_ind += 1;
                    writer.unwrap();
                }
                // text.curr_line_ind += 1;
                text.curr_word_ind = 0;
            }
            (true, Some(NavCommand::PrevLine)) => {
                if text.curr_line_ind == 0 {
                    write!(stdout, "{}", termion::clear::All).unwrap();
                } else if text.curr_line_ind > 0 {
                    write!(stdout, "{}", termion::clear::CurrentLine).unwrap();
                    text.curr_line_ind -= 1;
                    write!(
                        stdout,
                        "{}{}{}",
                        termion::cursor::Goto(1, text.curr_line_ind as u16),
                        color::Fg(color::LightCyan),
                        text.get_line(&text.curr_line_ind).unwrap()
                    )
                    .unwrap();
                }
                text.curr_word_ind = 0;
            }
            (true, Some(NavCommand::NextWord)) => {
                // If entering navigation mode and 'v' is the first key pressed, clear the leftover text about how to navigate a text
                if (text.curr_line_ind == 0) & (text.curr_word_ind == 0) {
                    write!(stdout, "{}", termion::clear::All).unwrap();
                }

                if let Some(word) = text.get_word(&text.curr_line_ind, &text.curr_word_ind) {
                    // print previous words
                    let part_line = text
                        .get_line_up_to_word(&text.curr_line_ind, &text.curr_word_ind)
                        .unwrap();
                    let word_to_print = if text.curr_word_ind == 0 {
                        word
                    } else {
                        format!(" {}", word)
                    };
                    write!(
                        stdout,
                        "{}{}{}{}{}",
                        termion::clear::CurrentLine,
                        termion::cursor::Goto(1, text.curr_line_ind as u16),
                        part_line,
                        termion::cursor::Goto(
                            (part_line.len() + 1) as u16,
                            text.curr_line_ind as u16
                        ),
                        word_to_print,
                    )
                    .unwrap();
                    text.curr_word_ind += 1;
                }
            }
            _ => (),
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
    // Raw mode is restored going out of scope, this is just to stop zsh adding a '%' for terminal exiting without a newline
    stdout.suspend_raw_mode().unwrap();
    println!("");
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

fn collect_text(query: &str, stdout: &mut RawTerminal<Stdout>) -> Text {
    // Artisanal hand-crafted path
    let mut text_fpath = "texts/".to_owned();
    text_fpath.push_str(query);
    text_fpath.push_str(".txt");
    let text_fpath = &text_fpath[..];

    let contents = fs::read_to_string(text_fpath).unwrap_or_else(|_| {
        write!(
            stdout,
            "{}{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            "No text found at path: ",
            &text_fpath,
        )
        .unwrap();
        stdout.suspend_raw_mode().unwrap();
        println!("");
        process::exit(1);
    });

    // Each string has to be owned
    let mut all_lines: Vec<Line> = Vec::new();
    for split_line in contents.split('\n') {
        let mut words: Vec<String> = Vec::new();
        for word in split_line.split(' ') {
            words.push(word.to_string());
        }
        all_lines.push(Line { words: words });
    }
    Text::new(all_lines)
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

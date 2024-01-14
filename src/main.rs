use std::io::{stdin, stdout, Stdout, Write};

use termion::clear;
use termion::color;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::style;

mod load_text;
mod models;
mod utils;

use crate::models::{MnemoState, NavCommand, NavMenu, Text};

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
            utils::exit_gracefully(&mut stdout)
        };

        // Main menu
        match (
            state.entering_text,
            state.navigating_text,
            NavMenu::from_event(key),
        ) {
            (false, _, Some(NavMenu::Quit)) => utils::exit_gracefully(&mut stdout),
            (false, false, Some(NavMenu::Help)) => help_message(&mut stdout),
            (false, _, Some(NavMenu::MainMenu)) => intro_message(&mut stdout),
            (false, false, Some(NavMenu::ListTexts)) => write!(
                stdout,
                "{}{}Available texts: {:?}{}Press 'm' to go back to the main menu",
                clear::All,
                cursor::Goto(1, 1),
                load_text::collect_all_texts(),
                cursor::Goto(1, 3),
            )
            .unwrap(),
            (false, false, Some(NavMenu::EnterText)) => {
                state.entering_text = true;
                write!(stdout, "{}{}Entering text:", clear::All, cursor::Goto(1, 1)).unwrap();
                // Flush and continue otherwise first character of entered text will always be 'e'
                stdout.flush().unwrap();
                continue;
            }
            (false, false, _) => {
                write!(
                    stdout,
                    "{}{}{}",
                    clear::All,
                    cursor::Goto(1, 1),
                    "Unhandled character, press 'm' to go back to the main manu"
                )
                .unwrap();
            }
            _ => (),
        }

        // Entering a text to mnemorise
        match (state.entering_text, key) {
            (true, Key::Char('\n')) => {
                state.entering_text = false;
                state.navigating_text = true;
                text = load_text::collect_text(&state.requested_text, &mut stdout);
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
                    clear::All,
                    cursor::Goto(1, 1),
                    &state.requested_text,
                    cursor::Goto(1, 3),
                    cursor::Goto(1, 4),
                    cursor::Goto(1, 5),
                )
                .unwrap();
            }
            (true, Key::Char(ch)) => {
                let writer = write!(stdout, "{}{}", cursor::Goto(state.stdout_index, 2), &ch);
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
                    cursor::Goto(state.stdout_index, 2),
                    clear::AfterCursor
                )
                .unwrap();
            }
            _ => (),
        }

        // Navigating the text
        match (state.navigating_text, NavCommand::from_event(key)) {
            (true, Some(NavCommand::NextLine)) => {
                // When entering navigation mode, clear the leftover helping info about how to navigate a text
                if (text.curr_word_ind == 0) & (text.curr_line_ind == 0) {
                    write!(stdout, "{}", clear::All).unwrap();
                }

                // Print the whole current line if we've already revealed some words
                if text.curr_word_ind > 0 {
                    write!(
                        stdout,
                        "{}{}{}",
                        clear::CurrentLine,
                        cursor::Goto(1, text.curr_line_ind as u16),
                        text.get_line(&text.curr_line_ind).unwrap()
                    )
                    .unwrap();
                } else if text.curr_word_ind == text.length {
                    text.curr_line_ind += 1;
                }
                // We may have no more lines to print
                if let Some(l) = text.get_line(&text.curr_line_ind) {
                    // If there's a previous line, remove its color
                    if text.curr_line_ind > 0 {
                        let prev_l = text.get_line(&(text.curr_line_ind - 1)).unwrap();
                        write!(
                            stdout,
                            "{}{}{}",
                            color::Fg(color::Reset),
                            cursor::Goto(1, (text.curr_line_ind) as u16),
                            prev_l
                        )
                        .unwrap();
                    }
                    write!(
                        stdout,
                        "{}{}{}",
                        color::Fg(color::LightCyan),
                        cursor::Goto(1, (text.curr_line_ind + 1) as u16),
                        l
                    )
                    .unwrap();
                    text.curr_line_ind += 1;
                }
                // text.curr_line_ind += 1;
                text.curr_word_ind = 0;
            }
            (true, Some(NavCommand::PrevLine)) => {
                if text.curr_line_ind == 0 {
                    write!(stdout, "{}", clear::All).unwrap();
                } else if text.curr_line_ind == 1 {
                    write!(stdout, "{}", clear::CurrentLine).unwrap();
                    text.curr_line_ind -= 1;
                } else if text.curr_line_ind > 0 {
                    write!(stdout, "{}", clear::CurrentLine).unwrap();
                    text.curr_line_ind -= 1;
                    write!(
                        stdout,
                        "{}{}{}{}",
                        cursor::Goto(1, text.curr_line_ind as u16),
                        clear::CurrentLine,
                        color::Fg(color::LightCyan),
                        text.get_line(&(text.curr_line_ind - 1)).unwrap()
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

                if (text.curr_word_ind == 0) || (text.curr_word_ind == text.length) {
                    text.curr_line_ind += 1;
                    write!(stdout, "{}", cursor::Goto(1, text.curr_line_ind as u16)).unwrap();
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

    write!(stdout, "{}", cursor::Show).unwrap();
    // Raw mode is restored going out of scope, this is just to stop zsh adding a '%' for terminal exiting without a newline
    stdout.suspend_raw_mode().unwrap();
    println!("");
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
        clear::All,
        cursor::Goto(1, 1),
        bold = style::Bold,
        italic = style::Italic,
        style_reset = style::Reset,
        goto2 = cursor::Goto(1, 2),
        goto3 = cursor::Goto(1, 3),
        goto4 = cursor::Goto(1, 4),
        goto5 = cursor::Goto(1, 5),
        hide_cursor = cursor::Hide
    )
    .unwrap();
}

fn help_message(stdout: &mut RawTerminal<Stdout>) {
    write!(
        stdout,
        "{}{}{bold}{italic}Mnemo{reset} is a tiny app to help you memorise short texts like poems, book openings, or quotes.{}Save the text into {italic}'texts/'{reset} and then run {bold}{italic}Mnemo{reset}
        {}Press 'm' to go back to the main menu",
        clear::All,
        cursor::Goto(1, 1),
        cursor::Goto(1, 3),
        cursor::Goto(1, 5),
        bold=style::Bold,
        italic=style::Italic,
        reset=style::Reset,
    ).unwrap()
}

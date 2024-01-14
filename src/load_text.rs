use std::fs;
use std::io::{Stdout, Write};

use std::env;
use std::path;
use termion::raw::RawTerminal;

use crate::models::{Line, Text};
use crate::utils;

fn get_texts_dir() -> path::PathBuf {
    let mut home_dir = if cfg!(target_os = "windows") {
        env::var("USERPROFILE").ok().map(path::PathBuf::from)
    } else {
        env::var("HOME").ok().map(path::PathBuf::from)
    }
    .unwrap();
    home_dir.push(".mnemo/texts");
    home_dir
}

pub fn collect_all_texts() -> Vec<String> {
    let texts_dir = get_texts_dir();
    let dir_entries = fs::read_dir(texts_dir).unwrap();
    let mut text_paths: Vec<String> = Vec::new();
    for dir_entry in dir_entries {
        let path = dir_entry.unwrap().path();
        let text_path = path.file_stem().unwrap().to_str().unwrap();
        text_paths.push(text_path.to_string());
    }
    text_paths
}

pub fn collect_text(query: &str, stdout: &mut RawTerminal<Stdout>) -> Text {
    // Artisanal hand-crafted path
    let texts_dir = get_texts_dir();
    let mut text_fpath = texts_dir.to_owned();
    text_fpath.push(query);
    text_fpath.set_extension("txt");
    let text_fpath = text_fpath.to_str().unwrap();

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
        utils::exit_gracefully(stdout);
        String::from("")
    });

    // Each string has to be owned
    let mut all_lines: Vec<Line> = Vec::new();
    for split_line in contents.split('\n') {
        let mut words: Vec<String> = Vec::new();
        for word in split_line.split(' ') {
            words.push(word.to_string());
        }
        all_lines.push(Line {
            words: words.clone(),
            length: words.len(),
        });
    }
    Text::new(all_lines)
}

use std::fs;
use std::io::{Stdout, Write};
use std::process;

use termion::raw::RawTerminal;

use crate::models::Text;

pub fn collect_all_texts() -> Vec<String> {
    let dir_entries = fs::read_dir("texts/").unwrap();
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
    let text = contents.split('\n').map(|s| s.to_owned()).collect();
    Text::new(text)
}

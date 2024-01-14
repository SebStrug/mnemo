use termion::event::Key;

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

#[derive(Clone)]
pub struct Line {
    pub words: Vec<String>,
}

pub struct Text {
    pub lines: Vec<Line>,
    pub curr_line_ind: usize,
    pub curr_word_ind: usize,
    pub length: usize,
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
}

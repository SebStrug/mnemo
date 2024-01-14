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
    ToEnd,
}

impl NavCommand {
    pub fn from_event(char: &Key) -> Option<Self> {
        match char {
            Key::Char('z') => Some(Self::FromBeginning),
            Key::Char('x') => Some(Self::PrevLine),
            Key::Char('c') => Some(Self::NextLine),
            Key::Char('v') => Some(Self::ToEnd),
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

pub struct Text {
    pub lines: Vec<String>,
    pub curr_line_ind: usize,
    pub curr_word_ind: usize,
    pub length: usize,
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

    pub fn get_line_by_ind(&self, ind: &usize) -> Option<&str> {
        if (ind < &0) || (ind > &(&self.length - 1)) {
            None
        } else {
            Some(&self.lines[*ind])
        }
    }
}

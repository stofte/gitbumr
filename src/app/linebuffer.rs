use termion::color;
use app::console;

pub struct Cursor {
    line: u16,
    start_offset: u16,
    start_take: u16,
}

pub struct Border {
    pub top: bool,
    pub right: bool,
    pub bottom: bool,
    pub left: bool,
}

pub struct LineBuffer {
    pub valid: bool,
    pub border: Border,
    pub lines: Vec<String>,
    pub title: String,
    pub cursor: Cursor,
    pub fg: color::Fg<color::Rgb>,
    pub bg: color::Bg<color::Rgb>,
    set_idx: u16,
}

impl LineBuffer {
    pub fn title(&mut self, str: String) {
        self.title = str;
    }
    pub fn size(&mut self, width: u16, height: u16) {
        self.lines = vec!["".to_string(); height as usize];
        self.valid = false;
    }
    pub fn set(&mut self, str: String) {
        self.lines[self.set_idx as usize] = str;
        self.set_idx = self.set_idx + 1;
    }
}

pub fn build_linebuffer() -> LineBuffer {
    LineBuffer {
        valid: false,
        lines: vec![],
        title: "".to_string(),
        fg: console::FG_PRIMARY,
        bg: console::BG_PRIMARY,
        set_idx: 0,
        border: Border {
            top: true,
            right: true,
            bottom: true,
            left: true,
        },
        cursor: Cursor {
            line: 0,
            start_offset: 0,
            start_take: 0,
        }
    }
}

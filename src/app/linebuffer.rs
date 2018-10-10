use termion::color;
use app::console;

pub struct Cursor {
    line: u16,
    start_offset: u16,
    start_take: u16,
}

pub struct LineBuffer {
    pub lines: Vec<String>,
    pub cursor: Cursor,
    pub fg: color::Fg<color::Rgb>,
    pub bg: color::Bg<color::Rgb>,
}

impl LineBuffer {
    fn size(&mut self, width: u16, height: u16) {
        self.lines = vec!["".to_string(); height as usize];
    }
    fn set(&mut self, line: u16, str: String) {
        self.lines[line as usize] = str;
    }
}

pub fn build_linebuffer() -> LineBuffer {
    LineBuffer {
        lines: vec![],
        fg: console::FG_PRIMARY,
        bg: console::BG_PRIMARY,
        cursor: Cursor {
            line: 0,
            start_offset: 0,
            start_take: 0,
        }
    }
}

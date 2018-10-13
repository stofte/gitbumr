use std::io::{Write, StdoutLock};
use termion::{color, cursor};
use app::{
    console,
    logger::Logger,
    layout::{Layout},
};

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
    pub name: String,
    pub cursor: Cursor,
    pub fg: color::Fg<color::Rgb>,
    pub bg: color::Bg<color::Rgb>,
    set_idx: u16,
}

fn cursor(top: u16, left: u16, stdout: &mut StdoutLock) {
    stdout.write(format!("{}", cursor::Goto(left, top)).as_bytes());
}

impl LineBuffer {
    pub fn size(&mut self, width: u16, height: u16) {
        self.lines = vec!["".to_string(); height as usize];
        self.valid = false;
        self.set_idx = 0;
    }
    pub fn set(&mut self, str: String) {
        self.lines[self.set_idx as usize] = str;
        self.set_idx = self.set_idx + 1;
    }
    pub fn render(&mut self, stdout: &mut StdoutLock, l: &mut Logger) {
        assert!("" != self.name);
        l.log(format!("linebuffer.render:{}", self.name));
        for i in 0..self.lines.len() {
            let l = &self.lines[i];
            // cursor(layout.left, layout.top + i as u16, stdout);
            stdout.write(l.as_bytes());
        }
    }
}

pub fn build_linebuffer() -> LineBuffer {
    LineBuffer {
        valid: false,
        lines: vec![],
        name: "".to_string(),
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

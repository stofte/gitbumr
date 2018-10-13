use std::io::{Write, StdoutLock};
use termion::{color, cursor};
use app::{
    console,
    logger::Logger,
    layout::{Layout, build_empty_layout},
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
    pub id: u32,
    pub valid: bool,
    pub border: Border,
    pub layout: Layout,
    pub lines: Vec<String>,
    pub name: String,
    pub cursor: Cursor,
    pub fg: color::Fg<color::Rgb>,
    pub bg: color::Bg<color::Rgb>,
    set_idx: u16,

    // layout
    pub top: u16,
    pub left: u16,
    pub width: u16,
    pub height: u16,
    pub visible: bool,
    pub focus: bool,
}

fn cursor(top: u16, left: u16, stdout: &mut StdoutLock, log: &mut Logger) {
    // log.log(format!("pos({}x{})", left + 1, top + 1));
    stdout.write(format!("{}", cursor::Goto(left + 1, top + 1)).as_bytes());
}

impl LineBuffer {
    pub fn size(&mut self, width: u16, height: u16) {
        self.lines = vec!["".to_string(); height as usize];
        self.valid = false;
        self.set_idx = 0;
        self.width = width;
        self.height = height;
    }
    pub fn set(&mut self, str: String) {
        self.lines[self.set_idx as usize] = str;
        self.set_idx = self.set_idx + 1;
    }
    pub fn render(&mut self, stdout: &mut StdoutLock, log: &mut Logger) {
        assert!("" != self.name);
        log.log(format!("linebuffer.render:{}:{}", self.id, self.name));
        for i in 0..self.lines.len() {
            let l = &self.lines[i];
            cursor(self.layout.left, self.layout.top + i as u16, stdout, log);
            stdout.write(l.as_bytes());
        }
    }
}

pub fn build_linebuffer(id: u32) -> LineBuffer {
    LineBuffer {
        id: id,
        valid: false,
        layout: build_empty_layout(),
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
        },
        top: 0,
        left: 0,
        width: 0,
        height: 0,
        visible: false,
        focus: false,
    }
}

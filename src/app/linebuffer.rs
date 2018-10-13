use std::io::{Write, StdoutLock};
use termion::{color, cursor};
use app::{
    console,
    logger::Logger,
    layout::{Layout, build_empty_layout},
};

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
    pub lines: Vec<String>,
    pub name: String,
    pub cursor: u16,
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

fn cursor(left: u16, top: u16, stdout: &mut StdoutLock, log: &mut Logger) {
    stdout.write(format!("{}", cursor::Goto(left + 1, top + 1)).as_bytes());
}

fn set_primary_colors(stdout: &mut StdoutLock) {
    let s = format!("{fg}{bg}", 
        fg=console::FG_PRIMARY,
        bg=console::BG_PRIMARY,
    );
    stdout.write(s.as_bytes());
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
        set_primary_colors(stdout);
        for i in 0..self.lines.len() {
            let l = &self.lines[i];
            // log.log(format!("linebuffer.render:{}:{} @ {}x{}", self.id, self.name, self.left, self.top + i as u16));
            cursor(self.left, self.top + i as u16, stdout, log);
            stdout.write(l.as_bytes());
        }
    }
    pub fn cursor(&mut self, line: u16) {
        self.cursor = line;
    }
}

pub fn build_linebuffer(id: u32) -> LineBuffer {
    LineBuffer {
        id: id,
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
        cursor: 0,
        top: 0,
        left: 0,
        width: 0,
        height: 0,
        visible: false,
        focus: false,
    }
}

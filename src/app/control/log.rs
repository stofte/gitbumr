use termion::{style, cursor, event::Key};
use std::{
    cmp,
    io::Stdout,
    any::Any,
};
use git2::{Repository, BranchType, Oid};
use app::{
    console,
    Layout,
    LayoutUpdate,
    empty_layout,
    UiFlags,
    git::{get_head},
    control::{Control, RepositoryControl, InputControl},
};

pub struct Log {
    revwalk: Vec<Oid>,
    rw_idx: usize, // top revision
    cursor_idx: usize,
    layout: Layout,
}

impl Control for Log {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn layout(&mut self, layout: &LayoutUpdate) {
        self.layout.top = 2;
        self.layout.left = 36;
        self.layout.width = layout.cols.unwrap();
        self.layout.height = layout.rows.unwrap() - self.layout.top;
    }
    fn render(&self, stdout: &mut Stdout) {
        if !self.layout.visible { return }
        console::start_drawing(self.layout.left, self.layout.top, console::FG_PRIMARY, console::BG_PRIMARY);
        let title = "Log".to_string();
        let title_b_h = console::BOX_H.to_string()
            .repeat(self.layout.width as usize - title.len() - 2);
        print!("{b_h}{title}{repeat}{b_dl}",
            title=title,
            repeat=title_b_h,
            b_h=console::BOX_H,
            b_dl=console::BOX_DL,
        );
        let mut t_off = 1;
        console::move_cursor(self.layout.left, self.layout.top + t_off);
        print!("{:?} / {}", self.revwalk.len(), self.layout.height - 2);
        t_off += 1;
        for i in self.rw_idx..(self.rw_idx + self.layout.height as usize - 1) {
            console::move_cursor(self.layout.left, self.layout.top + t_off);
            let mut c_fg = console::FG_PRIMARY;
            let mut c_bg = console::BG_PRIMARY;
            if i == self.cursor_idx {
                c_bg = console::BG_PRIMARY_CURSOR;
                c_fg = console::FG_PRIMARY_CURSOR;
            }
            print!("{c_fg}{c_bg}{:?}{fg}{bg}", self.revwalk[i],
                fg=console::FG_PRIMARY,
                bg=console::BG_PRIMARY,
                c_fg=c_fg,
                c_bg=c_bg,
            );
            t_off += 1;
            
        }
        console::stop_drawing();
    }
}

impl RepositoryControl for Log {
    fn update(&mut self, repo: &Repository) {
        self.layout.visible = true;
        let mut rv = repo.revwalk().unwrap();
        rv.push_head();
        for r in rv {
            self.revwalk.push(r.unwrap());
        }
    }
    fn none(&mut self) {
        self.layout.visible = false;
    }
}

impl InputControl for Log {
    fn handle(&mut self, key: Key) -> (bool, UiFlags) {
        let pass = (false, UiFlags::None);
        let handled = (true, UiFlags::None);
        match key {
            Key::Down => {
                let mut r = pass;
                if self.cursor_idx < self.revwalk.len() {
                    self.cursor_idx += 1;
                    r = handled;
                }
                if self.cursor_idx - self.rw_idx > self.layout.height as usize - 2 {
                    self.rw_idx += 1;
                }
                r
            }
            Key::Up => {
                let mut r = pass;
                if self.cursor_idx > 0 {
                    self.cursor_idx -= 1;
                    r = handled;
                }
                if self.cursor_idx < self.rw_idx {
                    self.rw_idx -= 1;
                }
                r
            },
            _ => pass
        }
    }
}

pub fn build_log() -> Log {
    Log {
        revwalk: vec![],
        cursor_idx: 0,
        rw_idx: 0,
        layout: empty_layout(),
    }
}

use termion::{style, cursor, event::Key};
use std::{
    cmp,
    io::Stdout,
    any::Any,
};
use git2::{Time, Repository, BranchType, Oid};
use chrono::prelude::*;
use app::{
    console,
    Layout,
    LayoutUpdate,
    empty_layout,
    UiFlags,
    git::{get_head, Commit, get_commit},
    control::{Control, RepositoryControl, InputControl},
};

pub struct Log {
    revwalk: Vec<Oid>,
    rw_idx: usize, // top revision
    cursor_idx: usize,
    rw_read_bk: bool,
    rw_read_fw: bool,
    layout: Layout,
    commits: Vec<Commit>,
    tz_offset_secs: i32
}

impl Control for Log {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn layout(&mut self, layout: &LayoutUpdate) {
        self.layout.top = 2;
        self.layout.left = 36;
        match layout.cols {
            Some(c) => self.layout.width = c - self.layout.left,
            _ => ()
        };
        match layout.rows {
            Some(r) => self.layout.height = r - self.layout.top,
            _ => ()
        };
    }
    fn render(&mut self, stdout: &mut Stdout) {
        if !self.layout.visible { return }
        let mut auth_vec = vec![];
        console::start_drawing(self.layout.left, self.layout.top, console::FG_PRIMARY, console::BG_PRIMARY);
        let title = "History".to_string();
        let title_b_h = console::BOX_H.to_string()
            .repeat(self.layout.width as usize - title.len());
        print!("{b_h}{title}{repeat}",
            title=title,
            repeat=title_b_h,
            b_h=console::BOX_H
        );
        let mut t_off = 1;
        let mut c_idx = 0;
        let max_rw_c = cmp::min(self.rw_idx + self.layout.height as usize, self.revwalk.len());
        for i in self.rw_idx..max_rw_c {
            console::move_cursor(self.layout.left, self.layout.top + t_off);
            let mut c_fg = console::FG_PRIMARY;
            let mut c_bg = console::BG_PRIMARY;
            let mut txt_fg = console::FG_PRIMARY;
            if i == self.cursor_idx {
                c_bg = console::BG_PRIMARY_CURSOR;
                c_fg = console::FG_PRIMARY_CURSOR;
            } else {
                txt_fg = console::FG_LIGHT_PRIMARY;
            }
            let commit = &self.commits[c_idx];
            let c_ts = commit.time.format("%Y/%m/%d %H:%M").to_string();
            let mut c_auth = &commit.author_abbrev;
            if !auth_vec.contains(&commit.author) {
                c_auth = &commit.author;
                auth_vec.push(commit.author.to_string());
            }
            let c_size = c_ts.len() + c_auth.len() + 3;
            let msg_cols = self.layout.width as usize - c_size;
            let msg_len = cmp::min(msg_cols, commit.message_line.len());
            let c_msg: String = commit.message_line.chars().take(msg_len).collect();
            let c_msg_blank = self.layout.width as usize - c_msg.len() - c_size;
            print!("{c_fg}{c_bg} {txt_fg}{time}{fg} {msg}{blank} {txt_fg}{auth}{fg} {fg}{bg}",
                blank=" ".repeat(c_msg_blank),
                txt_fg=txt_fg,
                msg=c_msg,
                time=c_ts,
                auth=c_auth,
                fg=console::FG_PRIMARY,
                bg=console::BG_PRIMARY,
                c_fg=c_fg,
                c_bg=c_bg,
            );
            t_off += 1;
            c_idx += 1;
            
        }
        console::stop_drawing();
    }
}

impl RepositoryControl for Log {
    fn update(&mut self, repo: &Repository) {
        self.layout.visible = true;
        let mut rv = repo.revwalk().unwrap();
        rv.push_head();
        self.revwalk.clear();
        self.rw_idx = 0;
        self.cursor_idx = 0;
        for r in rv {
            self.revwalk.push(r.unwrap());
        }
        let vis_c = self.layout.height as usize;
        let max_rw_c = cmp::min(self.rw_idx + vis_c, self.revwalk.len());
        self.commits.clear();
        for i in self.rw_idx..(max_rw_c) {
            let oid = self.revwalk[i];
            let commit = get_commit(oid, self.tz_offset_secs, &repo);
            self.commits.push(commit);
        }
    }
    fn read(&mut self, repo: &Repository) {
        if self.rw_read_fw {
            self.commits.remove(0);
            let oid = self.revwalk[self.cursor_idx];
            let commit = get_commit(oid, self.tz_offset_secs, &repo);
            self.commits.push(commit);
            self.rw_read_fw = false;
        } else if self.rw_read_bk {
            self.commits.pop();
            let oid = self.revwalk[self.cursor_idx];
            let commit = get_commit(oid, self.tz_offset_secs, &repo);
            self.commits.insert(0, commit);
            self.rw_read_bk = false;
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
        let handled_read = (true, UiFlags::RequestRepository);
        match key {
            Key::Down => {
                let mut r = pass;
                if self.cursor_idx < self.revwalk.len() - 1 {
                    self.cursor_idx += 1;
                    r = handled;
                }
                if self.cursor_idx - self.rw_idx > self.layout.height as usize - 1 {
                    self.rw_idx += 1;
                    self.rw_read_fw = true;
                    r = handled_read;
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
                    self.rw_read_bk = true;
                    r = handled_read;
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
        rw_read_bk: false,
        rw_read_fw: false,
        commits: vec![],
        tz_offset_secs: 2 * 60 * 60,
    }
}

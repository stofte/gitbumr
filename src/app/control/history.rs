use std::{
    cmp,
    io::Stdout,
};
use git2::{Oid};
use termion::event::Key;
use app::{
    git, console,
    settings::get_timesize_offset_secs,
    control::Control,
    event::{KeyArg, ConsumeArg, Event, EventArg},
    layout::{Layout, build_empty_layout},
    logger::Logger,
    linebuffer::{LineBuffer, build_linebuffer},
};

pub struct History {
    pub id: u32,
    revwalk: Vec<Oid>,
    rw_idx: usize, // top revision
    cursor_idx: usize,
    rw_read_bk: bool,
    rw_read_fw: bool,
    commits: Vec<git::Commit>,
    layout: Layout,
    tz_offset_secs: i32,
    buffer: LineBuffer,
}

impl Control for History {
    fn id(&self) -> u32 { self.id }
    fn buffer(&mut self) -> &mut LineBuffer { &mut self.buffer }
    fn render(&mut self, log: &mut Logger) {
        log.log(format!("history.render"));
        let mut auth_vec = vec![];
        let mut c_idx = 0;
        let max_rw_c = cmp::min(self.rw_idx + self.layout.height as usize, self.revwalk.len());
        for i in self.rw_idx..max_rw_c {
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
            self.buffer.set(format!("{c_fg}{c_bg} {txt_fg}{time}{fg} {msg}{blank} {txt_fg}{auth}{fg} {fg}{bg}",
                blank=" ".repeat(c_msg_blank),
                txt_fg=txt_fg,
                msg=c_msg,
                time=c_ts,
                auth=c_auth,
                fg=console::FG_PRIMARY,
                bg=console::BG_PRIMARY,
                c_fg=c_fg,
                c_bg=c_bg,
            ));
            c_idx += 1;
            
        }
        self.buffer.valid = true;
    }
    fn key(&mut self, k: Key, log: &mut Logger) -> KeyArg {
        log.log(format!("history.key"));
        let pass = KeyArg::Pass;
        let handled = KeyArg::Consumed(ConsumeArg::None);
        let handled_repo = KeyArg::Consumed(ConsumeArg::Repository);
        match k {
            Key::Up => {
                log.log(format!("history.key => key::up"));
                let mut r = pass;
                if self.cursor_idx > 0 {
                    self.cursor_idx -= 1;
                }
                if self.cursor_idx < self.rw_idx {
                    self.rw_idx -= 1;
                    self.rw_read_bk = true;
                    return handled_repo;
                }
                handled
            }
            Key::Down => {
                log.log(format!("history.key => key::down"));
                if self.cursor_idx < self.revwalk.len() - 1 {
                    self.cursor_idx += 1;
                }
                if self.cursor_idx - self.rw_idx > self.layout.height as usize - 1 {
                    self.rw_idx += 1;
                    self.rw_read_fw = true;
                    return handled_repo
                }
                handled
            }
            _ => pass
        }
    }
    fn ctx(&mut self, e: &mut Event, log: &mut Logger) -> EventArg {
        log.log(format!("history.ctx"));
        match e {
            Event::Start(_, r, cols, rows) => {
                self.layout.top = 2;
                self.layout.left = 36;
                self.layout.width = *cols - self.layout.left;
                self.layout.height = *rows - self.layout.top;
                self.buffer.size(self.layout.width, self.layout.height);
                match r {
                    Some(repo) => {
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
                            let commit = git::get_commit(oid, self.tz_offset_secs, &repo);
                            self.commits.push(commit);
                        }
                    },
                    None => ()
                }
            }
            Event::Focus(id) => self.layout.focus = *id == self.id,
            Event::ConsoleResize(cols, rows) => {
                self.layout.width = *cols - self.layout.left;
                self.layout.height = *rows - self.layout.top;
                self.buffer.size(self.layout.width, self.layout.height);
            }
            _ => ()
        };
        EventArg::None
    }
}

pub fn build_history(id: u32) -> History {
    let mut h = History {
        id: id,
        revwalk: vec![],
        cursor_idx: 0,
        rw_idx: 0,
        rw_read_bk: false,
        rw_read_fw: false,
        commits: vec![],
        tz_offset_secs: get_timesize_offset_secs(),
        layout: build_empty_layout(),
        buffer: build_linebuffer(),
    };
    h.buffer.name = "history".to_string();
    h
}

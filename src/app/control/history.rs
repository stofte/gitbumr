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
    event::{ConsumeArg, Event, EventArg},
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
    tz_offset_secs: i32,
}

impl History {
    fn input(&mut self, k: &Key, buf: &mut LineBuffer, log: &mut Logger) -> EventArg {
        log.log(format!("history.key"));
        let pass = EventArg::None;
        let handled = EventArg::InputConsumed(ConsumeArg::None);
        let handled_repo = EventArg::InputConsumed(ConsumeArg::Repository);
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
                // if self.cursor_idx - self.rw_idx > self.layout.height as usize - 1 {
                //     self.rw_idx += 1;
                //     self.rw_read_fw = true;
                //     return handled_repo
                // }
                handled
            }
            _ => pass
        }
    }
}

impl Control for History {
    fn id(&self) -> u32 { self.id }
    fn render(&mut self, buffer: &mut LineBuffer, log: &mut Logger) {
        assert_eq!(buffer.id, self.id);
        log.log(format!("history.render"));
        let mut auth_vec = vec![];
        let mut c_idx = 0;
        let max_rw_c = cmp::min(self.rw_idx + buffer.height as usize, self.revwalk.len());
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
            let msg_cols = buffer.width as usize - c_size - 1;
            let msg_len = cmp::min(msg_cols, commit.message_line.len());
            let c_msg: String = commit.message_line.chars().take(msg_len).collect();
            let c_msg_blank = buffer.width as usize - c_msg.len() - c_size - 1;
            buffer.set(format!("{c_fg}{c_bg} {txt_fg}{time}{fg} {msg}{blank} {txt_fg}{auth}{fg} {fg}{bg}",
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
        buffer.valid = true;
    }
    fn ctx(&mut self, e: &mut Event, buffer: &mut LineBuffer, log: &mut Logger) -> EventArg {
        assert_eq!(buffer.id, self.id);
        log.log(format!("history.ctx"));
        match e {
            Event::Start(_, r, cols, rows) => {
                buffer.top = 1;
                buffer.left = 34;
                let b_top = buffer.top;
                let b_left = buffer.left;
                buffer.size(*cols - b_left, *rows - b_top);
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
                        let vis_c = buffer.height as usize;
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
            Event::Focus(id) => buffer.focus = *id == self.id,
            Event::ConsoleResize(cols, rows) => {
                let b_top = buffer.top;
                let b_left = buffer.left;
                buffer.size(*cols - b_left, *rows - b_top);
            }
            Event::Input(k) => {
                return self.input(k, buffer, log);
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
    };
    h
}

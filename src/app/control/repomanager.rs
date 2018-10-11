use std::{
    cmp,
    io::{Stdout},
};
use termion::{
    cursor,
    event::Key,
};
use app::{
    console,
    settings::StoredRepository,
    layout::{Layout, build_empty_layout},
    event::{Event, ConsumeArg, KeyArg, EventArg},
    control::Control,
    logger::Logger,
    linebuffer::{LineBuffer, build_linebuffer},
};

pub struct RepoManager {
    pub id: u32,
    pub repos: Vec<StoredRepository>,
    pub layout: Layout,
    pub adding: bool,
    pub repo_path: Option<String>,
    pub input_txt: Vec<char>,
    pub input_cursor: u16,
    pub repo_cursor: u16,
    pub add_err: Option<String>,
    pub open_repo: Option<i64>,
    buffer: LineBuffer,
}

impl Control for RepoManager {
    fn id(&self) -> u32 { self.id }
    fn buffer(&mut self) -> &mut LineBuffer { &mut self.buffer }
    fn render(&mut self, _stdout: &mut Stdout, log: &mut Logger) {
        if !self.layout.visible { return }
        log.log(format!("repomgr.render"));
        if self.repos.len() == 0 {
            let txt = "  No repositories found".to_string();
            self.buffer.set(format!("{txt}{blank}", 
                txt=txt,
                blank=" ".repeat(self.layout.width as usize - txt.len()),
            ));
        }
        for i in 0..self.repos.len() {
            let repo = &self.repos[i];
            let txt = &repo.path;
            let mut open_mark = ' ';
            if repo.open {
                open_mark = console::PNT_R;
            }
            let mut c_fg = console::FG_PRIMARY;
            let mut c_bg = console::BG_PRIMARY;
            if i as u16 == self.repo_cursor && !self.adding {
                c_bg = console::BG_PRIMARY_CURSOR;
                c_fg = console::FG_PRIMARY_CURSOR;
            }
            self.buffer.set(format!("  {open_m}{c_fg}{c_bg}{txt}{blank}{fg}{bg}  ",
                txt=txt,
                open_m=open_mark,
                blank=" ".repeat(self.layout.width as usize - txt.len() - 7),
                fg=console::FG_PRIMARY,
                bg=console::BG_PRIMARY,
                c_fg=c_fg,
                c_bg=c_bg,
            ));
        }
        self.buffer.valid = true;
    }
    fn key(&mut self, k: Key, log: &mut Logger) -> KeyArg {
        log.log(format!("repomgr.key"));
        match k {
            Key::Char(c) => {
                if c == 'r' {
                    self.layout.visible = !self.layout.visible;
                    log.log(format!("repomgr visibility toggled to {}", self.layout.visible));
                    return KeyArg::Consumed(ConsumeArg::None)
                } else if c == 'a' && self.layout.visible && !self.adding {
                    self.adding = true;
                    return KeyArg::InputEdit(self.id, 1, 1, 1)
                } else if c == '\n' {
                    if self.layout.visible && !self.adding && self.repos.len() > 0 {
                        self.open_repo = Some(self.repos[self.repo_cursor as usize].id);
                        self.layout.visible = false;
                        let id = self.open_repo.unwrap();
                        log.log(format!("repomgr opening repo toggled to {}", &id));
                        return KeyArg::OpenRepository(id)
                    }
                }
                KeyArg::Pass
            }
            Key::Esc => {
                if self.adding {
                    self.adding = false;
                    return KeyArg::Consumed(ConsumeArg::None)
                } else if self.layout.visible {
                    self.layout.visible = false;
                    return KeyArg::Consumed(ConsumeArg::None)
                }
                KeyArg::Pass
            }
            Key::Up => {
                if self.layout.visible {
                    if !self.adding && self.repos.len() > 0 && self.repo_cursor > 0 {
                        self.repo_cursor -= 1;
                    }
                    return KeyArg::Consumed(ConsumeArg::None)
                }
                KeyArg::Pass
            }
            Key::Down => {
                if self.layout.visible {
                    if !self.adding && self.repos.len() > 0 {
                        self.repo_cursor = cmp::min(self.repos.len() as u16 - 1, self.repo_cursor + 1);
                    }
                    return KeyArg::Consumed(ConsumeArg::None)
                }
                KeyArg::Pass
            }
            _ => KeyArg::Pass
        }
        
    }
    fn ctx(&mut self, e: &mut Event, log: &mut Logger) -> EventArg {
        log.log(format!("repomgr.ctx"));
        match e {
            Event::Start(s, _, cols, rows) => {
                self.layout.top = 3;
                self.layout.left = 5;
                self.layout.width = *cols - 2 * (self.layout.left - 1);
                self.layout.height = *rows - 2 * (self.layout.top - 1);
                self.buffer.size(self.layout.width, self.layout.height);
                match s {
                    Some(settings) => {
                        self.repos = settings.get_repositories();
                    },
                    _ => ()
                };
            }
            Event::ConsoleResize(cols, rows) => {
                self.layout.width = *cols - 2 * (self.layout.left - 1);
                self.layout.height = *rows - 2 * (self.layout.top - 1);
                self.buffer.size(self.layout.width, self.layout.height);
            }
            Event::EditorInput(ref s) => {
                log.log(format!("repomgr.ctx input => {}", s));
                self.repo_path = Some(s.to_string());
            }
            Event::Focus(id) => self.layout.focus = *id == self.id,
            Event::Repository(_, ref s) => {
                self.repos = s.get_repositories();
            }
            _ => ()
        }
        EventArg::None
    }
}

fn print_blank(l: &Layout, top: u16) {
    print!("{move}{b_v}{blank}{b_v}", 
        move=cursor::Goto(l.left, l.top + top),
        blank=" ".repeat(l.width as usize - 2),
        b_v=console::BOX_V,
    );
}

pub fn build_repomanager(id: u32) -> RepoManager {
    RepoManager {
        id: id,
        repos: vec![],
        layout: build_empty_layout(),
        repo_path: None,
        input_txt: vec![],
        input_cursor: 0,
        repo_cursor: 0,
        add_err: None,
        open_repo: None,
        adding: false,
        buffer: build_linebuffer(),
    }
}

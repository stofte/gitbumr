pub mod git;
pub mod console;
pub mod settings;
pub mod control;

use std::{
    io::{Write, Stdout}
};
use git2::Repository;
use termion::{
    cursor,
    terminal_size,
    raw::RawTerminal,
    event::Key,
};
use bitflags;
use channel;

use app::{
    settings::{Settings, build_settings},
    control::{
        Control,
        RepositoryControl,
        SettingsControl,
        InputControl,
        header::{Header, build_header},
        branches::{Branches, build_branches},
        repomanager::{RepoManager, build_repomanager},
        log::{Log, build_log},
    },
};

pub struct Application {
    header: Header,
    branches: Branches,
    repomanager: RepoManager,
    log: Log,
}

pub struct Layout {
    pub top: u16,
    pub left: u16,
    pub width: u16,
    pub height: u16,
    pub visible: bool,
    pub console_rows: u16,
    pub console_cols: u16,
}

pub struct LayoutUpdate {
    pub rows: Option<u16>,
    pub cols: Option<u16>,
    pub invalidated: Option<bool>,
}

bitflags! {
    pub struct UiFlags: u32 {
        const None              = 0;
        const HideCursor        = 0b00000001;
        const AddedRepository   = 0b00000010;
        const OpenRepository    = 0b00000100;
        const RequestRepository = 0b00001000;
        const WindowClosed      = 0b00010000;
        const InputConsumed     = 0b00100000;
    }
}

impl Application {
    pub fn repository(&mut self, repo: &Repository) {
        self.header.update(repo);
        self.branches.update(repo);
        self.log.update(repo);
    }
    pub fn no_repository(&mut self) {
        self.header.none();
        self.branches.none();
        self.log.none();
    }
    pub fn settings(&mut self, settings: &mut Settings) -> UiFlags {
        let mut res = UiFlags::None;
        res |= self.repomanager.update(settings);
        res
    }
    pub fn console_size(&mut self) {
        let (size_col, size_row) = terminal_size().unwrap();
        let l = LayoutUpdate { cols: Some(size_col), rows: Some(size_row), invalidated: None };
        self.header.layout(&l);
        self.branches.layout(&l);
        self.repomanager.layout(&l);
        self.log.layout(&l);
    }
    pub fn invalidate(&mut self) {
        let l = LayoutUpdate { cols: None, rows: None, invalidated: Some(true) };
        self.header.layout(&l);
        self.branches.layout(&l);
        self.repomanager.layout(&l);
        self.log.layout(&l);
    }
    pub fn render(&mut self, stdout: &mut Stdout) {
        self.log.render(stdout);
        self.branches.render(stdout);
        self.repomanager.render(stdout);
        self.header.render(stdout);
        stdout.flush().unwrap();
    }
    pub fn key(&mut self, key: Key, repo: Option<&Repository>) -> UiFlags {
        let mut res = UiFlags::None;
        res |= self.repomanager.key(key, res);
        res |= self.log.key(key, res);
        if res & UiFlags::RequestRepository == UiFlags::RequestRepository {
            match repo {
                Some(r) => self.log.read(&r),
                _ => ()
            }
        }
        res
    }
    pub fn run(&mut self, mut stdout: RawTerminal<Stdout>, keys_r: channel::Receiver<Key>, size_r: channel::Receiver<(u16, u16)>) {
        let mut db = build_settings();
        db.init();
        let mut repo = git_repo_opt(&db);
        self.console_size();
        match &repo {
            Some(r) => self.repository(r),
            _ => self.no_repository()
        };
        self.settings(&mut db);
        console::reset();
        let mut invalidated = true;
        loop  {
            self.render(&mut stdout);
            invalidated = false;
            select! {
                recv(keys_r, key) => {
                    let c = key.unwrap();
                    match c {
                        Key::Ctrl('c') => break,
                        _ => ()
                    };
                    // if we didn't break, pass the input to the controls
                    let mut e = match &repo {
                        Some(r) => self.key(c, Some(r)),
                        _ => self.key(c, None)
                    };
                    e |= self.settings(&mut db);
                    if e & UiFlags::HideCursor == UiFlags::HideCursor {
                        print!("{}", cursor::Hide);
                    }
                    if e & UiFlags::AddedRepository == UiFlags::AddedRepository ||
                    e & UiFlags::OpenRepository == UiFlags::OpenRepository {
                        repo = git_repo_opt(&db);
                        match &repo {
                            Some(r) => self.repository(r),
                            _ => self.no_repository()
                        };
                    }
                    if e & UiFlags::WindowClosed == UiFlags::WindowClosed {
                        self.invalidate();
                    }
                },
                recv(size_r, size) => {
                    console::reset();
                    self.console_size();
                }
            }
        }
        console::reset();
        write!(stdout, "{}", cursor::Show).unwrap();
    }
}

pub fn empty_layout() -> Layout {
    Layout { top: 0, left: 0, width: 0, height: 0, visible: false, console_rows: 0, console_cols: 0 }
}

pub fn new_application() -> Application {
    let mut app = Application {
        header: build_header(),
        branches: build_branches(),
        repomanager: build_repomanager(),
        log: build_log(),
    };
    app
}

fn git_repo_opt(db: &Settings) -> Option<Repository> {
    let mut repo: Option<Repository> = None;
    match db.get_open_repository() {
        Some(sr) => {
            match Repository::open(sr.path) {
                Ok(gr) => {
                    repo = Some(gr);
                },
                _ => (),
            };
        },
        _ => ()
    };
    repo
}

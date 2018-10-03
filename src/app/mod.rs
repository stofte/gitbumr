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
    pub controls: Vec<Box<Control>>,
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
    }
}

impl Application {
    pub fn add_control(&mut self, ctrl: Box<Control>) {
        self.controls.push(ctrl);
    }
    pub fn repository(&mut self, repo: &Repository) {
        for cp in &mut self.controls {
            let c = &mut *cp;
            match c.as_any_mut().downcast_mut::<Header>() {
                Some(ref mut o) => { o.update(repo); continue },
                None => ()
            }
            match c.as_any_mut().downcast_mut::<Branches>() {
                Some(ref mut o) => { o.update(repo); continue },
                None => ()
            };
            match c.as_any_mut().downcast_mut::<Log>() {
                Some(ref mut o) => { o.update(repo); continue },
                None => ()
            };
        };
    }
    pub fn no_repository(&mut self) {
        for cp in &mut self.controls {
            let c = &mut *cp;
            match c.as_any_mut().downcast_mut::<Header>() {
                Some(ref mut o) => { o.none(); continue },
                None => ()
            }
            match c.as_any_mut().downcast_mut::<Branches>() {
                Some(ref mut o) => { o.none(); continue },
                None => ()
            };
            match c.as_any_mut().downcast_mut::<Log>() {
                Some(ref mut o) => { o.none(); continue },
                None => ()
            };
        };
    }
    pub fn settings(&mut self, settings: &mut Settings) -> UiFlags {
        let mut res = UiFlags::None;
        for cp in &mut self.controls {
            let mut matched = false;
            let c = &mut *cp;
            match c.as_any_mut().downcast_mut::<RepoManager>() {
                Some(ref mut o) => {
                    matched = true;
                    let f = o.update(settings);
                    if f & UiFlags::HideCursor == UiFlags::HideCursor {
                        print!("{}", cursor::Hide);
                    }
                    if f & UiFlags::AddedRepository == UiFlags::AddedRepository {
                        res = UiFlags::AddedRepository;
                    }
                },
                None => ()
            };
        };
        res
    }
    pub fn console_size(&mut self) {
        let (size_col, size_row) = terminal_size().unwrap();
        let l = LayoutUpdate { cols: Some(size_col), rows: Some(size_row), invalidated: None };
        for c in &mut self.controls {
            c.layout(&l);
        }
    }
    pub fn invalidate(&mut self) {
        let l = LayoutUpdate { cols: None, rows: None, invalidated: Some(true) };
        for c in &mut self.controls {
            c.layout(&l);
        }
    }
    pub fn render(&mut self, stdout: &mut Stdout) {
        for i in (0..self.controls.len()).rev() {
            self.controls[i].render(stdout);
        }
        stdout.flush().unwrap();
    }
    pub fn key(&mut self, key: Key, repo: Option<&Repository>) -> UiFlags {
        let mut res = UiFlags::None;
        for cp in &mut self.controls {
            let c = &mut *cp;
            match c.as_any_mut().downcast_mut::<RepoManager>() {
                Some(ref mut o) => {
                    let (handled, fs) = o.handle(key);
                    if fs & UiFlags::HideCursor == UiFlags::HideCursor {
                        print!("{}", cursor::Hide);
                    }
                    if fs & UiFlags::OpenRepository == UiFlags::OpenRepository {
                        res |= UiFlags::OpenRepository;
                    }
                    if fs & UiFlags::WindowClosed == UiFlags::WindowClosed {
                        res |= UiFlags::WindowClosed;
                    }
                    if (handled) {
                        break
                    } else {
                        continue
                    }
                },
                None => ()
            }
            match c.as_any_mut().downcast_mut::<Log>() {
                Some(ref mut o) => {
                    let (handled, fs) = o.handle(key);
                    if fs & UiFlags::RequestRepository == UiFlags::RequestRepository {
                        match repo {
                            Some(r) => o.read(&r),
                            _ => ()
                        }
                        // o.read(&repo.unwrap());
                    }
                    if (handled) {
                        break
                    } else {
                        continue
                    }
                },
                None => ()
            }
        };
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
        controls: vec![],
        header: build_header(),
        branches: build_branches(),
        repomanager: build_repomanager(),
        log: build_log(),
    };
    // order of insertion is z-index, latter being higher
    app.add_control(Box::new(build_repomanager()));
    app.add_control(Box::new(build_header()));
    app.add_control(Box::new(build_log()));
    app.add_control(Box::new(build_branches()));
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

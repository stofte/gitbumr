pub mod git;
pub mod console;
pub mod settings;
pub mod control;

use std::{
    io::{Write, Stdout}
};
use git2::Repository;
use termion::{terminal_size, event::Key, cursor};
use bitflags;
use app::{
    settings::{Settings},
    control::{
        Control,
        RepositoryControl,
        SettingsControl,
        InputControl,
        header::Header,
        branches::Branches,
        repomanager::{RepoManager, build_repomanager},
        log::{Log, build_log},
    },
};

pub struct Application {
    pub controls: Vec<Box<Control>>,
    pub cursor: bool,
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
}

bitflags! {
    pub struct UiFlags: u32 {
        const None              = 0;
        const HideCursor        = 0b00000001;
        const AddedRepository   = 0b00000010;
        const OpenRepository    = 0b00000100;
    }
}

impl Application {
    pub fn add_control(&mut self, ctrl: Box<Control>) {
        self.controls.push(ctrl);
    }
    pub fn repository(&mut self, r: Option<Repository>) {
        match r {
            Some(repo) => {
                for cp in &mut self.controls {
                    let mut matched = false;
                    let c = &mut *cp;
                    match c.as_any_mut().downcast_mut::<Header>() {
                        Some(ref mut o) => { matched = true; o.update(&repo); },
                        None => ()
                    }
                    if matched { continue; }
                    match c.as_any_mut().downcast_mut::<Branches>() {
                        Some(ref mut o) => { matched = true; o.update(&repo); },
                        None => ()
                    };
                    if matched { continue; }
                    match c.as_any_mut().downcast_mut::<Log>() {
                        Some(ref mut o) => { matched = true; o.update(&repo); },
                        None => ()
                    };
                };
            },
            None => {
                for cp in &mut self.controls {
                    let mut matched = false;
                    let c = &mut *cp;
                    match c.as_any_mut().downcast_mut::<Header>() {
                        Some(ref mut o) => { matched = true; o.none(); },
                        None => ()
                    }
                    if matched { continue; }
                    match c.as_any_mut().downcast_mut::<Branches>() {
                        Some(ref mut o) => { matched = true; o.none(); },
                        None => ()
                    };
                    if matched { continue; }
                    match c.as_any_mut().downcast_mut::<Log>() {
                        Some(ref mut o) => { matched = true; o.none(); },
                        None => ()
                    };
                };
            }
        }
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
        let l = LayoutUpdate { cols: Some(size_col), rows: Some(size_row) };
        for c in &mut self.controls {
            c.layout(&l);
        }
    }
    pub fn render(&self, stdout: &mut Stdout) {
        for c in &self.controls {
            c.render(stdout);
        }
        stdout.flush().unwrap();
    }
    pub fn key(&mut self, key: Key) -> UiFlags {
        let mut res = UiFlags::None;
        for cp in &mut self.controls {
            let mut matched = false;
            let c = &mut *cp;
            match c.as_any_mut().downcast_mut::<RepoManager>() {
                Some(ref mut o) => {
                    let (handled, fs) = o.handle(key);
                    matched = handled;
                    if fs & UiFlags::HideCursor == UiFlags::HideCursor {
                        print!("{}", cursor::Hide);
                    }
                    if fs & UiFlags::OpenRepository == UiFlags::OpenRepository {
                        res = UiFlags::OpenRepository;
                    }
                },
                None => ()
            }
            if matched { break; }
        };
        res
    }
}

pub fn empty_layout() -> Layout {
    Layout { top: 0, left: 0, width: 0, height: 0, visible: false, console_rows: 0, console_cols: 0 }
}

pub fn new_application() -> Application {
    let mut app = Application {
        cursor: false,
        controls: vec![]
    };
    // order of insertion is z-index, latter being higher
    app.add_control(Box::new(Header { repo_path: "".to_string(), state: "".to_string(), layout: empty_layout() }));
    app.add_control(Box::new(build_log()));
    let mut c = Branches { local: vec![], remote: vec![], checkedout_idx: None, layout: empty_layout() };
    c.layout.visible = true;
    app.add_control(Box::new(c));
    app.add_control(Box::new(build_repomanager()));
    app
}

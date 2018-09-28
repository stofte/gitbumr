pub mod git;
pub mod console;
pub mod db;
pub mod repo;
pub mod control;

use std::{
    io::{Write, Stdout}
};
use git2::Repository;
use termion::{terminal_size, event::Key, cursor};
use app::{
    db::{Database},
    control::{
        Control,
        RepositoryControl,
        DatabaseControl,
        InputControl,
        UiOption,
        header::Header,
        branches::Branches,
        repomanager::RepoManager
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

impl Application {
    pub fn add_control(&mut self, ctrl: Box<Control>) {
        self.controls.push(ctrl);
    }
    pub fn repository(&mut self, repo: &Repository) {
        for cp in &mut self.controls {
            let mut matched = false;
            let c = &mut *cp;
            match c.as_any_mut().downcast_mut::<Header>() {
                Some(ref mut o) => { matched = true; o.update(repo); },
                None => ()
            }
            if matched { continue; }
            match c.as_any_mut().downcast_mut::<Branches>() {
                Some(ref mut o) => { matched = true; o.update(repo); },
                None => ()
            };
        };
    }
    pub fn database(&mut self, db: &Database) {
        for cp in &mut self.controls {
            let mut matched = false;
            let c = &mut *cp;
            match c.as_any_mut().downcast_mut::<RepoManager>() {
                Some(ref mut o) => { matched = true; o.update(db); },
                None => ()
            }
        };
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
            if c.render(stdout) { // show cursor

            } else { // didn't show cursor

            }
        }
        stdout.flush().unwrap();
    }
    pub fn key(&mut self, key: Key) {
        for cp in &mut self.controls {
            let mut matched = false;
            let c = &mut *cp;
            match c.as_any_mut().downcast_mut::<RepoManager>() {
                Some(ref mut o) => {
                    let (handled, ui_opt) = o.handle(key);
                    matched = handled;
                    match ui_opt {
                        UiOption::HideCursor => {
                            print!("{}", cursor::Hide);
                        },
                        UiOption::None => ()
                    };
                },
                None => ()
            }
            if matched { break; }
        };
    }
}

fn empty_layout() -> Layout {
    Layout { top: 0, left: 0, width: 0, height: 0, visible: false, console_rows: 0, console_cols: 0 }
}

pub fn new_application() -> Application {
    let mut app = Application {
        cursor: false,
        controls: vec![]
    };
    // order of insertion is z-index, latter being higher
    app.add_control(Box::new(Header { repo_path: "".to_string(), state: "".to_string(), layout: empty_layout() }));
    let mut c = Branches { local: vec![], remote: vec![], checkedout_idx: None, layout: empty_layout() };
    c.layout.visible = true;
    app.add_control(Box::new(c));
    app.add_control(Box::new(RepoManager { repos: vec![], layout: empty_layout(), adding: false, input_txt: vec![] }));
    app
}

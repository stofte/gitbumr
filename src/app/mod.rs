pub mod git;
pub mod console;
pub mod db;
pub mod repo;
pub mod control;
pub mod color;

use std::{
    io::{Write, Stdout}
};
use git2::Repository;
use termion::{terminal_size};
use app::{
    db::{Database},
    control::{Control, RepositoryControl, DatabaseControl, header::Header, branches::Branches, repos::Repos},
};

pub struct UpdateData<'a, 'b> {
    pub console_width: Option<u16>,
    pub console_height: Option<u16>,
    pub key_value: Option<char>,
    pub git_repo: Option<&'a Repository>,
    pub db: Option<&'b Database<'b>>,
}

pub struct Application {
    pub controls: Vec<Box<Control>>
}

pub struct Layout {
    pub top: u16,
    pub left: u16,
    pub width: u16,
    pub height: u16,
}

pub struct LayoutUpdate {
    pub top: Option<u16>,
    pub left: Option<u16>,
    pub width: Option<u16>,
    pub height: Option<u16>,
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
            match c.as_any_mut().downcast_mut::<Repos>() {
                Some(ref mut o) => { matched = true; o.update(db); },
                None => ()
            }
        };
    }
    pub fn console_size(&mut self) {
        let (size_col, size_row) = terminal_size().unwrap();
        let l = LayoutUpdate { width: Some(size_col), height: Some(size_row), top: None, left: None };
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
}


fn empty_layout() -> Layout {
    Layout { top: 0, left: 0, width: 0, height: 0 }
}

pub fn empty_application() -> Application {
    let mut app = Application {
        controls: vec![]
    };
    app.add_control(Box::new(Header { repo_path: "".to_string(), state: "".to_string(), layout: empty_layout() }));
    app.add_control(Box::new(Branches { local: vec![], remote: vec![], checkedout_idx: None, layout: empty_layout(), visible: true }));
    app.add_control(Box::new(Repos { repos: vec![], layout: empty_layout(), visible: false }));
    app
}

pub fn fill_update_data<'a, 'b>(repo: &'a Repository, db: &'b Database) -> UpdateData<'a, 'b> {
    let (size_col, size_row) = terminal_size().unwrap();
    let ud = UpdateData {
        console_width: Some(size_col),
        console_height: Some(size_row),
        key_value: None,
        git_repo: Some(repo),
        db: Some(db),
    };
    ud
}

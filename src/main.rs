extern crate termion;
extern crate git2;
extern crate rusqlite;
mod app;


use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use git2::Repository;
use app::branches::{Branches};
use app::repo;
use app::UiControl;
use app::draw::{reset_screen, RenderableUiControl, RepositoryUiControl};
use app::db::{Database, get_repositories, add_repository};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let repo_path = "/mnt/c/src/CLEVER";
    let repo = match Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    
    let sqlite_conn = rusqlite::Connection::open_in_memory().unwrap();
    let db = Database { conn: &sqlite_conn };

    // iterate over the enum list for the controls, and when processing a single control,
    // grab the data without having to fight with borrowing values from the vec being iterated.
    let mut ctrl_type = vec![UiControl::Branches];
    let mut ctrl_data = vec![
        Branches{ local: vec![], remote: vec![], checkedout_idx: -1 }
    ];
    
    reset_screen(&mut stdout);

    // main ui loop:
    // - grab the console input
    // - for each control,
    // - call it's relevant update methods
    // - call controls render
    // - first in list is lower in z-index, added at end of list is top most ui z-index
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Char(c) => {
                for i in 0..ctrl_type.len() {
                    match &ctrl_type[i] {
                        UiControl::Branches => {
                            let z = &mut ctrl_data[i];
                            z.update(&repo);
                        }
                    }
                    ctrl_data[i].render(&mut stdout);
                }
            },
            _ => ()
        }
        stdout.flush().unwrap();
    }

    reset_screen(&mut stdout);
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

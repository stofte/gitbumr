extern crate termion;
extern crate git2;
extern crate rusqlite;
mod app;

use std::io::{Write, stdout, stdin, Stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use git2::Repository;
use app::{
    console,
    db::{Database},
    control::{Control, branches::Branches, header::Header},
    empty_application, fill_update_data, Application, UpdateData,
};

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

    // struct with all controls. todo: listify this
    let mut app = empty_application();

    console::reset();
    let iud = fill_update_data(&repo);
    update(&mut app, &iud);
    render(&app, &mut stdout);
    for c in stdin.keys() {
        let mut ud = UpdateData{ console_width: None, console_height: None, key_value: None, git_repo: Some(&repo) };
        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Char(c) => ud.key_value = Some(c),
            _ => ()
        }
        update(&mut app, &ud);
        render(&app, &mut stdout);
    }

    console::reset();
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn update(app: &mut Application, ud: &UpdateData) {
    app.header.update(&ud);
    app.branches.update(&ud);
}

fn render(app: &Application, stdout: &mut Stdout) {
    app.header.render(stdout);
    app.branches.render(stdout);
    stdout.flush().unwrap();
}

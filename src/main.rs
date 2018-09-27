extern crate termion;
extern crate git2;
extern crate rusqlite;
#[macro_use]
extern crate crossbeam_channel as channel;
mod app;

use std::{
    thread,
    time,
    io::{Write, stdout, stdin, Stdout}
};
use termion::{
    event::Key,
    terminal_size
};
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

    
    let mut stdout = stdout().into_raw_mode().unwrap();

    let repo_path = "/mnt/c/src/CLEVER";
    let repo = match Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    
    let sqlite_conn = rusqlite::Connection::open_in_memory().unwrap();
    let db = Database { conn: &sqlite_conn };

    let (keys_s, keys_r) = channel::bounded(0);
    thread::spawn(move || {
        let stdin = stdin();
        for c in stdin.keys() {
            keys_s.send(c.unwrap());
        }
    });
    let (size_s, size_r) = channel::bounded(0);
    let poll_interval = time::Duration::from_millis(50);
    thread::spawn(move || {
        let (mut size_col, mut size_row) = terminal_size().unwrap();
        loop {
            thread::sleep(poll_interval);
            let (n_size_col, n_size_row) = terminal_size().unwrap();
            if size_col != n_size_col || size_row != n_size_row {
                size_col = n_size_col;
                size_row = n_size_row;
                size_s.send((size_col, size_row));
            }
        }
    });

    let mut app = empty_application();
    console::reset();
    let iud = fill_update_data(&repo);
    update(&mut app, &iud);
    render(&app, &mut stdout);
    
    loop  {
        select! {
            recv(keys_r, key) => {
                match key {
                    Some(c) => {
                        let mut ud = UpdateData{ console_width: None, console_height: None, key_value: None, git_repo: Some(&repo) };
                        match c {
                            Key::Ctrl('c') => break,
                            Key::Char(c) => ud.key_value = Some(c),
                            _ => ()
                        }
                        update(&mut app, &ud);
                        render(&app, &mut stdout);
                    },
                    _ => ()
                }
            },
            recv(size_r, size) => {
                match size {
                    Some((size_width, size_height)) => {
                        render(&app, &mut stdout);
                    },
                    _ => ()
                }
            }
        }
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

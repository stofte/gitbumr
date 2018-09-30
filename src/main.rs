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
    terminal_size,
    input::TermRead,
    raw::IntoRawMode,
    screen::AlternateScreen,
};
use git2::Repository;
use app::{
    console, Layout,
    db::{Database},
    control::{Control, RepositoryControl, DatabaseControl, branches::Branches, header::Header},
    new_application, Application,
};

fn main() {
    let repo_path = "/mnt/c/src/CLEVER";
    let repo = match Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let sqlite_conn = rusqlite::Connection::open_in_memory().unwrap();
    let mut db = Database { conn: &sqlite_conn };
    db.init();

    let (keys_s, keys_r) = channel::bounded(0);
    let (size_s, size_r) = channel::bounded(0);
    let poll_interval = time::Duration::from_millis(50);

    thread::spawn(move || {
        let stdin = stdin();
        for c in stdin.keys() {
            keys_s.send(c.unwrap());
        }
    });

    // lock the scrollbar https://gitlab.redox-os.org/redox-os/termion/issues/117
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    write!(screen, "{}", termion::cursor::Hide).unwrap();

    // terminal seems to get fubared if this is done after terminal_size?
    let mut stdout = stdout().into_raw_mode().unwrap();

    thread::spawn(move || {
        // todo use unix signal instead?
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

    let mut app = new_application();
    
    app.console_size();
    app.repository(&repo);
    app.database(&mut db);
    
    console::reset();

    loop  {
        app.render(&mut stdout);
        select! {
            recv(keys_r, key) => {
                let c = key.unwrap();
                match c {
                    Key::Ctrl('c') => break,
                    _ => ()
                };
                // if we didn't break, pass the input to the controls
                app.key(c);
                app.database(&mut db);
            },
            recv(size_r, size) => {
                console::reset();
                app.console_size();
            }
        }
    }

    console::reset();
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

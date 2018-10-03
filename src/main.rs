extern crate termion;
extern crate git2;
extern crate rusqlite;
#[macro_use]
extern crate crossbeam_channel as channel;
#[macro_use]
extern crate bitflags;
extern crate chrono;
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
    console, Layout, UiFlags,
    settings::{Settings, build_settings},
    control::{Control, RepositoryControl, SettingsControl, branches::Branches, header::Header},
    new_application, Application,
};

fn main() {

    let (keys_s, keys_r) = channel::bounded(0);
    let (size_s, size_r) = channel::bounded(0);
    let poll_interval = time::Duration::from_millis(50);

    thread::spawn(move || {
        let stdin = stdin();
        for c in stdin.keys() {
            keys_s.send(c.unwrap());
        }
    });

    if false { // fix panic printing on alternate screen
        // lock the scrollbar https://gitlab.redox-os.org/redox-os/termion/issues/117
        let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
        write!(screen, "{}", termion::cursor::Hide).unwrap();
    }

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
    app.run(stdout, keys_r, size_r);
}

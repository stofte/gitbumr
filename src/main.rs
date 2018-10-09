extern crate termion;
extern crate git2;
extern crate rusqlite;
#[macro_use]
extern crate crossbeam_channel as channel;
extern crate chrono;
mod app;

use std::{
    thread, time, panic, io::{Write, stdout, stdin}
};
use termion::{
    terminal_size,
    raw::IntoRawMode,
    input::TermRead,
    screen::{AlternateScreen, ToMainScreen},
};
use app::{
    console,
    build_app,
    logger::get_log_path,
};

fn main() {
    println!("Log at {}", get_log_path());
    
    let (keys_s, keys_r) = channel::bounded(0);
    let (size_s, size_r) = channel::bounded(0);
    
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    write!(screen, "{}", termion::cursor::Hide).unwrap();
    let poll_interval = time::Duration::from_millis(50);
        
    thread::spawn(move || {
        let stdin = stdin();
        for c in stdin.keys() {
            keys_s.send(c.unwrap());
        }
    });

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

    // panics written to the alternate screen will be lost as soon as the process exits
    // so intercept the panic and switch the mainscreen and print out panic details.
    panic::set_hook(Box::new(|panic_info| {
        print!("{}", ToMainScreen);
        let mut msg: String = "panic!".to_string();
        if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            if let Some(l) = panic_info.location() {
                msg = format!("{} {}. {} @ {}", msg, s, l.file(), l.line()).to_string();
            } else {
                msg = format!("{} {}.", msg, s).to_string();
            }
        }
        print!("{}\r\n", msg);
    }));

    let mut app = build_app();
    app.run(screen, keys_r, size_r);
}

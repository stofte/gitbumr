extern crate termion;

use termion::event::{Key, Event};
use termion::input::{TermRead};
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout,
           "{}{}q to exit{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide)
            .unwrap();
    stdout.flush().unwrap();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => {
                write!(stdout, "{}", termion::clear::All);
                break;
            },
            _ => {}
        }
        stdout.flush().unwrap();
    }
}
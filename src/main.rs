extern crate termion;

use termion::event::{Key, Event};
use termion::input::{TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use std::io::{Write, Stdin, Stdout};

fn main_loop(stdin: Stdin, stdout: &mut RawTerminal<Stdout>) {
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

fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();

    write!(stdout,
           "{}{}q to exit{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide)
            .unwrap();
    stdout.flush().unwrap();
    main_loop(stdin, &mut stdout);
}
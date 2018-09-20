extern crate termion;

use termion::event::{Key, Event};
use termion::input::{TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use std::io::{Write, Stdin, Stdout};

fn main_loop(stdin: Stdin, stdout: &mut RawTerminal<Stdout>) {
    for c in stdin.events() {
        match c.unwrap() {
            Event::Key(Key::Ctrl('q')) => {
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
    write!(stdout, "{}", termion::clear::All);
    write!(stdout, "q to exit");
    stdout.flush().unwrap();
    main_loop(stdin, &mut stdout);
}

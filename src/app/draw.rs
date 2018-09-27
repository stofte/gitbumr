use std::io::{Write, Stdout};
use termion::{cursor, clear};

pub fn reset_screen(stdout: &mut Stdout) {
    write!(stdout, "{}{}{}",
        cursor::Goto(1, 1),
        clear::All,
        cursor::Hide
    );
    stdout.flush().unwrap();
}

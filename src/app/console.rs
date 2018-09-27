use std::io::{Write, Stdout};
use termion::{cursor, clear};

pub fn reset() {
    print!("{}{}{}",
        cursor::Goto(1, 1),
        clear::All,
        cursor::Hide
    );
}

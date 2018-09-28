use std::io::{Write, Stdout};
use termion::{cursor, clear};
use termion::color::{Bg, Fg, Reset, Rgb, Black};

pub fn reset() {
    print!("{}{}{}",
        cursor::Goto(1, 1),
        clear::All,
        cursor::Hide
    );
}

pub static FG_PRIMARY: Fg<Black> = Fg(Black);
pub static BG_PRIMARY: Bg<Rgb> = Bg(Rgb(205, 205, 205));
pub static BG_BRAND: Bg<Rgb> = Bg(Rgb(255, 150, 50));
pub static FG_BRAND: Bg<Rgb> = Bg(Rgb(255, 150, 50));

pub static BG_GRAY: Bg<Rgb> = Bg(Rgb(205, 205, 205));
pub static FG_BLACK: Fg<Black> = Fg(Black);
pub static BG_BLACK: Bg<Black> = Bg(Black);
pub static FG_WHITE: Fg<Rgb> = Fg(Rgb(255, 255, 255));
pub static BG_RESET: Bg<Reset> = Bg(Reset);
pub static FG_RESET: Fg<Reset> = Fg(Reset);

pub static BOX_DR: char = '\u{250f}';
pub static BOX_H: char = '\u{2501}';
pub static BOX_V: char = '\u{2503}';
pub static BOX_VL: char = '\u{252B}';
pub static BOX_VR: char = '\u{2523}';
pub static BOX_DL: char = '\u{2513}';
pub static BOX_UR: char = '\u{2517}';
pub static BOX_UL: char = '\u{251B}';

use std::io::{Write, Stdout};
use termion::{cursor, clear};
use termion::color::{Bg, Fg, Reset, Rgb, Black};

static BG_WHITE: Bg<Rgb> = Bg(Rgb(255, 255, 255));
static FG_BLACK: Fg<Black> = Fg(Black);
static FG_WHITE: Fg<Rgb> = Fg(Rgb(255, 255, 255));
static BG_GRAY: Bg<Rgb> = Bg(Rgb(205, 205, 205));
static BG_BRAND: Bg<Rgb> = Bg(Rgb(255, 150, 50));
static BG_RESET: Bg<Reset> = Bg(Reset);
static FG_RESET: Fg<Reset> = Fg(Reset);

pub fn header(area_title: &str, area_state: &str, width: u16, height: u16) {
    let right_off = width - area_state.len() as u16 + 1;
    println!("{}{}{}{}{}{}{}{}{}{}{}", 
        BG_BRAND,
        FG_WHITE,
        "Girbumr",
        BG_GRAY,
        FG_BLACK,
        area_title,
        " ".repeat(width as usize - area_title.len()),
        cursor::Goto(right_off, 1),
        area_state,
        BG_RESET,
        FG_RESET
    );
}

pub fn reset_screen(stdout: &mut Stdout) {
    write!(stdout, "{}{}{}",
        cursor::Goto(1, 1),
        clear::CurrentLine,
        cursor::Hide
    );
    stdout.flush().unwrap();
}

pub fn main_overview() {

}

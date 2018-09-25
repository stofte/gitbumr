use termion::{style, cursor, color};
use termion::color::{Bg, Fg, Color, Reset, Rgb, Black};

static bg_white: Bg<Rgb> = Bg(Rgb(255, 255, 255));
static fg_black: Fg<Black> = Fg(Black);
static bg_reset: Bg<Reset> = Bg(Reset);
static fg_white: Fg<Rgb> = Fg(Rgb(255, 255, 255));
static bg_gray: Bg<Rgb> = Bg(Rgb(205, 205, 205));
static bg_brand: Bg<Rgb> = Bg(Rgb(255, 150, 50));


pub fn header(area_title: &str, area_state: &str, width: u16, height: u16) {
    let right_off = width - area_state.len() as u16 + 1;
    println!("{}{}{}{} {}{}{}{}{}{}{}", 
        bg_brand,
        fg_white,
        "Branches ",
        bg_gray,
        fg_black,
        area_title,
        " ".repeat(width as usize - area_title.len()),
        cursor::Goto(right_off, 1),
        area_state,
        color::Fg(color::Reset),
        color::Bg(color::Reset),
    );
}

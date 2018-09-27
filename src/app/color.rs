use termion::color::{Bg, Fg, Reset, Rgb, Black};

pub static BG_WHITE: Bg<Rgb> = Bg(Rgb(255, 255, 255));
pub static FG_BLACK: Fg<Black> = Fg(Black);
pub static FG_WHITE: Fg<Rgb> = Fg(Rgb(255, 255, 255));
pub static BG_GRAY: Bg<Rgb> = Bg(Rgb(205, 205, 205));
pub static BG_BRAND: Bg<Rgb> = Bg(Rgb(255, 150, 50));
pub static BG_RESET: Bg<Reset> = Bg(Reset);
pub static FG_RESET: Fg<Reset> = Fg(Reset);
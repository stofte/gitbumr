use std::io::{Write, Stdout};
use termion::{cursor, clear};
use termion::color::{Bg, Fg, Reset, Rgb, Black};
use git2::Repository;

static BG_WHITE: Bg<Rgb> = Bg(Rgb(255, 255, 255));
static FG_BLACK: Fg<Black> = Fg(Black);
static FG_WHITE: Fg<Rgb> = Fg(Rgb(255, 255, 255));
static BG_GRAY: Bg<Rgb> = Bg(Rgb(205, 205, 205));
static BG_BRAND: Bg<Rgb> = Bg(Rgb(255, 150, 50));
static BG_RESET: Bg<Reset> = Bg(Reset);
static FG_RESET: Fg<Reset> = Fg(Reset);

// ctrl which can display something about a repository
pub trait RepositoryUiControl {
    fn update(&mut self, &Repository);
}

// ctrl which can account for window size
pub trait SizeAwareUiControl {
    fn update(&mut self, width: u16, height: u16);
}

// ctrl which can be positioned according to top/left semantics
pub trait PositionableUiControl {
    fn place_tl(&mut self, top: u16, left: u16);
    fn place_tr(&mut self, top: u16, right: u16);
    fn place_br(&mut self, bottom: u16, right: u16);
    fn place_bl(&mut self, bottom: u16, left: u16);

}

pub trait RenderableUiControl {
    fn render(&self, &mut Stdout);
}

pub fn header(area_title: &str, area_state: &str, width: u16, height: u16) {
    let right_off = width - area_state.len() as u16 + 1;
    println!("{}{}{}{}{}{}{}{}{}{}{}", 
        BG_BRAND,
        FG_WHITE,
        "Gitbumr",
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
        clear::All,
        cursor::Hide
    );
    stdout.flush().unwrap();
}

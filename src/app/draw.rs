use termion::{style, cursor, color};

// static bg_white : color::Bg<color::Color> = color::Bg(color::Rgb(255, 255, 255));
// static bg_reset : color::Bg<color::Color> = color::Bg(color::Reset);

pub fn header(area_title: &str, area_state: &str, width: u16, height: u16) {
    let right_off = width - area_state.len() as u16 + 1;
    println!("{}{}{}{} {}{}{}{}{}{}{}", 
        color::Bg(color::Rgb(255, 150, 50)),
        color::Fg(color::Rgb(255, 255, 255)),
        "Branch",
        color::Bg(color::Rgb(255, 255, 255)),
        color::Fg(color::Black),
        area_title,
        " ".repeat(width as usize - area_title.len()),
        cursor::Goto(right_off, 1),
        area_state,
        color::Fg(color::Reset),
        color::Bg(color::Reset),
    );
}

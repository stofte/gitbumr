use std::io::{Stdout};
use termion::{cursor, clear};
use app::{Layout, UpdateData};
use app::color::{BG_BRAND, FG_WHITE, BG_GRAY, FG_BLACK, BG_RESET, FG_RESET};
use app::control::{Control};

pub struct Header {
    pub repo_path: String,
    pub state: String,
    pub width: u16,
    pub height: u16,
    pub layout: Layout
}

impl Control for Header {
    fn layout(&mut self, layout: Layout) {
        
    }
    fn update(&mut self, data: &UpdateData) {
        match data.git_repo {
            Some(repo) => {
                let path = repo.path().to_str().unwrap().to_string();
                if path.ends_with("/.git/") {
                    self.repo_path = path.chars().take(path.len() - 6).collect();
                }
                else {
                    self.repo_path = path;
                }
                let zz = format!("{:?}", repo.state());
                self.state = zz;
            },
            _ => ()
        };
        match data.console_width {
            Some(w) => self.width = w,
            _ => ()
        };
        match data.console_height {
            Some(h) => self.height = h,
            _ => ()
        };
    }
    fn render(&self, stdout: &mut Stdout) {
        let right_off = self.width - self.state.len() as u16 + 1;
        print!("{}", cursor::Goto(1, 1));
        print!("{}{}{}{}{}{}{}{}{}{}{}", 
            BG_BRAND,
            FG_WHITE,
            "Gitbumr",
            BG_GRAY,
            FG_BLACK,
            self.repo_path,
            " ".repeat(self.width as usize - self.repo_path.len()),
            cursor::Goto(right_off, 1),
            self.state,
            BG_RESET,
            FG_RESET
        );
    }
}

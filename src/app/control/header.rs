use std::io::{Stdout};
use std::any::Any;
use git2::{Repository};
use termion::{cursor, clear};
use app::{
    Layout, LayoutUpdate,
    console::{BG_BRAND, FG_WHITE, BG_GRAY, FG_BLACK, BG_RESET, FG_RESET},
    control::{Control, RepositoryControl},
};

pub struct Header {
    pub repo_path: String,
    pub state: String,
    pub layout: Layout
}

static APP_NAME: &'static str = "Gitbumr";

impl Control for Header {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn layout(&mut self, layout: &LayoutUpdate) {
        self.layout.width = layout.cols.unwrap();
        self.layout.height = layout.rows.unwrap();
    }
    fn render(&self, stdout: &mut Stdout) -> bool {
        let right_off = self.layout.width - self.state.len() as u16 + 1;
        print!("{}", cursor::Goto(1, 1));
        print!("{}{}{}{}{}{}{}{}{}{}{}", 
            BG_BRAND,
            FG_WHITE,
            APP_NAME,
            BG_GRAY,
            FG_BLACK,
            self.repo_path,
            " ".repeat(self.layout.width as usize - self.repo_path.len() - APP_NAME.len()),
            cursor::Goto(right_off, 1),
            self.state,
            BG_RESET,
            FG_RESET
        );
        false
    }
}

impl RepositoryControl for Header {
    fn update(&mut self, repo: &Repository) {
        let path = repo.path().to_str().unwrap().to_string();
        if path.ends_with("/.git/") {
            self.repo_path = path.chars().take(path.len() - 6).collect();
        }
        else {
            self.repo_path = path;
        }
        let zz = format!("{:?}", repo.state());
        self.state = zz;
    }
}

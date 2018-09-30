use std::io::{Stdout};
use std::any::Any;
use git2::{Repository};
use termion::{cursor, clear};
use app::{
    Layout, LayoutUpdate,
    console,
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
        let blank_cnt = self.layout.width as usize - self.repo_path.len() - APP_NAME.len() - self.state.len();
        print!("{move}{b_fg}{b_bg}{name}{fg}{bg}{path}{blank}{state}{fg_r}{bg_r}",
            move=cursor::Goto(1, 1),
            name=APP_NAME,
            path=self.repo_path,
            blank=" ".repeat(blank_cnt),
            state=self.state,
            b_fg=console::FG_BRAND,
            b_bg=console::BG_BRAND,
            fg=console::FG_PRIMARY,
            bg=console::BG_PRIMARY,
            bg_r=console::BG_RESET,
            fg_r=console::FG_RESET,
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
    fn none(&mut self) {
        self.repo_path = "None".to_string();
    }
}

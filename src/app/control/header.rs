use std::io::{Stdout};
use std::any::Any;
use git2::{Repository};
use termion::{cursor, clear};
use app::{
    Layout, LayoutUpdate, empty_layout,
    console,
    control::{Control, RepositoryControl},
};

pub struct Header {
    pub repo_path: String,
    pub state: String,
    pub layout: Layout,
    pub render: bool,

}

static APP_NAME: &'static str = "Gitbumr";

impl Control for Header {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn layout(&mut self, layout: &LayoutUpdate) {
        match layout.cols {
            Some(c) => self.layout.width = c,
            _ => ()
        };
        match layout.rows {
            Some(r) => self.layout.height = r,
            _ => ()
        };
        match layout.invalidated {
            Some(true) => self.render = true,
            _ => ()
        };
    }
    fn render(&mut self, stdout: &mut Stdout) {
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
    fn read(&mut self, repo: &Repository) { }
}

pub fn build_header() -> Header {
    Header {
        repo_path: "".to_string(),
        state: "".to_string(),
        layout: empty_layout(),
        render: true
    }
}

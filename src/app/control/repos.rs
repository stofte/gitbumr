use std::{
    io::Stdout,
};
use app::{
    Layout, UpdateData,
    db::{StoredRepository},
    control::{Control},
};

pub struct Repos {
    pub visible: bool,
    pub repos: Vec<StoredRepository>,
    pub layout: Layout
}

impl Control for Repos {
    fn layout(&mut self, layout: Layout) {
        
    }
    fn update(&mut self, data: &UpdateData) {
        match data.git_repo {
            Some(repo) => {

            },
            _ => ()
        };
        match data.console_width {
            Some(w) => self.layout.width = w,
            _ => ()
        };
        match data.console_height {
            Some(h) => self.layout.height = h,
            _ => ()
        };
    }
    fn render(&self, stdout: &mut Stdout) {
    }
}
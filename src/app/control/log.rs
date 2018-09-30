use termion::{style, cursor};
use std::{
    cmp,
    io::Stdout,
    any::Any,
};
use git2::{Repository, BranchType};
use app::{
    console,
    Layout,
    LayoutUpdate,
    empty_layout,
    git::{get_head},
    control::{Control, RepositoryControl},
};

pub struct Log {
    pub entries: Vec<String>,
    pub layout: Layout,
}

impl Control for Log {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn layout(&mut self, layout: &LayoutUpdate) {
        self.layout.top = 2;
        self.layout.left = 36;
        self.layout.width = layout.cols.unwrap();
        self.layout.height = layout.rows.unwrap() - self.layout.top;
    }
    fn render(&self, stdout: &mut Stdout) {
        if !self.layout.visible { return }
        console::start_drawing(self.layout.left, self.layout.top, console::FG_PRIMARY, console::BG_PRIMARY);
        let title = "Log".to_string();
        let title_b_h = console::BOX_H.to_string()
            .repeat(self.layout.width as usize - title.len() - 2);
        print!("{b_h}{title}{repeat}{b_dl}",
            title=title,
            repeat=title_b_h,
            b_h=console::BOX_H,
            b_dl=console::BOX_DL,
        );
        console::stop_drawing();
    }
}

impl RepositoryControl for Log {
    fn update(&mut self, repo: &Repository) {
        self.layout.visible = true;
    }
    fn none(&mut self) {
        self.layout.visible = false;
    }
}

pub fn build_log() -> Log {
    Log {
        entries: vec![],
        layout: empty_layout(),
    }
}

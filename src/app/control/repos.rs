use std::{
    io::Stdout,
    any::Any,
};
use app::{
    Layout, LayoutUpdate, UpdateData,
    db::{StoredRepository, Database},
    control::{Control, DatabaseControl},
};

pub struct Repos {
    pub visible: bool,
    pub repos: Vec<StoredRepository>,
    pub layout: Layout
}

impl Control for Repos {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn layout(&mut self, layout: &LayoutUpdate) {
        self.layout.width = layout.width.unwrap();
        self.layout.height = layout.height.unwrap();
    }
    fn render(&self, stdout: &mut Stdout) {
    }
}

impl DatabaseControl for Repos {
    fn update(&mut self, db: &Database) {

    }
}

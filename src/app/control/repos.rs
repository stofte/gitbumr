use std::{
    io::Stdout,
    any::Any,
};
use termion::{
    cursor,
    event::Key
};
use app::{
    Layout, LayoutUpdate,
    db::{StoredRepository, Database},
    control::{Control, DatabaseControl, InputControl},
};

pub struct Repos {
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
        match layout.visible {
            Some(b) => {
                self.layout.visible = b;
            },
            _ => ()
        };
    }
    fn render(&self, stdout: &mut Stdout) {
        if !self.layout.visible { return; }
        print!("{}REPOS TODO", cursor::Goto(1, 2));
    }
}

impl DatabaseControl for Repos {
    fn update(&mut self, db: &Database) {

    }
}

impl InputControl for Repos {
    fn handle(&mut self, key: Key) -> bool {
        match key {
            Key::Ctrl('r') => {
                if !self.layout.visible {
                    self.layout.visible = true;
                    return true
                }
                false
            },
            Key::Esc => {
                if self.layout.visible {
                    self.layout.visible = false;
                    return true
                }
                false
            }
            _ => false
        }
    }
}

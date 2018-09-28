pub mod branches;
pub mod header;
pub mod repos;

use std::any::Any;
use std::io::{Stdout};
use git2::{Repository};
use termion::{event::Key};
use app::{db::Database, LayoutUpdate};

pub trait Control {
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn layout(&mut self, &LayoutUpdate);
    fn render(&self, &mut Stdout);
}

pub trait RepositoryControl {
    fn update(&mut self, &Repository);
}

pub trait DatabaseControl {
    fn update(&mut self, &Database);
}

pub trait InputControl {
    fn handle(&mut self, Key) -> bool;
}

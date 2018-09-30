pub mod branches;
pub mod header;
pub mod repomanager;

use std::any::Any;
use std::io::{Stdout};
use git2::{Repository};
use termion::{event::Key};
use app::{settings::Settings, LayoutUpdate, UiOption};

pub trait Control {
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn layout(&mut self, &LayoutUpdate);
    fn render(&self, &mut Stdout) -> bool;
}

pub trait RepositoryControl {
    fn none(&mut self);
    fn update(&mut self, &Repository);
}

pub trait SettingsControl {
    fn update(&mut self, &mut Settings) -> UiOption;
}

pub trait InputControl {
    fn handle(&mut self, Key) -> (bool, UiOption);
}

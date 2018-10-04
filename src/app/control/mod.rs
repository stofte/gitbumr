pub mod branches;
pub mod header;
pub mod repomanager;
pub mod log;

use std::any::Any;
use std::io::{Stdout};
use git2::{Repository};
use termion::{event::Key};
use app::{settings::Settings, LayoutUpdate, UiFlags};

pub trait Control {
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn layout(&mut self, &LayoutUpdate);
    fn render(&mut self, &mut Stdout);
}

pub trait RepositoryControl {
    // when no repo is selected
    fn none(&mut self);
    // when repo changes
    fn update(&mut self, &Repository);
    // control requested read
    fn read(&mut self, &Repository);
}

pub trait SettingsControl {
    fn update(&mut self, &mut Settings) -> UiFlags;
}

pub trait InputControl {
    fn key(&mut self, Key, UiFlags) -> UiFlags;
    fn render_input(&mut self, &mut Stdout);
}

pub mod header;
pub mod branches;
pub mod repomanager;
pub mod history;

use std::io::{Stdout};
use termion::event;
use app::{
    event::{KeyArg, Event, EventArg},
    logger::Logger,
};

pub trait Control {
    fn render(&mut self, &mut Stdout, &mut Logger);
    fn key(&mut self, event::Key, &mut Logger) -> KeyArg;
    fn ctx(&mut self, &mut Event, &mut Logger) -> EventArg;
    fn id(&self) -> u32;
}

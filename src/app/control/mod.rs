pub mod header;
pub mod branches;
pub mod repomanager;
pub mod history;

use std::io::{Stdout};
use termion::event;
use app::{
    event::{Event, EventArg},
    logger::Logger,
    linebuffer::LineBuffer,
};

pub trait Control {
    fn id(&self) -> u32;
    fn render(&mut self, &mut LineBuffer, &mut Logger);
    fn ctx(&mut self, &mut Event, &mut LineBuffer, &mut Logger) -> EventArg;
}

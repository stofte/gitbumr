use std::io::{Stdout};

pub mod branches;
pub mod header;

use app::{ Layout, UpdateData };


pub trait Control {
    fn layout(&mut self, Layout);
    fn update(&mut self, &UpdateData);
    fn render(&self, &mut Stdout);
}

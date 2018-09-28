use std::io::{Stdout};

pub mod branches;
pub mod header;
pub mod repos;

use app::{ Layout, UpdateData };


pub trait Control {
    fn layout(&mut self, Layout);
    fn update(&mut self, &UpdateData);
    fn render(&self, &mut Stdout);
}

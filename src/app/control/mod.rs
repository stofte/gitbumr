use std::io::{Stdout};

pub mod branches;
pub mod header;

use app::{ UpdateData };

pub trait Control {
    fn update(&mut self, &UpdateData);
    fn render(&self, &mut Stdout); 
}

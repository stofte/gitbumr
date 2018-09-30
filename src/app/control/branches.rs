use termion::{style, cursor};
use std::{
    cmp,
    io::Stdout,
    any::Any,
};
use git2::{Repository, BranchType};
use app::git::{get_head};
use app::control::{Control, RepositoryControl};
use app::{Layout, LayoutUpdate};

pub struct Branches {
    pub local: Vec<String>,
    pub remote: Vec<String>,
    pub checkedout_idx: Option<u16>,
    pub layout: Layout,
}

impl Control for Branches {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn layout(&mut self, layout: &LayoutUpdate) {
        self.layout.top = 2;
        self.layout.left = 1;
        self.layout.width = 25;
        self.layout.height = layout.rows.unwrap() - self.layout.top;
    }
    fn render(&self, stdout: &mut Stdout) {
        if !self.layout.visible { return }

        match self.checkedout_idx {
            Some(i) => {
                print!("{}", cursor::Goto(1, 2));
                for j in 0..self.local.len() {
                    let s = &self.local[j];
                    if i as usize == j {
                        println!("{}{}{}{}{}", cursor::Save, style::Bold, s, style::Reset, cursor::Restore);
                    } 
                    else {
                        println!("{}{}{}", cursor::Save, s, cursor::Restore);
                    }
                }
            },
            _ => ()
        };
    }
}

impl RepositoryControl for Branches {
    fn update(&mut self, repo: &Repository) {
        let mut vec = Vec::new();
        let bs = repo.branches(Some(BranchType::Local)).unwrap();
        for b in bs {
            let bb = b.unwrap().0;
            let name = bb.name().unwrap().unwrap().to_owned();
            vec.push(name);
        }
        vec.sort();
        let head_name = get_head(&repo);
        for i in 0..vec.len() {
            if head_name == vec[i] {
                self.checkedout_idx = Some(i as u16);
            }
        }
        self.local = vec;
        self.layout.visible = true;
    }
    fn none(&mut self) {
        self.layout.visible = false;
    }
}

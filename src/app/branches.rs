use termion::{style, cursor};
use std::io::Stdout;
use git2::{Repository, BranchType};
use app::git::{local_branches, get_head};
use app::draw::{header, RepositoryUiControl, RenderableUiControl};

pub struct Branches {
    pub local: Vec<String>,
    pub remote: Vec<String>,
    pub checkedout_idx: i32,
}

impl RepositoryUiControl for Branches {
    fn update(&mut self, repo: &Repository) {
        let mut vec = Vec::new();
        let bs = repo.branches(Some(BranchType::Local)).unwrap();
        for b in bs {
            let bb = b.unwrap().0;
            let name = bb.name().unwrap().unwrap().to_owned();
            let mut chkout = false;
            vec.push(name);
        }
        vec.sort();
        let head_name = get_head(&repo);
        for i in 00..vec.len() {
            if head_name == vec[i] {
                self.checkedout_idx = i as i32;
            }
        }
        self.local = vec;
    }
}

impl RenderableUiControl for Branches {
    fn render(&self, stdout: &mut Stdout) {
        let mut i = self.checkedout_idx;
        println!("{}", cursor::Goto(1, 1));
        for j in 0..self.local.len() {
            let s = &self.local[j];
            if i > -1 && i as usize == j {
                println!("{}{}{}{}{}", cursor::Save, style::Bold, s, style::Reset, cursor::Restore);
            } 
            else {
                println!("{}{}{}", cursor::Save, s, cursor::Restore);
            }
        }
    }
}

use termion::{style, cursor};
use std::io::Stdout;
use git2::{BranchType};
use app::git::{get_head};
use app::control::{Control};
use app::{UpdateData};

pub struct Branches {
    pub local: Vec<String>,
    pub remote: Vec<String>,
    pub checkedout_idx: Option<u16>,
}

impl Control for Branches {
    fn update(&mut self, data: &UpdateData) {
        match data.git_repo {
            Some(repo) => {
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
            },
            _ => ()
        }
    }
    fn render(&self, stdout: &mut Stdout) {
        match self.checkedout_idx {
            Some(i) => {
                println!("{}", cursor::Goto(1, 1));
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
        }
    }
}

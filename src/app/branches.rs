use termion::{style, cursor};
use std::io::Stdout;
use git2::{Repository, BranchType};
use app::state::UiState;
use app::git::{local_branches, get_head};
use app::draw::{header, RepositoryUiControl, RenderableUiControl};

struct Branches {
    local: Vec<String>,
    remote: Vec<String>,
    checkedout_idx: u32,
}

impl RepositoryUiControl for Branches {
    fn update(&mut self, repo: &Repository) {
        let mut vec = Vec::new();
        let bs = repo.branches(Some(BranchType::Local)).unwrap();
        let head_name = get_head(&repo);
        let mut idx = 0;
        for b in bs {
            let bb = b.unwrap().0;
            let name = bb.name().unwrap().unwrap().to_owned();
            let mut chkout = false;
            if head_name == name {
                self.checkedout_idx = idx;
            }
            else {
                idx = idx + 1;
            }
            vec.push(name);
        }
        vec.sort();
        self.local = vec;
    }
}

impl RenderableUiControl for Branches {
    fn render(&self, stdout: &mut Stdout) {
        
    }
}

pub fn view(state: &mut UiState) {
    header(state.repository, &format!("{:?}", state.git_repo.state()), state.width, state.height);
    println!("{}", cursor::Goto(1, 1));
    for b in local_branches(&state.git_repo) {
        if b.checkedout == true {
            println!("{}{}{}{}{}", cursor::Save, style::Bold, b.name, style::Reset, cursor::Restore);
        } else {
            println!("{}{}{}", cursor::Save, b.name, cursor::Restore);
        }
    }
}

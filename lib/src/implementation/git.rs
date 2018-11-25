use git2::Repository;
use interface::{ GitTrait, GitEmitter };
use super::{ Branches, BranchesItem, fill_branches };
use utils::local_branches;

pub struct Git {
    emit: GitEmitter,
    branches: Branches,
    git: Option<Repository>,
}

impl GitTrait for Git {
    fn new(emit: GitEmitter, branches: Branches) -> Git {
        Git {
            emit,
            branches,
            git: None
        }
    }
    fn emit(&mut self) -> &mut GitEmitter {
        &mut self.emit
    }
    fn branches(&self) -> &Branches {
        &self.branches
    }
    fn branches_mut(&mut self) -> &mut Branches {
        &mut self.branches
    }
    fn load(&mut self, path: String) {
        match Repository::open(path) {
            Ok(r) => {
                fill_branches(&mut self.branches, local_branches(&r));
                self.git = Some(r);
            }
            Err(..) => panic!("not a git Repository")
        };
    }
}

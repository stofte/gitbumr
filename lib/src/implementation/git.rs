#![allow(unused_variables)]
use interface::{ GitTrait, GitEmitter };
use super::Branches;

pub struct Git {
    emit: GitEmitter,
    branches: Branches,
}

impl GitTrait for Git {
    fn new(emit: GitEmitter, branches: Branches) -> Git {
        Git {
            emit,
            branches,
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
}

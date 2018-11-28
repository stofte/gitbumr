use git2::{Repository};
use interface::{ GitTrait, GitEmitter };
use super::{ Branches, fill_branches };
use utils::local_branches;

pub struct Git {
    emit: GitEmitter,
    branches: Branches,
    git: Option<Repository>,
    revwalk_filter: String,
}

impl GitTrait for Git {
    fn new(emit: GitEmitter, branches: Branches) -> Git {
        Git {
            emit,
            branches,
            git: None,
            revwalk_filter: "".to_string(),
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
                // here, we set the head for the revwalk. currently head
                // when the git path changes, or any branch change is made,
                // the revwalk_filter property should be updated
                self.revwalk_filter = get_revwalk_oid(&self);
                self.emit.revwalk_filter_changed();
            }
            Err(..) => panic!("not a git Repository")
        };
    }
    fn revwalk_filter(&self) -> &str {
        &self.revwalk_filter
    }
}

// return "" on none found/error, which is treated as falsy in qml
// otherwise returns the sha hash for the commit. i think.
fn get_revwalk_oid(repo: &Git) -> String {
    match &repo.git {
        Some(r) => {
            match r.head() {
                Ok(h) => {
                    match h.target() {
                        Some(t) => {
                            t.to_string()
                        },
                        _ => panic!("get_revwalk_oid unexpected case")
                    }
                },
                _ => panic!("get_revwalk_oid unexpected case")
            }
        },
        None => "".to_string()
    }
}

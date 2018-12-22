use git2::{Repository, Oid};
use chrono::FixedOffset;
use interface::{ GitTrait, GitEmitter };
use super::{ Branches, fill_branches, CommitModel, fill_commit, TreeModel, fill_treemodel };
use utils::{local_branches, get_timesize_offset};

pub struct Git {
    emit: GitEmitter,
    branches: Branches,
    commit: CommitModel,
    tree: TreeModel,
    git: Option<Repository>,
    revwalk_filter: String,
    tz_offset: FixedOffset,
}

impl GitTrait for Git {
    fn new(emit: GitEmitter, branches: Branches, commit: CommitModel, tree: TreeModel) -> Git {
        Git {
            emit,
            branches,
            commit,
            tree,
            git: None,
            revwalk_filter: "".to_string(),
            tz_offset: get_timesize_offset(),
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
    fn commit(&self) -> &CommitModel {
        &self.commit
    }
    fn commit_mut(&mut self) -> &mut CommitModel {
        &mut self.commit
    }
    fn tree(&self) -> &TreeModel {
        &self.tree
    }
    fn tree_mut(&mut self) -> &mut TreeModel {
        &mut self.tree
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
    fn load_commit(&mut self, oid: String) {
        match &self.git {
            Some(r) => {
                let oid = Oid::from_str(&oid).unwrap();
                let commit = r.find_commit(oid).unwrap();
                fill_commit(&mut self.commit, &commit, &r, self.tz_offset);
                fill_treemodel(&mut self.tree, &commit, &r);
            }
            None => panic!("expected git repo in load_commit")
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

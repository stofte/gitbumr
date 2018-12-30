use git2::{Repository, Oid};
use chrono::FixedOffset;
use interface::{ GitTrait, GitEmitter };
use super::{ Branches, fill_branches, Commit, fill_commit, Diffs, fill_diffs, Hunks, fill_hunks };
use utils::{local_branches, get_timesize_offset};

pub struct Git {
    emit: GitEmitter,
    branches: Branches,
    commit: Commit,
    hunks: Hunks,
    diffs: Diffs,
    git: Option<Repository>,
    revwalk_filter: String,
    tz_offset: FixedOffset,
}

impl GitTrait for Git {
    fn new(emit: GitEmitter, branches: Branches, commit: Commit, diffs: Diffs, hunks: Hunks) -> Git {
        Git {
            emit,
            branches,
            commit,
            diffs,
            hunks,
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
    fn commit(&self) -> &Commit {
        &self.commit
    }
    fn commit_mut(&mut self) -> &mut Commit {
        &mut self.commit
    }
    fn diffs(&self) -> &Diffs {
        &self.diffs
    }
    fn diffs_mut(&mut self) -> &mut Diffs {
        &mut self.diffs
    }
    fn hunks(&self) -> &Hunks {
        &self.hunks
    }
    fn hunks_mut(&mut self) -> &mut Hunks {
        &mut self.hunks
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
                fill_commit(&mut self.commit, &commit, self.tz_offset);
                fill_diffs(&mut self.diffs, &commit, &r, oid.to_string());
            }
            None => panic!("expected git repo in load_commit")
        };
    }
    fn load_diff(&mut self, oid: String, index: u64) {
        fill_hunks(&mut self.hunks, &self.diffs, index, &oid);
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

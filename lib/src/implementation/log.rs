use std::{cmp::min};
use git2::{Sort, Repository, Oid};
use utils::{get_commit, get_timesize_offset_secs};
use interface::{
    LogList, LogEmitter, LogTrait
};

#[derive(Default, Clone)]
pub struct LogItem {
    pub oid: String,
    pub time: String,
    pub author: String,
    pub message: String,
}

pub struct Log {
    emit: LogEmitter,
    model: LogList,
    list: Vec<LogItem>,
    revwalk: Vec<Oid>,
    git: Option<Repository>,
    tz_offset_sec: i32,
}

impl LogTrait for Log {
    fn new(emit: LogEmitter, model: LogList) -> Log {
        Log {
            emit,
            model,
            list: vec![],
            revwalk: vec![],
            git: None,
            tz_offset_sec: get_timesize_offset_secs(),
        }
    }
    fn emit(&mut self) -> &mut LogEmitter {
        &mut self.emit
    }
    fn row_count(&self) -> usize {
        self.list.len()
    }
    fn author(&self, index: usize) -> &str {
        &self.list[index].author
    }
    fn message(&self, index: usize) -> &str {
        &self.list[index].message
    }
    fn oid(&self, index: usize) -> &str {
        &self.list[index].oid
    }
    fn time(&self, index: usize) -> &str {
        &self.list[index].time
    }
    fn load(&mut self, path: String) {
        self.git= Some(Repository::open(&path).unwrap());
    }
    fn filter(&mut self, filter: String) {
        self.model.begin_reset_model();
        let oid = Oid::from_str(&filter).unwrap();
        match &self.git {
            Some(git) => {
                let mut rv = git.revwalk().unwrap();
                // to view the specific branch log, push the head's oid of the branch.
                // following code should show a combined history for entire repo.
                // rv.push_glob("refs/remotes/origin/*").unwrap();
                // rv.push_glob("refs/heads/*").unwrap();
                // see https://git-scm.com/book/id/v2/Git-Internals-The-Refspec
                rv.push(oid).unwrap();
                rv.set_sorting(Sort::TIME);
                self.revwalk.clear();
                self.model.begin_reset_model();
                self.list.clear();
                self.model.end_reset_model();
                // this can take a long ass time (eg linux kernel repo)
                for e in rv {
                    self.revwalk.push(e.unwrap());
                }
            },
            None => panic!("no git found on log element")
        }
        self.model.end_reset_model();
    }
    fn can_fetch_more(&self) -> bool {
        let has_more = self.list.len() < self.revwalk.len();
        has_more
    }
    fn fetch_more(&mut self) {
        match self.git {
            Some(ref git) => {
                let oid_idx = self.list.len();
                let max_idx = oid_idx + min(10000, self.revwalk.len() - oid_idx);
                self.model.begin_insert_rows(oid_idx, max_idx - 1);
                for i in oid_idx..max_idx {
                    let oid = self.revwalk[i];
                    let e = get_commit(oid, self.tz_offset_sec, git);
                    self.list.push(e);
                }
                self.model.end_insert_rows();
            }
            _ => panic!("fetch_more unexpected case")
        }
    }
}

use std::{thread, cmp::min};
use git2::{Repository, Oid};
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
}

impl LogTrait for Log {
    fn new(emit: LogEmitter, model: LogList) -> Log {
        Log {
            emit,
            model,
            list: vec![],
            revwalk: vec![],
            git: None,
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
                rv.push(oid).unwrap();
                self.revwalk.clear();
                for e in rv {
                    self.revwalk.push(e.unwrap());
                }
                println!("log.filter, revwalk n is {}", self.revwalk.len());
            },
            None => panic!("no git found on log element")
        }
        self.model.end_reset_model();
    }
    fn can_fetch_more(&self) -> bool {
        let has_more = self.list.len() < self.revwalk.len();
        println!("can_fetch_more {}", has_more);
        has_more
    }
    fn fetch_more(&mut self) {
        match self.git {
            Some(ref git) => {
                let mut oid_idx = 0;
                if self.list.len() > 0 {
                    oid_idx = self.list.len() - 1;
                }
                let max_count = min(1000, self.revwalk.len() - 1 - oid_idx);
                println!("fetch_more: {}..{}", oid_idx, max_count);
                self.model.begin_insert_rows(oid_idx, max_count);
                for i in oid_idx..max_count {
                    let oid = self.revwalk[i];
                    let e = get_log_item(&oid, git);
                    println!("fetch_more {} => {}", i, e.oid);
                    self.list.push(e);
                }
                self.model.end_insert_rows();
            }
            _ => panic!("fetch_more unexpected case")
        }
    }
}

fn get_log_item(oid: &Oid, git: &Repository) -> LogItem {
    LogItem { oid: oid.to_string(), time: "".to_string(), author: "".to_string(), message: "".to_string() }
}

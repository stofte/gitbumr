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
        let oid = Oid::from_str(&filter).unwrap();
        match &self.git {
            Some(git) => {
                let mut rv = git.revwalk().unwrap();
                rv.push(oid).unwrap();
                self.revwalk.clear();
                for e in rv {
                    self.revwalk.push(e.unwrap());
                }
            },
            None => panic!("no git found on log element")
        }
    }
}

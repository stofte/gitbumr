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
        match Repository::open(path) {
            Ok(repo) => {
                {
                    let mut rv = repo.revwalk().unwrap();
                    // get the revwalk for the current head
                    rv.push_head();
                    self.revwalk.clear();
                    let mut i = 0;
                    for e in rv {
                        self.revwalk.push(e.unwrap());
                        i += 1;
                    }
                    println!("revwalk count was {}", i);
                }
                self.git = Some(repo);
            }
            Err(..) => return,
        };

    }
    fn filter(&mut self, filter: String) {

    }
}

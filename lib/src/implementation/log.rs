use git2::{Repository, Oid};
use crossbeam::channel::Receiver;
use utils::{get_commit, get_timesize_offset_secs, get_chan_revwalker};
use interface::{
    LogList, LogEmitter, LogTrait
};

#[derive(Default, Clone)]
pub struct LogItem {
    pub cid_short: String,
    pub time: String,
    pub author: String,
    pub message: String,
}

pub struct Log {
    emit: LogEmitter,
    model: LogList,
    list: Vec<LogItem>,
    git: Option<Repository>,
    git_path: String,
    tz_offset_sec: i32,
    revwalker: Option<Receiver<(Vec<Oid>, bool)>>,
    revwalker_has_more: bool,
}

impl Log {
    fn load_from_channel(&mut self) {
        // loading from the revwalk is kinda bitchy since the revwalk is owned by the repo is belongs to.
        // to get around this, and get a "simple" way to resume the revwalk, we hide it away in a thread,
        // which simply walks the revwalk and pushes vectors of fixed sizes to the main thread.
        // as the ui requires more rows, the ui must block on receiving from the revwalking thread.
        // the ui then does the actual work of creating the commit and inserting it into the ui.
        match self.git {
            Some(ref mut r) => {
                match self.revwalker {
                    Some(ref mut rc) => {
                        let (data, has_more) = rc.recv().unwrap();
                        let ins_idx = self.list.len();
                        self.model.begin_insert_rows(ins_idx, ins_idx + data.len() - 1);
                        for oid in data {
                            let e = get_commit(oid, self.tz_offset_sec, &r);
                            self.list.push(e);
                        }
                        self.model.end_insert_rows();
                        self.revwalker_has_more = has_more;
                    },
                    None => panic!("load_from_channel unexpected case")
                }
            },
            None => panic!("load_from_channel unexpected case")
        }
    }
}

impl LogTrait for Log {
    fn new(emit: LogEmitter, model: LogList) -> Log {
        Log {
            emit,
            model,
            list: vec![],
            git: None,
            git_path: "".to_string(),
            revwalker: None,
            revwalker_has_more: false,
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
    fn cid_short(&self, index: usize) -> &str {
        &self.list[index].cid_short
    }
    fn time(&self, index: usize) -> &str {
        &self.list[index].time
    }
    fn load(&mut self, path: String) {
        self.git = Some(Repository::open(&path).unwrap());
        self.git_path = path;
    }
    fn filter(&mut self, filter: String) {
        let c = get_chan_revwalker(self.git_path.clone(), filter, 10000);
        self.revwalker = Some(c);
        self.model.begin_reset_model();
        self.list.clear();
        self.model.end_reset_model();
        self.load_from_channel();
    }
    fn can_fetch_more(&self) -> bool {
        self.revwalker_has_more
    }
    fn fetch_more(&mut self) {
        self.load_from_channel();
    }
}



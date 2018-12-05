use git2::{Repository, Oid};
use crossbeam::channel::Receiver;
use utils::{get_commit, get_timesize_offset_secs, get_chan_revwalker};
use interface::{
    LogList, LogEmitter, LogTrait
};

const GRAPH_LANE: u8 = 1;
const GRAPH_COMMIT: u8 = 2;
const GRAPH_LEAF: u8 = 4;
const GRAPH_ROOT: u8 = 8;
const GRAPH_MERGE: u8 = 16;
const GRAPH_BRANCH: u8 = 32;

pub struct LogGraph {
    lanes: Vec<Oid>,
}

impl LogGraph {
    fn add_commit(&mut self, commit: &LogItem) -> Vec<u8> {
        let is_debug = false;
            // format!("{:?}", commit.id) == "6835bd0829d6b9ed474405ccf2d6c39d1f510913" ||
            // format!("{:?}", commit.id) == "1f2124e44a1cc5d64bc344f21b2874049c6f95d4" ||
            // format!("{:?}", commit.id) == "bcee17445c7633335b499871b78852b62513a947" ||
            // format!("{:?}", commit.id) == "a9918ba8e479c7901dd44fb30dec25cf85b53533";
        if is_debug {
            println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
            println!("commit.id: {}", commit.id);
            println!("commit.parents: {:?}", commit.parents);
            println!("lanes: {:?}", self.lanes);
        }
        let mut graph_vals = vec![GRAPH_LANE; self.lanes.len()];
        let mut c_idx = 0;
        if self.lanes.len() == 0 {
            self.lanes = commit.parents.clone();
            graph_vals.push(GRAPH_LEAF);
        } else {
            let mut branched_lanes: Vec<usize> = vec![]; // branced from here
            let mut first_matched = false;
            // determine what lane the commit belongs to, and what if any might be closed here
            for i in 0..self.lanes.len() {
                let lane = &mut self.lanes[i];
                if lane == &commit.id && !first_matched {
                    first_matched = true;
                    c_idx = i;
                    if commit.parents.len() > 0 {
                        graph_vals[i] |= GRAPH_COMMIT;
                        *lane = commit.parents[0];
                    } else {
                        graph_vals[i] = GRAPH_ROOT;
                    }
                } else if lane == &commit.id && first_matched {
                    graph_vals[i] = GRAPH_BRANCH;
                    branched_lanes.push(i);
                }
            }
            let mut reused_branched_idx = 0;
            for i in 1..commit.parents.len() {
                let parent = &commit.parents[i];
                if self.lanes.contains(parent) {
                    let mut p_idx = -1;
                    for ip in 0..self.lanes.len() {
                        let l = self.lanes[ip];
                        if l == *parent {
                            p_idx = ip as isize;
                            break;
                        }
                    }
                    assert!(p_idx > -1);
                    graph_vals[p_idx as usize] |= GRAPH_MERGE;
                } else {
                    if reused_branched_idx < branched_lanes.len() {
                        // reuse lane that was to be cloned
                        let lane_idx = branched_lanes[reused_branched_idx];
                        self.lanes[lane_idx] = *parent;
                        graph_vals[lane_idx] |= GRAPH_MERGE;
                        reused_branched_idx += 1;
                    } else {
                        // add new lane (that was merged to here)
                        self.lanes.push(*parent);
                        graph_vals.push(GRAPH_MERGE);
                    }
                }
            }
            // clone any lanes branched from here
            while reused_branched_idx < branched_lanes.len() {
                let last_idx = self.lanes.len() - 1;
                self.lanes.remove(last_idx);
                reused_branched_idx += 1;
            }
        }
        if is_debug {println!("graph_vals {:?}", graph_vals);}
        graph_vals.insert(0, c_idx as u8);
        graph_vals
    }
    fn reset(&mut self) {
        self.lanes = vec![];
    }
}

// removed "Default" trait, seems fine?
#[derive(Clone)]
pub struct LogItem {
    pub id: Oid,
    pub cid_short: String,
    pub time: String,
    pub author: String,
    pub message: String,
    // indicates what graph lane holds the commit
    pub graph_lane: i32,
    // all parents
    pub parents: Vec<Oid>,
    pub is_leaf: bool,
    pub graph: Vec<u8>,
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
    graph: LogGraph,
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
                            let mut e = get_commit(oid, self.tz_offset_sec, &r);
                            e.graph = self.graph.add_commit(&e);
                            //e.graph = "[{\"isCommit\": true},{},{}]".to_string();
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
            graph: LogGraph { lanes: vec![] },
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
        self.graph.reset();
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
    fn graph(&self, index: usize) -> &[u8] {
        &self.list[index].graph
    }
}

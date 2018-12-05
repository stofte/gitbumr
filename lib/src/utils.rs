use std::{path::{PathBuf}, thread};
use git2::{Repository, BranchType, Oid, Sort};
use chrono::prelude::*;
use chrono_humanize::HumanTime;
use crossbeam::{channel::Receiver, channel};
use implementation::{branches::BranchesItem, log::LogItem};

pub fn pathbuf_filename_to_string(pb: &PathBuf) -> String {
    pb.file_name().unwrap().to_string_lossy().to_string()
}

pub fn pathbuf_to_string(pb: PathBuf) -> String {
    pb.into_os_string().to_string_lossy().to_string()
}

pub fn get_timesize_offset_secs() -> i32 {
    let ldt = Local::now();
    let z = ldt.offset().local_minus_utc();
    z
}

pub fn strip_whitespace(str: &str) -> String {
    str.replace("\r\n", " ").replace("\n", " ")
}

pub fn is_git_repo(path: &str) -> Result<(), &'static str> {
	match Repository::open(path) {
        Ok(..) => Ok(()),
        Err(..) => Err("Folder is not a git repository")
    }
}

pub fn get_head(git_repo: &Repository) -> String {
    let hr = git_repo.head().unwrap();
    let n = hr.name().unwrap();
    let prefix = "refs/head/";
    let n_str = &n[prefix.len() + 1 .. n.len()];
    return n_str.to_string()
}

pub fn local_branches(repo: &Repository) -> Vec<BranchesItem> {
    let mut vec = Vec::new();
    let bs = repo.branches(Some(BranchType::Local)).unwrap();
    let head_name = get_head(&repo);
    for b in bs {
        let bb = b.unwrap().0;
        let b_ref = &*bb.get();
        let name = bb.name().unwrap().unwrap().to_owned();
        let mut chkout = false;
        if head_name == name {
            chkout = true;
        }
        let oid_str = b_ref.target().unwrap().to_string();
        vec.push(BranchesItem{ name: name, checkedout: chkout, oid: oid_str });
    }
    vec.sort_by(|a, b| a.name.cmp(&b.name));
    vec
}

pub fn get_commit(oid: Oid, tz_offset_sec: i32, repo: &Repository) -> LogItem {
    let c = repo.find_commit(oid).unwrap();
    let fo = FixedOffset::east(tz_offset_sec);
    let dt: DateTime<FixedOffset> = DateTime::from_utc(NaiveDateTime::from_timestamp(c.time().seconds(), 0), fo);
    let ht = HumanTime::from(dt);
    let m = strip_whitespace(c.message().unwrap());
    let a = c.author();
    let n = a.name().unwrap();
    let idstr = format!("{:?}", oid).to_string();
    let ps: Vec<Oid> = c.parents().map(|x| x.id()).collect();
    LogItem {
        id: oid,
        cid_short: idstr.chars().take(8).collect(),
        time: format!("{}", ht).to_string(),
        author: n.to_string(),
        message: m.to_string(),
        parents: ps,
        graph_lane: 0,
        graph: vec![],
        is_leaf: false // set by loggraph when parsing revwalk
    }
}

// filter is assumed to be a branch head oid str. see also implementation/log.rs
pub fn get_chan_revwalker(path: String, filter: String, max_count: usize) -> Receiver<(Vec<Oid>, bool)> {
    let (oids_s, oids_r) = channel::bounded(0);
    thread::spawn(move || {
        let oid = Oid::from_str(&filter).unwrap();
        let git = Repository::open(&path).unwrap();
        let mut rv = git.revwalk().unwrap();
        rv.push(oid).unwrap();
        rv.set_sorting(Sort::TIME | Sort::TOPOLOGICAL);
        let mut prv = rv.peekable();
        let mut data = vec![];
        let mut is_empty = false;
        loop {
            if data.len() >= max_count || is_empty {
                let has_more = prv.peek().is_some();
                oids_s.send((data, has_more)).unwrap();
                if is_empty {
                    break;
                }
                data = vec![];
            } else {
                match prv.next() {
                    Some(r) => data.push(r.unwrap()),
                    None => is_empty = true
                }
            }
        }
    });
    oids_r
}

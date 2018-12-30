use std::{path::{PathBuf}, thread, str};
use git2::{Repository, BranchType, Oid, Sort};
use chrono::{FixedOffset, DateTime, NaiveDateTime, Local};
use chrono_humanize::HumanTime;
use crossbeam::{channel::Receiver, channel};
use implementation::{branches::BranchesItem, log::LogItem, diffs::DiffsItem};

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

pub fn get_timesize_offset() -> FixedOffset {
    let ldt = Local::now();
    let z = ldt.offset().local_minus_utc();
    FixedOffset::east(z)
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
        cid: idstr.chars().collect(),
        time: format!("{}", ht).to_string(),
        time_humanized: format!("{}", ht).to_string(),
        author: n.to_string(),
        message: c.message().unwrap().to_string(),
        summary: c.summary().unwrap().to_string(),
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
                match oids_s.send((data, has_more)) {
                    Ok(..) => {
                        if is_empty {
                            break;
                        }
                        data = vec![];
                    },
                    // if we couldnt send the channel is broken and we just exit
                    Err(..) => break
                }
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

pub fn parse_diff_parent(commit: &git2::Commit, repo: &Repository) -> (Vec<DiffsItem>, usize) {
    let mut list = vec![];
    let mut max_filename_len = 0;
    match commit.parent_id(0) {
        Ok(id) => {
            let t = commit.tree().unwrap();
            let parent_c = repo.find_commit(id).unwrap();
            let parent_tree = repo.find_tree(parent_c.tree_id()).unwrap();
            let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&t), None).unwrap();
            let delta_cnt = diff.deltas().len();
            for idx in 0..delta_cnt {
                let delta = diff.get_delta(idx).unwrap();
                let delta_old_file = delta.old_file().path().unwrap();
                let delta_new_file = delta.new_file().path().unwrap();
                let delta_status = delta.status();
                let mut patch = git2::Patch::from_diff(&diff, idx).unwrap().unwrap();
                let mut hunks = vec![];
                let mut hunks_origins = vec![];
                let mut hunks_lineno_new = vec![];
                let mut hunks_lineno_old = vec![];
                for h_idx in 0..patch.num_hunks() {
                    let h_lines = patch.num_lines_in_hunk(h_idx).unwrap();
                    let mut hunk_str = String::with_capacity(300);
                    let mut hunk_origins_vec = vec![];
                    let mut hunk_lineno_new_vec = vec![];
                    let mut hunk_lineno_old_vec = vec![];
                    for h_line_idx in 0..h_lines {
                        let h_line = patch.line_in_hunk(h_idx, h_line_idx).unwrap();
                        // attempt to decode the hunk line as utf8. if this fails, assume it's a binary thing and skip the hunk
                        match str::from_utf8(h_line.content()) {
                            Ok(h_line_decoded) => {
                                hunk_str.push_str(str::from_utf8(h_line.content()).unwrap());
                                hunk_origins_vec.push(h_line.origin());
                                hunk_lineno_new_vec.push(h_line.new_lineno());
                                hunk_lineno_old_vec.push(h_line.old_lineno());
                            },
                            Err(..) => {
                                break;
                            }
                        }
                    }
                    hunks.push(hunk_str);
                    hunks_origins.push(hunk_origins_vec);
                    hunks_lineno_new.push(hunk_lineno_new_vec);
                    hunks_lineno_old.push(hunk_lineno_old_vec);
                }
                let patch_buf = patch.to_buf().unwrap();
                let mut patch_str = "";
                match patch_buf.as_str() {
                    Some(pstr) => patch_str = pstr,
                    None => ()
                };
                let fn_str = format!("{}", pathbuf_to_string(delta_new_file.to_path_buf()));
                if fn_str.len() > max_filename_len {
                    max_filename_len = fn_str.len();
                }
                list.push(DiffsItem {
                    filename: fn_str,
                    status: format!("{:?}", delta_status),
                    patch: patch_str.to_string(),
                    hunks: hunks,
                    lines_origin: hunks_origins,
                    lines_new: hunks_lineno_new,
                    lines_old: hunks_lineno_old,
                });
            }
        },
        Err(..) => panic!("handle root node!")
    }
    (list, max_filename_len)
}

use std::{path::{PathBuf}, thread, str};
use git2::{Repository, BranchType, Oid, Sort, DiffOptions, DiffFindOptions};
use chrono::{FixedOffset, DateTime, NaiveDateTime, Local};
use chrono_humanize::HumanTime;
use crossbeam::{channel::Receiver, channel};
use implementation::{branches::BranchesItem, log::LogItem, diffs::DiffsItem};

pub static MAX_U32_INT: u32 = 4294967295;

fn unpack_opt_str(str: Option<&str>) -> String {
    match str {
        Some(s) => s.to_string(),
        None => "".to_string()
    }
}

fn ends_with_newline(s: &str) -> bool {
    let last_char: char = s.chars().rev().take(1).collect::<Vec<char>>()[0];
    match last_char {
        '\n' => true,
        _ => false
    }
}

fn map_git_origin_sigil(c: char) -> u8 {
    match c {
        ' ' => 0, // context line
        '+' => 1, // added line
        '-' => 2, // deleted line
        '<' => 3,
        '>' => 4,
        '=' => 5,
        _   => panic!("unexpected sigil in map_git_origin_sigil: {}", c)
    }
}

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

pub fn is_git_repo(path: &str) -> Result<(), &'static str> {
	match Repository::open(path) {
        Ok(..) => Ok(()),
        Err(..) => Err("Folder is not a git repository")
    }
}

pub fn get_head(git_repo: &Repository) -> Result<String, &'static str> {
    let hr = git_repo.head().unwrap();
    let n = hr.name().unwrap();
    // if the head is detached, there's no branch ref
    if n == "HEAD" {
        return Ok(n.to_string())
    }
    let prefix = "refs/head/";
    let n_str = &n[prefix.len() + 1 .. n.len()];
    return Ok(n_str.to_string())
}

pub fn local_branches(repo: &Repository) -> Vec<BranchesItem> {
    let mut vec = Vec::new();
    let bs = repo.branches(Some(BranchType::Local)).unwrap();
    let head_name = get_head(&repo).unwrap();
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
    match repo.find_commit(oid) {
        Ok(c) => {
            let fo = FixedOffset::east(tz_offset_sec);
            let dt: DateTime<FixedOffset> = DateTime::from_utc(NaiveDateTime::from_timestamp(c.time().seconds(), 0), fo);
            let ht = HumanTime::from(dt);
            let a = c.author();
            let n = unpack_opt_str(a.name());
            let mesg = unpack_opt_str(c.message());
            let summ = unpack_opt_str(c.summary());
            let idstr = format!("{:?}", oid).to_string();
            let ps: Vec<Oid> = c.parents().map(|x| x.id()).collect();
            let is_merge = ps.len() > 1;
            LogItem {
                id: oid,
                cid: idstr.chars().collect(),
                time: format!("{}", ht).to_string(),
                time_humanized: format!("{}", ht).to_string(),
                author: n,
                message: mesg,
                summary: summ,
                parents: ps,
                graph_lane: 0,
                graph: vec![],
                is_merge,
                is_leaf: false // set by loggraph when parsing revwalk
            }
        },
        Err(..) => panic!("could not find commit {:?}", oid)
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
                    Some(r) => {
                        match r {
                            Ok(rr) => data.push(rr),
                            Err(..) => panic!("unpacked none in get_chan_revwalker")
                        }
                    },
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
    let t = commit.tree().unwrap();
    let mut diff_find_opts = DiffFindOptions::new();
    // not sure what the diff opts are really used for.
    // find_options controls the actual similary/rename checking.
    let mut diff_opts = DiffOptions::new();
    let mut diff = match commit.parent_id(0) {
        Ok(id) => {
            let parent_c = repo.find_commit(id).unwrap();
            let parent_tree = repo.find_tree(parent_c.tree_id()).unwrap();
            repo.diff_tree_to_tree(Some(&parent_tree), Some(&t), Some(&mut diff_opts)).unwrap()
        },
        Err(..) => repo.diff_tree_to_tree(None, Some(&t), Some(&mut diff_opts)).unwrap()
    };
    diff.find_similar(Some(&mut diff_find_opts)).unwrap();
    let delta_cnt = diff.deltas().len();
    for idx in 0..delta_cnt {
        let delta = diff.get_delta(idx).unwrap();
        let delta_old_file = delta.old_file().path().unwrap();
        let delta_new_file = delta.new_file().path().unwrap();
        let delta_status = delta.status();
        let mut patch = git2::Patch::from_diff(&diff, idx).unwrap().unwrap();
        let mut hunks = vec![];
        let mut hunk_listings = String::with_capacity(300);
        let mut hunks_lines = vec![];
        let mut hunks_max_line_length = vec![];
        let mut hunks_origins = vec![];
        let mut hunks_lineno_new = vec![];
        let mut hunks_lineno_old = vec![];
        hunk_listings.push_str(&"[");
        let patch_hunk_count = patch.num_hunks();
        for h_idx in 0..patch_hunk_count {
            let h_lines = patch.num_lines_in_hunk(h_idx).unwrap();
            let mut hunk_str = String::with_capacity(300);
            let mut hunk_origins_vec = vec![];
            let mut hunk_lineno_new_vec = vec![];
            let mut hunk_lineno_old_vec = vec![];
            let mut hunk_max_line_length = 0;
            let mut hunk_lines = 0;
            hunk_listings.push_str(&"[");
            for h_line_idx in 0..h_lines {
                let h_line = patch.line_in_hunk(h_idx, h_line_idx).unwrap();
                // attempt to decode the hunk line as utf8. if this fails, assume it's a binary thing and skip the hunk
                match str::from_utf8(h_line.content()) {
                    Ok(h_line_decoded) => {
                        if h_line_decoded.len() > hunk_max_line_length {
                            hunk_max_line_length = h_line_decoded.len();
                        }
                        hunk_str.push_str(h_line_decoded);
                        let new_lineno = match h_line.new_lineno() {
                            Some(i) => i,
                            None => 0
                        };
                        let old_lineno = match h_line.old_lineno() {
                            Some(i) => i,
                            None => 0
                        };
                        let line_sigil = map_git_origin_sigil(h_line.origin());
                        hunk_listings.push_str(&format!("{},{},{}", old_lineno, new_lineno, line_sigil));
                        if h_line_idx + 1 < h_lines {
                            hunk_listings.push_str(&",");
                        }
                        hunk_origins_vec.push(h_line.origin());
                        hunk_lineno_new_vec.push(h_line.new_lineno());
                        hunk_lineno_old_vec.push(h_line.old_lineno());
                        hunk_lines += 1;
                        // if the last line ends with a newline, we make sure thats also counted
                        let has_newline_in_last_line = h_line_idx == h_lines - 1 && ends_with_newline(h_line_decoded);
                        if has_newline_in_last_line {
                            hunk_lines += 1;
                            hunk_listings.push_str(&",0,0,0");
                        }
                    },
                    Err(..) => {
                        hunk_str.clear();
                        // todo handle this case for listings
                        // hunk_listings.clear();
                        hunk_origins_vec = vec![];
                        hunk_lineno_new_vec = vec![];
                        hunk_lineno_old_vec = vec![];
                        break;
                    }
                }
            }
            hunks.push(hunk_str);
            hunk_listings.push_str(&"]");
            if h_idx + 1 < patch_hunk_count {
                hunk_listings.push_str(&",");
            }
            hunks_lines.push(hunk_lines);
            hunks_origins.push(hunk_origins_vec);
            hunks_lineno_new.push(hunk_lineno_new_vec);
            hunks_lineno_old.push(hunk_lineno_old_vec);
            hunks_max_line_length.push(hunk_max_line_length);
        }
        hunk_listings.push_str(&"]");
        let patch_buf = patch.to_buf().unwrap();
        let mut patch_str = "";
        match patch_buf.as_str() {
            Some(pstr) => patch_str = pstr,
            None => ()
        };
        let old_fn_str = format!("{}", pathbuf_to_string(delta_old_file.to_path_buf()));
        let fn_str = format!("{}", pathbuf_to_string(delta_new_file.to_path_buf()));
        if fn_str.len() > max_filename_len {
            max_filename_len = fn_str.len();
        }
        list.push(DiffsItem {
            filename_old: old_fn_str,
            filename_new: fn_str,
            status: format!("{:?}", delta_status),
            patch: patch_str.to_string(),
            hunks: hunks,
            hunk_listings: hunk_listings,
            hunk_lines: hunks_lines,
            hunks_max_line_length: hunks_max_line_length,
            lines_origin: hunks_origins,
            lines_new: hunks_lineno_new,
            lines_old: hunks_lineno_old,
        });
    }
    (list, max_filename_len)
}

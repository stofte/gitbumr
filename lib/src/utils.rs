use std::path::{PathBuf};
use git2::{Repository, BranchType, Oid};
use chrono::prelude::*;
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
    let m = strip_whitespace(c.message().unwrap());
    let a = c.author();
    let n = a.name().unwrap();
    let idstr = format!("{:?}", oid).to_string();
    LogItem {
        oid: idstr.chars().take(8).collect(),
        time: dt.to_string(),
        author: n.to_string(),
        message: m.to_string(),
    }
}

use git2::{Oid, Repository, BranchType};
use chrono::prelude::*;

pub struct Branch {
    pub name: String,
    pub checkedout: bool
}

pub struct Commit {
    pub id_long: String,
    pub id: String,
    pub time: DateTime<FixedOffset>,
    pub author: String,
    pub author_abbrev: String,
    pub message: String,
    pub message_line: String,
}

pub fn get_head(git_repo: &Repository) -> String {
    let hr = git_repo.head().unwrap();
    let n = hr.name().unwrap();
    let prefix = "refs/head/";
    let n_str = &n[prefix.len() + 1 .. n.len()];
    return n_str.to_string()
}

pub fn local_branches(repo: &Repository) -> Vec<Branch> {
    let mut vec = Vec::new();
    let bs = repo.branches(Some(BranchType::Local)).unwrap();
    let head_name = get_head(&repo);
    for b in bs {
        let bb = b.unwrap().0;
        let name = bb.name().unwrap().unwrap().to_owned();
        let mut chkout = false;
        if head_name == name {
            chkout = true;
        }
        vec.push(Branch{ name: name, checkedout: chkout });
    }
    vec.sort_by(|a, b| a.name.cmp(&b.name));
    vec
}

pub fn get_repository_path(repo: &Repository) -> String {
    let mut path = repo.path().to_str().unwrap().to_string();
    if path.ends_with("/.git/") {
        path = path.chars().take(path.len() - 6).collect();
    }
    path
}

pub fn get_commit(oid: Oid, tz_offset_sec: i32, repo: &Repository) -> Commit {
    let c = repo.find_commit(oid).unwrap();
    let dt: DateTime<FixedOffset> = DateTime::from_utc(NaiveDateTime::from_timestamp(c.time().seconds(), 0), FixedOffset::east(tz_offset_sec));
    let m = c.message().unwrap();
    // todo filters on ascii values, probably very brittle
    let zz = m.chars().into_iter().filter(|c| *c > 0x1F as char && *c < 0x7F as char).collect();
    let a = c.author();
    let n = a.name().unwrap();
    let idstr = format!("{:?}", oid).to_string();
    let author_abbrev: String = n.split(" ").filter(|s| s.len() > 0).map(|s| s.chars().next().unwrap()).collect();
    Commit {
        id: idstr.chars().take(8).collect(),
        id_long: idstr,
        time: dt,
        author: n.to_string(),
        author_abbrev: author_abbrev.to_string(),
        message: m.to_string(),
        message_line: zz,
    }
}

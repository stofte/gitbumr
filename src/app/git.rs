use git2::{Oid, Repository, BranchType};
use chrono::prelude::*;

pub struct Branch {
    pub name: String,
    pub checkedout: bool
}

pub struct Commit {
    pub id_long: String,
    pub id: String,
    pub time: DateTime<Utc>,
    pub author: String,
    pub message: String,
    pub message_line: String,
}

pub fn local_branches(git_repo: &Repository) -> Vec<Branch> {
    let mut vec = Vec::new();
    let bs = git_repo.branches(Some(BranchType::Local)).unwrap();
    let head_name = get_head(&git_repo);
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

pub fn get_head(git_repo: &Repository) -> String {
    let hr = git_repo.head().unwrap();
    let n = hr.name().unwrap();
    let prefix = "refs/head/";
    let n_str = &n[prefix.len() + 1 .. n.len()];
    return n_str.to_string()
}

pub fn get_commit(oid: Oid, repo: &Repository) -> Commit {
    let c = repo.find_commit(oid).unwrap();
    let dt: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp(c.time().seconds(), 0), Utc);
    let m = c.message().unwrap();
    let zz = m.chars().into_iter().filter(|c| *c != '\n' && *c != '\r' && *c != '\t').collect();
    let a = c.author();
    let n = a.name().unwrap();
    let idstr = format!("{:?}", oid).to_string();
    Commit {
        id: idstr.chars().take(8).collect(),
        id_long: idstr,
        time: dt,
        author: n.to_string(),
        message: m.to_string(),
        message_line: zz,
    }
}

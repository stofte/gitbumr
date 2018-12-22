use git2::{Repository, Oid, Commit};
use chrono::{FixedOffset, DateTime, NaiveDateTime};
use interface::{CommitModelTrait, CommitModelEmitter};

pub struct CommitModel {
    emit: CommitModelEmitter,
    cid: Option<String>,
    author: Option<String>,
    committer: Option<String>,
    message: Option<String>,
    time: Option<String>,
    tree: Option<String>,
    oid: Oid
}

pub fn fill_commit(commit: &mut CommitModel, c: &Commit, repo: &Repository, tz_offset: FixedOffset) {
    let dt: DateTime<FixedOffset> = DateTime::from_utc(NaiveDateTime::from_timestamp(c.time().seconds(), 0), tz_offset);
    let dt_str = dt.format("%c").to_string();
    let cid = format!("{:?}", c.id()).to_string();
    let tree_id = format!("{:?}", c.tree_id()).to_string();
    let msg = c.message().unwrap().to_string();
    let author = c.author();
    let author_str = author.name().unwrap().to_string();
    let comitter = c.committer();
    let comitter_str = comitter.name().unwrap().to_string();
    commit.cid = Some(cid);
    commit.emit.cid_changed();
    commit.time = Some(dt_str);
    commit.emit.time_changed();
    commit.tree = Some(tree_id);
    commit.emit.tree_changed();
    commit.message = Some(msg);
    commit.emit.message_changed();
    commit.author = Some(author_str);
    commit.emit.author_changed();
    commit.committer = Some(comitter_str);
    commit.emit.committer_changed();
}

impl CommitModelTrait for CommitModel {
    fn new(emit: CommitModelEmitter) -> CommitModel {
        CommitModel {
            oid: Oid::zero(),
            cid: None,
            author: None,
            committer: None,
            message: None,
            time: None,
            tree: None,
            emit
        }
    }
    fn emit(&mut self) -> &mut CommitModelEmitter {
        &mut self.emit
    }
    fn author(&self) -> &str {
        match self.author {
            Some(ref v) => &v,
            None => ""
        }
    }
    fn cid(&self) -> &str {
        match self.cid {
            Some(ref v) => &v,
            None => ""
        }
    }
    fn committer(&self) -> &str {
        match self.committer {
            Some(ref v) => &v,
            None => ""
        }
    }
    fn message(&self) -> &str {
        match self.message {
            Some(ref v) => &v,
            None => ""
        }
    }
    fn time(&self) -> &str {
        match self.time {
            Some(ref v) => &v,
            None => ""
        }
    }
    fn tree(&self) -> &str {
        match self.tree {
            Some(ref v) => &v,
            None => ""
        }
    }
}

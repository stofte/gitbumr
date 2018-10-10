use std::{env, path::{Path}, fs::{File, canonicalize}};
use rusqlite::{Connection};
use git2::Repository;
use chrono::prelude::*;

pub struct StoredRepository {
    pub id: i64,
    pub path: String,
    pub open: bool,
}

struct Table {
    name: String
}

pub struct Settings {
    conn: Connection,
}

impl Settings {
    pub fn init(&mut self) {
        let mut stmt = self.conn.prepare("SELECT name FROM sqlite_master WHERE type='table';").unwrap();
        let tbl_iter = stmt.query_map(&[], |row| {
            Table {
                name: row.get(0)
            }
        }).unwrap();
        let mut has_repos = false;
        for tbl in tbl_iter {
            let n = tbl.unwrap().name;
            if n == "repos" {
                has_repos = true;
            }
        }
        if !has_repos {
            self.conn.execute("
                CREATE TABLE repos (
                    path TEXT UNIQUE NOT NULL,
                    open BIT NOT NULL
                )", &[]).unwrap();
        }
    }
    pub fn get_repositories(&self) -> Vec<StoredRepository> {
        let mut repos = vec![];
        let mut stmt = self.conn.prepare("SELECT rowid, path, open FROM repos ORDER BY path;").unwrap();
        let rows = stmt.query_map(&[], |row| {
            StoredRepository {
                id: row.get(0),
                path: row.get(1),
                open: row.get(2),
            }
        }).unwrap();
        for r in rows {
            repos.push(r.unwrap());
        }
        repos
    }
    pub fn add_repository(&mut self, path: &str) -> Result<(), &'static str> {
        // get canonical name (using os functions, so hits the filesystem)
        let mut p = path.to_string();
        match canonicalize(path) {
            Ok(cpb) => p = cpb.into_os_string().into_string().unwrap(),
            // can't return err msg itself, must explicitly return static strings
            Err(..) => return Err("accessing path")
        };
        // verify we have git repo
        match Repository::open(&p) {
            Ok(..) => (),
            Err(..) => return Err("not a git repository")
        };
        // todo use try! macro https://docs.rs/rusqlite/0.14.0/rusqlite/struct.Transaction.html#example
        let tx = self.conn.transaction().unwrap();
        tx.execute("UPDATE repos SET open=0 WHERE 1=1", &[]);
        // this works for now
        match tx.execute("INSERT INTO repos(path, open) VALUES(?1, 1)", &[&p]) {
            Err(..) => {
                tx.rollback().unwrap();
                Err("already in list")
            }
            _ => {
                tx.commit().unwrap();
                Ok(())
            }
        }
    }
    pub fn get_open_repository(&self) -> Option<StoredRepository> {
        let repos = self.get_repositories();
        for repo in repos {
            if repo.open {
                return Some(StoredRepository {
                    id: repo.id,
                    path: repo.path.clone(),
                    open: repo.open,
                })
            }
        }
        None
    }
    pub fn open_repository(&mut self, id: i64) {
        let tx = self.conn.transaction().unwrap();
        tx.execute("UPDATE repos SET open=0 WHERE 1=1", &[]);
        match tx.execute("UPDATE repos SET open=1 WHERE rowid=?1", &[&id]) {
            Err(..) => {
                tx.rollback().unwrap();
                panic!("open_repository received unknown id")
            }
            _ => {
                tx.commit().unwrap();
            }
        };
    }
    pub fn get_open_repo(&mut self) -> Option<Repository> {
        match self.get_open_repository() {
            Some(repo) => {
                match Repository::open(repo.path) {
                    Ok(r) => Some(r),
                    _ => None
                }
            }
            None => None
        }
    }
}

pub fn build_settings() -> Settings {
    let db_path = format!("{}/.gitbumrdb", env::var("HOME").unwrap());
    let p = Path::new(&db_path);
    if !p.exists() {
        File::create(&db_path).unwrap();
    }
    let sqlite_conn = Connection::open(p).unwrap();
    Settings { conn: sqlite_conn }
}

pub fn get_timesize_offset_secs() -> i32 {
    let ldt = Local::now();
    let z = ldt.offset().local_minus_utc();
    z
}

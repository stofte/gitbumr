use std::{error::Error, env, path::Path, fs::File};
use rusqlite::{Connection, Transaction};

pub struct Settings {
    conn: Connection
}

pub struct StoredRepository {
    pub path: String,
    pub open: bool,
}

struct Table {
    name: String
}

impl Settings {
    pub fn init(&mut self) {
        let mut stmt = self.conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='repos';").unwrap();
        let tbl_iter = stmt.query_map(&[], |row| {
            Table {
                name: row.get(0)
            }
        }).unwrap();
        let mut has_repos = false;
        let mut has_version = false;
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
        let mut stmt = self.conn.prepare("SELECT path, open FROM repos ORDER BY path;").unwrap();
        let rows = stmt.query_map(&[], |row| {
            StoredRepository {
                path: row.get(0),
                open: row.get(1),
            }
        }).unwrap();
        for r in rows {
            repos.push(r.unwrap());
        }
        repos
    }
    pub fn add_repository(&mut self, path: &str) -> Result<(), &'static str> {
        // todo use try! macro https://docs.rs/rusqlite/0.14.0/rusqlite/struct.Transaction.html#example
        let tx = self.conn.transaction().unwrap();
        tx.execute("UPDATE repos SET open=0 WHERE 1=1", &[]);
        // this works for now
        match tx.execute("INSERT INTO repos(path, open) VALUES(?1, 1)", &[&path]) {
            Err(..) => {
                tx.rollback();
                Err("Failed to add repository")
            }
            _ => {
                tx.commit();
                Ok(())
            }
        }
    }
}

pub fn build_settings() -> Settings {
    let db_path = format!("{}/.gitbumrdb", env::var("HOME").unwrap());
    let p = Path::new(&db_path);
    if !p.exists() {
        File::create(&db_path);
    }
    let sqlite_conn = Connection::open(p).unwrap();
    Settings { conn: sqlite_conn }
}

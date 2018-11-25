#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use std::{path::Path, println};
use rusqlite::{Connection, NO_PARAMS};
use url::Url;
use git::is_git_repo;
use interface::*;

#[derive(Default, Clone)]
pub struct RepositoriesItem {
    id: i64,
    current: bool,
    path: String,
    display_name: String,
}

pub struct Repositories {
    emit: RepositoriesEmitter,
    model: RepositoriesList,
    list: Vec<RepositoriesItem>,
    conn: Option<Connection>,
    active_repository_idx: usize,
    add_last_error_text: String,
}

impl RepositoriesTrait for Repositories {
    fn new(emit: RepositoriesEmitter, model: RepositoriesList) -> Repositories {
        Repositories {
            emit,
            model,
            list: vec![],
            conn: None,
            active_repository_idx: 0,
            add_last_error_text: "".to_string()
        }
    }
    fn init(&mut self, db_file_name: String) {
        let p = Path::new(&db_file_name);
        let sqlite_conn = Connection::open(p).unwrap();
        self.conn = Some(sqlite_conn);
        init_sqlite(&self);
        let repos = get_repositories(&self);
        // insert_rows takes start and end indexes of inserted items,
        // so insert one item is 0,0, and two items is 0,1.
        if repos.len() > 0 {
            self.model.begin_insert_rows(0, repos.len() - 1);
            self.list = repos;
            self.model.end_insert_rows();
            for idx in 0..self.list.len() {
                let e = &self.list[idx];
                if e.current {
                    self.active_repository_idx = idx;
                    self.emit.active_repository_changed();
                    break;
                }
            }
        }
    }
    fn emit(&mut self) -> &mut RepositoriesEmitter {
        &mut self.emit
    }
    fn row_count(&self) -> usize {
        self.list.len()
    }
    fn current(&self, index: usize) -> bool {
        self.list[index].current
    }
    fn set_current(&mut self, id: i64) {
        set_open_repository(self, id);
        let mut idx = 0;
        let mut i = 0;
        self.model.begin_reset_model();
        for elm in &mut self.list {
            elm.current = false;
            if elm.id == id && idx == 0 {
                idx = i
            }
            i += 1;
        }
        self.list[idx].current = true;
        self.model.end_reset_model();
        self.active_repository_idx = idx;
        self.emit.active_repository_changed();
    }
    fn display_name(&self, index: usize) -> &str {
        &self.list[index].display_name
    }
    fn id(&self, index: usize) -> i64 {
        self.list[index].id
    }
    fn add(&mut self, path: String) -> bool {
        // path strings suck in rust
        let os_path = Url::parse(&path).unwrap().to_file_path().unwrap().into_os_string();
        let p = os_path.to_string_lossy().to_string();
        match is_git_repo(&p) {
            Err(txt) => {
                self.add_last_error_text = txt.to_string();
                return false
            },
            Ok(..) => ()
        };
        let rowid = match add_repository(self, &p) {
            Err(txt) => {
                self.add_last_error_text = txt.to_string();
                return false
            }
            Ok(id) => id
        };
        // mark others as inactive
        for mut e in &mut self.list {
            e.current = false;
        }
        let item = RepositoriesItem {
            current: true,
            display_name: p.clone(),
            path: p,
            id: 0
        };
        let idx = 0; // inserts at the top
        self.model.begin_insert_rows(idx, idx);
        self.list.insert(idx, item);
        self.model.end_insert_rows();
        self.active_repository_idx = 0;
        self.emit.active_repository_changed();
        true
    }
    fn remove(&mut self, id: u64) -> bool {
        false
    }
    fn active_repository(&self) -> &str {
        &self.list[self.active_repository_idx].display_name
    }
    fn add_last_error(&self) -> String {
        self.add_last_error_text.clone()
    }
}

struct Table { name: String }

fn init_sqlite(repos: &Repositories) {
    // must be some nicer way?
    match &repos.conn {
        Some(conn) => {
            let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table';").unwrap();
            let tbl_iter = stmt.query_map(NO_PARAMS, |row| {
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
                conn.execute("
                    CREATE TABLE repos (
                        path TEXT UNIQUE NOT NULL,
                        open BIT NOT NULL
                    )", NO_PARAMS).unwrap();
            }            
        }
        None => panic!("expected connection")
    };
}

fn get_repositories(repos: &Repositories) -> Vec<RepositoriesItem> {
    match &repos.conn {
        Some(conn) => {
            let mut res = vec![];
            let mut stmt = conn.prepare("SELECT rowid, path, open FROM repos ORDER BY path;").unwrap();
            let rows = stmt.query_map(NO_PARAMS, |row| {
                RepositoriesItem {
                    id: row.get(0),
                    path: row.get(1),
                    current: row.get(2),
                    display_name: row.get(1),
                }
            }).unwrap();

            for r in rows {
                res.push(r.unwrap());
            }
            res
        }
        None => panic!("expected connection")
    }
}

fn add_repository(repos: &mut Repositories, path: &str) -> Result<i64, &'static str> {
    match &mut repos.conn {
        Some(conn) => {
            let mut res = Err("err");
            {
                // todo use try! macro https://docs.rs/rusqlite/0.14.0/rusqlite/struct.Transaction.html#example
                let tx = conn.transaction().unwrap();
                tx.execute("UPDATE repos SET open=0 WHERE 1=1", NO_PARAMS).unwrap();
                // this works for now
                res = match tx.execute("INSERT INTO repos(path, open) VALUES(?1, 1)", &[&path]) {
                    Err(..) => {
                        tx.rollback().unwrap();
                        Err("Repository already added")
                    }
                    _ => {
                        tx.commit().unwrap();
                        Ok(())
                    }
                }
            }
            match res {
                Err(txt) => Err(txt),
                Ok(()) => Ok(conn.last_insert_rowid())
            }
        }
        None => panic!("expected connection")
    }
}

fn set_open_repository(repos: &mut Repositories, id: i64) {
    match &mut repos.conn {
        Some(conn) => {
            let tx = conn.transaction().unwrap();
            tx.execute("UPDATE repos SET open=0 WHERE 1=1", NO_PARAMS).unwrap();
            match tx.execute("UPDATE repos SET open=1 WHERE rowid=?1", &[&id]) {
                Err(..) => {
                    tx.rollback().unwrap();
                    panic!("set_open_repository received unknown id")
                }
                _ => {
                    tx.commit().unwrap();
                }
            };
        }
        None => panic!("expected connection")
    }
}

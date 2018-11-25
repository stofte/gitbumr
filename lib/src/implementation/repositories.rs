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
    add_last_error_text: String,
}

impl RepositoriesTrait for Repositories {
    fn new(emit: RepositoriesEmitter, model: RepositoriesList) -> Repositories {
        Repositories {
            emit,
            model,
            list: vec![],
            conn: None,
            add_last_error_text: "".to_string()
        }
    }
    fn init(&mut self, db_file_name: String) {
        let p = Path::new(&db_file_name);
        let sqlite_conn = Connection::open(p).unwrap();
        self.conn = Some(sqlite_conn);
        init_sqlite(&self);
        let repos = get_repositories(&self);
        println!("repo count from db => {}", repos.len());
        // insert_rows takes start and end indexes of inserted items,
        // so insert one item is 0,0, and two items is 0,1.
        if repos.len() > 0
        {
            self.model.begin_insert_rows(0, repos.len() - 1);
            self.list = repos;
            self.model.end_insert_rows();
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
    fn set_current(&mut self, index: u64) {
        for elm in &mut self.list {
            elm.current = false;
        }
        self.list[index as usize].current = true;
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
        match add_repository(self, &p) {
            Err(txt) => {
                self.add_last_error_text = txt.to_string();
                return false
            }
            Ok(..) => ()
        };
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
        true
    }
    fn remove(&mut self, id: u64) -> bool {
        false
    }
    fn active_repository(&self) -> &str {
        "C:\\src\\CLEVER"
    }
    fn add_last_error(&self) -> String {
        println!("add_last_error retunring {}", self.add_last_error_text);
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

fn add_repository(repos: &mut Repositories, path: &str) -> Result<(), &'static str> {
    match &mut repos.conn {
        Some(conn) => {
            // todo use try! macro https://docs.rs/rusqlite/0.14.0/rusqlite/struct.Transaction.html#example
            let tx = conn.transaction().unwrap();
            tx.execute("UPDATE repos SET open=0 WHERE 1=1", NO_PARAMS).unwrap();
            // this works for now
            match tx.execute("INSERT INTO repos(path, open) VALUES(?1, 1)", &[&path]) {
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
        None => panic!("expected connection")
    }
}

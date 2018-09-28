use rusqlite::Connection;

pub struct Database<'a> {
    pub conn: &'a Connection
}

pub struct StoredRepository {
    pub path: String,
}

pub fn get_repositories(conn: &Database) -> Vec<StoredRepository> {
    println!("get_repositories");
    vec![]
}

pub fn add_repository(conn: &Connection) {
    println!("read_rows");
}

use rusqlite::Connection;

pub struct Database<'a> {
    pub conn: &'a Connection
}

pub fn get_repositories(conn: &Database) {
    println!("read_rows");
}

pub fn add_repository(conn: &Connection) {
    println!("read_rows");
}

use app::db::{get_repositories, Database};

pub fn view(conn: &Database) {
    let repos = get_repositories(&conn);
}

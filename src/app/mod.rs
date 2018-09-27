pub mod git;
pub mod console;
pub mod db;
pub mod repo;
pub mod control;
pub mod color;

use git2::Repository;
use termion::{terminal_size};
use app::control::{header::Header, branches::Branches};

pub struct UpdateData<'a> {
    pub console_width: Option<u16>,
    pub console_height: Option<u16>,
    pub key_value: Option<char>,
    pub git_repo: Option<&'a Repository>
}

pub struct Application {
    pub header: Header,
    pub branches: Branches,
}

pub fn fill_update_data(repo: &Repository) -> UpdateData {
    let (size_col, size_row) = terminal_size().unwrap();
    let ud = UpdateData {
        console_width: Some(size_col),
        console_height: Some(size_row),
        key_value: None,
        git_repo: Some(repo)
    };
    ud
}

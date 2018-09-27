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

pub struct Layout {
    pub top: u16,
    pub left: u16,
    pub width: u16,
    pub height: u16,
}

fn empty_layout() -> Layout {
    Layout {
        top: 0,
        left: 0,
        width: 0,
        height: 0
    }
}

pub fn empty_application() -> Application {
    Application {
        header: Header{ repo_path: "".to_string(), state: "".to_string(), width: 0, height: 0, layout: empty_layout() },
        branches: Branches{ local: vec![], remote: vec![], checkedout_idx: None, layout: empty_layout() },
    }
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

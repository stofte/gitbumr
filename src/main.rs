extern crate termion;
extern crate git2;
extern crate rusqlite;
mod app;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use git2::Repository;
use app::branch;
use app::draw::reset_screen;
use app::db::{Database, get_repositories, add_repository};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let repo_path = "/mnt/c/src/CLEVER";
    let repo = match Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    
    let size = termion::terminal_size().unwrap();
    let mut ui_state = app::state::UiState { repository: repo_path, git_repo: &repo, width: size.0, height: size.1 };

    let sqlite_conn = rusqlite::Connection::open_in_memory().unwrap();
    let db = Database { conn: &sqlite_conn };
    
    reset_screen(&mut stdout);
    get_repositories(&db);

    for c in stdin.keys() {
        reset_screen(&mut stdout);
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('b') => branch::view(&mut ui_state),
            Key::Char(c) => println!("{}", c),
            Key::Alt(c) => println!("^{}", c),
            Key::Ctrl(c) => println!("*{}", c),
            Key::Esc => println!("ESC"),
            Key::Left => println!("←"),
            Key::Right => println!("→"),
            Key::Up => println!("↑"),
            Key::Down => println!("↓"),
            Key::Backspace => println!("×"),
            _ => {}
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

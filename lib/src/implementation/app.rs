#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use interface::*;
use super::*;

pub struct App {
    emit: AppEmitter,
    repositories: Repositories,
}

impl AppTrait for App {
    fn new(emit: AppEmitter,
        repositories: Repositories) -> Self {
        let mut app = App {
            emit,
            repositories,
        };
        {
            let repos = app.repositories_mut();
            repos.list.push(RepositoriesItem { current: false, display_name: "hej mor!".to_string(), id: 0 });
        }
        app
    }
    fn init(&mut self) {

    }
    fn add_repository(&mut self, path: String) -> u64 {
        1
    }
    fn add_repository_get_last_error(&self) -> String {
        "Folder was not a git repository".to_string()
    }
    fn repository_index(&self, id: u64) -> u64 {
        0
    }
    fn emit(&mut self) -> &mut AppEmitter {
        &mut self.emit
    }
    fn repositories(&self) -> &Repositories {
        &self.repositories
    }
    fn repositories_mut(&mut self) -> &mut Repositories {
        &mut self.repositories
    }
    fn set_active_repository(&mut self, id: u64) {

    }
    fn active_repository(&self) -> u64 {
        1
    }
    fn active_repository_display_name(&self) -> &str {
        "foo"
    }
    fn active_repository_path(&self) -> &str {
        "C:\\some\\where"
    }
}

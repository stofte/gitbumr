#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use interface::*;

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
            repos.count = repos.list.len() as u64;
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
}

#[derive(Default, Clone)]
pub struct RepositoriesItem {
    current: bool,
    display_name: String,
    id: u64,
}

pub struct Repositories {
    emit: RepositoriesEmitter,
    model: RepositoriesList,
    pub count: u64,
    pub list: Vec<RepositoriesItem>,
}

impl RepositoriesTrait for Repositories {
    fn new(emit: RepositoriesEmitter, model: RepositoriesList) -> Repositories {
        Repositories {
            emit,
            model,
            list: vec![],
            count: 0,
        }
    }
    fn emit(&mut self) -> &mut RepositoriesEmitter {
        &mut self.emit
    }
    fn count(&self) -> u64 {
        self.count
    }
    fn row_count(&self) -> usize {
        self.list.len()
    }
    fn current(&self, index: usize) -> bool {
        self.list[index].current
    }
    fn set_current(&mut self, index: usize, v: bool) -> bool {
        self.list[index].current = v;
        true
    }
    fn display_name(&self, index: usize) -> &str {
        &self.list[index].display_name
    }
    fn id(&self, index: usize) -> u64 {
        self.list[index].id
    }
    fn add(&mut self, index: u64, path: String) -> bool {
        let item = RepositoriesItem { 
            current: true,
            display_name: path,
            id: 0
        };
        let idx = index as usize;
        self.model.begin_insert_rows(idx, idx);
        self.list.insert(idx, item);
        self.model.end_insert_rows();
        true

    }
    fn remove(&mut self, id: u64) -> bool {
        false
    }
}

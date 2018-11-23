#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use interface::*;

#[derive(Default, Clone)]
pub struct RepositoriesItem {
    pub current: bool,
    pub display_name: String,
    pub id: u64,
}

pub struct Repositories {
    emit: RepositoriesEmitter,
    model: RepositoriesList,
    pub list: Vec<RepositoriesItem>,
}

impl RepositoriesTrait for Repositories {
    fn new(emit: RepositoriesEmitter, model: RepositoriesList) -> Repositories {
        Repositories {
            emit,
            model,
            list: vec![],
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
    fn active_repository(&self) -> &str {
        "C:\\src\\CLEVER"
    }
    fn add_last_error(&self) -> String {
        "".to_string()
    }
}

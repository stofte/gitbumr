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

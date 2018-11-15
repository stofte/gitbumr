#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use interface::*;

#[derive(Default, Clone)]
struct RepositoriesItem {
    current: bool,
    display_name: String,
    id: u64,
}

pub struct Repositories {
    emit: RepositoriesEmitter,
    model: RepositoriesList,
    count: u64,
    list: Vec<RepositoriesItem>,
}

impl RepositoriesTrait for Repositories {
    fn new(emit: RepositoriesEmitter, model: RepositoriesList) -> Repositories {
        Repositories {
            emit,
            model,
            list: vec![
                RepositoriesItem { current: true, display_name: "hej mor".to_string(), id: 0 },
                RepositoriesItem { current: false, display_name: "noget mere tekst her".to_string(), id: 1 },
                RepositoriesItem { current: false, display_name: "dpi aware?".to_string(), id: 2 },
            ],
            count: 3,
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
    fn add(&mut self, path: String) -> bool {
        let item = RepositoriesItem { 
            current: true,
            display_name: path,
            id: 0
        };
        let end = self.list.len();
        self.model.begin_insert_rows(end, end);
        self.list.insert(end, item);
        self.model.end_insert_rows();
        true

    }
    fn remove(&mut self, id: u64) -> bool {
        false
    }
}

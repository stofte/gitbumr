#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use interface::*;

#[derive(Default, Clone)]
pub struct HistoryItem {
    pub oid: String,
    pub time: String,
    pub author: String,
    pub message: String,
}

pub struct History {
    emit: HistoryEmitter,
    model: HistoryList,
    list: Vec<HistoryItem>,
}

impl HistoryTrait for History {
    fn new(emit: HistoryEmitter, model: HistoryList) -> History {
        History {
            emit,
            model,
            list: vec![],
        }
    }
    fn emit(&mut self) -> &mut HistoryEmitter {
        &mut self.emit
    }
    fn row_count(&self) -> usize {
        self.list.len()
    }
    fn author(&self, index: usize) -> &str {
        &self.list[index].author
    }
    fn message(&self, index: usize) -> &str {
        &self.list[index].message
    }
    fn oid(&self, index: usize) -> &str {
        &self.list[index].oid
    }
    fn time(&self, index: usize) -> &str {
        &self.list[index].time
    }
    fn load(&mut self, path: String) {
    }
}

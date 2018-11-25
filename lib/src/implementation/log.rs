use interface::{
    LogList, LogEmitter, LogTrait
};

#[derive(Default, Clone)]
pub struct LogItem {
    pub oid: String,
    pub time: String,
    pub author: String,
    pub message: String,
}

pub struct Log {
    emit: LogEmitter,
    model: LogList,
    list: Vec<LogItem>,
}

impl LogTrait for Log {
    fn new(emit: LogEmitter, model: LogList) -> Log {
        Log {
            emit,
            model,
            list: vec![],
        }
    }
    fn emit(&mut self) -> &mut LogEmitter {
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
    fn filter(&mut self, filter: String) {

    }
}

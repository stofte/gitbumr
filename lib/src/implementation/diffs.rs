use git2::{Repository, Commit};
use interface::{DiffsTrait, DiffsEmitter, DiffsList};
use utils::{parse_diff_parent};

#[derive(Default, Clone)]
pub struct DiffsItem {
    pub filename: String,
    pub status: String,
    pub patch: String,
}

pub struct Diffs {
    emit: DiffsEmitter,
    model: DiffsList,
    list: Vec<DiffsItem>,
}

pub fn fill_diffs(diffs: &mut Diffs, commit: &Commit, repo: &Repository) {
    diffs.model.begin_reset_model();
    diffs.list = parse_diff_parent(commit, repo);
    diffs.model.end_reset_model();
}

impl DiffsTrait for Diffs {
    fn new(emit: DiffsEmitter, model: DiffsList) -> Diffs {
        Diffs {
            emit,
            model,
            list: vec![]
        }
    }
    fn emit(&mut self) -> &mut DiffsEmitter {
        &mut self.emit
    }
    fn row_count(&self) -> usize {
        self.list.len()
    }
    fn filename(&self, index: usize) -> &str {
        &self.list[index].filename
    }
    fn patch(&self, index: usize) -> &str {
        &self.list[index].patch
    }
    fn status(&self, index: usize) -> &str {
        &self.list[index].status
    }
}

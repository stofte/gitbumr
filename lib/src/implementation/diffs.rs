use git2::{Repository, Commit};
use interface::{DiffsTrait, DiffsEmitter, DiffsList};
use utils::{parse_diff_parent};

#[derive(Default, Clone)]
pub struct DiffsItem {
    pub filename_new: String,
    pub filename_old: String,
    // https://docs.rs/git2/0.7.5/git2/enum.Delta.html
    // Added
    // Deleted
    // Modified
    // Renamed
    // Copied
    // Ignored
    // Untracked
    // Typechange
    // Unreadable
    // Conflicted
    pub status: String,
    pub patch: String,
    // The following cannot be accesed via the diffs list directly, but when a file
    // in the diff list is clicked, the hunks model is loaded up with the below data,
    // via git.rs. Some minimal parsing is done to convert vectors to bytearrays
    // (which becomes a typed array in qml/js), when the list is loaded into hunks.
    pub hunks: Vec<String>,
    pub lines_origin: Vec<Vec<char>>,
    pub lines_new: Vec<Vec<Option<u32>>>,
    pub lines_old: Vec<Vec<Option<u32>>>,
}

pub struct Diffs {
    emit: DiffsEmitter,
    model: DiffsList,
    pub list: Vec<DiffsItem>,
    pub commit_oid: String,
    pub max_filename_length: u64,
}

pub fn fill_diffs(diffs: &mut Diffs, commit: &Commit, repo: &Repository, oid: String) {
    diffs.model.begin_reset_model();
    let (newdifflist, difflist_max_fn_len) = parse_diff_parent(commit, repo);
    diffs.list = newdifflist;
    diffs.commit_oid = oid;
    diffs.model.end_reset_model();
    diffs.max_filename_length = difflist_max_fn_len as u64;
    diffs.emit.max_filename_length_changed();
}

impl DiffsTrait for Diffs {
    fn new(emit: DiffsEmitter, model: DiffsList) -> Diffs {
        Diffs {
            emit,
            model,
            list: vec![],
            commit_oid: "".to_string(),
            max_filename_length: 0,
        }
    }
    fn emit(&mut self) -> &mut DiffsEmitter {
        &mut self.emit
    }
    fn row_count(&self) -> usize {
        self.list.len()
    }
    fn filename_old(&self, index: usize) -> &str {
        &self.list[index].filename_old
    }
    fn filename_new(&self, index: usize) -> &str {
        &self.list[index].filename_new
    }
    fn patch(&self, index: usize) -> &str {
        &self.list[index].patch
    }
    fn status(&self, index: usize) -> &str {
        &self.list[index].status
    }
    fn commit_oid(&self) -> &str {
        &self.commit_oid
    }
    fn max_filename_length(&self) -> u64 {
        self.max_filename_length
    }
}

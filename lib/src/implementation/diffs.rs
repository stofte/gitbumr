use git2::{Repository, Commit, Patch};
use interface::{DiffsTrait, DiffsEmitter, DiffsList};

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
    match commit.parent_id(0) {
        Ok(id) => {
            let t = commit.tree().unwrap();
            let parent_c = repo.find_commit(id).unwrap();
            let parent_tree = repo.find_tree(parent_c.tree_id()).unwrap();
            let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&t), None).unwrap();
            let delta_cnt = diff.deltas().len();
            let mut list = vec![];
            for idx in 0..delta_cnt {
                let mut patch = Patch::from_diff(&diff, idx).unwrap().unwrap();
                let patch_buf = patch.to_buf().unwrap();
                let mut patch_str = "";
                match patch_buf.as_str() {
                    Some(pstr) => patch_str = pstr,
                    None => ()
                };
                list.push(DiffsItem {
                    filename: "".to_string(),
                    status: "".to_string(),
                    patch: patch_str.to_string(),
                });
            }
            diffs.model.begin_reset_model();
            diffs.list = list;
            diffs.model.end_reset_model();
        },
        Err(..) => panic!("handle root node!")
    }
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

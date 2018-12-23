use git2::{Repository, Oid, Commit, ObjectType, DiffFormat, DiffDelta, DiffHunk, DiffLine, Patch};
use interface::{TreeModelTrait, TreeModelEmitter, TreeModelList};

#[derive(Default, Clone)]
pub struct TreeModelItem {
    pub filename: String,
    pub status: String,
    pub patch: String,
}

pub struct TreeModel {
    emit: TreeModelEmitter,
    model: TreeModelList,
    list: Vec<TreeModelItem>,
}


pub fn fill_treemodel(tree: &mut TreeModel, commit: &Commit, repo: &Repository) {
    match commit.parent_id(0) {
        Ok(id) => {
            fn diff_callback(dd: DiffDelta, dh: Option<DiffHunk>, dl: DiffLine) -> bool {
                true
            }
            let t = commit.tree().unwrap();
            let parent_c = repo.find_commit(id).unwrap();
            let parent_tree = repo.find_tree(parent_c.tree_id()).unwrap();
            let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&t), None).unwrap();
            let delta_cnt = diff.deltas().len();
            //diff.print(DiffFormat::Patch, diff_callback);
            let mut list = vec![];
            for idx in 0..delta_cnt {
                let delta = diff.get_delta(idx).unwrap();
                let mut patch = Patch::from_diff(&diff, idx).unwrap().unwrap();
                let patch_buf = patch.to_buf().unwrap();
                //println!("diff: {:?}, {:?}", d.status(), d.new_file().path());
                let mut patch_str = "";
                match patch_buf.as_str() {
                    Some(pstr) => patch_str = pstr,
                    None => ()
                };
                list.push(TreeModelItem {
                    filename: "".to_string(),
                    status: "".to_string(),
                    patch: patch_str.to_string(),
                });
            }
            tree.model.begin_reset_model();
            tree.list = list;
            tree.model.end_reset_model();
        },
        Err(..) => panic!("handle root node!")
    }
}

impl TreeModelTrait for TreeModel {
    fn new(emit: TreeModelEmitter, model: TreeModelList) -> TreeModel {
        TreeModel {
            emit,
            model,
            list: vec![]
        }
    }
    fn emit(&mut self) -> &mut TreeModelEmitter {
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
}

use interface::{
    BranchesTrait, BranchesEmitter, BranchesList
};

#[derive(Default, Clone)]
pub struct BranchesItem {
    pub name: String,
    pub checkedout: bool,
}

pub struct Branches {
    emit: BranchesEmitter,
    model: BranchesList,
    list: Vec<BranchesItem>,
}

impl BranchesTrait for Branches {
    fn new(emit: BranchesEmitter, model: BranchesList) -> Branches {
        Branches {
            emit,
            model,
            list: vec![],
        }
    }
    fn emit(&mut self) -> &mut BranchesEmitter {
        &mut self.emit
    }
    fn row_count(&self) -> usize {
        self.list.len()
    }
    fn name(&self, index: usize) -> &str {
        &self.list[index].name
    }
    fn checkedout(&self, index: usize) -> bool {
        self.list[index].checkedout
    }
}

pub fn fill_branches(b: &mut Branches, items: Vec<BranchesItem>) {
    if items.len() == 0 {
        return
    }
    b.model.begin_reset_model();
    b.list = items;
    b.model.end_reset_model();
}

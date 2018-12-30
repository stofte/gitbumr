use interface::{HunksTrait, HunksEmitter, HunksList};
use implementation::diffs::Diffs;

#[derive(Default, Clone)]
pub struct HunksItem {
    pub hunk: String,
    pub lines_origin: Vec<u8>,
    pub lines_old: Vec<u8>,
    pub lines_new: Vec<u8>,
}

pub struct Hunks {
    emit: HunksEmitter,
    model: HunksList,
    list: Vec<HunksItem>,
}

fn map_git_origin_sigil(c: char) -> u8 {
    match c {
        ' ' => 0,
        '+' => 1,
        '-' => 2,
        _   => panic!("unexpected sigil in map_git_origin_sigil: {}", c)
    }
}

fn map_from_diff_to_hunk_list(hunk: &str, lines_origin: &Vec<char>, lines_old: &Vec<Option<u32>>, lines_new: &Vec<Option<u32>>) -> HunksItem {
    HunksItem {
        hunk: hunk.to_string(),
        lines_origin: vec![],
        lines_old: vec![],
        lines_new: vec![],
    }
}

pub fn fill_hunks(hunks: &mut Hunks, diffs: &Diffs, index: u64, oid: &str) {
    // todo: probably possible via timing to load a hunk list from an unloaded commit?
    assert!(oid == diffs.commit_oid);
    assert!(index < diffs.list.len() as u64);
    let mut hv = vec![];
    let diff = &diffs.list[index as usize];
    for h_idx in 0..diff.hunks.len() {
        hv.push(map_from_diff_to_hunk_list(&diff.hunks[h_idx], &vec![], &vec![], &vec![]));
    }
    hunks.model.begin_reset_model();
    hunks.list = hv;
    hunks.model.end_reset_model();
}

impl HunksTrait for Hunks {
    fn new(emit: HunksEmitter, model: HunksList) -> Hunks {
        Hunks {
            emit,
            model,
            list: vec![]
        }
    }
    fn emit(&mut self) -> &mut HunksEmitter {
        &mut self.emit
    }
    fn row_count(&self) -> usize {
        self.list.len()
    }
    fn hunk(&self, index: usize) -> &str {
        &self.list[index].hunk
    }
    fn lines_origin(&self, index: usize) -> &[u8] {
        &self.list[index].lines_origin
    }
    fn lines_new(&self, index: usize) -> &[u8] {
        &self.list[index].lines_new
    }
    fn lines_old(&self, index: usize) -> &[u8] {
        &self.list[index].lines_old
    }
}

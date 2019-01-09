use interface::{HunksTrait, HunksEmitter, HunksList};
use implementation::diffs::Diffs;
use utils::MAX_U32_INT;

#[derive(Default, Clone)]
pub struct HunksItem {
    pub hunk: String,
    pub hunk_max_line_length: u64,
    pub lines: u64,
    pub lines_origin: Vec<u8>,
    pub lines_old: Vec<u8>,
    pub lines_old_cols: u64,
    pub lines_new: Vec<u8>,
    pub lines_new_cols: u64,
    pub lines_from: u64,
    pub lines_to: u64,
}

pub struct Hunks {
    emit: HunksEmitter,
    model: HunksList,
    list: Vec<HunksItem>,
}

fn map_git_origin_sigil(c: char) -> u8 {
    match c {
        ' ' => 0, // context line
        '+' => 1, // added line
        '-' => 2, // deleted line
        '<' => 3,
        '>' => 4,
        '=' => 5,
        _   => panic!("unexpected sigil in map_git_origin_sigil: {}", c)
    }
}

fn split_into_bytes(inp: Vec<u32>) -> Vec<u8> {
    let mut outp = vec![];
    for i in inp.iter() {
        outp.push(*i as u8);
        outp.push((*i >> 8) as u8);
        outp.push((*i >> 16) as u8);
        outp.push((*i >> 24) as u8);
    }
    outp
}

fn map_from_diff_to_hunk_list(hunk: &str, status: &str, hunk_max_line_length: usize, lines_origin: &Vec<char>, lines_old: &Vec<Option<u32>>, lines_new: &Vec<Option<u32>>) -> HunksItem {
    let l_origins_as_ints: Vec<u8> = lines_origin.iter().map(|c| map_git_origin_sigil(*c)).collect();
    let l_old_as_ints: Vec<u32> = lines_old.iter().map(|c| match c { Some(cv) => *cv, None => MAX_U32_INT }).collect();
    let l_new_as_ints: Vec<u32> = lines_new.iter().map(|c| match c { Some(cv) => *cv, None => MAX_U32_INT }).collect();
    // todo find something less retarded
    let mut l_old_max = 0 as u64;
    let mut l_new_max = 0 as u64;
    let mut l_from = 0 as u64;
    let mut l_to = 0 as u64;
    if lines_old.iter().any(|c| c.is_some()) {
        let lmax = lines_old.iter().max().unwrap().unwrap() as u64;
        l_old_max = format!("{}", lmax).len() as u64;
        if status == "Deleted" {
            l_from = lines_old.iter().filter(|c| c.is_some()).min().unwrap().unwrap() as u64;
            l_to = lmax;
        }
    }
    if lines_new.iter().any(|c| c.is_some()) {
        let lmax = lines_new.iter().max().unwrap().unwrap() as u64;
        l_new_max = format!("{}", lmax).len() as u64;
        if status != "Deleted" {
            l_from = lines_new.iter().filter(|c| c.is_some()).min().unwrap().unwrap() as u64;
            l_to = lmax;
        }
    }
    HunksItem {
        hunk: hunk.to_string(),
        lines: lines_origin.len() as u64,
        hunk_max_line_length: hunk_max_line_length as u64,
        lines_origin: l_origins_as_ints,
        lines_old: split_into_bytes(l_old_as_ints),
        lines_old_cols: l_old_max,
        lines_new: split_into_bytes(l_new_as_ints),
        lines_new_cols: l_new_max,
        lines_from: l_from,
        lines_to: l_to,
    }
}

pub fn fill_hunks(hunks: &mut Hunks, diffs: &Diffs, index: u64, oid: &str) {
    // todo: probably possible via timing to load a hunk list from an unloaded commit?
    assert!(oid == diffs.commit_oid);
    assert!(index < diffs.list.len() as u64);
    let mut hv = vec![];
    let diff = &diffs.list[index as usize];
    for h_idx in 0..diff.hunks.len() {
        hv.push(map_from_diff_to_hunk_list(
            &diff.hunks[h_idx],
            &diff.status,
            diff.hunks_max_line_length[h_idx],
            &diff.lines_origin[h_idx],
            &diff.lines_old[h_idx],
            &diff.lines_new[h_idx],
        ));
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
    fn lines(&self, index: usize) -> u64 {
        self.list[index].lines
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
    fn hunk_max_line_length(&self, index: usize) -> u64 {
        self.list[index].hunk_max_line_length as u64
    }
    fn lines_old_cols(&self, index: usize) -> u64 {
        self.list[index].lines_old_cols as u64
    }
    fn lines_new_cols(&self, index: usize) -> u64 {
        self.list[index].lines_new_cols as u64
    }
    fn lines_from(&self, index: usize) -> u64 {
        self.list[index].lines_from as u64
    }
    fn lines_to(&self, index: usize) -> u64 {
        self.list[index].lines_to as u64
    }
}

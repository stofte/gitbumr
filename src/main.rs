extern crate git2;
use git2::{Repository};

fn main() {
    let repo = match Repository::open("C:\\src\\CLEVER") {
        Ok(res) => res,
        Err(e) => panic!("failed to init: {}", e)
    };

    let biter = match repo.branches(Some(git2::BranchType::Remote)) {
        Ok(res) => res,
        Err(e) => panic!("failed to get branches: {}", e)
    };

    let mut idx = 0;
    for b in biter {
        let (branch, branch_type) = b.unwrap();
        println!("{:?}: {}", branch_type, branch.name().unwrap().unwrap());
        idx = &idx + 1;
    }
}

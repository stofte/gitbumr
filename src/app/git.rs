use git2::{Repository, BranchType};

pub struct Branch {
    pub name: String,
    pub checkedout: bool
}

pub fn local_branches(git_repo: &Repository) -> Vec<Branch> {
    let mut vec = Vec::new();
    let bs = git_repo.branches(Some(BranchType::Local)).unwrap();
    let head_name = get_head(&git_repo);
    for b in bs {
        let bb = b.unwrap().0;
        let name = bb.name().unwrap().unwrap().to_owned();
        let mut chkout = false;
        if head_name == name {
            chkout = true;
        }
        vec.push(Branch{ name: name, checkedout: chkout });
    }
    vec.sort_by(|a, b| a.name.cmp(&b.name));
    vec
}

pub fn get_head(git_repo: &Repository) -> String {
    let hr = git_repo.head().unwrap();
    let n = hr.name().unwrap();
    let prefix = "refs/head/";
    let n_str = &n[prefix.len() + 1 .. n.len()];
    return n_str.to_string()
}

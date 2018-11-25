use std::path::{PathBuf};
use git2::Repository;

pub fn pathbuf_filename_to_string(pb: &PathBuf) -> String {
    pb.file_name().unwrap().to_string_lossy().to_string()
}

pub fn pathbuf_to_string(pb: PathBuf) -> String {
    pb.into_os_string().to_string_lossy().to_string()
}

pub fn is_git_repo(path: &str) -> Result<(), &'static str> {
	match Repository::open(path) {
        Ok(..) => Ok(()),
        Err(..) => Err("Folder is not a git repository")
    }
}

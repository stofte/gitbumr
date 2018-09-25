use git2::Repository;

pub struct UiState<'a, 'b> {
    pub repository: &'a str,
    pub git_repo: &'b Repository,
    pub width: u16,
    pub height: u16
}

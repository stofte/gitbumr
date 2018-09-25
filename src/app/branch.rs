use termion::{style, cursor};
use app::state::UiState;
use app::git::local_branches;

pub fn view(state: &mut UiState) {
    println!("Branches {} ({:?})", state.repository, state.git_repo.state());
    println!("{}", cursor::Goto(1,1));
    for b in local_branches(&state.git_repo) {
        if b.checkedout == true {
            println!("{}{}{}{}{}", cursor::Save, style::Bold, b.name, style::Reset, cursor::Restore);
        } else {
            println!("{}{}{}", cursor::Save, b.name, cursor::Restore);
        }
    }
}

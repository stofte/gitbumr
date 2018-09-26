use termion::{style, cursor};
use app::state::UiState;
use app::git::local_branches;
use app::draw::header;

pub fn view(state: &mut UiState) {
    header(state.repository, &format!("{:?}", state.git_repo.state()), state.width, state.height);
    println!("{}", cursor::Goto(1, 1));
    for b in local_branches(&state.git_repo) {
        if b.checkedout == true {
            println!("{}{}{}{}{}", cursor::Save, style::Bold, b.name, style::Reset, cursor::Restore);
        } else {
            println!("{}{}{}", cursor::Save, b.name, cursor::Restore);
        }
    }
}

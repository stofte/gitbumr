use termion::event::Key;

pub enum EditorArg {
    Pass,
    Consumed(char),
    Completed,
}

pub fn handle_editor_input(vec: &mut Vec<char>, k: Key) -> EditorArg {
    match k {
        Key::Esc => EditorArg::Completed,
        Key::Char('\n') => EditorArg::Completed,
        Key::Char(c) => {
            vec.push(c);
            EditorArg::Consumed(c)
        },
        _ => EditorArg::Pass,
    }
}

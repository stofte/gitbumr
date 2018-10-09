use git2::Repository;
use app::settings::Settings;

#[derive(Debug)]
pub enum KeyArg {
    Pass,
    Consumed,
    OpenRepository(i64),
    InputEdit(u16, u16, u16, u16),
}

// #[derive(Debug)]
pub enum Event<'a> {
    None,
    Start(Option<&'a mut Settings>, Option<&'a mut Repository>, u16, u16),
    ConsoleResize(u16, u16),
    EditorInput(String),
    Repository(&'a mut Repository, &'a mut Settings),
    Settings(&'a mut Settings),
}

pub enum EventArg {
    None,
    Settings,
}

pub fn event_arg_to_string(ctx: &Event) -> String {
    match ctx {
        Event::None => "None".to_string(),
        Event::Start(..) => "Start".to_string(),
        Event::ConsoleResize(..) => "ConsoleResize".to_string(),
        Event::EditorInput(..) => "EditorInput".to_string(),
        Event::Repository(..) => "Repository".to_string(),
        Event::Settings(..) => "Settings".to_string(),
    }
}

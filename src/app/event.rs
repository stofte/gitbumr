use git2::Repository;
use app::settings::Settings;

#[derive(Debug)]
pub enum ConsumeArg {
    None,
    Repository,
}

#[derive(Debug)]
pub enum KeyArg {
    Pass,
    Consumed(ConsumeArg),
    OpenRepository(i64),
    InputEdit(u16, u16, u16, u16),
}

// #[derive(Debug)]
pub enum Event<'a> {
    Start(Option<&'a mut Settings>, Option<&'a mut Repository>, u16, u16),
    ConsoleResize(u16, u16),
    EditorInput(String),
    Repository(&'a mut Repository, &'a mut Settings),
}

pub enum EventArg {
    None,
}

pub fn event_arg_to_string(ctx: &Event) -> String {
    match ctx {
        Event::Start(..) => "Start".to_string(),
        Event::ConsoleResize(..) => "ConsoleResize".to_string(),
        Event::EditorInput(..) => "EditorInput".to_string(),
        Event::Repository(..) => "Repository".to_string(),
    }
}

pub mod control;
pub mod editor;
pub mod event;
pub mod settings;
pub mod git;
pub mod layout;
pub mod console;
pub mod logger;

use std::{
    io::{Write, Stdout},
};
use termion::{
    terminal_size,
    event::Key, 
    raw::{RawTerminal},
    screen::AlternateScreen,
};
use channel;
use git2::Repository;
use app::{
    event::{KeyArg, Event},
    settings::{Settings, build_settings},
    editor::{EditorArg, handle_editor_input},
    logger::{Logger, build_logger},
    control::{
        Control,
        branches::{build_branches},
        header::{build_header},
        repomanager::{build_repomanager},
        history::{build_history},
    },
};

pub struct App {
    controls: Vec<Box<Control>>,
    repo: Option<Repository>,
    settings: Settings,
    logger: Logger,
    input_buffer: Vec<char>,
    input_control: Option<u16>,
}

impl App {
    fn startup(&mut self) {
        self.logger.log(&format!("app.startup => repo counts: {}", self.settings.get_repositories().len()));
        self.repo = self.settings.get_open_repo();
        let (cols, rows) = terminal_size().unwrap();
        let mut settings = match self.repo {
            Some(ref mut r) => {
                Event::Start(Some(&mut self.settings), Some(r), cols, rows)
            },
            None => Event::Start(Some(&mut self.settings), None, cols, rows)
        };
        
        for i in 0..self.controls.len() {
            let ctrl = &mut self.controls[i];
            ctrl.ctx(&mut settings, &mut self.logger);
        }
    }
    fn context(&mut self, e: &mut Event) {

        for i in 0..self.controls.len() {
            let ctrl = &mut self.controls[i];
            ctrl.ctx(e, &mut self.logger);
        }
    }
    fn repo_changed(&mut self, id: i64) {
        self.settings.open_repository(id);
        self.repo = self.settings.get_open_repo();
        match self.repo {
            Some(ref mut r) => {
                let mut ctx = Event::Repository(r, &mut self.settings);
                for i in 0..self.controls.len() {
                    let ctrl = &mut self.controls[i];
                    ctrl.ctx(&mut ctx, &mut self.logger);
                }
            },
            None => {
                panic!("repo_changed failed to open repo {}", id);
            }
        }
    }
    fn input_completed(&mut self) {
        let ctrl_id = self.input_control.unwrap();
        let inp = self.input_buffer.clone();
        for i in 0..self.controls.len() {
            let ctrl = &mut self.controls[i];
            if ctrl.id() == ctrl_id {
                let mut ctx = Event::EditorInput(inp.into_iter().collect::<String>());
                ctrl.ctx(&mut ctx, &mut self.logger);
                break;
            }
        }
        self.input_control = None;
        self.input_buffer = vec![];
    }
    fn render(&mut self, stdout: &mut Stdout) {
        for i in (0..self.controls.len()).rev() {
            let ctrl = &mut self.controls[i];
            ctrl.render(stdout, &mut self.logger);
        }
    }
    fn key(&mut self, k: Key) {
        let mut res = KeyArg::Pass;
        for i in 0..self.controls.len() {
            let ctrl = &mut self.controls[i];
            res = ctrl.key(k, &mut self.logger);
            self.logger.log(&format!(" => {:?}", res));
            match res {
                KeyArg::Pass => continue,
                _ => break,
            };
        }
        match res {
            KeyArg::OpenRepository(id) => {
                self.repo_changed(id);
            },
            KeyArg::InputEdit(id, _, _, _) => {
                if self.input_control != None {
                    panic!("input_control was Some()");
                }
                self.input_control = Some(id);
            },
            _ => ()
        };
    }
    pub fn run(&mut self, mut stdout: AlternateScreen<RawTerminal<Stdout>>, keys_r: channel::Receiver<Key>, size_r: channel::Receiver<(u16, u16)>) {
        let mut idx = 0;
        console::reset();
        self.startup();
        loop {
            let input_edit = self.input_control != None;
            if !input_edit {
                self.logger.log(&format!("{}\t============================================================================", idx));
                self.render(&mut stdout);
                stdout.flush().unwrap();
            }
            select! {
                recv(keys_r, key) => {
                    let k = key.unwrap();
                    match k {
                        Key::Ctrl('c') => break,
                        _ => (),
                    };
                    if input_edit {
                        match handle_editor_input(&mut self.input_buffer, k) {
                            EditorArg::Consumed(c) => {
                                self.logger.log(&format!("EditorArg::Consumed => {}", c));
                            },
                            EditorArg::Completed => {
                                self.input_completed();
                            },
                            _ => ()
                        }
                    } else {
                        self.key(k);
                    }
                },
                recv(size_r, size) => {
                    console::reset();
                    let (cols,rows) = size.unwrap();
                    self.context(&mut Event::ConsoleResize(cols, rows));
                }
            }
            idx = idx + 1;
        }
    }
}

pub fn build_app() -> App {
    let mut app = App {
        controls: vec![],
        settings: build_settings(),
        repo: None,
        logger: build_logger(),
        input_buffer: vec![],
        input_control: None,
    };
    app.controls.push(Box::new(build_header(1)));
    app.controls.push(Box::new(build_repomanager(2)));
    app.controls.push(Box::new(build_branches(3)));
    app.controls.push(Box::new(build_history(4)));
    app.settings.init(); // ensures db file exists
    app
}

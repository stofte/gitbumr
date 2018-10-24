pub mod control;
pub mod editor;
pub mod event;
pub mod settings;
pub mod git;
pub mod layout;
pub mod console;
pub mod logger;
pub mod linebuffer;

use std::{
    io::{Write, Stdout, StdoutLock},
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
    event::{ConsumeArg, Event, EventArg},
    settings::{Settings, build_settings},
    editor::{EditorArg, handle_editor_input},
    logger::{Logger, build_logger},
    linebuffer::{LineBuffer, build_linebuffer},
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
    buffers: Vec<LineBuffer>,
    repo: Option<Repository>,
    settings: Settings,
    logger: Logger,
    focus_id: usize, // usize for use as index into vecs
    input_buffer: Vec<char>,
    input_control: Option<u32>,
}

impl App {
    fn startup(&mut self) {
        self.logger.log(format!("app.startup => repo counts: {}", self.settings.get_repositories().len()));
        self.repo = self.settings.get_open_repo();
        let (cols, rows) = terminal_size().unwrap();
        let mut ctx = match self.repo {
            Some(ref mut r) => {
                Event::Start(Some(&mut self.settings), Some(r), cols, rows)
            },
            None => Event::Start(Some(&mut self.settings), None, cols, rows)
        };
        for i in 0..self.controls.len() {
            let ctrl = &mut self.controls[i];
            let buff = &mut self.buffers[i];
            ctrl.event(&mut ctx, buff, &mut self.logger);
        }
    }
    fn context(&mut self, e: &mut Event) {
        for i in 0..self.controls.len() {
            let ctrl = &mut self.controls[i];
            let buff = &mut self.buffers[i];
            ctrl.event(e, buff, &mut self.logger);
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
                    let buff = &mut self.buffers[i];
                    ctrl.event(&mut ctx, buff, &mut self.logger);
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
            let buff = &mut self.buffers[i];
            if ctrl.id() == ctrl_id {
                let mut ctx = Event::EditorInput(inp.into_iter().collect::<String>());
                ctrl.event(&mut ctx, buff, &mut self.logger);
                break;
            }
        }
        self.input_control = None;
        self.input_buffer = vec![];
    }
    fn render(&mut self, stdout: &mut StdoutLock) {
        for i in (0..self.controls.len()).rev() {
            let ctrl = &mut self.controls[i];
            let buff = &mut self.buffers[i];
            let cid = ctrl.id();
            if !buff.valid {
                self.logger.log(format!("buffer {} was invalid", cid));
                ctrl.render(buff, &mut self.logger);
            }
            buff.render(stdout, &mut self.logger);
        }
    }
    fn input(&mut self, k: Key) {
        let mut res = EventArg::None;
        {
            let f_id = self.focus_id;
            let buff = &mut self.buffers[f_id];
            if !buff.focus {
                panic!("expected buffer to be focused");
            }
            let ctrl = &mut self.controls[f_id];
            res = ctrl.event(&mut Event::Input(k), buff, &mut self.logger);
            self.logger.log(format!(" => {:?}", res));
            // check for control only eventargs
            match res {
                // the control passed the input. no state change should happen in the ctrl.
                // the control consumed the key, but requires more repo access, eg history ctrl needs more data.
                EventArg::InputConsumed(ConsumeArg::Repository) => {
                    match self.repo {
                        Some(ref mut r) => {
                            let mut ctx = Event::Repository(r, &mut self.settings);
                            ctrl.event(&mut ctx, buff, &mut self.logger);
                        },
                        None => {
                            panic!("key => KeyArg::InputConsumed(Repository) => repo property was none");
                        }
                    }
                }
                EventArg::InputEdit(id, _, _, _) => {
                    if self.input_control != None {
                        panic!("input_control was Some()");
                    }
                    self.input_control = Some(id);
                },
                _ => (),
            };
        }
        // check global event args
        match res {
            EventArg::OpenRepository(id) => {
                self.repo_changed(id);
            },
            _ => ()
        };
    }
    pub fn run(&mut self, mut stdout: AlternateScreen<RawTerminal<StdoutLock>>, keys_r: channel::Receiver<Key>, size_r: channel::Receiver<(u16, u16)>) {
        let mut idx = 0;
        console::reset();
        self.startup();
        let focused_id = self.focus_id;
        self.context(&mut Event::Focus(focused_id as u32));
        loop {
            let input_edit = self.input_control != None;
            if !input_edit {
                self.logger.log(format!("{}\t========================================", idx));
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
                                self.logger.log(format!("EditorArg::Consumed => {}", c));
                            },
                            EditorArg::Completed => {
                                self.input_completed();
                            },
                            _ => ()
                        }
                    } else {
                        self.input(k);
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
        buffers: vec![],
        settings: build_settings(),
        repo: None,
        logger: build_logger(),
        input_buffer: vec![],
        input_control: None,
        focus_id: 3,
    };
    app.settings.init(); // ensures db file exists
    app.controls.push(Box::new(build_header(0)));
    app.controls.push(Box::new(build_repomanager(1)));
    app.controls.push(Box::new(build_branches(2)));
    app.controls.push(Box::new(build_history(3)));
    let mut b0 = build_linebuffer(0);
    b0.name = "header".to_string();
    app.buffers.push(b0);
    let mut b1 = build_linebuffer(1);
    b1.name = "repomgr".to_string();
    app.buffers.push(b1);
    let mut b2 = build_linebuffer(2);
    b2.name = "branches".to_string();
    app.buffers.push(b2);
    let mut b3 = build_linebuffer(3);
    b3.name = "history".to_string();
    app.buffers.push(b3);
    assert_eq!(app.buffers.len(), app.controls.len());
    for i in 0..app.controls.len() {
        assert_eq!(app.controls[i].id(), app.buffers[i].id);
    }
    app
}

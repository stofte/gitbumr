use std::io::{Stdout};
use termion::{
    cursor,
    event::{Key},
};
use app::{
    git, console,
    layout::{Layout, build_empty_layout},
    control::{Control},
    event::{KeyArg, Event, EventArg, event_arg_to_string},
    logger::Logger,
};

static APP_NAME: &'static str = "Gitbumr";

pub struct Header {
    id: u32,
    repo_path: String,
    state: String,
    layout: Layout,
}

impl Control for Header {
    fn id(&self) -> u32 { self.id }
    fn render(&mut self, _stdout: &mut Stdout, log: &mut Logger) {
        log.log(format!("header.render (w: {})", self.layout.width));
        let blank_cnt = self.layout.width as usize - self.repo_path.len() - APP_NAME.len() - self.state.len();
        print!("{move}{b_fg}{b_bg}{name}{fg}{bg}{path}{blank}{state}{fg_r}{bg_r}",
            move=cursor::Goto(1, 1),
            name=APP_NAME,
            path=self.repo_path,
            blank=" ".repeat(blank_cnt),
            state=self.state,
            b_fg=console::FG_BRAND,
            b_bg=console::BG_BRAND,
            fg=console::FG_PRIMARY,
            bg=console::BG_PRIMARY,
            bg_r=console::BG_RESET,
            fg_r=console::FG_RESET,
        );
    }
    fn key(&mut self, _k: Key, log: &mut Logger) -> KeyArg {
        log.log(format!("header.key"));
        KeyArg::Pass
    }
    fn ctx(&mut self, e: &mut Event, log: &mut Logger) -> EventArg {
        log.log(format!("header.ctx {:?}", event_arg_to_string(e)));
        match e {
            Event::Start(_, r, c, _) => {
                self.layout.width = *c;
                self.layout.height = 1;
                match r {
                    Some(repo) => {
                        self.repo_path = git::get_repository_path(&repo);
                        self.state = format!("{:?}", repo.state());
                        log.log(format!("\"{}\" repo was passed to start", self.repo_path));
                    },
                    None => {
                        log.log(format!("no repo was passed to start"));
                    }
                }
            }
            Event::Repository(ref r, _) => {
                self.repo_path = git::get_repository_path(r);
                self.state = format!("{:?}", r.state());
            }
            Event::ConsoleResize(c, _) => {
                self.layout.width = *c;
            }
            _ => ()
        };
        EventArg::None
    }
}

pub fn build_header(id: u32) -> Header {
    Header {
        id: id,
        repo_path: "".to_string(),
        state: "".to_string(),
        layout: build_empty_layout(),
    }
}

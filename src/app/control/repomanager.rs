use std::{
    cmp,
    io::{Stdout},
};
use termion::{
    cursor,
    event::Key,
};
use app::{
    console,
    settings::StoredRepository,
    layout::{Layout, build_empty_layout},
    event::{Event, ConsumeArg, KeyArg, EventArg},
    control::Control,
    logger::Logger,
};

pub struct RepoManager {
    pub id: u32,
    pub repos: Vec<StoredRepository>,
    pub layout: Layout,
    pub adding: bool,
    pub repo_path: Option<String>,
    pub input_txt: Vec<char>,
    pub input_cursor: u16,
    pub repo_cursor: u16,
    pub add_err: Option<String>,
    pub open_repo: Option<i64>,
}

impl Control for RepoManager {
    fn id(&self) -> u32 { self.id }
    fn render(&mut self, _stdout: &mut Stdout, log: &mut Logger) {
        if !self.layout.visible { return }
        log.log(format!("repomgr.render"));
        let title = "Repositories".to_string();
        let title_b_h = console::BOX_H.to_string()
            .repeat(self.layout.width as usize - title.len() - 5);
        console::start_drawing(self.layout.left, self.layout.top, console::FG_PRIMARY, console::BG_PRIMARY);
        print!("{b_dr}{b_h}{b_vl}{title}{b_vr}{repeat}{b_dl}",
            title=title,
            repeat=title_b_h,
            b_dr=console::BOX_DR,
            b_h=console::BOX_H,
            b_vl=console::BOX_VL,
            b_vr=console::BOX_VR,
            b_dl=console::BOX_DL,
        );
        print_blank(&self.layout, 1);
        let mut bottom_off = 2;
        if self.adding {
            let mut txt = "  Add repository".to_string();
            match &self.add_err {
                Some(err) => {
                    txt = format!("  Error: {}", err).to_string();
                },
                _ => ()
            };
            console::move_cursor(self.layout.left, self.layout.top + bottom_off);
            print!("{b_v}{txt}{blank}{b_v}",
                txt=txt,
                blank=" ".repeat(self.layout.width as usize - txt.len() - 2),
                b_v=console::BOX_V,
            );
            bottom_off = bottom_off + 1;
            print_blank(&self.layout, bottom_off);
            bottom_off = bottom_off + 1;
            let label_txt = "  Path: ".to_string();
            console::move_cursor(self.layout.left, self.layout.top + bottom_off);
            // we draw input txt at the bottom
            print!("{b_v}{lbl}{c_fg}{c_bg}{blank}{fg}{bg}  {b_v}",
                lbl=label_txt,
                blank=" ".repeat(self.layout.width as usize - label_txt.len() - 4),
                b_v=console::BOX_V,
                c_bg=console::BG_PRIMARY_CURSOR,
                c_fg=console::FG_PRIMARY_CURSOR,
                fg=console::FG_PRIMARY,
                bg=console::BG_PRIMARY,
            );
            bottom_off = bottom_off + 1;
            print_blank(&self.layout, bottom_off);
            bottom_off = bottom_off + 1;
            print_blank(&self.layout, bottom_off);
            bottom_off = bottom_off + 1;
        }
        if self.repos.len() == 0 {
            let txt = "  No repositories found".to_string();
            console::move_cursor(self.layout.left, self.layout.top + bottom_off);
            print!("{b_v}{txt}{blank}{b_v}",
                txt=txt,
                blank=" ".repeat(self.layout.width as usize - txt.len() - 2),
                b_v=console::BOX_V,
            );
            bottom_off = bottom_off + 1;
        }
        for i in 0..self.repos.len() {
            let repo = &self.repos[i];
            let txt = &repo.path;
            let mut open_mark = ' ';
            if repo.open {
                open_mark = console::PNT_R;
            }
            let mut c_fg = console::FG_PRIMARY;
            let mut c_bg = console::BG_PRIMARY;
            if i as u16 == self.repo_cursor && !self.adding {
                c_bg = console::BG_PRIMARY_CURSOR;
                c_fg = console::FG_PRIMARY_CURSOR;
            }
            console::move_cursor(self.layout.left, self.layout.top + bottom_off);
            print!("{b_v}  {open_m}{c_fg}{c_bg}{txt}{blank}{fg}{bg}  {b_v}",
                txt=txt,
                open_m=open_mark,
                blank=" ".repeat(self.layout.width as usize - txt.len() - 7),
                fg=console::FG_PRIMARY,
                bg=console::BG_PRIMARY,
                c_fg=c_fg,
                c_bg=c_bg,
                b_v=console::BOX_V,
            );
            bottom_off = bottom_off + 1;
        }
        print_blank(&self.layout, bottom_off);
        bottom_off = bottom_off + 1;
        let mut help_txt = " a: Add repository, esc: Close ".to_string();
        if self.adding {
            help_txt = " esc: Cancel ".to_string();
        }
        let bottom_b_h = console::BOX_H.to_string().repeat(self.layout.width as usize - 3 - help_txt.len());
        console::move_cursor(self.layout.left, self.layout.top + bottom_off);
        print!("{b_ur}{repeat}{help}{b_h}{b_ul}",
            repeat=bottom_b_h,
            help=help_txt,
            b_ur=console::BOX_UR,
            b_ul=console::BOX_UL,
            b_h=console::BOX_H,
        );
        if self.adding {
            let inp_txt: String = self.input_txt.clone().into_iter().collect();
            console::move_cursor(2, 2);
            print!("{c_fg}{c_bg}{inp}{show}",
                inp=inp_txt,
                show=cursor::Show,
                c_bg=console::BG_PRIMARY_CURSOR,
                c_fg=console::FG_PRIMARY_CURSOR,
            );
        }
        console::stop_drawing();
    }
    fn key(&mut self, k: Key, log: &mut Logger) -> KeyArg {
        log.log(format!("repomgr.key"));
        match k {
            Key::Char(c) => {
                if c == 'r' {
                    self.layout.visible = !self.layout.visible;
                    log.log(format!("repomgr visibility toggled to {}", self.layout.visible));
                    return KeyArg::Consumed(ConsumeArg::None)
                } else if c == 'a' && self.layout.visible && !self.adding {
                    self.adding = true;
                    return KeyArg::InputEdit(self.id, 1, 1, 1)
                } else if c == '\n' {
                    if self.layout.visible && !self.adding && self.repos.len() > 0 {
                        self.open_repo = Some(self.repos[self.repo_cursor as usize].id);
                        self.layout.visible = false;
                        let id = self.open_repo.unwrap();
                        log.log(format!("repomgr opening repo toggled to {}", &id));
                        return KeyArg::OpenRepository(id)
                    }
                }
                KeyArg::Pass
            }
            Key::Esc => {
                if self.adding {
                    self.adding = false;
                    return KeyArg::Consumed(ConsumeArg::None)
                } else if self.layout.visible {
                    self.layout.visible = false;
                    return KeyArg::Consumed(ConsumeArg::None)
                }
                KeyArg::Pass
            }
            Key::Up => {
                if self.layout.visible {
                    if !self.adding && self.repos.len() > 0 && self.repo_cursor > 0 {
                        self.repo_cursor -= 1;
                    }
                    return KeyArg::Consumed(ConsumeArg::None)
                }
                KeyArg::Pass
            }
            Key::Down => {
                if self.layout.visible {
                    if !self.adding && self.repos.len() > 0 {
                        self.repo_cursor = cmp::min(self.repos.len() as u16 - 1, self.repo_cursor + 1);
                    }
                    return KeyArg::Consumed(ConsumeArg::None)
                }
                KeyArg::Pass
            }
            _ => KeyArg::Pass
        }
        
    }
    fn ctx(&mut self, e: &mut Event, log: &mut Logger) -> EventArg {
        log.log(format!("repomgr.ctx"));
        match e {
            Event::Start(s, _, cols, rows) => {
                self.layout.top = 3;
                self.layout.left = 5;
                self.layout.width = *cols - 2 * (self.layout.left - 1);
                self.layout.height = *rows - 2 * (self.layout.top - 1);
                match s {
                    Some(settings) => {
                        self.repos = settings.get_repositories();
                    },
                    _ => ()
                };
            }
            Event::ConsoleResize(cols, rows) => {
                self.layout.width = *cols - 2 * (self.layout.left - 1);
                self.layout.height = *rows - 2 * (self.layout.top - 1);
            }
            Event::EditorInput(ref s) => {
                log.log(format!("repomgr.ctx input => {}", s));
                self.repo_path = Some(s.to_string());
            }
            Event::Repository(_, ref s) => {
                self.repos = s.get_repositories();
            }
            _ => ()
        }
        EventArg::None
    }
}

fn print_blank(l: &Layout, top: u16) {
    print!("{move}{b_v}{blank}{b_v}", 
        move=cursor::Goto(l.left, l.top + top),
        blank=" ".repeat(l.width as usize - 2),
        b_v=console::BOX_V,
    );
}

pub fn build_repomanager(id: u32) -> RepoManager {
    RepoManager {
        id: id,
        repos: vec![],
        layout: build_empty_layout(),
        repo_path: None,
        input_txt: vec![],
        input_cursor: 0,
        repo_cursor: 0,
        add_err: None,
        open_repo: None,
        adding: false,
    }
}

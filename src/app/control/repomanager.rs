use std::{
    cmp,
    io::{Write, Stdout},
    any::Any,
};
use termion::{
    cursor,
    style,
    event::Key
};
use app::{
    console, empty_layout,
    Layout, LayoutUpdate, UiFlags,
    settings::{Settings, StoredRepository},
    control::{Control, SettingsControl, InputControl},
};

pub struct RepoManager {
    pub repos: Vec<StoredRepository>,
    pub layout: Layout,
    pub adding: bool,
    pub pending_add: bool,
    pub input_txt: Vec<char>,
    pub input_cursor: u16,
    pub repo_cursor: u16,
    pub add_err: Option<String>,
    pub open_repo: Option<i64>,
}

fn print_blank(l: &Layout, top: u16) {
    print!("{move}{b_v}{blank}{b_v}", 
        move=cursor::Goto(l.left, l.top + top),
        blank=" ".repeat(l.width as usize - 2),
        b_v=console::BOX_V,
    );
}

impl Control for RepoManager {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn layout(&mut self, layout: &LayoutUpdate) {
        self.layout.top = 3;
        self.layout.left = 5;
        self.layout.console_cols = layout.cols.unwrap();
        self.layout.console_rows = layout.rows.unwrap();
        self.layout.width = self.layout.console_cols - 2 * (self.layout.left - 1);
        self.layout.height = self.layout.console_rows - 2 * (self.layout.top - 1);
    }
    fn render(&self, stdout: &mut Stdout) {
        if !self.layout.visible { return }
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
            let add_txt = "  Path: ".to_string();
            console::move_cursor(self.layout.left, self.layout.top + bottom_off);
            print!("{b_v}{txt}{s_ul}{blank}{s_nul}  {b_v}",
                txt=add_txt,
                blank=" ".repeat(self.layout.width as usize - add_txt.len() - 4),
                b_v=console::BOX_V,
                s_ul=style::Underline,
                s_nul=style::NoUnderline,
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
            let mut opentxt = "";
            let mut open_mark = ' ';
            if repo.open {
                opentxt = " [open]";
                open_mark = console::PNT_R;
            }
            let mut c_fg = console::FG_PRIMARY;
            let mut c_bg = console::BG_PRIMARY;
            if i as u16 == self.repo_cursor {
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
            console::move_cursor(self.layout.left + 9, self.layout.top + 4);
            print!("{s_ul}{inp}{s_nul}{show}",
                inp=inp_txt,
                show=cursor::Show,
                s_ul=style::Underline,
                s_nul=style::NoUnderline,
            );
        }
        console::stop_drawing();
    }
}

impl SettingsControl for RepoManager {
    fn update(&mut self, settings: &mut Settings) -> UiFlags {
        let mut res = UiFlags::None;
        if self.pending_add {
            self.pending_add = false;
            let path: String = self.input_txt.clone().into_iter().collect();
            match settings.add_repository(&path) {
                Ok(()) => {
                    self.input_txt.clear();
                    self.adding = false;
                    self.add_err = None;
                    res = UiFlags::HideCursor | UiFlags::AddedRepository;
                },
                Err(err) => {
                    self.add_err = Some(err.to_string());
                }
            };
        }
        match self.open_repo {
            Some(id) => {
                settings.open_repository(id);
                res = UiFlags::OpenRepository;
            }
            _ => ()
        }
        self.repos = settings.get_repositories();
        res
    }
}

impl InputControl for RepoManager {
    fn handle(&mut self, key: Key) -> (bool, UiFlags) {
        let handled = (true, UiFlags::None);
        let handled_cursor = (true, UiFlags::HideCursor);
        let handled_repo = (true, UiFlags::OpenRepository);
        let pass = (false, UiFlags::None);
        match key {
            Key::Char(c) => {
                if c == 'r' && !self.layout.visible {
                    self.layout.visible = true;
                    return handled
                } else if c == 'a' && self.layout.visible && !self.adding {
                    self.adding = true;
                    return handled
                } else if c == '\n' {
                    if self.adding {
                        self.pending_add = self.input_txt.len() > 0;
                        return handled
                    } else if self.layout.visible && !self.adding && self.repos.len() > 0 {
                        self.open_repo = Some(self.repos[self.repo_cursor as usize].id);
                        self.layout.visible = false;
                        return handled_repo                        
                    }
                    return pass
                } else if c == '\t' && self.adding {
                    return pass
                } else if self.adding {
                    self.input_txt.push(c);
                    return handled
                }
                pass
            }
            Key::Backspace => {
                if self.adding && self.input_txt.len() > 0 {
                    self.input_txt.pop();
                    return handled
                }
                pass
            }
            Key::Esc => {
                if self.adding {
                    self.adding = false;
                    return handled_cursor
                } else if self.layout.visible {
                    self.layout.visible = false;
                    return handled
                }
                pass
            }
            Key::Up => {
                if self.layout.visible && !self.adding && self.repos.len() > 0 && self.repo_cursor > 0 {
                    self.repo_cursor -= 1;
                    return handled
                }
                pass
            }
            Key::Down => {
                if self.layout.visible && !self.adding && self.repos.len() > 0 {
                    self.repo_cursor = cmp::min(self.repos.len() as u16 - 1, self.repo_cursor + 1);
                    return handled
                }
                pass
            }
            _ => pass
        }
    }
}

pub fn build_repomanager() -> RepoManager {
    RepoManager {
        repos: vec![],
        layout: empty_layout(),
        adding: false,
        pending_add: false,
        input_txt: vec![],
        input_cursor: 0,
        repo_cursor: 0,
        add_err: None,
        open_repo: None
    }
}

use std::{
    io::{Write, Stdout},
    any::Any,
};
use termion::{
    cursor,
    style,
    event::Key
};
use app::{
    console,
    Layout, LayoutUpdate,
    db::{StoredRepository, Database},
    control::{Control, DatabaseControl, InputControl, UiOption},
};

pub struct RepoManager {
    pub repos: Vec<StoredRepository>,
    pub layout: Layout,
    pub adding: bool,
    pub input_txt: Vec<char>,
}

impl RepoManager {
    fn print_blank(&self, top: u16) {
        print!("{move}{fg}{bg}{b_v}{blank}{b_v}{fg_r}{bg_r}", 
            move=cursor::Goto(self.layout.left, self.layout.top + top),
            blank=" ".repeat(self.layout.width as usize - 2),
            fg=console::FG_PRIMARY,
            bg=console::BG_PRIMARY,
            bg_r=console::BG_RESET,
            fg_r=console::FG_RESET,
            b_v=console::BOX_V,
        );
    }
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
    fn render(&self, stdout: &mut Stdout) -> bool {
        if !self.layout.visible { return false }
        let title = "Repositories".to_string();
        let title_b_h = console::BOX_H.to_string()
            .repeat(self.layout.width as usize - title.len() - 5);
        print!("{move}{fg}{bg}{b_dr}{b_h}{b_vl}{title}{b_vr}{repeat}{b_dl}{fg_r}{bg_r}",
            move=cursor::Goto(self.layout.left, self.layout.top),
            title=title,
            repeat=title_b_h,
            fg=console::FG_PRIMARY,
            bg=console::BG_PRIMARY,
            bg_r=console::BG_RESET,
            fg_r=console::FG_RESET,
            b_dr=console::BOX_DR,
            b_h=console::BOX_H,
            b_vl=console::BOX_VL,
            b_vr=console::BOX_VR,
            b_dl=console::BOX_DL,
        );
        self.print_blank(1);
        let mut bottom_off = 2;
        if self.repos.len() == 0 {
            let mut txt = "  No repositories found".to_string();
            if self.adding {
                txt = "  Add existing repository".to_string();
            }
            print!("{move}{fg}{bg}{b_v}{txt}{blank}{b_v}{fg_r}{bg_r}",
                move=cursor::Goto(self.layout.left, self.layout.top + bottom_off),
                txt=txt,
                blank=" ".repeat(self.layout.width as usize - txt.len() - 2),
                fg=console::FG_PRIMARY,
                bg=console::BG_PRIMARY,
                bg_r=console::BG_RESET,
                fg_r=console::FG_RESET,
                b_v=console::BOX_V,
            );
            bottom_off = bottom_off + 1;
        }
        if self.adding {
            self.print_blank(bottom_off);
            bottom_off = bottom_off + 1;
            let add_txt = "  Path: ".to_string();
            print!("{move}{fg}{bg}{b_v}{txt}{s_ul}{blank}{s_nul}{fg}{bg}  {b_v}{fg_r}{bg_r}",
                move=cursor::Goto(self.layout.left, self.layout.top + bottom_off),
                txt=add_txt,
                blank=" ".repeat(self.layout.width as usize - add_txt.len() - 4),
                fg=console::FG_PRIMARY,
                bg=console::BG_PRIMARY,
                bg_r=console::BG_RESET,
                fg_r=console::FG_RESET,
                b_v=console::BOX_V,
                s_ul=style::Underline,
                s_nul=style::NoUnderline,
            );
            bottom_off = bottom_off + 1;
        }
        self.print_blank(bottom_off);
        bottom_off = bottom_off + 1;
        let help_txt = " Ctrl+A: Add repository ".to_string();
        let bottom_b_h = console::BOX_H.to_string().repeat(self.layout.width as usize - 3 - help_txt.len());
        print!("{move}{fg}{bg}{b_ur}{repeat}{help}{b_h}{b_ul}{fg_r}{bg_r}",
            move=cursor::Goto(self.layout.left, self.layout.top + bottom_off),
            repeat=bottom_b_h,
            help=help_txt,
            fg=console::FG_PRIMARY,
            bg=console::BG_PRIMARY,
            bg_r=console::BG_RESET,
            fg_r=console::FG_RESET,
            b_ur=console::BOX_UR,
            b_ul=console::BOX_UL,
            b_h=console::BOX_H,
        );
        if self.adding {
            let inp_txt: String = self.input_txt.clone().into_iter().collect();
            print!("{move}{fg}{bg}{s_ul}{inp}{s_nul}{show}{fg_r}{bg_r}",
                move=cursor::Goto(self.layout.left + 9, self.layout.top + 4),
                inp=inp_txt,
                show=cursor::Show,
                fg=console::FG_PRIMARY,
                bg=console::BG_PRIMARY,
                bg_r=console::BG_RESET,
                fg_r=console::FG_RESET,
                s_ul=style::Underline,
                s_nul=style::NoUnderline,
            );
            return true
        }
        false
    }
}

impl DatabaseControl for RepoManager {
    fn update(&mut self, db: &Database) {

    }
}

impl InputControl for RepoManager {
    fn handle(&mut self, key: Key) -> (bool, UiOption) {
        let handled = (true, UiOption::None);
        let pass = (false, UiOption::None);
        match key {
            Key::Ctrl('r') => {
                if !self.layout.visible {
                    self.layout.visible = true;
                    return handled
                }
                pass
            },
            Key::Ctrl('a') => {
                if !self.adding {
                    self.adding = true;
                    return handled
                }
                pass
            },
            Key::Char('\t') => pass,
            Key::Char('\n') => {
                if self.adding {
                    self.adding = false;
                    self.input_txt.clear();
                    return (true, UiOption::HideCursor)
                }
                pass
            },
            Key::Char(c) => {
                if self.adding {
                    self.input_txt.push(c);
                    return handled
                }
                pass
            },
            Key::Backspace => {
                if self.adding && self.input_txt.len() > 0 {
                    self.input_txt.pop();
                    return handled
                }
                pass
            },
            Key::Esc => {
                if self.layout.visible {
                    self.layout.visible = false;
                    return handled
                }
                pass
            }
            _ => pass
        }
    }
}

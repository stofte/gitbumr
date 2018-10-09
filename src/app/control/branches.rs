use std::io::{Stdout};
use termion::event;
use app::{
    console, git,
    layout::{Layout, build_empty_layout},
    control::{Control},
    event::{KeyArg, Event, EventArg},
    logger::Logger,
};

pub struct Branches {
    id: u16,
    local: Vec<git::Branch>,
    remote: Vec<String>,
    checkedout_idx: Option<u16>,
    layout: Layout,
}

impl Control for Branches {
    fn id(&self) -> u16 { self.id }
    fn render(&mut self, stdout: &mut Stdout, log: &mut Logger) {
        log.log(&format!("branches.render"));
        // console::start_drawing(self.layout.left, self.layout.top, console::FG_PRIMARY, console::BG_PRIMARY);
        console::start_drawing(self.layout.left, self.layout.top, console::FG_PRIMARY, console::BG_PRIMARY);
        let title = "Branches".to_string();
        let title_b_h = console::BOX_H.to_string()
            .repeat(self.layout.width as usize - title.len() - 2);
        print!("{b_h}{title}{repeat}{b_dl}",
            title=title,
            repeat=title_b_h,
            b_h=console::BOX_H,
            b_dl=console::BOX_DH,
        );
        let mut cidx = -1;
        match self.checkedout_idx {
            Some(i) => {
                cidx = i as isize;
            },
            _ => ()
        };
        let mut c_off = 1;
        for j in 0..self.local.len() {
            console::move_cursor(self.layout.left, self.layout.top + c_off);
            let s = &self.local[j];
            let mut open_mark = ' ';
            let mut trunc_mark = "".to_string();
            if cidx == j as isize {
                open_mark = console::PNT_R;
            }
            let mut trunc_c = 0;
            if s.name.len() > self.layout.width as usize - 2 {
                trunc_c = s.name.len() - (self.layout.width as usize - 3);
                trunc_mark = format!("{}", console::ELLIP_H).to_string();
            }
            let trunc_name: String = s.name.chars().skip(trunc_c).collect();
            print!("{c_m}{t_m}{name}{blank}{b_v}",
                c_m=open_mark,
                name=trunc_name,
                t_m=trunc_mark,
                blank=" ".repeat(self.layout.width as usize - trunc_name.len() - 2 - (if trunc_c > 0 { 1 } else { 0 })),
                b_v=console::BOX_V,
            );
            c_off += 1;
        }
        let spacing_rows = self.layout.height as usize - self.local.len();
        for i in 0..spacing_rows {
            console::move_cursor(self.layout.left, self.layout.top + c_off);
            print!("{blank}{b_v}",
                b_v=console::BOX_V,
                blank=" ".repeat(self.layout.width as usize - 1),
            );
            c_off += 1;
        }
        console::stop_drawing();
    }
    fn key(&mut self, k: event::Key, log: &mut Logger) -> KeyArg {
        log.log(&format!("branches.key"));
        KeyArg::Pass
    }
    fn ctx(&mut self, e: &mut Event, log: &mut Logger) -> EventArg {
        log.log(&format!("branches.ctx"));
        match e {
            Event::Start(_, r, _, rows) => {
                self.layout.top = 2;
                self.layout.left = 1;
                self.layout.width = 35;
                self.layout.height = *rows - self.layout.top;
                match r {
                    Some(ref r) => self.local = git::local_branches(r),
                    _ => ()
                };
            },
            Event::ConsoleResize(_, rows) => {
                self.layout.height = *rows - self.layout.top;
            }
            Event::Repository(ref r, _) => self.local = git::local_branches(r),
            _ => ()
        };
        EventArg::None
    }
}

pub fn build_branches(id: u16) -> Branches {
    let mut b = Branches {
        id: id,
        local: vec![],
        remote: vec![],
        checkedout_idx: None,
        layout: build_empty_layout()
    };
    b
}

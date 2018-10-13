use std::io::{Stdout};
use termion::event;
use app::{
    console, git,
    layout::{Layout, build_empty_layout},
    control::{Control},
    event::{KeyArg, Event, EventArg},
    logger::Logger,
    linebuffer::{LineBuffer, build_linebuffer}
};

pub struct Branches {
    id: u32,
    local: Vec<git::Branch>,
    checkedout_idx: Option<u16>,
}

impl Control for Branches {
    fn id(&self) -> u32 { self.id }
    fn render(&mut self, buffer: &mut LineBuffer, log: &mut Logger) {
        assert_eq!(buffer.id, self.id);
        log.log(format!("branches.render"));
        for j in 0..self.local.len() {
            let s = &self.local[j];
            let mut open_mark = ' ';
            let mut trunc_mark = "".to_string();
            if s.checkedout {
                open_mark = console::PNT_R;
            }
            let b_width = buffer.width as usize;
            let mut trunc_c = 0;
            if s.name.len() > b_width - 2 {
                trunc_c = s.name.len() - (b_width - 3);
                trunc_mark = format!("{}", console::ELLIP_H).to_string();
            }
            let trunc_name: String = s.name.chars().skip(trunc_c).collect();
            let line_str = format!("{c_m}{t_m}{name}{blank}", 
                c_m=open_mark,
                name=trunc_name,
                t_m=trunc_mark,
                blank=" ".repeat(b_width - trunc_name.len() - 2 - (if trunc_c > 0 { 1 } else { 0 })),
            );
            buffer.set(line_str);
        }
        buffer.valid = true;
    }
    fn key(&mut self, _k: event::Key, log: &mut Logger) -> KeyArg {
        log.log(format!("branches.key"));
        KeyArg::Pass
    }
    fn ctx(&mut self, e: &mut Event, buffer: &mut LineBuffer, log: &mut Logger) -> EventArg {
        assert_eq!(buffer.id, self.id);
        log.log(format!("branches.ctx"));
        match e {
            Event::Start(_, r, _, rows) => {
                buffer.top = 1;
                buffer.left = 0;
                let b_top = buffer.top;
                buffer.size(35, *rows - b_top);
                match r {
                    Some(ref r) => self.local = git::local_branches(r),
                    _ => ()
                };
            },
            Event::ConsoleResize(_, rows) => {
                buffer.height = *rows - buffer.top;
            }
            Event::Repository(ref r, _) => self.local = git::local_branches(r),
            _ => ()
        };
        EventArg::None
    }
}

pub fn build_branches(id: u32) -> Branches {
    let mut x = Branches {
        id: id,
        local: vec![],
        checkedout_idx: None,
    };
    x
}

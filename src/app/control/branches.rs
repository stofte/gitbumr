use termion::{style, cursor};
use std::{
    cmp,
    io::Stdout,
    any::Any,
};
use git2::{Repository, BranchType};
use app::{
    console,
    Layout,
    LayoutUpdate,
    empty_layout,
    git::{get_head},
    control::{Control, RepositoryControl},
};

pub struct Branches {
    pub local: Vec<String>,
    pub remote: Vec<String>,
    pub checkedout_idx: Option<u16>,
    pub layout: Layout,
}

impl Control for Branches {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn layout(&mut self, layout: &LayoutUpdate) {
        self.layout.top = 2;
        self.layout.left = 1;
        self.layout.width = 35;
        self.layout.height = layout.rows.unwrap() - self.layout.top;
    }
    fn render(&self, stdout: &mut Stdout) {
        if !self.layout.visible { return }
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
            if s.len() > self.layout.width as usize - 2 {
                trunc_c = s.len() - (self.layout.width as usize - 3);
                trunc_mark = format!("{}", console::ELLIP_H).to_string();
            }
            let trunc_name: String = s.chars().skip(trunc_c).collect();
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
}

impl RepositoryControl for Branches {
    fn update(&mut self, repo: &Repository) {
        let mut vec = Vec::new();
        let bs = repo.branches(Some(BranchType::Local)).unwrap();
        for b in bs {
            let bb = b.unwrap().0;
            let name = bb.name().unwrap().unwrap().to_owned();
            vec.push(name);
        }
        vec.sort();
        let head_name = get_head(&repo);
        for i in 0..vec.len() {
            if head_name == vec[i] {
                self.checkedout_idx = Some(i as u16);
            }
        }
        self.local = vec;
        self.layout.visible = true;
    }
    fn none(&mut self) {
        self.layout.visible = false;
    }
    fn read(&mut self, repo: &Repository) { }
}

pub fn build_branches() -> Branches {
    let mut b = Branches {
        local: vec![],
        remote: vec![],
        checkedout_idx: None,
        layout: empty_layout()
    };
    b.layout.visible = true;
    b
}

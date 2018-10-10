pub struct Cursor {
    line: u16,
    start_offset: u16,
    start_take: u16,
}

pub struct LineBuffer {
    lines: Vec<String>,
    cursor: Cursor,
}

impl LineBuffer {
    fn size(&mut self, width: u16, height: u16) {
        
    }
}

pub fn build_linebuffer() -> LineBuffer {
    LineBuffer {
        lines: vec![],
        cursor: Cursor {
            line: 0,
            start_offset: 0,
            start_take: 0,
        }
    }
}

pub struct Layout {
    pub top: u16,
    pub left: u16,
    pub width: u16,
    pub height: u16,
    pub visible: bool,
    pub focus: bool,
    pub console_rows: u16,
    pub console_cols: u16,
}

pub fn build_empty_layout() -> Layout {
    Layout {
        top: 0,
        left: 0,
        width: 0,
        height: 0,
        focus: false,
        console_rows: 0,
        console_cols: 0,
        visible: false
    }
}

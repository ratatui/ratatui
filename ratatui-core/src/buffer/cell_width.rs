// use unicode_width::UnicodeWidthStr;

pub trait StrCellWidth {
    fn cell_width(&self) -> u16;
}

impl StrCellWidth for str {
    fn cell_width(&self) -> u16 {
        1
        // if self.len() == 1 {
        //     1
        // } else {
        //     self.width() as u16
        // }
    }
}
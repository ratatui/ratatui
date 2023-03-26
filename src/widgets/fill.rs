use crate::{
    buffer::{Buffer, Cell},
    layout::Rect,
    widgets::Widget,
};

/// ```
/// # use ratatui::widgets::Fill;
/// # use ratatui::layout::Rect;
/// # use ratatui::buffer::Cell;
/// # use ratatui::Terminal;
/// # use ratatui::backend::Backend;
/// fn draw_on_clear<B: Backend>(terminal: &mut Terminal<B>, area: Rect) {
///     terminal.render_widget(Fill::new(Cell::default().set_symbol("a")), area);
/// }
/// ```
/// A widget to fill a given area.
#[derive(Debug, Clone)]
pub struct Fill<'a> {
    cell: &'a Cell,
}

impl Fill<'_> {
    pub fn new(cell: &Cell) -> Fill {
        Fill { cell }
    }
}

impl Widget for Fill<'_> {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        buffer.expand_if_needed(area);
        for x in area.left()..area.right() {
            for y in area.top()..area.bottom() {
                let buffer_cell = buffer.get_mut(x, y);
                *buffer_cell = self.cell.clone();
            }
        }
    }
}

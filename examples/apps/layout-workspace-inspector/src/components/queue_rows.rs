use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Paragraph, Widget};
use ratatui_layout::table::{CellPosition, TableCellContext, TableItems};

use crate::domain::ReleaseItem;

/// Logical columns in the work queue.
///
/// The enum is the table shape. Header labels and body-cell formatting derive from it so keyboard
/// column movement is not coupled to rendered text.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) enum QueueColumn {
    /// Compact release reference.
    Id,

    /// Item title.
    Item,

    /// Current release state.
    State,

    /// Owner team or person.
    Owner,

    /// Item age in minutes.
    Age,
}

impl QueueColumn {
    /// Columns rendered by the queue, in left-to-right order.
    pub(super) const ALL: [Self; 5] = [Self::Id, Self::Item, Self::State, Self::Owner, Self::Age];

    /// Number of logical queue columns.
    pub(super) const COUNT: usize = Self::ALL.len();

    /// Returns a column from a table column index.
    fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }

    /// Returns the header label for this column.
    const fn header(self) -> &'static str {
        match self {
            Self::Id => "id",
            Self::Item => "item",
            Self::State => "state",
            Self::Owner => "owner",
            Self::Age => "age",
        }
    }

    /// Returns the layout constraint for this queue column.
    ///
    /// The same enum now owns column identity, labels, and width. `WorkQueue` can ask for these
    /// constraints in display order without carrying a second count-sensitive array.
    pub(super) const fn constraint(self) -> Constraint {
        match self {
            Self::Id => Constraint::Length(10),
            Self::Item => Constraint::Length(28),
            Self::State | Self::Owner => Constraint::Length(12),
            Self::Age => Constraint::Length(8),
        }
    }
}

/// Adapter that lets `VirtualTable` render queue rows and headers.
///
/// Like `TreeRows`, this owns no scroll math. It receives a cell position and area from the virtual
/// table and turns domain data into terminal cells.
///
/// `QueueRows` is the rendering adapter for `WorkQueue`. It knows how to display headers, body
/// cells, selection, hover, and status color. It does not decide which cell is selected or what a
/// click means; those responsibilities stay in `WorkQueue` and `App`.
pub(crate) struct QueueRows<'a> {
    /// Release items currently visible to the table.
    pub(crate) items: &'a [&'a ReleaseItem],

    /// Selected cell from `VirtualTableState`.
    pub(crate) selected: Option<CellPosition>,

    /// Hovered cell from the previous frame's mouse routing.
    pub(crate) hovered: Option<CellPosition>,
}

impl QueueRows<'_> {
    /// Returns the style for one table cell before it is rendered.
    ///
    /// Header, selection, and hover are visual states owned by the table adapter. Status color is
    /// domain presentation, so this helper adds it only for body status cells that are not selected.
    fn cell_style(&self, position: CellPosition) -> Style {
        let selected = self.selected == Some(position);
        if position.row.is_none() {
            return Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD);
        }
        if selected {
            return Style::new().fg(Color::Black).bg(Color::White);
        }
        if self.hovered == Some(position) {
            return Style::new().fg(Color::Yellow);
        }
        self.status_style(position).unwrap_or_default()
    }

    /// Returns the status style for an unselected body status cell.
    ///
    /// Keeping this separate from `cell_style` prevents the render path from mixing table state
    /// priority with the domain-specific rule for status coloring.
    fn status_style(&self, position: CellPosition) -> Option<Style> {
        let row = position.row?;
        if QueueColumn::from_index(position.column) == Some(QueueColumn::State) {
            Some(self.items[row].status.style())
        } else {
            None
        }
    }

    /// Returns the text for one table cell.
    ///
    /// Header cells and body cells have different sources. The caller should not need to know that
    /// `CellPosition::row` uses `None` for headers; this helper turns the table position into the
    /// text the user sees.
    fn cell_text(&self, position: CellPosition) -> String {
        match position.row {
            None => QueueColumn::from_index(position.column)
                .map_or_else(String::new, |column| column.header().to_string()),
            Some(row) => self.body_cell_text(row, position.column),
        }
    }

    /// Returns the text for one body cell.
    ///
    /// The work queue uses stable release item ids for behavior and compact display ids for the
    /// first column. Keeping the column mapping here makes `render_cell` read as rendering code
    /// instead of as a domain-to-table conversion routine.
    fn body_cell_text(&self, row: usize, column: usize) -> String {
        let item = self.items[row];
        match QueueColumn::from_index(column) {
            Some(QueueColumn::Id) => item.short_ref(),
            Some(QueueColumn::Item) => item.title.clone(),
            Some(QueueColumn::State) => item.status.label().to_string(),
            Some(QueueColumn::Owner) => item.owner.clone(),
            Some(QueueColumn::Age) => format!("{}m", item.age),
            None => String::new(),
        }
    }
}

impl TableItems for QueueRows<'_> {
    /// Reports how many body rows the table can render.
    ///
    /// Header rows are managed by `VirtualTable`; this count is only the filtered item rows.
    fn row_count(&self) -> usize {
        self.items.len()
    }

    /// Reports how many columns each row has.
    ///
    /// The count comes from `QueueColumn`, which is the logical table shape.
    fn column_count(&self) -> usize {
        QueueColumn::COUNT
    }

    /// Renders one header or body cell into the area chosen by `VirtualTable`.
    ///
    /// `CellPosition::row` is `None` for headers and `Some(row)` for body cells. The render path
    /// uses that distinction for text, styling, and whether status colors should apply.
    fn render_cell(
        &mut self,
        position: CellPosition,
        area: Rect,
        buf: &mut Buffer,
        _: TableCellContext,
    ) {
        let text = self.cell_text(position);
        let style = self.cell_style(position);
        Paragraph::new(text)
            .style(style)
            .render(cell_text_area(area), buf);
    }
}

/// Returns the text area inside a queue cell while preserving the cell's full hit area.
///
/// `VirtualTable` gives adjacent cells adjacent rectangles. Rendering text into a slightly smaller
/// rectangle creates a visual gutter without changing the layout, mouse target, or focus target
/// that the surrounding frame snapshot records.
const fn cell_text_area(area: Rect) -> Rect {
    if area.width <= 2 {
        area
    } else {
        Rect {
            x: area.x + 1,
            width: area.width - 2,
            ..area
        }
    }
}

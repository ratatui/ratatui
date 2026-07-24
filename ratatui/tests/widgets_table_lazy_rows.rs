use std::panic::RefUnwindSafe;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, HighlightSpacing, Row, Table, TableState};

/// Build a lazy table through closures that are **not** known to be [`UnwindSafe`].
///
/// `Table<'static>: Send + Sync + UnwindSafe + RefUnwindSafe` is already covered by
/// `ratatui-widgets/tests/auto_traits.rs`, which this must not weaken. What that test cannot see is
/// how strictly the closures themselves are bounded: `Arc<T>: UnwindSafe` only needs
/// `T: RefUnwindSafe`, so neither closure needs `UnwindSafe` and callers must not be asked for it.
///
/// The assertion lives in this where-clause, which deliberately omits `UnwindSafe`. Tightening
/// either bound on [`Table::lazy_rows`] or [`Table::row_height_with`] turns that into a compile
/// error here.
///
/// [`UnwindSafe`]: std::panic::UnwindSafe
fn lazy_table_from_closures<H, F>(row_count: usize, height: H, row: F) -> Table<'static>
where
    H: Fn(usize) -> u16 + Send + Sync + RefUnwindSafe + 'static,
    F: Fn(usize) -> Row<'static> + Send + Sync + RefUnwindSafe + 'static,
{
    Table::lazy_rows(row_count, [Constraint::Length(10)], row).row_height_with(height)
}

#[test]
fn lazy_closures_do_not_require_unwind_safe() {
    let table = lazy_table_from_closures(2, |_| 1, |index| Row::new([format!("row {index}")]));
    let mut state = TableState::default();

    let buffer = render_table(table, &mut state, 12, 2);

    assert_content(&buffer, ["row 0       ", "row 1       "]);
}

fn render_table(table: Table<'_>, state: &mut TableState, width: u16, height: u16) -> Buffer {
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|frame| {
            frame.render_stateful_widget(table, frame.area(), state);
        })
        .unwrap();
    terminal.backend().buffer().clone()
}

fn assert_content<'line, Lines>(buffer: &Buffer, expected: Lines)
where
    Lines: IntoIterator,
    Lines::Item: Into<Line<'line>>,
{
    let expected_buf = Buffer::with_lines(expected);
    for (i, (actual, exp)) in buffer
        .content
        .iter()
        .zip(expected_buf.content.iter())
        .enumerate()
    {
        assert_eq!(
            actual.symbol(),
            exp.symbol(),
            "cell {i} symbol mismatch: got {:?}, want {:?}",
            actual.symbol(),
            exp.symbol()
        );
    }
}

#[test]
fn lazy_table_rows_build_only_the_visible_window() {
    let built = Arc::new(Mutex::new(Vec::new()));
    let rows = {
        let built = Arc::clone(&built);
        move |index| {
            built.lock().unwrap().push(index);
            Row::new([format!("row {index:04}")])
        }
    };
    let table = Table::lazy_rows(10_000, [Constraint::Length(10)], rows);
    let mut state = TableState::default();

    let buffer = render_table(table, &mut state, 18, 4);

    assert_content(
        &buffer,
        [
            "row 0000          ",
            "row 0001          ",
            "row 0002          ",
            "row 0003          ",
        ],
    );
    assert_eq!(state.offset(), 0);
    assert_eq!(&*built.lock().unwrap(), &[0, 1, 2, 3]);
}

#[test]
fn lazy_table_rows_start_from_the_state_offset() {
    let built = Arc::new(Mutex::new(Vec::new()));
    let rows = {
        let built = Arc::clone(&built);
        move |index| {
            built.lock().unwrap().push(index);
            Row::new([format!("row {index:04}")])
        }
    };
    let table = Table::lazy_rows(10_000, [Constraint::Length(10)], rows);
    let mut state = TableState::default().with_offset(5_000);

    let buffer = render_table(table, &mut state, 18, 4);

    assert_content(
        &buffer,
        [
            "row 5000          ",
            "row 5001          ",
            "row 5002          ",
            "row 5003          ",
        ],
    );
    assert_eq!(state.offset(), 5_000);
    assert_eq!(&*built.lock().unwrap(), &[5_000, 5_001, 5_002, 5_003]);
}

#[test]
fn lazy_table_rows_jump_to_a_selected_row_without_building_preceding_rows() {
    let built = Arc::new(Mutex::new(Vec::new()));
    let rows = {
        let built = Arc::clone(&built);
        move |index| {
            built.lock().unwrap().push(index);
            Row::new([format!("row {index:04}")])
        }
    };
    let table = Table::lazy_rows(10_000, [Constraint::Length(10)], rows)
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_symbol(">")
        .row_highlight_style(Style::new().bg(Color::Blue));
    let mut state = TableState::default().with_selected(5_000);

    let buffer = render_table(table, &mut state, 18, 4);

    assert_content(
        &buffer,
        [
            " row 4997         ",
            " row 4998         ",
            " row 4999         ",
            ">row 5000         ",
        ],
    );
    assert_eq!(state.offset(), 4_997);
    assert_eq!(&*built.lock().unwrap(), &[4_997, 4_998, 4_999, 5_000]);
}

#[test]
fn lazy_table_rows_with_fewer_rows_than_viewport_builds_only_those_rows() {
    let built = Arc::new(Mutex::new(Vec::new()));
    let rows = {
        let built = Arc::clone(&built);
        move |index| {
            built.lock().unwrap().push(index);
            Row::new([format!("row {index:04}")])
        }
    };
    // 1000 total rows but viewport holds only 4 — exactly 4 must be built
    let table = Table::lazy_rows(1_000, [Constraint::Length(10)], rows);
    let mut state = TableState::default();

    let buffer = render_table(table, &mut state, 15, 4);

    assert_content(
        &buffer,
        [
            "row 0000       ",
            "row 0001       ",
            "row 0002       ",
            "row 0003       ",
        ],
    );
    assert_eq!(state.offset(), 0);
    // Only the 4 visible rows must be built; the remaining 996 must never be touched
    assert_eq!(&*built.lock().unwrap(), &[0, 1, 2, 3]);
}

#[test]
fn lazy_table_rows_rows_exactly_fill_viewport() {
    let built = Arc::new(Mutex::new(Vec::new()));
    let rows = {
        let built = Arc::clone(&built);
        move |index| {
            built.lock().unwrap().push(index);
            Row::new([format!("item {index:04}")])
        }
    };
    let table = Table::lazy_rows(2_000, [Constraint::Length(10)], rows);
    let mut state = TableState::default();

    let buffer = render_table(table, &mut state, 15, 4);

    assert_content(
        &buffer,
        [
            "item 0000      ",
            "item 0001      ",
            "item 0002      ",
            "item 0003      ",
        ],
    );
    assert_eq!(state.offset(), 0);
    assert_eq!(&*built.lock().unwrap(), &[0, 1, 2, 3]);
}

#[test]
fn lazy_table_rows_with_multi_line_row_height() {
    let built = Arc::new(Mutex::new(Vec::new()));
    let rows = {
        let built = Arc::clone(&built);
        move |index| {
            built.lock().unwrap().push(index);
            Row::new([format!("row {index:04}")])
        }
    };
    // height=2 means each row occupies 2 lines; 6-line viewport fits 3 rows
    let table = Table::lazy_rows(5_000, [Constraint::Length(10)], rows).row_height(2);
    let mut state = TableState::default();

    let buffer = render_table(table, &mut state, 15, 6);

    assert_content(
        &buffer,
        [
            "row 0000       ",
            "               ",
            "row 0001       ",
            "               ",
            "row 0002       ",
            "               ",
        ],
    );
    assert_eq!(state.offset(), 0);
    assert_eq!(&*built.lock().unwrap(), &[0, 1, 2]);
}

#[test]
fn lazy_table_rows_partial_row_at_bottom_is_included() {
    let built = Arc::new(Mutex::new(Vec::new()));
    let rows = {
        let built = Arc::clone(&built);
        move |index| {
            built.lock().unwrap().push(index);
            Row::new([format!("row {index:04}")])
        }
    };
    // height=3, area_height=5: fits 1 full row + 1 partial row (2 lines of the second row)
    let table = Table::lazy_rows(5_000, [Constraint::Length(10)], rows).row_height(3);
    let mut state = TableState::default();

    let buffer = render_table(table, &mut state, 15, 5);

    assert_content(
        &buffer,
        [
            "row 0000       ",
            "               ",
            "               ",
            "row 0001       ",
            "               ",
        ],
    );
    assert_eq!(state.offset(), 0);
    // Both row 0 and the partial row 1 must be built
    assert_eq!(&*built.lock().unwrap(), &[0, 1]);
}

#[test]
fn lazy_table_rows_selected_first_row() {
    let built = Arc::new(Mutex::new(Vec::new()));
    let rows = {
        let built = Arc::clone(&built);
        move |index| {
            built.lock().unwrap().push(index);
            Row::new([format!("row {index:04}")])
        }
    };
    let table = Table::lazy_rows(5_000, [Constraint::Length(10)], rows);
    let mut state = TableState::default().with_selected(0);

    let buffer = render_table(table, &mut state, 15, 4);

    assert_content(
        &buffer,
        [
            "row 0000       ",
            "row 0001       ",
            "row 0002       ",
            "row 0003       ",
        ],
    );
    assert_eq!(state.offset(), 0);
    assert_eq!(&*built.lock().unwrap(), &[0, 1, 2, 3]);
}

#[test]
fn lazy_table_rows_selected_last_row_scrolls_to_show_it() {
    let built = Arc::new(Mutex::new(Vec::new()));
    let rows = {
        let built = Arc::clone(&built);
        move |index| {
            built.lock().unwrap().push(index);
            Row::new([format!("row {index:04}")])
        }
    };
    let table = Table::lazy_rows(5_000, [Constraint::Length(10)], rows);
    // Select the very last row; the table must scroll to show it at the bottom
    let mut state = TableState::default().with_selected(4_999);

    let buffer = render_table(table, &mut state, 15, 4);

    assert_content(
        &buffer,
        [
            "row 4996       ",
            "row 4997       ",
            "row 4998       ",
            "row 4999       ",
        ],
    );
    assert_eq!(state.offset(), 4_996);
    assert_eq!(&*built.lock().unwrap(), &[4_996, 4_997, 4_998, 4_999]);
}

#[test]
fn lazy_table_rows_offset_beyond_count_is_clamped() {
    let built = Arc::new(Mutex::new(Vec::new()));
    let rows = {
        let built = Arc::clone(&built);
        move |index| {
            built.lock().unwrap().push(index);
            Row::new([format!("row {index:04}")])
        }
    };
    let table = Table::lazy_rows(1_000, [Constraint::Length(10)], rows);
    // Offset wildly exceeds row count — clamps to last_row (999), only that row is visible
    let mut state = TableState::default().with_offset(99_999);

    let buffer = render_table(table, &mut state, 15, 4);

    assert_content(
        &buffer,
        [
            "row 0999       ",
            "               ",
            "               ",
            "               ",
        ],
    );
    assert_eq!(state.offset(), 999, "offset must clamp to last row");
    let b = built.lock().unwrap();
    // Only the clamped row is built, not all 1000
    assert_eq!(&*b, &[999]);
}

#[test]
fn lazy_table_rows_selection_highlight_style_applied_to_selected_row() {
    let built = Arc::new(Mutex::new(Vec::new()));
    let rows = {
        let built = Arc::clone(&built);
        move |index| {
            built.lock().unwrap().push(index);
            Row::new([format!("row {index:04}")])
        }
    };
    let table = Table::lazy_rows(5_000, [Constraint::Length(10)], rows)
        .row_highlight_style(Style::new().bg(Color::Red));
    let mut state = TableState::default().with_selected(1);

    let buffer = render_table(table, &mut state, 15, 4);

    assert_content(
        &buffer,
        [
            "row 0000       ",
            "row 0001       ",
            "row 0002       ",
            "row 0003       ",
        ],
    );
    // The selected row (y=1) must have the red background; others must not
    let selected_cell = &buffer.content[15]; // row 1, col 0 (width=15)
    assert_eq!(
        selected_cell.style().bg,
        Some(Color::Red),
        "selected row must have red background"
    );
    let unselected_cell = &buffer.content[0]; // row 0, col 0
    assert_ne!(
        unselected_cell.style().bg,
        Some(Color::Red),
        "unselected row must not have red background"
    );
    // Only 4 rows built (not all 5000)
    assert_eq!(&*built.lock().unwrap(), &[0, 1, 2, 3]);
}

#[test]
fn lazy_table_rows_highlight_symbol_rendered_for_selected_row() {
    let built = Arc::new(Mutex::new(Vec::new()));
    let rows = {
        let built = Arc::clone(&built);
        move |index| {
            built.lock().unwrap().push(index);
            Row::new([format!("row {index:04}")])
        }
    };
    let table = Table::lazy_rows(5_000, [Constraint::Length(10)], rows)
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_symbol(">> ");
    let mut state = TableState::default().with_selected(2);

    let buffer = render_table(table, &mut state, 18, 4);

    assert_content(
        &buffer,
        [
            "   row 0000       ",
            "   row 0001       ",
            ">> row 0002       ",
            "   row 0003       ",
        ],
    );
    assert_eq!(state.offset(), 0);
    assert_eq!(&*built.lock().unwrap(), &[0, 1, 2, 3]);
}

#[test]
fn lazy_table_rows_with_header_does_not_affect_row_indexing() {
    let built = Arc::new(Mutex::new(Vec::new()));
    let rows = {
        let built = Arc::clone(&built);
        move |index| {
            built.lock().unwrap().push(index);
            Row::new([format!("row {index:04}")])
        }
    };
    let header = Row::new(["Header"]);
    // 4-line viewport: 1 header + 3 data rows visible
    let table = Table::lazy_rows(5_000, [Constraint::Length(10)], rows).header(header);
    let mut state = TableState::default();

    let buffer = render_table(table, &mut state, 15, 4);

    assert_content(
        &buffer,
        [
            "Header         ",
            "row 0000       ",
            "row 0001       ",
            "row 0002       ",
        ],
    );
    assert_eq!(state.offset(), 0);
    // Only 3 data rows were built despite 5000 total
    assert_eq!(&*built.lock().unwrap(), &[0, 1, 2]);
}

#[test]
fn lazy_table_rows_with_footer_does_not_affect_row_indexing() {
    let built = Arc::new(Mutex::new(Vec::new()));
    let rows = {
        let built = Arc::clone(&built);
        move |index| {
            built.lock().unwrap().push(index);
            Row::new([format!("row {index:04}")])
        }
    };
    let footer = Row::new(["Footer"]);
    // 4-line viewport: 3 data rows + 1 footer
    let table = Table::lazy_rows(5_000, [Constraint::Length(10)], rows).footer(footer);
    let mut state = TableState::default();

    let buffer = render_table(table, &mut state, 15, 4);

    assert_content(
        &buffer,
        [
            "row 0000       ",
            "row 0001       ",
            "row 0002       ",
            "Footer         ",
        ],
    );
    assert_eq!(state.offset(), 0);
    // Only 3 data rows were built
    assert_eq!(&*built.lock().unwrap(), &[0, 1, 2]);
}

#[test]
fn lazy_table_rows_with_block_reduces_inner_area() {
    let built = Arc::new(Mutex::new(Vec::new()));
    let rows = {
        let built = Arc::clone(&built);
        move |index| {
            built.lock().unwrap().push(index);
            Row::new([format!("row {index:04}")])
        }
    };
    // 6-line viewport with bordered block: inner area is 4 lines
    let table =
        Table::lazy_rows(5_000, [Constraint::Length(8)], rows).block(Block::bordered().title("T"));
    let mut state = TableState::default();

    let buffer = render_table(table, &mut state, 15, 6);

    assert_content(
        &buffer,
        [
            "┌T────────────┐",
            "│row 0000     │",
            "│row 0001     │",
            "│row 0002     │",
            "│row 0003     │",
            "└─────────────┘",
        ],
    );
    assert_eq!(state.offset(), 0);
    // Only 4 rows built (inner area is 4 lines after borders)
    assert_eq!(&*built.lock().unwrap(), &[0, 1, 2, 3]);
}

#[test]
fn lazy_table_rows_mid_list_offset_without_selection() {
    let built = Arc::new(Mutex::new(Vec::new()));
    let rows = {
        let built = Arc::clone(&built);
        move |index| {
            built.lock().unwrap().push(index);
            Row::new([format!("row {index:05}")])
        }
    };
    let table = Table::lazy_rows(10_000, [Constraint::Length(10)], rows);
    let mut state = TableState::default().with_offset(999);

    let buffer = render_table(table, &mut state, 15, 3);

    assert_content(
        &buffer,
        ["row 00999      ", "row 01000      ", "row 01001      "],
    );
    assert_eq!(state.offset(), 999);
    assert_eq!(&*built.lock().unwrap(), &[999, 1_000, 1_001]);
}

#[test]
fn lazy_table_rows_selection_above_offset_scrolls_back() {
    let built = Arc::new(Mutex::new(Vec::new()));
    let rows = {
        let built = Arc::clone(&built);
        move |index| {
            built.lock().unwrap().push(index);
            Row::new([format!("row {index:04}")])
        }
    };
    let table = Table::lazy_rows(5_000, [Constraint::Length(10)], rows);
    // Offset is at 100 but we select row 50 — must scroll back
    let mut state = TableState::default().with_offset(100).with_selected(50);

    render_table(table, &mut state, 15, 4);

    assert_eq!(
        state.offset(),
        50,
        "table must scroll back to show selected row"
    );
    let b = built.lock().unwrap();
    assert!(b.contains(&50), "selected row must be built");
    assert!(
        b.iter().all(|&i| (50..54).contains(&i)),
        "only 4 rows near selection should be built"
    );
}

#[test]
fn lazy_table_rows_multiple_columns_layout() {
    let built = Arc::new(Mutex::new(Vec::new()));
    let rows = {
        let built = Arc::clone(&built);
        move |index| {
            built.lock().unwrap().push(index);
            Row::new([format!("name{index:03}"), format!("val{index:04}")])
        }
    };
    let table = Table::lazy_rows(5_000, [Constraint::Length(7), Constraint::Length(7)], rows);
    let mut state = TableState::default().with_offset(10);

    let buffer = render_table(table, &mut state, 20, 3);

    assert_content(
        &buffer,
        [
            "name010 val0010     ",
            "name011 val0011     ",
            "name012 val0012     ",
        ],
    );
    assert_eq!(&*built.lock().unwrap(), &[10, 11, 12]);
}

#[test]
fn lazy_table_rows_no_rows_built_for_zero_height_area() {
    let built = Arc::new(Mutex::new(Vec::new()));
    let rows = {
        let built = Arc::clone(&built);
        move |index| {
            built.lock().unwrap().push(index);
            Row::new([format!("row {index}")])
        }
    };
    let table = Table::lazy_rows(5_000, [Constraint::Length(10)], rows);
    let mut state = TableState::default();

    // Render into a zero-height area — nothing should be built
    let backend = TestBackend::new(15, 5);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|frame| {
            frame.render_stateful_widget(table, Rect::new(0, 0, 15, 0), &mut state);
        })
        .unwrap();

    assert!(
        built.lock().unwrap().is_empty(),
        "factory must not be called for a zero-height area"
    );
}

#[test]
fn lazy_table_rows_multiline_scroll_uses_floor_not_ceil() {
    let built = Arc::new(Mutex::new(Vec::new()));
    let rows = {
        let built = Arc::clone(&built);
        move |index| {
            built.lock().unwrap().push(index);
            Row::new([format!("row {index:04}")])
        }
    };
    let table = Table::lazy_rows(100, [Constraint::Length(10)], rows).row_height(2);
    let mut state = TableState::default().with_selected(99);

    let buffer = render_table(table, &mut state, 15, 5);

    assert_content(
        &buffer,
        [
            "row 0098       ",
            "               ",
            "row 0099       ",
            "               ",
            "               ",
        ],
    );
    assert_eq!(
        state.offset(),
        98,
        "offset must be 98 (floor anchor), not 97 (ceil anchor)"
    );
    // Exactly rows 98 and 99 built — row 97 must NOT be called
    assert_eq!(&*built.lock().unwrap(), &[98, 99]);
}

#[test]
fn lazy_table_rows_select_last_with_usize_max_clamps_correctly() {
    let built = Arc::new(Mutex::new(Vec::new()));
    let rows = {
        let built = Arc::clone(&built);
        move |index| {
            built.lock().unwrap().push(index);
            Row::new([format!("row {index:04}")])
        }
    };
    let table = Table::lazy_rows(100, [Constraint::Length(10)], rows);
    // select_last() sets selected to usize::MAX — must clamp to last real row
    let mut state = TableState::default();
    state.select_last();

    let buffer = render_table(table, &mut state, 15, 4);

    assert_eq!(
        state.selected(),
        Some(99),
        "usize::MAX selection must clamp to last row"
    );
    assert_content(
        &buffer,
        [
            "row 0096       ",
            "row 0097       ",
            "row 0098       ",
            "row 0099       ",
        ],
    );
    // Only 4 rows near end built, not all 100
    assert_eq!(&*built.lock().unwrap(), &[96, 97, 98, 99]);
}

#[test]
fn lazy_table_rows_multiline_rows_with_explicit_offset() {
    let built = Arc::new(Mutex::new(Vec::new()));
    let rows = {
        let built = Arc::clone(&built);
        move |index| {
            built.lock().unwrap().push(index);
            Row::new([format!("row {index:04}")])
        }
    };
    // row_height=2, area=6: 3 full rows visible.  offset=20 means start at row 20.
    let table = Table::lazy_rows(500, [Constraint::Length(10)], rows).row_height(2);
    let mut state = TableState::default().with_offset(20);

    let buffer = render_table(table, &mut state, 15, 6);

    assert_content(
        &buffer,
        [
            "row 0020       ",
            "               ",
            "row 0021       ",
            "               ",
            "row 0022       ",
            "               ",
        ],
    );
    assert_eq!(state.offset(), 20);
    // Rows 20, 21, 22 built — the offset is a row index, not a line index
    assert_eq!(&*built.lock().unwrap(), &[20, 21, 22]);
}

#[test]
fn lazy_table_rows_multiline_with_header_and_selection_scroll() {
    let built = Arc::new(Mutex::new(Vec::new()));
    let rows = {
        let built = Arc::clone(&built);
        move |index| {
            built.lock().unwrap().push(index);
            Row::new([format!("row {index:04}")])
        }
    };
    let header = Row::new(["Name"]);
    let table = Table::lazy_rows(500, [Constraint::Length(10)], rows)
        .row_height(2)
        .header(header);
    let mut state = TableState::default().with_selected(200);

    let buffer = render_table(table, &mut state, 15, 7);

    assert_content(
        &buffer,
        [
            "Name           ",
            "row 0198       ",
            "               ",
            "row 0199       ",
            "               ",
            "row 0200       ",
            "               ",
        ],
    );
    assert_eq!(state.offset(), 198);
    // Exactly 3 data rows built
    assert_eq!(&*built.lock().unwrap(), &[198, 199, 200]);
}

/// A row factory that records, in order, the indexes it was called with.
fn recording_rows() -> (
    Arc<Mutex<Vec<usize>>>,
    impl Fn(usize) -> Row<'static> + Send + Sync + RefUnwindSafe,
) {
    let built = Arc::new(Mutex::new(Vec::new()));
    let factory = {
        let built = Arc::clone(&built);
        move |index: usize| {
            built.lock().unwrap().push(index);
            Row::new([format!("row {index:04}")])
        }
    };
    (built, factory)
}

/// Rows alternate between two lines (even indexes) and one line (odd indexes).
const fn alternating_height(index: usize) -> u16 {
    if index.is_multiple_of(2) { 2 } else { 1 }
}

#[test]
fn lazy_variable_rows_are_laid_out_at_their_own_heights() {
    let (built, rows) = recording_rows();
    let table =
        Table::lazy_rows(5, [Constraint::Length(10)], rows).row_height_with(alternating_height);
    let mut state = TableState::default();

    let buffer = render_table(table, &mut state, 12, 6);

    assert_content(
        &buffer,
        [
            "row 0000    ", // height 2
            "            ",
            "row 0001    ", // height 1
            "row 0002    ", // height 2
            "            ",
            "row 0003    ", // height 1
        ],
    );
    assert_eq!(state.offset(), 0);
    assert_eq!(&*built.lock().unwrap(), &[0, 1, 2, 3]);
}

#[test]
fn lazy_variable_rows_start_from_the_state_offset() {
    let (built, rows) = recording_rows();
    let table =
        Table::lazy_rows(1_000, [Constraint::Length(10)], rows).row_height_with(alternating_height);
    let mut state = TableState::default().with_offset(10);

    let buffer = render_table(table, &mut state, 12, 6);

    assert_content(
        &buffer,
        [
            "row 0010    ",
            "            ",
            "row 0011    ",
            "row 0012    ",
            "            ",
            "row 0013    ",
        ],
    );
    assert_eq!(state.offset(), 10);
    assert_eq!(&*built.lock().unwrap(), &[10, 11, 12, 13]);
}

#[test]
fn lazy_variable_rows_anchor_a_selection_below_the_window() {
    let (built, rows) = recording_rows();
    // Every tenth row is three lines tall, the rest are single lines.
    let heights = |index: usize| if index.is_multiple_of(10) { 3 } else { 1 };
    let table = Table::lazy_rows(100, [Constraint::Length(10)], rows).row_height_with(heights);
    let mut state = TableState::default().with_selected(99);

    let buffer = render_table(table, &mut state, 12, 5);

    assert_content(
        &buffer,
        [
            "row 0095    ",
            "row 0096    ",
            "row 0097    ",
            "row 0098    ",
            "row 0099    ",
        ],
    );
    assert_eq!(state.offset(), 95);
    assert_eq!(&*built.lock().unwrap(), &[95, 96, 97, 98, 99]);
}

#[test]
fn lazy_variable_rows_scroll_back_less_for_a_tall_selected_row() {
    let (built, rows) = recording_rows();
    // Row 20 is three lines tall, so it leaves room for only one row above it in a 4 line area.
    let heights = |index: usize| if index == 20 { 3 } else { 1 };
    let table = Table::lazy_rows(50, [Constraint::Length(10)], rows).row_height_with(heights);
    let mut state = TableState::default().with_selected(20);

    let buffer = render_table(table, &mut state, 12, 4);

    assert_content(
        &buffer,
        [
            "row 0019    ",
            "row 0020    ", // three lines tall
            "            ",
            "            ",
        ],
    );
    assert_eq!(state.offset(), 19);
    assert_eq!(&*built.lock().unwrap(), &[19, 20]);
}

#[test]
fn lazy_variable_rows_partial_tall_row_at_the_bottom_is_clipped() {
    let (built, rows) = recording_rows();
    // Row 1 is five lines tall but only two lines of space are left for it.
    let heights = |index: usize| if index == 1 { 5 } else { 1 };
    let table = Table::lazy_rows(10, [Constraint::Length(10)], rows).row_height_with(heights);
    let mut state = TableState::default();

    let buffer = render_table(table, &mut state, 12, 3);

    assert_content(&buffer, ["row 0000    ", "row 0001    ", "            "]);
    assert_eq!(state.offset(), 0);
    assert_eq!(&*built.lock().unwrap(), &[0, 1]);
}

#[test]
fn lazy_variable_rows_selection_above_offset_scrolls_back() {
    let (built, rows) = recording_rows();
    let table =
        Table::lazy_rows(1_000, [Constraint::Length(10)], rows).row_height_with(alternating_height);
    let mut state = TableState::default().with_offset(100).with_selected(50);

    let buffer = render_table(table, &mut state, 12, 6);

    assert_content(
        &buffer,
        [
            "row 0050    ",
            "            ",
            "row 0051    ",
            "row 0052    ",
            "            ",
            "row 0053    ",
        ],
    );
    assert_eq!(state.offset(), 50);
    assert_eq!(&*built.lock().unwrap(), &[50, 51, 52, 53]);
}

#[test]
fn lazy_variable_rows_with_header_use_the_remaining_height() {
    let (built, rows) = recording_rows();
    let table = Table::lazy_rows(100, [Constraint::Length(10)], rows)
        .row_height_with(alternating_height)
        .header(Row::new(["Name"]));
    let mut state = TableState::default();

    let buffer = render_table(table, &mut state, 12, 6);

    assert_content(
        &buffer,
        [
            "Name        ",
            "row 0000    ",
            "            ",
            "row 0001    ",
            "row 0002    ",
            "            ",
        ],
    );
    assert_eq!(state.offset(), 0);
    assert_eq!(&*built.lock().unwrap(), &[0, 1, 2]);
}

#[test]
fn lazy_variable_rows_selection_highlight_covers_the_whole_row_height() {
    let (_built, rows) = recording_rows();
    let table = Table::lazy_rows(100, [Constraint::Length(10)], rows)
        .row_height_with(alternating_height)
        .row_highlight_style(Style::new().bg(Color::Red));
    // Row 2 is two lines tall and starts on the fourth line (2 + 1 lines above it).
    let mut state = TableState::default().with_selected(2);

    let buffer = render_table(table, &mut state, 12, 6);

    for (line, highlighted) in [(0, false), (1, false), (2, false), (3, true), (4, true)] {
        let cell = &buffer.content[line * 12];
        assert_eq!(
            cell.style().bg == Some(Color::Red),
            highlighted,
            "line {line} should{} be highlighted",
            if highlighted { "" } else { " not" }
        );
    }
}

#[test]
fn lazy_variable_rows_height_closure_is_not_called_for_every_row() {
    let (built, rows) = recording_rows();
    let height_calls = Arc::new(AtomicUsize::new(0));
    let heights = {
        let height_calls = Arc::clone(&height_calls);
        move |_index: usize| {
            height_calls.fetch_add(1, Ordering::Relaxed);
            1
        }
    };
    let table =
        Table::lazy_rows(1_000_000, [Constraint::Length(10)], rows).row_height_with(heights);
    let mut state = TableState::default().with_selected(999_999);

    render_table(table, &mut state, 12, 4);

    assert_eq!(state.offset(), 999_996);
    assert_eq!(
        &*built.lock().unwrap(),
        &[999_996, 999_997, 999_998, 999_999]
    );
    // Scrolling a million rows down must stay proportional to the viewport, not to the distance
    // scrolled: a handful of height lookups, and nothing like a million of them.
    let calls = height_calls.load(Ordering::Relaxed);
    assert!(calls <= 16, "height closure was called {calls} times");
}

#[test]
fn lazy_variable_rows_of_zero_height_do_not_build_the_whole_table() {
    let (built, rows) = recording_rows();
    let table = Table::lazy_rows(1_000_000, [Constraint::Length(10)], rows).row_height_with(|_| 0);
    let mut state = TableState::default();

    let buffer = render_table(table, &mut state, 12, 4);

    assert_content(&buffer, ["            "; 4]);
    // Zero-height rows can never fill the area, so the window is bounded by the area height
    // instead of running through the whole dataset.
    let built = built.lock().unwrap();
    assert!(built.len() <= 5, "built {} rows", built.len());
}

#[test]
fn lazy_rows_ignore_the_height_and_margins_set_by_the_factory() {
    let (built, _rows) = recording_rows();
    let rows = {
        let built = Arc::clone(&built);
        move |index: usize| {
            built.lock().unwrap().push(index);
            // All of these are ignored: the table's row height is authoritative, because the
            // scroll calculation never sees the row itself.
            Row::new([format!("row {index:04}")])
                .height(3)
                .top_margin(2)
                .bottom_margin(2)
        }
    };
    let table = Table::lazy_rows(100, [Constraint::Length(10)], rows);
    let mut state = TableState::default();

    let buffer = render_table(table, &mut state, 12, 4);

    assert_content(
        &buffer,
        [
            "row 0000    ",
            "row 0001    ",
            "row 0002    ",
            "row 0003    ",
        ],
    );
    assert_eq!(&*built.lock().unwrap(), &[0, 1, 2, 3]);
}

#[test]
fn lazy_rows_are_replaced_by_explicitly_set_rows() {
    let (built, rows) = recording_rows();
    let table = Table::lazy_rows(1_000, [Constraint::Length(10)], rows)
        .rows([Row::new(["first"]), Row::new(["second"])]);
    let mut state = TableState::default();

    let buffer = render_table(table, &mut state, 12, 3);

    assert_content(&buffer, ["first       ", "second      ", "            "]);
    assert!(
        built.lock().unwrap().is_empty(),
        "the lazy factory must not be used once rows are set explicitly"
    );
}

#[test]
fn lazy_rows_column_count_comes_from_the_widths() {
    let (_built, rows) = recording_rows();
    let table = Table::lazy_rows(100, [Constraint::Length(5), Constraint::Length(5)], rows);
    let mut state = TableState::default();
    state.select_column(Some(9));

    render_table(table, &mut state, 12, 3);

    assert_eq!(
        state.selected_column(),
        Some(1),
        "column selection must clamp to the number of widths"
    );
}

#[test]
fn lazy_rows_with_no_rows_clear_the_selection() {
    let (built, rows) = recording_rows();
    let table = Table::lazy_rows(0, [Constraint::Length(10)], rows);
    let mut state = TableState::default().with_selected(5);

    let buffer = render_table(table, &mut state, 12, 3);

    assert_content(&buffer, ["            "; 3]);
    assert_eq!(state.selected(), None);
    assert!(built.lock().unwrap().is_empty());
}

#[test]
fn lazy_tables_are_cloneable_and_comparable() {
    let (_built, rows) = recording_rows();
    let table = Table::lazy_rows(100, [Constraint::Length(10)], rows);

    assert_eq!(table.clone(), table, "a clone shares the same row factory");
    assert!(!format!("{table:?}").is_empty(), "Table stays Debug");

    let (_built, other_rows) = recording_rows();
    let other = Table::lazy_rows(100, [Constraint::Length(10)], other_rows);
    assert_ne!(table, other, "different factories are different tables");
}

#[test]
fn lazy_rows_keep_an_oversized_selected_row_visible() {
    // Rows are three lines tall but the area is only two lines high, so no row ever fits.
    let (built, rows) = recording_rows();
    let table = Table::lazy_rows(100, [Constraint::Length(10)], rows).row_height_with(|_| 3);
    let mut state = TableState::default().with_selected(0);

    let buffer = render_table(table, &mut state, 12, 2);

    assert_content(&buffer, ["row 0000    ", "            "]);
    assert_eq!(state.offset(), 0);
    assert_eq!(&*built.lock().unwrap(), &[0]);

    // The same holds for a selection further down: the window anchors on it and clips it, rather
    // than scrolling past it.
    let (built, rows) = recording_rows();
    let table = Table::lazy_rows(100, [Constraint::Length(10)], rows).row_height_with(|_| 3);
    let mut state = TableState::default().with_selected(50);

    let buffer = render_table(table, &mut state, 12, 2);

    assert_content(&buffer, ["row 0050    ", "            "]);
    assert_eq!(state.offset(), 50);
    assert_eq!(&*built.lock().unwrap(), &[50]);
}

#[test]
fn the_last_row_height_setting_wins() {
    let (built, rows) = recording_rows();
    let table = Table::lazy_rows(100, [Constraint::Length(10)], rows)
        .row_height_with(|_| 3)
        .row_height(1);
    let mut state = TableState::default();

    let buffer = render_table(table, &mut state, 12, 3);

    assert_content(&buffer, ["row 0000    ", "row 0001    ", "row 0002    "]);
    assert_eq!(&*built.lock().unwrap(), &[0, 1, 2]);
}

#[test]
fn row_height_is_ignored_by_eagerly_built_tables() {
    let table = Table::new(
        [Row::new(["first"]), Row::new(["second"])],
        [Constraint::Length(10)],
    )
    .row_height(3);
    let mut state = TableState::default();

    let buffer = render_table(table, &mut state, 12, 3);

    // Eager rows carry their own height, so both rows stay one line tall.
    assert_content(&buffer, ["first       ", "second      ", "            "]);
}

/// Render the same table lazily and eagerly and assert the two are indistinguishable.
#[track_caller]
fn assert_matches_eager_table(
    row_count: usize,
    heights: fn(usize) -> u16,
    area: (u16, u16),
    offset: usize,
    selected: Option<usize>,
) {
    let row = |index: usize| Row::new([format!("row {index:04}")]);
    let (width, height) = area;

    if let Some(selected) = selected
        && heights(selected.min(row_count - 1)) > height
    {
        return;
    }

    let mut lazy_state = TableState::default().with_offset(offset);
    lazy_state.select(selected);
    let lazy = Table::lazy_rows(row_count, [Constraint::Length(10)], row).row_height_with(heights);
    let lazy_buffer = render_table(lazy, &mut lazy_state, width, height);

    let mut eager_state = TableState::default().with_offset(offset);
    eager_state.select(selected);
    let eager = Table::new(
        (0..row_count).map(|index| row(index).height(heights(index))),
        [Constraint::Length(10)],
    );
    let eager_buffer = render_table(eager, &mut eager_state, width, height);

    assert_eq!(
        lazy_buffer, eager_buffer,
        "lazy and eager rendering diverged for {row_count} rows in {width}x{height}, \
         offset {offset}, selected {selected:?}"
    );
    assert_eq!(
        lazy_state.offset(),
        eager_state.offset(),
        "lazy and eager offsets diverged for {row_count} rows in {width}x{height}, \
         offset {offset}, selected {selected:?}"
    );
}

#[test]
fn lazy_rows_render_like_eager_rows_of_uniform_height() {
    for row_height in [1_u16, 2, 3] {
        for area_height in 1_u16..=8 {
            for offset in [0_usize, 1, 7, 40] {
                for selected in [None, Some(0), Some(1), Some(8), Some(39)] {
                    assert_matches_eager_table(
                        40,
                        match row_height {
                            1 => |_| 1,
                            2 => |_| 2,
                            _ => |_| 3,
                        },
                        (12, area_height),
                        offset,
                        selected,
                    );
                }
            }
        }
    }
}

#[test]
fn lazy_rows_render_like_eager_rows_of_varying_height() {
    for heights in [
        alternating_height as fn(usize) -> u16,
        |index| if index.is_multiple_of(3) { 3 } else { 1 },
        |index| (index % 4) as u16 + 1,
        |index| if index == 12 { 4 } else { 1 },
    ] {
        for area_height in 1_u16..=8 {
            for offset in [0_usize, 1, 7, 30] {
                for selected in [None, Some(0), Some(2), Some(12), Some(29)] {
                    assert_matches_eager_table(30, heights, (12, area_height), offset, selected);
                }
            }
        }
    }
}

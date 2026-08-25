/// A Ratatui example that demonstrates how to create an interactive table with a scrollbar.
///
/// This example runs with the Ratatui library code in the branch that you are currently
/// reading. See the [`latest`] branch for the code which works with the most recent Ratatui
/// release.
///
/// [`latest`]: https://github.com/ratatui/ratatui/tree/latest
use clap::Parser;
use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyModifiers};
use itertools::Itertools;
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::style::{self, Color, Modifier, Style, Stylize};
use ratatui::text::Text;
use ratatui::widgets::{
    Block, BorderType, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
};
use ratatui::{DefaultTerminal, Frame};
use ratatui_table::{Cell, HighlightSpacing, Row, Table, TableState};
use style::palette::tailwind;
use unicode_width::UnicodeWidthStr;

const PALETTES: [tailwind::Palette; 4] = [
    tailwind::BLUE,
    tailwind::EMERALD,
    tailwind::INDIGO,
    tailwind::RED,
];
const INFO_TEXT_SIZE: usize = 3;
const INFO_TEXT: [&str; INFO_TEXT_SIZE] = [
    "(Esc) quit | (↑) move up | (↓) move down | (←) move left | (→) move right",
    "(PgUp) page up | (PgDn) page down | (Home) move to start | (End) move to end",
    "(Shift + →) next color | (Shift + ←) previous color",
];

const ITEM_HEIGHT: u16 = 4; // 2 lines of text + top/bottom spacing.
const HEADER_HEIGHT: u16 = 1;
const FOOTER_HEIGHT: u16 = INFO_TEXT_SIZE as u16 + 2; // 2 is for the borders.
const MIN_TABLE_HEIGHT: u16 = ITEM_HEIGHT + HEADER_HEIGHT;

/// Command-line arguments for the large-table example.
#[derive(Parser)]
struct Args {
    /// Number of rows to generate. Use 0 for an empty table.
    #[arg(short = 'n', default_value_t = 20)]
    list_size: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();
    color_eyre::install()?;
    ratatui::run(|terminal| App::new(args.list_size).run(terminal))
}
struct TableColors {
    buffer_bg: Color,
    header_bg: Color,
    header_fg: Color,
    row_fg: Color,
    selected_row_style_fg: Color,
    selected_column_style_fg: Color,
    selected_cell_style_fg: Color,
    normal_row_color: Color,
    alt_row_color: Color,
    footer_border_color: Color,
}

impl TableColors {
    const fn new(color: &tailwind::Palette) -> Self {
        Self {
            buffer_bg: tailwind::SLATE.c950,
            header_bg: color.c900,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_row_style_fg: color.c400,
            selected_column_style_fg: color.c400,
            selected_cell_style_fg: color.c600,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: color.c400,
        }
    }
}

struct Data {
    name: String,
    address: String,
    email: String,
}

impl Data {
    const fn ref_array(&self) -> [&String; 3] {
        [&self.name, &self.address, &self.email]
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn address(&self) -> &str {
        &self.address
    }

    fn email(&self) -> &str {
        &self.email
    }
}

struct App {
    state: TableState,
    items: Vec<Data>,
    longest_item_lens: (u16, u16, u16), // order is (name, address, email)
    scroll_state: ScrollbarState,
    colors: TableColors,
    color_index: usize,
    complete_rows: Option<usize>,
    offset: usize,
    selected: Option<usize>,
}

impl App {
    fn new(list_size: usize) -> Self {
        let data_vec = generate_fake_names(list_size);
        let is_empty = data_vec.is_empty();
        Self {
            state: TableState::default(),
            longest_item_lens: constraint_len_calculator(&data_vec),
            scroll_state: ScrollbarState::new(
                (data_vec.len().saturating_sub(1)) * ITEM_HEIGHT as usize,
            ),
            colors: TableColors::new(&PALETTES[0]),
            color_index: 0,
            items: data_vec,
            complete_rows: None,
            offset: 0,
            selected: (!is_empty).then_some(0),
        }
    }

    pub const fn update_row_state(&mut self, i: usize) {
        self.selected = Some(i);
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT as usize);
    }

    pub const fn first_row(&mut self) {
        if self.items.is_empty() {
            return;
        }
        self.update_row_state(0);
    }

    pub const fn last_row(&mut self) {
        if self.items.is_empty() {
            return;
        }
        self.update_row_state(self.items.len() - 1);
    }

    pub const fn next_row(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = match self.selected {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.update_row_state(i);
    }

    pub const fn previous_row(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = match self.selected {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.update_row_state(i);
    }

    pub const fn page_down_row(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let Some(complete_rows) = self.complete_rows else {
            return;
        };
        // page the viewport from the current offset, not the selected row: the
        // selection can sit mid-viewport after arrow-key navigation, and paging
        // from it would scroll by more than one page
        let i = self.offset + complete_rows;
        let max = self.items.len() - 1;
        let i = if i > max { max } else { i };
        self.update_row_state(i);
        // scroll the viewport by a full page, but keep it filled at the end of the
        // list: setting the offset to the selection would leave blank rows below the
        // last item when paging to the end
        let max_offset = self.items.len().saturating_sub(complete_rows);
        self.offset = if i > max_offset { max_offset } else { i };
    }

    pub const fn page_up_row(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let Some(complete_rows) = self.complete_rows else {
            return;
        };
        // page the viewport from the current offset, not the selected row
        let i = self.offset.saturating_sub(complete_rows);
        self.update_row_state(i);
        self.offset = i;
    }

    pub fn next_column(&mut self) {
        self.state.select_next_column();
    }

    pub fn previous_column(&mut self) {
        self.state.select_previous_column();
    }

    pub const fn next_color(&mut self) {
        self.color_index = (self.color_index + 1) % PALETTES.len();
    }

    pub const fn previous_color(&mut self) {
        let count = PALETTES.len();
        self.color_index = (self.color_index + count - 1) % count;
    }

    pub const fn set_colors(&mut self) {
        self.colors = TableColors::new(&PALETTES[self.color_index]);
    }

    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            if let Some(key) = event::read()?.as_key_press_event() {
                let shift_pressed = key.modifiers.contains(KeyModifiers::SHIFT);
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Char('j') | KeyCode::Down => self.next_row(),
                    KeyCode::Char('k') | KeyCode::Up => self.previous_row(),
                    KeyCode::Char('g') | KeyCode::Home => self.first_row(),
                    KeyCode::Char('G') | KeyCode::End => self.last_row(),
                    KeyCode::Char('d') | KeyCode::PageDown => self.page_down_row(),
                    KeyCode::Char('u') | KeyCode::PageUp => self.page_up_row(),
                    KeyCode::Char('L') | KeyCode::Right if shift_pressed => self.next_color(),
                    KeyCode::Char('H') | KeyCode::Left if shift_pressed => {
                        self.previous_color();
                    }
                    KeyCode::Char('l') | KeyCode::Right => self.next_column(),
                    KeyCode::Char('h') | KeyCode::Left => self.previous_column(),
                    _ => {}
                }
            }
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::vertical([
            Constraint::Min(MIN_TABLE_HEIGHT),
            Constraint::Length(FOOTER_HEIGHT),
        ]);
        let rects = frame.area().layout_vec(&layout);

        let complete_rows =
            ((rects[0].height.saturating_sub(HEADER_HEIGHT)) / ITEM_HEIGHT) as usize;
        // after shrinking the window, the offset may have been pushed to the end of the
        // list to keep the selection visible; regrowing would then leave blank rows below
        // the last item, so clamp the offset back to the start of the last page
        self.offset = self
            .offset
            .min(self.items.len().saturating_sub(complete_rows));
        // keep the selected row inside the viewport (previously done by Table's
        // visible_rows on the full row set; now the table only sees a window)
        if let Some(selected) = self.selected {
            if selected >= self.offset + complete_rows {
                self.offset = selected.saturating_sub(complete_rows.saturating_sub(1));
            } else if selected < self.offset {
                self.offset = selected;
            } else {
                // selection already visible
            }
        }
        self.complete_rows = Some(complete_rows);

        self.set_colors();

        self.render_table(frame, rects[0]);
        self.render_scrollbar(frame, rects[0]);
        self.render_footer(frame, rects[1]);
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let header_style = Style::default()
            .fg(self.colors.header_fg)
            .bg(self.colors.header_bg);
        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_row_style_fg);
        let selected_col_style = Style::default().fg(self.colors.selected_column_style_fg);
        let selected_cell_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_cell_style_fg);

        let header = ["Name", "Address", "Email"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(HEADER_HEIGHT);
        // Use ceil so a partially visible row at the bottom of the area still
        // renders; paging and offset clamping instead use the floor count
        // complete_rows so that a page step moves by whole rows only
        let visible_rows = area
            .height
            .saturating_sub(HEADER_HEIGHT)
            .div_ceil(ITEM_HEIGHT) as usize;
        let window_start = self.offset;
        let window_end = (window_start + visible_rows).min(self.items.len());
        let rows = self.items[window_start..window_end]
            .iter()
            .enumerate()
            .map(|(k, data)| {
                let i = window_start + k;
                let color = match i % 2 {
                    0 => self.colors.normal_row_color,
                    _ => self.colors.alt_row_color,
                };
                let item = data.ref_array();
                item.into_iter()
                    .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
                    .enumerate()
                    .map(|(idx, cell)| {
                        if i == 3 && idx == 1 {
                            Cell::from(Text::from(
                                // Gratuitously long error message to demonstrate column_span(2)
                                "\n[no address or email address is available for this person]\n"
                                    .to_string(),
                            ))
                            .column_span(2)
                        } else {
                            cell
                        }
                    })
                    .collect::<Row>()
                    .style(Style::new().fg(self.colors.row_fg).bg(color))
                    .height(ITEM_HEIGHT)
            });
        let bar = " █ ";
        let t = Table::new(
            rows,
            [
                // + 1 is for padding.
                Constraint::Length(self.longest_item_lens.0 + 1),
                Constraint::Min(self.longest_item_lens.1 + 1),
                Constraint::Min(self.longest_item_lens.2),
            ],
        )
        .header(header)
        .row_highlight_style(selected_row_style)
        .column_highlight_style(selected_col_style)
        .cell_highlight_style(selected_cell_style)
        .highlight_symbol(Text::from(vec![
            "".into(),
            bar.into(),
            bar.into(),
            "".into(),
        ]))
        .bg(self.colors.buffer_bg)
        .highlight_spacing(HighlightSpacing::Always);
        // sync the table state to window coordinates before rendering, then read
        // back the (possibly auto-scrolled) offsets into global state.
        // When the window is empty (degenerate tiny terminal or empty items),
        // render the table without clobbering self.selected/self.offset.
        if window_start < window_end {
            *self.state.offset_mut() = 0;
            *self.state.selected_mut() = self.selected.map(|s| s - window_start);
            frame.render_stateful_widget(t, area, &mut self.state);
            self.offset = self.state.offset() + window_start;
            self.selected = self.state.selected().map(|s| s + window_start);
        } else {
            frame.render_stateful_widget(t, area, &mut self.state);
        }
    }

    fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None),
            area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut self.scroll_state,
        );
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let info_footer = Paragraph::new(Text::from_iter(INFO_TEXT))
            .style(
                Style::new()
                    .fg(self.colors.row_fg)
                    .bg(self.colors.buffer_bg),
            )
            .centered()
            .block(
                Block::bordered()
                    .border_type(BorderType::Double)
                    .border_style(Style::new().fg(self.colors.footer_border_color)),
            );
        frame.render_widget(info_footer, area);
    }
}

fn generate_fake_names(list_size: usize) -> Vec<Data> {
    use fakeit::{address, contact, name};

    (0..list_size)
        .map(|_| {
            let name = name::full();
            let address = format!(
                "{}\n{}, {} {}",
                address::street(),
                address::city(),
                address::state(),
                address::zip()
            );
            let email = contact::email();

            Data {
                name,
                address,
                email,
            }
        })
        .sorted_by(|a, b| a.name.cmp(&b.name))
        .collect()
}

fn constraint_len_calculator(items: &[Data]) -> (u16, u16, u16) {
    let name_len = items
        .iter()
        .map(Data::name)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let address_len = items
        .iter()
        .map(Data::address)
        .flat_map(str::lines)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let email_len = items
        .iter()
        .map(Data::email)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    #[expect(clippy::cast_possible_truncation)]
    (name_len as u16, address_len as u16, email_len as u16)
}

#[cfg(test)]
mod tests {
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    use crate::{Data, FOOTER_HEIGHT, HEADER_HEIGHT, ITEM_HEIGHT};

    const WIDTH: u16 = 80;

    #[test]
    fn constraint_len_calculator() {
        let test_data = vec![
            Data {
                name: "Emirhan Tala".to_string(),
                address: "Cambridgelaan 6XX\n3584 XX Utrecht".to_string(),
                email: "tala.emirhan@gmail.com".to_string(),
            },
            Data {
                name: "thistextis26characterslong".to_string(),
                address: "this line is 31 characters long\nbottom line is 33 characters long"
                    .to_string(),
                email: "thisemailis40caharacterslong@ratatui.com".to_string(),
            },
        ];
        let (longest_name_len, longest_address_len, longest_email_len) =
            crate::constraint_len_calculator(&test_data);

        assert_eq!(26, longest_name_len);
        assert_eq!(33, longest_address_len);
        assert_eq!(40, longest_email_len);
    }

    #[test]
    fn resize_clamps_offset_to_fill_viewport() {
        let list_size = 20;
        let mut app = crate::App::new(list_size);
        let item_height = ITEM_HEIGHT;

        let big_visible_rows = 9;
        let small_visible_rows = 5;
        // terminal heights derived from the same formula as render():
        //   complete_rows = (height - FOOTER_HEIGHT - HEADER_HEIGHT) / item_height
        //   →  height = complete_rows * item_height + HEADER_HEIGHT + FOOTER_HEIGHT
        let big_height = big_visible_rows * item_height + HEADER_HEIGHT + FOOTER_HEIGHT;
        let small_height = small_visible_rows * item_height + HEADER_HEIGHT + FOOTER_HEIGHT;
        let big_complete_rows = big_visible_rows as usize;
        let small_complete_rows = small_visible_rows as usize;

        let mut terminal = Terminal::new(TestBackend::new(WIDTH, big_height)).unwrap();

        // first render to set complete_rows
        terminal.draw(|f| app.render(f)).unwrap();

        // page down to the end
        let page_down_count = (list_size - 1).div_ceil(big_complete_rows);
        for _ in 0..page_down_count {
            app.page_down_row();
        }

        assert_eq!(app.selected, Some(list_size - 1));
        assert_eq!(app.offset, list_size - big_complete_rows);

        // shrink: render scrolls the offset to keep selected in view
        terminal = Terminal::new(TestBackend::new(WIDTH, small_height)).unwrap();
        terminal.draw(|f| app.render(f)).unwrap();
        // offset moves to list_size - small_complete_rows so that
        // the last row sits at the bottom of the viewport
        assert_eq!(app.offset, list_size - small_complete_rows);

        // regrow: clamp pulls offset back to list_size - big_complete_rows
        terminal = Terminal::new(TestBackend::new(WIDTH, big_height)).unwrap();
        terminal.draw(|f| app.render(f)).unwrap();
        assert_eq!(app.offset, list_size - big_complete_rows);
        assert_eq!(app.selected, Some(list_size - 1));
    }

    #[test]
    fn page_up_row_keeps_offset_in_sync() {
        let list_size = 20;
        let item_height = ITEM_HEIGHT;
        let visible_rows = 9;
        let height = visible_rows * item_height + HEADER_HEIGHT + FOOTER_HEIGHT;
        let complete_rows = visible_rows as usize;

        let mut app = crate::App::new(list_size);
        let mut terminal = Terminal::new(TestBackend::new(WIDTH, height)).unwrap();
        // first render sets complete_rows
        terminal.draw(|f| app.render(f)).unwrap();

        // page down to the end, matching existing test setup
        for _ in 0..(list_size - 1).div_ceil(complete_rows) {
            app.page_down_row();
        }
        assert_eq!(app.selected, Some(list_size - 1));
        assert_eq!(app.offset, list_size - complete_rows);

        // no draw in between: offset must move with the selection.
        // page_up by a page from the offset reached at the end of the list
        let offset_before = app.offset;
        app.page_up_row();
        let expected = offset_before.saturating_sub(complete_rows);
        assert_eq!(app.selected, Some(expected));
        assert_eq!(app.offset, expected);
    }

    #[test]
    fn empty_list_navigation_is_safe() {
        let mut app = crate::App::new(0);
        assert_eq!(app.selected, None);

        // all navigation must be no-ops on an empty list
        app.first_row();
        app.last_row();
        app.next_row();
        app.previous_row();
        app.page_down_row();
        app.page_up_row();
        assert_eq!(app.selected, None);

        // rendering an empty list must not panic or clobber state
        let mut terminal = Terminal::new(TestBackend::new(WIDTH, 100)).unwrap();
        terminal.draw(|f| app.render(f)).unwrap();
        assert_eq!(app.selected, None);
        assert_eq!(app.offset, 0);
    }

    #[test]
    fn arrow_navigation_wraps_around() {
        let mut app = crate::App::new(5);

        // next_row advances and wraps from the last row back to the first
        for expected in [1, 2, 3, 4, 0] {
            app.next_row();
            assert_eq!(app.selected, Some(expected));
        }

        // previous_row moves back and wraps from the first row to the last
        for expected in [4, 3, 2, 1, 0] {
            app.previous_row();
            assert_eq!(app.selected, Some(expected));
        }

        // Home/End jump straight to the bounds
        app.last_row();
        assert_eq!(app.selected, Some(4));
        app.first_row();
        assert_eq!(app.selected, Some(0));
    }

    #[test]
    fn resize_pulls_offset_back_when_selection_is_above_viewport() {
        let list_size = 20;
        let mut app = crate::App::new(list_size);
        let item_height = ITEM_HEIGHT;
        let visible_rows = 9;
        let height = visible_rows * item_height + HEADER_HEIGHT + FOOTER_HEIGHT;
        let complete_rows = visible_rows as usize;

        let mut terminal = Terminal::new(TestBackend::new(WIDTH, height)).unwrap();
        terminal.draw(|f| app.render(f)).unwrap();

        // page to the end, then jump to the first row without scrolling:
        // the selection now sits above the viewport start
        for _ in 0..(list_size - 1).div_ceil(complete_rows) {
            app.page_down_row();
        }
        app.first_row();
        assert_eq!(app.selected, Some(0));
        assert!(app.offset > 0);

        // the next render must pull the offset back to reveal the selection
        terminal.draw(|f| app.render(f)).unwrap();
        assert_eq!(app.offset, 0);
        assert_eq!(app.selected, Some(0));
    }

    #[test]
    fn partially_visible_row_is_rendered() {
        // deterministic data: avoid fakeit flakiness
        let items = (0..10)
            .map(|i| Data {
                name: format!("Name{i:02}"),
                address: format!("Main Street\nCity, State {i:05}"),
                email: format!("name{i:02}@example.com"),
            })
            .collect::<Vec<_>>();
        let mut app = crate::App::new(0);
        let len = items.len();
        app.items = items;
        app.longest_item_lens = crate::constraint_len_calculator(&app.items);
        app.selected = Some(0);
        let table_height = HEADER_HEIGHT + 2 * ITEM_HEIGHT + 2; // +2 partial lines for row 2
        let height = table_height + FOOTER_HEIGHT;
        let mut terminal = Terminal::new(TestBackend::new(WIDTH, height)).unwrap();
        let buffer_text = |terminal: &Terminal<TestBackend>| -> String {
            terminal
                .backend()
                .buffer()
                .content()
                .iter()
                .map(ratatui::buffer::Cell::symbol)
                .collect()
        };

        // walk from the first to the last row, rendering after each step so
        // the window scrolls; assert the visible window contains exactly the
        // rows in [offset, offset+3) and that a partially visible last row
        // hides its second address line
        for selected in 0..len {
            if selected > 0 {
                app.next_row();
            }
            terminal.draw(|f| app.render(f)).unwrap();
            let text = buffer_text(&terminal);

            // window covers rows [offset, offset+3), clamped to the list end
            let window_end = (app.offset + 3).min(len);
            for (i, item) in app.items.iter().enumerate() {
                assert_eq!(
                    text.contains(item.name.as_str()),
                    (app.offset..window_end).contains(&i),
                    "row {i} presence mismatch"
                );
            }

            // a window of 3 rows ends in a partially visible row: its second
            // address line must stay hidden
            if window_end - app.offset == 3 {
                let last = window_end - 1;
                assert!(
                    !text.contains(&format!("City, State {last:05}")),
                    "row {last} second address line leaked"
                );
            }
        }

        // the last window (2 rows) is fully visible, so the final row's
        // second address line renders
        let text = buffer_text(&terminal);
        assert!(text.contains("City, State 00009"));
    }

    #[test]
    fn paging_moves_by_full_rows_only() {
        let list_size = 10;
        let mut app = crate::App::new(list_size);
        let table_height = HEADER_HEIGHT + 2 * ITEM_HEIGHT + 2; // +2 partial lines for row 2
        let height = table_height + FOOTER_HEIGHT;
        let mut terminal = Terminal::new(TestBackend::new(WIDTH, height)).unwrap();
        terminal.draw(|f| app.render(f)).unwrap();

        app.page_down_row();
        // page step is the floor count (2 complete rows), not the ceil count (3)
        assert_eq!(app.selected, Some(2));
        assert_eq!(app.offset, 2);
    }
}

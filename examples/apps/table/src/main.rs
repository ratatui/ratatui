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
    Block, BorderType, Cell, HighlightSpacing, Paragraph, Row, Scrollbar, ScrollbarOrientation,
    ScrollbarState, Table, TableState,
};
use ratatui::{DefaultTerminal, Frame};
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

/// Table example with pagination support.
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
    rows_per_page: Option<usize>,
}

impl App {
    fn new(list_size: usize) -> Self {
        let data_vec = generate_fake_names(list_size);
        Self {
            state: TableState::default().with_selected(0),
            longest_item_lens: constraint_len_calculator(&data_vec),
            scroll_state: ScrollbarState::new(
                (data_vec.len().saturating_sub(1)) * ITEM_HEIGHT as usize,
            ),
            colors: TableColors::new(&PALETTES[0]),
            color_index: 0,
            items: data_vec,
            rows_per_page: None,
        }
    }

    pub const fn update_row_state(&mut self, i: usize) {
        self.state.select(Some(i));
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
        let i = match self.state.selected() {
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
        let i = match self.state.selected() {
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
        let Some(rows_per_page) = self.rows_per_page else {
            return;
        };
        // page the viewport from the current offset, not the selected row: the
        // selection can sit mid-viewport after arrow-key navigation, and paging
        // from it would scroll by more than one page
        let i = self.state.offset() + rows_per_page;
        let max = self.items.len() - 1;
        let i = if i > max { max } else { i };
        self.update_row_state(i);
        // scroll the viewport by a full page, but keep it filled at the end of the
        // list: setting the offset to the selection would leave blank rows below the
        // last item when paging to the end
        let max_offset = self.items.len().saturating_sub(rows_per_page);
        *self.state.offset_mut() = if i > max_offset { max_offset } else { i };
    }

    pub const fn page_up_row(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let Some(rows_per_page) = self.rows_per_page else {
            return;
        };
        // page the viewport from the current offset, not the selected row
        let i = self.state.offset().saturating_sub(rows_per_page);
        self.update_row_state(i);
        *self.state.offset_mut() = i;
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

        let rows_per_page =
            ((rects[0].height.saturating_sub(HEADER_HEIGHT)) / ITEM_HEIGHT) as usize;
        // after shrinking the window, the offset may have been pushed to the end of the
        // list to keep the selection visible; regrowing would then leave blank rows below
        // the last item, so clamp the offset back to the start of the last page
        let max_offset = self.items.len().saturating_sub(rows_per_page);
        if self.state.offset() > max_offset {
            *self.state.offset_mut() = max_offset;
        }
        self.rows_per_page = Some(rows_per_page);

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
        let rows = self.items.iter().enumerate().map(|(i, data)| {
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
        frame.render_stateful_widget(t, area, &mut self.state);
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
        //   rows_per_page = (height - FOOTER_HEIGHT - HEADER_HEIGHT) / item_height
        //   →  height = rows_per_page * item_height + HEADER_HEIGHT + FOOTER_HEIGHT
        let big_height = big_visible_rows * item_height + HEADER_HEIGHT + FOOTER_HEIGHT;
        let small_height = small_visible_rows * item_height + HEADER_HEIGHT + FOOTER_HEIGHT;
        let big_rows_per_page = big_visible_rows as usize;
        let small_rows_per_page = small_visible_rows as usize;

        let mut terminal = Terminal::new(TestBackend::new(WIDTH, big_height)).unwrap();

        // first render to set rows_per_page
        terminal.draw(|f| app.render(f)).unwrap();

        // page down to the end
        let page_down_count = (list_size - 1).div_ceil(big_rows_per_page);
        for _ in 0..page_down_count {
            app.page_down_row();
        }

        assert_eq!(app.state.selected(), Some(list_size - 1));
        assert_eq!(app.state.offset(), list_size - big_rows_per_page);

        // shrink: render scrolls the offset to keep selected in view
        terminal = Terminal::new(TestBackend::new(WIDTH, small_height)).unwrap();
        terminal.draw(|f| app.render(f)).unwrap();
        assert_eq!(app.state.offset(), list_size - small_rows_per_page);

        // regrow: clamp pulls offset back to list_size - big_rows_per_page
        terminal = Terminal::new(TestBackend::new(WIDTH, big_height)).unwrap();
        terminal.draw(|f| app.render(f)).unwrap();
        assert_eq!(app.state.offset(), list_size - big_rows_per_page);
        assert_eq!(app.state.selected(), Some(list_size - 1));
    }
}

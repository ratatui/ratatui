//! # [Ratatui] Constraint explorer example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui-org/ratatui
//! [examples]: https://github.com/ratatui-org/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui-org/ratatui/blob/main/examples/README.md

use std::io::{self, stdout};

use color_eyre::{config::HookBuilder, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    layout::{Constraint::*, Flex},
    prelude::*,
    style::palette::tailwind::*,
    symbols::line,
    widgets::*,
};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

#[derive(Default)]
struct App {
    mode: AppMode,
    spacing: u16,
    constraints: Vec<Constraint>,
    selected_index: usize,
    value: u16,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum AppMode {
    #[default]
    Select,
    Edit(ConstraintEditor),
    Quit,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct ConstraintEditor {
    constraint_type: ConstraintName,
    value: String,
}

/// A variant of [`Constraint`] that can be rendered as a tab.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumIter, FromRepr, Display)]
enum ConstraintName {
    #[default]
    Length,
    Percentage,
    Ratio,
    Min,
    Max,
    Fill,
}

/// A widget that renders a [`Constraint`] as a block. E.g.:
/// ```plain
/// ┌──────────────┐
/// │  Length(16)  │
/// │     16px     │
/// └──────────────┘
/// ```
struct ConstraintBlock {
    constraint: Constraint,
}

/// A widget that renders a spacer with a label indicating the width of the spacer. E.g.:
///
/// ```plain
/// ┌      ┐
///   8 px
/// └      ┘
/// ```
struct SpacerBlock;

fn main() -> Result<()> {
    init_error_hooks()?;
    let terminal = init_terminal()?;
    App::default().run(terminal)?;
    restore_terminal()?;
    Ok(())
}

// App behaviour
impl App {
    fn run(&mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        self.constraints = vec![
            Constraint::Length(20),
            Constraint::Length(20),
            Constraint::Length(20),
        ];
        self.value = 20;
        while self.is_running() {
            self.draw(&mut terminal)?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.mode != AppMode::Quit
    }

    fn draw(&self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.size()))?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        use KeyCode::*;
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                Char('q') | Esc => self.exit(),
                Char('s') | Enter => self.toggle_edit(),
                Char('1') => self.swap_constraint(ConstraintName::Min),
                Char('2') => self.swap_constraint(ConstraintName::Max),
                Char('3') => self.swap_constraint(ConstraintName::Length),
                Char('4') => self.swap_constraint(ConstraintName::Percentage),
                Char('5') => self.swap_constraint(ConstraintName::Ratio),
                Char('6') => self.swap_constraint(ConstraintName::Fill),
                _ => self.handle_mode_events(key),
            },
            _ => {}
        }
        Ok(())
    }

    fn handle_mode_events(&mut self, key: KeyEvent) {
        match &mut self.mode {
            AppMode::Select => self.handle_select_event(key),
            AppMode::Edit(editor) => editor.handle_key_event(key),
            AppMode::Quit => {}
        }
    }

    fn handle_select_event(&mut self, key: KeyEvent) {
        use KeyCode::*;
        match key.code {
            Char('+') => self.increment_spacing(),
            Char('-') => self.decrement_spacing(),
            Char('x') => self.delete_block(),
            Char('a') => self.insert_block(),
            Up | Char('k') => self.increment_value(),
            Down | Char('j') => self.decrement_value(),
            Left | Char('h') => self.prev_block(),
            Right | Char('l') => self.next_block(),
            _ => (),
        }
    }

    /// select the next block with wrap around
    fn increment_value(&mut self) {
        if self.constraints.is_empty() {
            return;
        }
        self.constraints[self.selected_index] = match self.constraints[self.selected_index] {
            Constraint::Length(v) => Constraint::Length(v.saturating_add(1)),
            Constraint::Min(v) => Constraint::Min(v.saturating_add(1)),
            Constraint::Max(v) => Constraint::Max(v.saturating_add(1)),
            Constraint::Fill(v) => Constraint::Fill(v.saturating_add(1)),
            Constraint::Percentage(v) => Constraint::Percentage(v.saturating_add(1)),
            Constraint::Ratio(n, d) => Constraint::Ratio(n, d.saturating_add(1)),
        };
    }

    fn decrement_value(&mut self) {
        if self.constraints.is_empty() {
            return;
        }
        self.constraints[self.selected_index] = match self.constraints[self.selected_index] {
            Constraint::Length(v) => Constraint::Length(v.saturating_sub(1)),
            Constraint::Min(v) => Constraint::Min(v.saturating_sub(1)),
            Constraint::Max(v) => Constraint::Max(v.saturating_sub(1)),
            Constraint::Fill(v) => Constraint::Fill(v.saturating_sub(1)),
            Constraint::Percentage(v) => Constraint::Percentage(v.saturating_sub(1)),
            Constraint::Ratio(n, d) => Constraint::Ratio(n, d.saturating_sub(1)),
        };
    }

    /// select the next block with wrap around
    fn next_block(&mut self) {
        if self.constraints.is_empty() {
            return;
        }
        let len = self.constraints.len();
        self.selected_index = (self.selected_index + 1) % len;
    }

    /// select the previous block with wrap around
    fn prev_block(&mut self) {
        if self.constraints.is_empty() {
            return;
        }
        let len = self.constraints.len();
        self.selected_index = (self.selected_index + self.constraints.len() - 1) % len;
    }

    /// delete the selected block
    fn delete_block(&mut self) {
        if self.constraints.is_empty() {
            return;
        }
        self.constraints.remove(self.selected_index);
        self.selected_index = self.selected_index.saturating_sub(1);
    }

    /// insert a block after the selected block
    fn insert_block(&mut self) {
        let index = self
            .selected_index
            .saturating_add(1)
            .min(self.constraints.len());
        let constraint = Constraint::Length(self.value);
        self.constraints.insert(index, constraint);
        self.selected_index = index;
    }

    fn increment_spacing(&mut self) {
        self.spacing = self.spacing.saturating_add(1);
    }

    fn decrement_spacing(&mut self) {
        self.spacing = self.spacing.saturating_sub(1);
    }

    // exits edit mode or the app
    fn exit(&mut self) {
        self.mode = match self.mode {
            // ignore the editor state and move back to select mode
            AppMode::Edit(_) => AppMode::Select,
            _ => AppMode::Quit,
        }
    }

    fn swap_constraint(&mut self, name: ConstraintName) {
        // save the editor state
        let constraint = match name {
            ConstraintName::Length => Length(self.value),
            ConstraintName::Percentage => Percentage(self.value),
            ConstraintName::Min => Min(self.value),
            ConstraintName::Max => Max(self.value),
            ConstraintName::Fill => Fill(self.value),
            ConstraintName::Ratio => Ratio(1, self.value as u32),
        };
        self.constraints[self.selected_index] = constraint;
        self.mode = AppMode::Select;
    }

    // edits if in select mode, selects if in edit mode
    fn toggle_edit(&mut self) {
        if self.constraints.is_empty() {
            return;
        }
        match &self.mode {
            AppMode::Select => {
                // move into edit mode
                let selected = *self.selected_constraint().unwrap();
                self.mode = AppMode::Edit(ConstraintEditor::from(selected));
            }
            AppMode::Edit(editor) => {
                // save the editor state
                let constraint = Constraint::from(editor);
                self.constraints[self.selected_index] = constraint;
                self.mode = AppMode::Select;
            }
            AppMode::Quit => {}
        }
    }
}

// ConstraintName behaviour
impl ConstraintName {
    fn prev(&self) -> Self {
        let current_index: usize = *self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(*self)
    }

    fn next(&self) -> Self {
        let current_index = *self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(*self)
    }
}

impl From<Constraint> for ConstraintName {
    fn from(constraint: Constraint) -> Self {
        use Constraint::*;
        match constraint {
            Length(_) => ConstraintName::Length,
            Percentage(_) => ConstraintName::Percentage,
            Ratio(_, _) => ConstraintName::Ratio,
            Min(_) => ConstraintName::Min,
            Max(_) => ConstraintName::Max,
            Fill(_) => ConstraintName::Fill,
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, instructions_area, axis_area, blocks_area, editor_area] =
            area.split(&Layout::vertical([
                Length(1),
                Length(1),
                Length(1),
                Length(6 * 5),
                Length(3),
            ]));

        self.header().render(header_area, buf);
        self.instructions().render(instructions_area, buf);

        self.axis(axis_area.width).render(axis_area, buf);
        self.render_layout_blocks(blocks_area, buf);
        if let AppMode::Edit(editor) = &self.mode {
            editor.render(editor_area, buf);
        }
    }
}

// App rendering
impl App {
    const HEADER_COLOR: Color = SLATE.c200;
    const TEXT_COLOR: Color = SLATE.c400;
    const AXIS_COLOR: Color = SLATE.c300;

    fn render_layout_blocks(&self, area: Rect, buf: &mut Buffer) {
        let [start, center, end, space_around, space_between] = area.split(&Layout::vertical([
            Length(6),
            Length(6),
            Length(6),
            Length(6),
            Length(6),
        ]));

        self.render_layout_block(Flex::Start, start, buf);
        self.render_layout_block(Flex::Center, center, buf);
        self.render_layout_block(Flex::End, end, buf);
        self.render_layout_block(Flex::SpaceAround, space_around, buf);
        self.render_layout_block(Flex::SpaceBetween, space_between, buf)
    }

    fn render_layout_block(&self, flex: Flex, area: Rect, buf: &mut Buffer) {
        let [_, label, layout, cursor] = area.split(&Layout::vertical([
            Length(1),
            Length(1),
            Length(3),
            Length(1),
        ]));

        format!("Flex::{:?}", flex).bold().render(label, buf);

        let (blocks, spacers) = Layout::horizontal(&self.constraints)
            .flex(flex)
            .spacing(self.spacing)
            .split_with_spacers(layout);

        for (area, constraint) in blocks.iter().zip(self.constraints.iter()) {
            ConstraintBlock::new(*constraint).render(*area, buf);
        }

        for area in spacers.iter() {
            SpacerBlock.render(*area, buf);
        }

        if let Some(block) = blocks.get(self.selected_index) {
            let arrow_area = Rect {
                x: block.x,
                y: cursor.y,
                width: block.width,
                height: cursor.height,
            };
            self.cursor(block.width).render(arrow_area, buf);
        }
    }

    fn header(&self) -> impl Widget {
        let text = "Constraint Explorer";
        text.bold().fg(Self::HEADER_COLOR).to_centered_line()
    }

    fn instructions(&self) -> impl Widget {
        let text = "◄ ►: select, ▲ ▼: edit, s: swap, a: add, x: delete, q: quit";
        text.fg(Self::TEXT_COLOR).to_centered_line()
    }

    /// A bar like `<----- 80 px (gap: 2 px) ----->`
    ///
    /// Only shows the gap when spacing is not zero
    fn axis(&self, width: u16) -> impl Widget {
        let label = if self.spacing != 0 {
            format!("{} px (gap: {} px)", width, self.spacing)
        } else {
            format!("{} px", width)
        };
        let bar_width = width.saturating_sub(2) as usize; // we want to `<` and `>` at the ends
        let width_bar = format!("<{label:-^bar_width$}>");
        Paragraph::new(width_bar).fg(Self::AXIS_COLOR).centered()
    }

    /// A cursor like `└─────┘` that points to the selected block
    fn cursor(&self, width: u16) -> impl Widget {
        let cursor = format!("└{}┘", "─".repeat(width.saturating_sub(2) as usize));
        Paragraph::new(cursor).fg(Self::AXIS_COLOR)
    }

    fn selected_constraint(&self) -> Option<&Constraint> {
        self.constraints.get(self.selected_index)
    }
}

impl Widget for ConstraintBlock {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_color = ConstraintName::from(self.constraint).color();
        let label = self.label(area.width);
        let block = Block::bordered()
            .border_set(symbols::border::QUADRANT_OUTSIDE)
            .border_style(Style::reset().fg(main_color).reversed())
            .style(Style::default().fg(Self::TEXT_COLOR).bg(main_color));
        Paragraph::new(label)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl ConstraintBlock {
    const TEXT_COLOR: Color = SLATE.c200;

    fn new(constraint: Constraint) -> Self {
        Self { constraint }
    }

    fn label(&self, width: u16) -> String {
        let long_width = format!("{} px", width);
        let short_width = format!("{}", width);
        // border takes up 2 columns
        let available_space = width.saturating_sub(2) as usize;
        let width_label = if long_width.len() < available_space {
            long_width
        } else if short_width.len() < available_space {
            short_width
        } else {
            "".to_string()
        };
        format!("{}\n{}", self.constraint, width_label)
    }
}

impl Widget for SpacerBlock {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width > 1 {
            Self::block().render(area, buf);
        } else {
            Self::line().render(area, buf);
        }
        let row = area.rows().nth(1).unwrap_or_default();
        Self::label(area.width).render(row, buf);
    }
}

impl SpacerBlock {
    const TEXT_COLOR: Color = SLATE.c400;
    const BORDER_COLOR: Color = SLATE.c600;

    fn block() -> impl Widget {
        let corners_only = symbols::border::Set {
            top_left: line::NORMAL.top_left,
            top_right: line::NORMAL.top_right,
            bottom_left: line::NORMAL.bottom_left,
            bottom_right: line::NORMAL.bottom_right,
            vertical_left: " ",
            vertical_right: " ",
            horizontal_top: " ",
            horizontal_bottom: " ",
        };
        Block::bordered()
            .border_set(corners_only)
            .border_style(Self::BORDER_COLOR)
    }

    fn line() -> impl Widget {
        Paragraph::new(Text::from(vec![
            Line::from(""),
            Line::from("│"),
            Line::from("│"),
            Line::from(""),
        ]))
        .style(Self::BORDER_COLOR)
    }

    fn label(width: u16) -> impl Widget {
        let long_label = format!("{width} px");
        let short_label = format!("{width}");
        let label = if long_label.len() < width as usize {
            long_label
        } else if short_label.len() < width as usize {
            short_label
        } else {
            "".to_string()
        };
        Line::styled(label, Self::TEXT_COLOR).centered()
    }
}

impl Widget for &ConstraintEditor {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [labels, values] = area.split(&Layout::horizontal([Length(17), Fill(0)]));

        let vertical = Layout::vertical([Length(1), Length(1)]);

        // labels
        let [constraint_type, value] = labels.split(&vertical);
        Line::from("Constraint Type:").render(constraint_type, buf);
        match self.constraint_type {
            ConstraintName::Ratio => {
                Line::from("Denominator:").render(value, buf);
            }
            _ => Line::from("Length:").render(value, buf),
        }

        // values
        let [constraint_type, value] = values.split(&vertical);
        self.constraint_types().render(constraint_type, buf);
        match self.constraint_type {
            ConstraintName::Ratio => {
                Paragraph::new(self.value.as_str())
                    .style(ConstraintEditor::TEXT_COLOR)
                    .render(value, buf);
            }
            _ => {
                Paragraph::new(self.value.as_str())
                    .style(ConstraintEditor::TEXT_COLOR)
                    .render(value, buf);
            }
        }
    }
}

// TODO handle focus and editing values
impl ConstraintEditor {
    const TEXT_COLOR: Color = SLATE.c400;

    fn constraint_types(&self) -> impl Widget {
        let titles = ConstraintName::iter().map(ConstraintName::to_tab_title);
        Tabs::new(titles)
            .highlight_style(Modifier::REVERSED)
            .select(self.constraint_type as usize)
            .padding("", "")
            .divider(" ")
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        use KeyCode::*;
        match key.code {
            Left => self.prev_constraint(),
            Right => self.next_constraint(),
            _ => (),
        }
    }

    fn next_constraint(&mut self) {
        self.constraint_type = self.constraint_type.next();
    }

    fn prev_constraint(&mut self) {
        self.constraint_type = self.constraint_type.prev();
    }
}

impl From<Constraint> for ConstraintEditor {
    fn from(constraint: Constraint) -> Self {
        use Constraint::*;
        let constraint_name = ConstraintName::from(constraint);
        let value = match constraint {
            Length(value) => value.to_string(),
            Percentage(value) => value.to_string(),
            Ratio(_, value) => value.to_string(),
            Min(value) => value.to_string(),
            Max(value) => value.to_string(),
            Fill(value) => value.to_string(),
        };
        Self {
            constraint_type: constraint_name,
            value,
        }
    }
}

impl From<&ConstraintEditor> for Constraint {
    fn from(editor: &ConstraintEditor) -> Self {
        use Constraint::*;
        if editor.constraint_type == ConstraintName::Ratio {
            // ratio uses u32 values
            // assume use input is always denominator
            Ratio(1, editor.value.parse().unwrap_or_default())
        } else {
            let value = editor.value.parse().unwrap_or_default();
            match editor.constraint_type {
                ConstraintName::Length => Length(value),
                ConstraintName::Percentage => Percentage(value),
                ConstraintName::Min => Min(value),
                ConstraintName::Max => Max(value),
                ConstraintName::Fill => Fill(value),
                _ => unreachable!(),
            }
        }
    }
}

impl ConstraintName {
    fn to_tab_title(self) -> Line<'static> {
        format!("  {self}  ").fg(SLATE.c200).bg(self.color()).into()
    }

    fn color(&self) -> Color {
        match self {
            Self::Length => SLATE.c700,
            Self::Percentage => SLATE.c800,
            Self::Ratio => SLATE.c900,
            Self::Fill => SLATE.c950,
            Self::Min => BLUE.c900,
            Self::Max => BLUE.c800,
        }
    }
}
fn init_error_hooks() -> Result<()> {
    let (panic, error) = HookBuilder::default().into_hooks();
    let panic = panic.into_panic_hook();
    let error = error.into_eyre_hook();
    color_eyre::eyre::set_hook(Box::new(move |e| {
        let _ = restore_terminal();
        error(e)
    }))?;
    std::panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal();
        panic(info)
    }));
    Ok(())
}

fn init_terminal() -> Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

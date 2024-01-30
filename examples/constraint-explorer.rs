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

#[derive(Default, Clone)]
struct App {
    state: AppState,
    layout_blocks: LayoutBlocks,
    // TODO: move this to a separate editing struct
    prompt_state: PromptState,
    selected_constraint: ConstraintName,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quit,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum PromptState {
    #[default]
    SelectConstraint,
    PromptConstraintDetails(ConstraintName),
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
        while self.is_running() {
            self.draw(&mut terminal)?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.state == AppState::Running
    }

    fn draw(&self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.size()))?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        use KeyCode::*;
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                Char('q') | Esc => self.quit(),
                Char('h') => self.prev_constraint(),
                Char('l') => self.next_constraint(),
                Enter => match self.prompt_state {
                    PromptState::SelectConstraint => self.prompt(),
                    PromptState::PromptConstraintDetails(selection) => self.confirm(selection),
                },
                _ => self.layout_blocks.handle_key_event(key),
            },
            _ => {}
        }
        Ok(())
    }

    fn next_constraint(&mut self) {
        self.selected_constraint = self.selected_constraint.next();
    }

    fn prev_constraint(&mut self) {
        self.selected_constraint = self.selected_constraint.prev();
    }

    fn prompt(&mut self) {
        self.prompt_state = PromptState::PromptConstraintDetails(self.selected_constraint);
    }

    fn confirm(&mut self, selection: ConstraintName) {
        use ConstraintName::*;
        let c = match selection {
            Length => Constraint::Length(10),
            Percentage => Constraint::Percentage(10),
            Ratio => Constraint::Ratio(1, 10),
            Min => Constraint::Min(10),
            Max => Constraint::Max(10),
            Fill => Constraint::Fill(1),
        };
        self.layout_blocks.constraints.push(c);
        self.prompt_state = PromptState::SelectConstraint;
    }

    fn quit(&mut self) {
        self.state = AppState::Quit;
    }
}

// ConstraintName behaviour
impl ConstraintName {
    /// Get the previous tab, if there is no previous tab return the current tab.
    fn prev(&self) -> Self {
        let current_index: usize = *self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(*self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
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
        let [header, demo, instructions, tabs] = area.split(&Layout::vertical([
            Length(1),
            Length(6),
            Length(1),
            Fill(0),
        ]));

        self.header().render(header, buf);
        self.layout_blocks.render(demo, buf);
        self.instructions().render(instructions, buf);
        self.tabs().render(tabs, buf);

        if let PromptState::PromptConstraintDetails(_selected) = self.prompt_state {
            let [area] = area.split(&Layout::vertical([Percentage(50)]).flex(Flex::Center));
            let [area] = area.split(&Layout::horizontal([Percentage(50)]).flex(Flex::Center));
            self.render_prompt(area, buf);
        }
    }
}

// App rendering
impl App {
    const HEADER_COLOR: Color = SLATE.c200;
    const TEXT_COLOR: Color = SLATE.c400;

    fn header(&self) -> impl Widget {
        let text = "Constraint Explorer";
        text.bold().fg(Self::HEADER_COLOR).to_centered_line()
    }

    fn instructions(&self) -> impl Widget {
        let text = "◄ ►: select constraint, e: edit, i: insert, d: delete, q: quit";
        text.fg(Self::TEXT_COLOR).to_centered_line()
    }

    fn tabs(&self) -> impl Widget {
        let titles = ConstraintName::iter().map(ConstraintName::to_tab_title);
        Tabs::new(titles)
            .highlight_style(Modifier::REVERSED)
            .select(self.selected_constraint as usize)
            .padding("", "")
            .divider(" ")
    }

    fn render_prompt(&self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        Paragraph::new("Input")
            .style(Self::TEXT_COLOR)
            .alignment(Alignment::Center)
            .block(Block::bordered())
            .render(area, buf);
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

/// A widget that renders a set of [`ConstraintBlock`]s and [`SpacerBlock`]s with an axis that shows
/// the width of the layout. E.g.: `<----- 80 px (gap: 2 px) ----->`
///
/// TODO: make this a stateful widget and store the constraints and spacing in it rather than in
/// the app
#[derive(Debug, Default, Clone)]
struct LayoutBlocks {
    constraints: Vec<Constraint>,
    flex: Flex,
    spacing: u16,
    selected_index: usize,
}

impl Widget for &LayoutBlocks {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [axis_area, layout_area, arrows] =
            area.split(&Layout::vertical([Length(1), Fill(0), Length(1)]));

        self.axis(axis_area.width).render(axis_area, buf);

        let (blocks, spacers) = Layout::horizontal(&self.constraints)
            .flex(self.flex)
            .spacing(self.spacing)
            .split_with_spacers(layout_area);

        for (area, constraint) in blocks.iter().zip(self.constraints.iter()) {
            ConstraintBlock::new(*constraint).render(*area, buf);
        }

        for area in spacers.iter() {
            SpacerBlock.render(*area, buf);
        }

        if let Some(block) = blocks.get(self.selected_index) {
            let arrow_area = Rect {
                x: block.x,
                y: arrows.y,
                width: block.width,
                height: arrows.height,
            };
            LayoutBlocks::cursor(block.width).render(arrow_area, buf);
        }
    }
}

impl LayoutBlocks {
    const AXIS_COLOR: Color = SLATE.c300;

    fn handle_key_event(&mut self, key: KeyEvent) {
        use KeyCode::*;
        match key.code {
            // TODO move this keyboard handling into layout blocks
            Char('+') => self.increment_spacing(),
            Char('-') => self.decrement_spacing(),
            Left => self.prev_block(),
            Right => self.next_block(),
            _ => (),
        }
    }
    /// select the next block with wrap around
    fn next_block(&mut self) {
        if self.constraints.is_empty() {
            return;
        }
        self.selected_index = (self.selected_index + 1) % self.constraints.len();
    }

    /// select the previous block with wrap around
    fn prev_block(&mut self) {
        if self.constraints.is_empty() {
            return;
        }
        self.selected_index =
            (self.selected_index + self.constraints.len() - 1) % self.constraints.len();
    }

    fn increment_spacing(&mut self) {
        self.spacing = self.spacing.saturating_add(1);
    }

    fn decrement_spacing(&mut self) {
        self.spacing = self.spacing.saturating_sub(1);
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

    /// A cursor like `└───┬───┘` that points to the selected block
    fn cursor(width: u16) -> impl Widget {
        let repeat = width.saturating_sub(2) as usize;
        let mid = "┬";
        let arrow = format!("└{mid:─^repeat$}┘");
        Paragraph::new(arrow).fg(LayoutBlocks::AXIS_COLOR)
    }
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

/// A widget that renders a spacer with a label indicating the width of the spacer. E.g.:
///
/// ```plain
/// ┌      ┐
///   8 px
/// └      ┘
/// ```
struct SpacerBlock;

impl Widget for SpacerBlock {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width > 1 {
            Self::block().render(area, buf);
        } else {
            Self::line().render(area, buf);
        }
        let row = area.rows().nth(2).unwrap_or_default();
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

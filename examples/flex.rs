//! # [Ratatui] Flex example
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
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    layout::{Constraint::*, Flex},
    prelude::*,
    style::palette::tailwind,
    widgets::{block::Title, *},
};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

const EXAMPLE_DATA: &[(&str, &[Constraint])] = &[
    (
        "Min(u16) takes any excess space when using `Stretch` or `StretchLast`",
        &[Fixed(20), Min(20), Max(20)],
    ),
    (
        "Proportional(u16) takes any excess space in all `Flex` layouts",
        &[Length(20), Percentage(20), Ratio(1, 5), Proportional(1)],
    ),
    (
        "In `StretchLast`, last constraint of lowest priority takes excess space",
        &[Length(20), Fixed(20), Percentage(20)],
    ),
    ("", &[Fixed(20), Percentage(20), Length(20)]),
    ("", &[Percentage(20), Length(20), Fixed(20)]),
    ("", &[Length(20), Length(15)]),
    ("Spacing has no effect in `SpaceAround` and `SpaceBetween`", &[Proportional(1), Proportional(1)]),
    ("", &[Length(20), Fixed(20)]),
    (
        "When not using `Flex::Stretch` or `Flex::StretchLast`,\n`Min(u16)` and `Max(u16)` collapse to their lowest values",
        &[Min(20), Max(20)],
    ),
    (
        "`SpaceBetween` stretches when there's only one constraint",
        &[Max(20)],
    ),
    ("", &[Min(20), Max(20), Length(20), Fixed(20)]),
    ("`Proportional(u16)` always fills up space in every `Flex` layout", &[Proportional(0), Proportional(0)]),
    (
        "`Proportional(1)` can be to scale with respect to other `Proportional(2)`",
        &[Proportional(1), Proportional(2)],
    ),
    (
        "`Proportional(0)` collapses if there are other non-zero `Proportional(_)`\nconstraints. e.g. `[Proportional(0), Proportional(0), Proportional(1)]`:",
        &[
            Proportional(0),
            Proportional(0),
            Proportional(1),
        ],
    ),
];

#[derive(Default, Clone, Copy)]
struct App {
    selected_tab: SelectedTab,
    scroll_offset: u16,
    spacing: u16,
    state: AppState,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Example {
    constraints: Vec<Constraint>,
    description: String,
    flex: Flex,
    spacing: u16,
}

/// Tabs for the different layouts
///
/// Note: the order of the variants will determine the order of the tabs this uses several derive
/// macros from the `strum` crate to make it easier to iterate over the variants.
/// (`FromRepr`,`Display`,`EnumIter`).
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, FromRepr, Display, EnumIter)]
enum SelectedTab {
    #[default]
    StretchLast,
    Stretch,
    Start,
    Center,
    End,
    SpaceAround,
    SpaceBetween,
}

fn main() -> Result<()> {
    // assuming the user changes spacing about a 100 times or so
    Layout::init_cache(EXAMPLE_DATA.len() * SelectedTab::iter().len() * 100);
    init_error_hooks()?;
    let terminal = init_terminal()?;

    // Each line in the example is a layout
    // so 13 examples * 7 = 91 currently
    // Plus additional layout for tabs ...
    Layout::init_cache(120);

    App::default().run(terminal)?;

    restore_terminal()?;
    Ok(())
}

impl App {
    fn run(&mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        self.draw(&mut terminal)?;
        while self.is_running() {
            self.handle_events()?;
            self.draw(&mut terminal)?;
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.state == AppState::Running
    }

    fn draw(self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.size()))?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        use KeyCode::*;
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                Char('q') | Esc => self.quit(),
                Char('l') | Right => self.next(),
                Char('h') | Left => self.previous(),
                Char('j') | Down => self.down(),
                Char('k') | Up => self.up(),
                Char('g') | Home => self.top(),
                Char('G') | End => self.bottom(),
                Char('+') => self.increment_spacing(),
                Char('-') => self.decrement_spacing(),
                _ => (),
            },
            _ => {}
        }
        Ok(())
    }

    fn next(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    fn previous(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }

    fn up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1)
    }

    fn down(&mut self) {
        self.scroll_offset = self
            .scroll_offset
            .saturating_add(1)
            .min(max_scroll_offset())
    }

    fn top(&mut self) {
        self.scroll_offset = 0;
    }

    fn bottom(&mut self) {
        self.scroll_offset = max_scroll_offset();
    }

    fn increment_spacing(&mut self) {
        self.spacing = self.spacing.saturating_add(1);
    }

    fn decrement_spacing(&mut self) {
        self.spacing = self.spacing.saturating_sub(1);
    }

    fn quit(&mut self) {
        self.state = AppState::Quit;
    }
}

// when scrolling, make sure we don't scroll past the last example
fn max_scroll_offset() -> u16 {
    example_height()
        - EXAMPLE_DATA
            .last()
            .map(|(desc, _)| get_description_height(desc) + 4)
            .unwrap_or(0)
}

/// The height of all examples combined
///
/// Each may or may not have a title so we need to account for that.
fn example_height() -> u16 {
    EXAMPLE_DATA
        .iter()
        .map(|(desc, _)| get_description_height(desc) + 4)
        .sum()
}

impl Widget for App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([Fixed(3), Fixed(1), Proportional(0)]);
        let [tabs, axis, demo] = area.split(&layout);
        self.tabs().render(tabs, buf);
        let scroll_needed = self.render_demo(demo, buf);
        let axis_width = if scroll_needed {
            axis.width - 1
        } else {
            axis.width
        };
        self.axis(axis_width, self.spacing).render(axis, buf);
    }
}

impl App {
    fn tabs(&self) -> impl Widget {
        let tab_titles = SelectedTab::iter().map(SelectedTab::to_tab_title);
        let block = Block::new()
            .title(Title::from("Flex Layouts ".bold()))
            .title(" Use ◄ ► to change tab, ▲ ▼  to scroll, - + to change spacing ");
        Tabs::new(tab_titles)
            .block(block)
            .highlight_style(Modifier::REVERSED)
            .select(self.selected_tab as usize)
            .divider(" ")
            .padding("", "")
    }

    /// a bar like `<----- 80 px (gap: 2 px)? ----->`
    fn axis(&self, width: u16, spacing: u16) -> impl Widget {
        let width = width as usize;
        // only show gap when spacing is not zero
        let label = if spacing != 0 {
            format!("{} px (gap: {} px)", width, spacing)
        } else {
            format!("{} px", width)
        };
        let bar_width = width - 2; // we want to `<` and `>` at the ends
        let width_bar = format!("<{label:-^bar_width$}>");
        Paragraph::new(width_bar.dark_gray()).centered()
    }

    /// Render the demo content
    ///
    /// This function renders the demo content into a separate buffer and then splices the buffer
    /// into the main buffer. This is done to make it possible to handle scrolling easily.
    ///
    /// Returns bool indicating whether scroll was needed
    fn render_demo(self, area: Rect, buf: &mut Buffer) -> bool {
        // render demo content into a separate buffer so all examples fit we add an extra
        // area.height to make sure the last example is fully visible even when the scroll offset is
        // at the max
        let height = example_height();
        let demo_area = Rect::new(0, 0, area.width, height);
        let mut demo_buf = Buffer::empty(demo_area);

        let scrollbar_needed = self.scroll_offset != 0 || height > area.height;
        let content_area = if scrollbar_needed {
            Rect {
                width: demo_area.width - 1,
                ..demo_area
            }
        } else {
            demo_area
        };

        let mut spacing = self.spacing;
        self.selected_tab
            .render(content_area, &mut demo_buf, &mut spacing);

        let visible_content = demo_buf
            .content
            .into_iter()
            .skip((area.width * self.scroll_offset) as usize)
            .take(area.area() as usize);
        for (i, cell) in visible_content.enumerate() {
            let x = i as u16 % area.width;
            let y = i as u16 / area.width;
            *buf.get_mut(area.x + x, area.y + y) = cell;
        }

        if scrollbar_needed {
            let area = area.intersection(buf.area);
            let mut state = ScrollbarState::new(max_scroll_offset() as usize)
                .position(self.scroll_offset as usize);
            Scrollbar::new(ScrollbarOrientation::VerticalRight).render(area, buf, &mut state);
        }
        scrollbar_needed
    }
}

impl SelectedTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    fn previous(&self) -> Self {
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

    /// Convert a `SelectedTab` into a `Line` to display it by the `Tabs` widget.
    fn to_tab_title(value: SelectedTab) -> Line<'static> {
        use tailwind::*;
        use SelectedTab::*;
        let text = value.to_string();
        let color = match value {
            StretchLast => ORANGE.c400,
            Stretch => ORANGE.c300,
            Start => SKY.c400,
            Center => SKY.c300,
            End => SKY.c200,
            SpaceAround => INDIGO.c400,
            SpaceBetween => INDIGO.c300,
        };
        format!(" {text} ").fg(color).bg(Color::Black).into()
    }
}

impl StatefulWidget for SelectedTab {
    type State = u16;
    fn render(self, area: Rect, buf: &mut Buffer, spacing: &mut Self::State) {
        let spacing = *spacing;
        match self {
            SelectedTab::StretchLast => self.render_examples(area, buf, Flex::StretchLast, spacing),
            SelectedTab::Stretch => self.render_examples(area, buf, Flex::Stretch, spacing),
            SelectedTab::Start => self.render_examples(area, buf, Flex::Start, spacing),
            SelectedTab::Center => self.render_examples(area, buf, Flex::Center, spacing),
            SelectedTab::End => self.render_examples(area, buf, Flex::End, spacing),
            SelectedTab::SpaceAround => self.render_examples(area, buf, Flex::SpaceAround, spacing),
            SelectedTab::SpaceBetween => {
                self.render_examples(area, buf, Flex::SpaceBetween, spacing)
            }
        }
    }
}

impl SelectedTab {
    fn render_examples(&self, area: Rect, buf: &mut Buffer, flex: Flex, spacing: u16) {
        let heights = EXAMPLE_DATA
            .iter()
            .map(|(desc, _)| get_description_height(desc) + 4);
        let areas = Layout::vertical(heights).flex(Flex::Start).split(area);
        for (area, (description, constraints)) in areas.iter().zip(EXAMPLE_DATA.iter()) {
            Example::new(constraints, description, flex, spacing).render(*area, buf);
        }
    }
}

impl Example {
    fn new(constraints: &[Constraint], description: &str, flex: Flex, spacing: u16) -> Self {
        Self {
            constraints: constraints.into(),
            description: description.into(),
            flex,
            spacing,
        }
    }
}

impl Widget for Example {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title_height = get_description_height(&self.description);
        let layout = Layout::vertical([Fixed(title_height), Proportional(0)]);
        let [title, illustrations] = area.split(&layout);
        let blocks = Layout::horizontal(&self.constraints)
            .flex(self.flex)
            .spacing(self.spacing)
            .split(illustrations);

        if !self.description.is_empty() {
            Paragraph::new(
                self.description
                    .split('\n')
                    .map(|s| format!("// {}", s).italic().fg(tailwind::SLATE.c400))
                    .map(Line::from)
                    .collect::<Vec<Line>>(),
            )
            .render(title, buf);
        }

        for (block, constraint) in blocks.iter().zip(&self.constraints) {
            self.illustration(*constraint, block.width)
                .render(*block, buf);
        }
    }
}

impl Example {
    fn illustration(&self, constraint: Constraint, width: u16) -> Paragraph {
        let main_color = color_for_constraint(constraint);
        let fg_color = Color::White;
        let title = format!("{constraint}");
        let content = format!("{width} px");
        let text = format!("{title}\n{content}");
        let block = Block::bordered()
            .border_set(symbols::border::QUADRANT_OUTSIDE)
            .border_style(Style::reset().fg(main_color).reversed())
            .style(Style::default().fg(fg_color).bg(main_color));
        Paragraph::new(text).centered().block(block)
    }
}

fn color_for_constraint(constraint: Constraint) -> Color {
    use tailwind::*;
    match constraint {
        Constraint::Fixed(_) => RED.c900,
        Constraint::Min(_) => BLUE.c900,
        Constraint::Max(_) => BLUE.c800,
        Constraint::Length(_) => SLATE.c700,
        Constraint::Percentage(_) => SLATE.c800,
        Constraint::Ratio(_, _) => SLATE.c900,
        Constraint::Proportional(_) => SLATE.c950,
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

fn get_description_height(s: &str) -> u16 {
    if s.is_empty() {
        0
    } else {
        s.split('\n').count() as u16
    }
}

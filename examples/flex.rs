use std::{
    io::{self, stdout},
    str::FromStr,
};

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
    ("Min(20) takes excess space", &[Fixed(20), Min(20), Max(20)]),
    (
        "",
        &[Length(20), Percentage(20), Ratio(1, 5), Proportional(1)],
    ),
    ("", &[Length(20), Fixed(20), Percentage(20)]),
    ("", &[Fixed(20), Percentage(20), Length(20)]),
    ("", &[Percentage(20), Length(20), Fixed(20)]),
    ("", &[Length(20), Length(15)]),
    ("", &[Length(20), Fixed(20)]),
    ("", &[Min(20), Max(20)]),
    ("", &[Max(20)]),
    ("", &[Min(20), Max(20), Length(20), Fixed(20)]),
    ("", &[Proportional(0), Proportional(0)]),
    (
        "",
        &[Proportional(1), Proportional(1), Length(20), Fixed(20)],
    ),
    (
        "",
        &[
            Min(10),
            Proportional(3),
            Proportional(2),
            Length(15),
            Fixed(15),
        ],
    ),
];

#[derive(Default, Clone, Copy)]
struct App {
    selected_tab: SelectedTab,
    scroll_offset: u16,
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

    fn quit(&mut self) {
        self.state = AppState::Quit;
    }
}

// when scrolling, make sure we don't scroll past the last example
fn max_scroll_offset() -> u16 {
    example_height()
        - EXAMPLE_DATA
            .last()
            .map(|(desc, _)| if desc.is_empty() { 4 } else { 5 })
            .unwrap_or(0)
}

/// The height of all examples combined
///
/// Each may or may not have a title so we need to account for that.
fn example_height() -> u16 {
    EXAMPLE_DATA
        .iter()
        .map(|(desc, _)| if desc.is_empty() { 4 } else { 5 })
        .sum()
}

impl Widget for App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([Fixed(3), Fixed(3), Proportional(0)]);
        let [tabs, axis, demo] = area.split(&layout);
        self.tabs().render(tabs, buf);
        self.axis(axis.width).render(axis, buf);
        self.render_demo(demo, buf);
    }
}

impl App {
    fn tabs(&self) -> impl Widget {
        let tab_titles = SelectedTab::iter().map(SelectedTab::to_tab_title);
        let block = Block::new()
            .title(Title::from("Flex Layouts ".bold()))
            .title(" Use h l or ◄ ► to change tab and j k or ▲ ▼  to scroll");
        Tabs::new(tab_titles)
            .block(block)
            .highlight_style(Modifier::REVERSED)
            .select(self.selected_tab as usize)
            .divider(" ")
            .padding("", "")
    }

    /// a bar like `<----- 80 px ----->`
    fn axis(&self, width: u16) -> impl Widget {
        let width = width as usize;
        let label = format!("{} px", width);
        let bar_width = width - label.len() / 2;
        let width_bar = format!("<{label:-^bar_width$}>",);
        Paragraph::new(width_bar.dark_gray())
            .alignment(Alignment::Center)
            .block(Block::default().padding(Padding {
                left: 0,
                right: 0,
                top: 1,
                bottom: 0,
            }))
    }

    /// Render the demo content
    ///
    /// This function renders the demo content into a separate buffer and then splices the buffer
    /// into the main buffer. This is done to make it possible to handle scrolling easily.
    fn render_demo(self, area: Rect, buf: &mut Buffer) {
        let height = example_height();
        let mut demo_buf = Buffer::empty(Rect { height, ..area });
        self.selected_tab.render(demo_buf.area, &mut demo_buf);

        // Splice the visible area into the main buffer
        let start = buf.index_of(area.left(), area.top());
        let end = buf.content.len().saturating_sub(area.area() as usize);
        let visible_content = demo_buf
            .content
            .into_iter()
            .skip((buf.area.width * self.scroll_offset) as usize)
            .take(area.area() as usize);
        buf.content.splice(start..end, visible_content);
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
        format!(" {text} ").fg(Color::Black).bg(color).into()
    }
}

impl Widget for SelectedTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            SelectedTab::StretchLast => self.render_examples(area, buf, Flex::StretchLast),
            SelectedTab::Stretch => self.render_examples(area, buf, Flex::Stretch),
            SelectedTab::Start => self.render_examples(area, buf, Flex::Start),
            SelectedTab::Center => self.render_examples(area, buf, Flex::Center),
            SelectedTab::End => self.render_examples(area, buf, Flex::End),
            SelectedTab::SpaceAround => self.render_examples(area, buf, Flex::SpaceAround),
            SelectedTab::SpaceBetween => self.render_examples(area, buf, Flex::SpaceBetween),
        }
    }
}

impl SelectedTab {
    fn render_examples(&self, area: Rect, buf: &mut Buffer, flex: Flex) {
        let heights = EXAMPLE_DATA
            .iter()
            .map(|(desc, _)| if desc.is_empty() { 4 } else { 5 });
        let areas = Layout::vertical(heights).flex(flex).split(area);
        for (area, (description, constraints)) in areas.iter().zip(EXAMPLE_DATA.iter()) {
            Example::new(constraints, description, flex).render(*area, buf);
        }
    }
}

impl Example {
    fn new(constraints: &[Constraint], description: &str, flex: Flex) -> Self {
        Self {
            constraints: constraints.into(),
            description: description.into(),
            flex,
        }
    }
}

impl Widget for Example {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title_height = if self.description.is_empty() { 0 } else { 1 };
        let layout = Layout::vertical([Fixed(title_height), Proportional(0)]);
        let [title, illustrations] = area.split(&layout);
        let blocks = Layout::horizontal(&self.constraints)
            .flex(self.flex)
            .split(illustrations);

        if !self.description.is_empty() {
            format!("// {}", self.description)
                .italic()
                .fg(Color::from_str("#908caa").unwrap())
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
        Paragraph::new(text)
            .alignment(Alignment::Center)
            .block(block)
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

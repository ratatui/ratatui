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

const SPACER_HEIGHT: u16 = 1;
const ILLUSTRATION_HEIGHT: u16 = 4;
const EXAMPLE_HEIGHT: u16 = ILLUSTRATION_HEIGHT + SPACER_HEIGHT;
const N_EXAMPLES_PER_TAB: u16 = 11;

// priority 1
const FIXED_COLOR: Color = tailwind::RED.c900;
// priority 2
const MIN_COLOR: Color = tailwind::BLUE.c900;
const MAX_COLOR: Color = tailwind::BLUE.c800;
// priority 3
const LENGTH_COLOR: Color = tailwind::SLATE.c700;
const PERCENTAGE_COLOR: Color = tailwind::SLATE.c800;
const RATIO_COLOR: Color = tailwind::SLATE.c900;
// priority 4
const PROPORTIONAL_COLOR: Color = tailwind::SLATE.c950;

#[derive(Default, Clone, Copy)]
struct App {
    selected_example: ExampleSelection,
    scroll_offset: u16,
    max_scroll_offset: u16,
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

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
enum ExampleSelection {
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

    // always show the last example when scrolling
    let max_scroll_offset = (N_EXAMPLES_PER_TAB - 1) * EXAMPLE_HEIGHT;
    App::with_max_scroll_offset(max_scroll_offset).run(terminal)?;

    restore_terminal()?;
    Ok(())
}

impl App {
    fn with_max_scroll_offset(max_scroll_offset: u16) -> Self {
        Self {
            max_scroll_offset,
            ..Default::default()
        }
    }

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
                Char('q') => self.quit(),
                Char('l') | Right => self.next(),
                Char('h') | Left => self.previous(),
                Char('j') | Down => self.down(),
                Char('k') | Up => self.up(),
                _ => (),
            },
            _ => {}
        }
        Ok(())
    }

    fn next(&mut self) {
        self.selected_example = self.selected_example.next();
    }

    fn previous(&mut self) {
        self.selected_example = self.selected_example.previous();
    }

    fn up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1)
    }

    fn down(&mut self) {
        self.scroll_offset = self
            .scroll_offset
            .saturating_add(1)
            .min(self.max_scroll_offset)
    }

    fn quit(&mut self) {
        self.state = AppState::Quit;
    }
}

impl Widget for App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([Fixed(3), Fixed(3), Proportional(0)]);
        let [tabs, axis, demo] = area.split(&layout);
        self.render_tabs(tabs, buf);
        self.render_axis(axis, buf);
        self.render_demo(demo, buf);
    }
}

impl App {
    /// renders a bar like `<----- 80 px ----->`
    fn render_axis(&self, area: Rect, buf: &mut Buffer) {
        let width = area.width as usize;
        let width_label = format!("{} px", width);
        let width_bar = format!(
            "<{width_label:-^width$}>",
            width = width - width_label.len() / 2
        );
        Paragraph::new(width_bar.dark_gray())
            .alignment(Alignment::Center)
            .block(Block::default().padding(Padding {
                left: 0,
                right: 0,
                top: 1,
                bottom: 0,
            }))
            .render(area, buf);
    }

    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Title::from("Flex Layouts ".bold()))
            .title(" Use h l or ◄ ► to change tab and j k or ▲ ▼  to scroll");
        Tabs::new(
            [
                ExampleSelection::StretchLast,
                ExampleSelection::Stretch,
                ExampleSelection::Start,
                ExampleSelection::Center,
                ExampleSelection::End,
                ExampleSelection::SpaceAround,
                ExampleSelection::SpaceBetween,
            ]
            .into_iter()
            .map(Line::from),
        )
        .block(block)
        .highlight_style(Style::default().bold())
        .select(self.selected_example.selected())
        .divider(" ")
        .padding("", "")
        .render(area, buf);
    }

    fn render_demo(self, demo_area: Rect, buf: &mut Buffer) {
        // render demo content into a separate buffer so all examples fit
        let mut demo_buf = Buffer::empty(Rect {
            height: N_EXAMPLES_PER_TAB * EXAMPLE_HEIGHT,
            ..demo_area
        });
        self.selected_example.render(demo_buf.area, &mut demo_buf);

        // Splice the demo_buffer into the main buffer
        // NOTE: You shouldn't do this in a production app
        let start = buf.index_of(demo_area.x, demo_area.y);
        let end = demo_area.area() as usize; // this crashes right now - TODO
        buf.content.splice(
            start..end,
            demo_buf
                .content
                .into_iter()
                .skip((buf.area.width * self.scroll_offset) as usize)
                .take(demo_area.area() as usize),
        );
    }
}

impl From<ExampleSelection> for Line<'static> {
    fn from(example: ExampleSelection) -> Self {
        use ExampleSelection::*;
        let [stretch_last, stretch, start, center, end, space_around, space_between] = [
            "#28410b", "#3b520a", "#40041b", "#52051f", "#630826", "#313073", "#071b24",
        ]
        .iter()
        .map(|c| Color::from_str(c).unwrap())
        .collect::<Vec<Color>>()
        .try_into()
        .unwrap();
        match example {
            StretchLast => " StretchLast ".bg(stretch_last).into(),
            Stretch => " Stretch ".bg(stretch).into(),
            Start => " Start ".bg(start).into(),
            Center => " Center ".bg(center).into(),
            End => " End ".bg(end).into(),
            SpaceAround => " SpaceAround ".bg(space_around).into(),
            SpaceBetween => " SpaceBetween ".bg(space_between).into(),
        }
    }
}

impl ExampleSelection {
    fn previous(&self) -> Self {
        use ExampleSelection::*;
        match *self {
            StretchLast => StretchLast,
            Stretch => StretchLast,
            Start => Stretch,
            Center => Start,
            End => Center,
            SpaceAround => End,
            SpaceBetween => SpaceAround,
        }
    }

    fn next(&self) -> Self {
        use ExampleSelection::*;
        match *self {
            StretchLast => Stretch,
            Stretch => Start,
            Start => Center,
            Center => End,
            End => SpaceAround,
            SpaceAround => SpaceBetween,
            SpaceBetween => SpaceBetween,
        }
    }

    fn selected(&self) -> usize {
        use ExampleSelection::*;
        match self {
            StretchLast => 0,
            Stretch => 1,
            Start => 2,
            Center => 3,
            End => 4,
            SpaceAround => 5,
            SpaceBetween => 6,
        }
    }
}

impl Widget for ExampleSelection {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            ExampleSelection::StretchLast => self.render_example(area, buf, Flex::StretchLast),
            ExampleSelection::Stretch => self.render_example(area, buf, Flex::Stretch),
            ExampleSelection::Start => self.render_example(area, buf, Flex::Start),
            ExampleSelection::Center => self.render_example(area, buf, Flex::Center),
            ExampleSelection::End => self.render_example(area, buf, Flex::End),
            ExampleSelection::SpaceAround => self.render_example(area, buf, Flex::SpaceAround),
            ExampleSelection::SpaceBetween => self.render_example(area, buf, Flex::SpaceBetween),
        }
    }
}

impl ExampleSelection {
    fn render_example(&self, area: Rect, buf: &mut Buffer, flex: Flex) {
        let areas = &Layout::vertical([Fixed(EXAMPLE_HEIGHT); N_EXAMPLES_PER_TAB as usize])
            .flex(Flex::Start)
            .split(area);

        let examples = [
            (
                vec![Fixed(20), Min(20), Max(20)],
                "Min(20) takes excess space.",
            ),
            (
                vec![Length(20), Percentage(20), Ratio(1, 5), Proportional(1)],
                "",
            ),
            (vec![Length(20), Fixed(20), Percentage(20)], ""),
            (vec![Fixed(20), Percentage(20), Length(20)], ""),
            (vec![Percentage(20), Length(20), Fixed(20)], ""),
            (vec![Length(20), Length(15)], ""),
            (vec![Length(20), Fixed(20)], ""),
            (vec![Min(20), Max(20)], ""),
            (vec![Max(20)], ""),
            (vec![Min(20), Max(20), Length(20), Fixed(20)], ""),
            (vec![Proportional(0), Proportional(0)], ""),
            (
                vec![Proportional(1), Proportional(1), Length(20), Fixed(20)],
                "",
            ),
            (
                vec![
                    Min(10),
                    Proportional(3),
                    Proportional(2),
                    Length(15),
                    Fixed(15),
                ],
                "",
            ),
        ];

        for (area, (constraints, description)) in areas.iter().zip(examples) {
            Example::new(constraints, description.into())
                .flex(flex)
                .render(*area, buf);
        }
    }
}

impl Example {
    fn new<C>(constraints: C, description: String) -> Self
    where
        C: Into<Vec<Constraint>>,
    {
        Self {
            constraints: constraints.into(),
            description,
            flex: Flex::default(),
        }
    }

    fn flex(mut self, flex: Flex) -> Self {
        self.flex = flex;
        self
    }
}

impl Widget for Example {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if !self.description.is_empty() {
            let [title, _] = area.split(&Layout::vertical([Fixed(1), Proportional(0)]));
            Paragraph::new(
                format!("// {}", self.description)
                    .italic()
                    .fg(Color::from_str("#908caa").unwrap()),
            )
            .render(title, buf)
        }
        let blocks = Layout::horizontal(&self.constraints)
            .flex(self.flex)
            .split(area);

        for (block, constraint) in blocks.iter().zip(&self.constraints) {
            let [_, block] = block.split(&Layout::vertical([
                Fixed(SPACER_HEIGHT),
                Fixed(ILLUSTRATION_HEIGHT),
            ]));
            self.illustration(*constraint, block.width)
                .render(block, buf);
        }
    }
}

impl Example {
    fn illustration(&self, constraint: Constraint, width: u16) -> Paragraph {
        let (color, fg) = match constraint {
            Constraint::Fixed(_) => (FIXED_COLOR, Color::White),
            Constraint::Length(_) => (LENGTH_COLOR, Color::White),
            Constraint::Percentage(_) => (PERCENTAGE_COLOR, Color::White),
            Constraint::Ratio(_, _) => (RATIO_COLOR, Color::White),
            Constraint::Proportional(_) => (PROPORTIONAL_COLOR, Color::White),
            Constraint::Min(_) => (MIN_COLOR, Color::White),
            Constraint::Max(_) => (MAX_COLOR, Color::White),
        };
        let title = format!("{constraint}");
        let content = format!("{width} px");
        let text = format!("{title}\n{content}");
        let block = Block::bordered()
            .border_set(symbols::border::QUADRANT_OUTSIDE)
            .border_style(Style::reset().fg(color).reversed())
            .style(Style::default().fg(fg).bg(color));
        Paragraph::new(text)
            .alignment(Alignment::Center)
            .block(block)
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

    // Each line in the example is a layout
    // so EXAMPLE_HEIGHT * N_EXAMPLES_PER_TAB = 55 currently
    // Plus additional layout for tabs ...
    Layout::init_cache(50);

    Ok(terminal)
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

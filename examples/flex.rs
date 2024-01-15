use std::{error::Error, io, str::FromStr};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
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

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Each line in the example is a layout
    // so EXAMPLE_HEIGHT * N_EXAMPLES_PER_TAB = 55 currently
    // Plus additional layout for tabs ...
    Layout::init_cache(50);

    // create app and run it
    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    // we always want to show the last example when scrolling
    let mut app = App::default().max_scroll_offset((N_EXAMPLES_PER_TAB - 1) * EXAMPLE_HEIGHT);

    loop {
        terminal.draw(|f| f.render_widget(app, f.size()))?;

        if let Event::Key(key) = event::read()? {
            use KeyCode::*;
            match key.code {
                Char('q') => break Ok(()),
                Char('l') | Right => app.next(),
                Char('h') | Left => app.previous(),
                Char('j') | Down => app.down(),
                Char('k') | Up => app.up(),
                _ => (),
            }
        }
    }
}

#[derive(Default, Clone, Copy)]
struct App {
    selected_example: ExampleSelection,
    scroll_offset: u16,
    max_scroll_offset: u16,
}

impl App {
    fn max_scroll_offset(mut self, max_scroll_offset: u16) -> Self {
        self.max_scroll_offset = max_scroll_offset;
        self
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

    fn render_tabs_and_axis(&self, area: Rect, buf: &mut Buffer) {
        let [tabs, axis] = area.split(&Layout::vertical([
            Constraint::Fixed(3),
            Constraint::Fixed(3),
        ]));
        self.render_tabs(tabs, buf);
        self.render_axis(axis, buf);
    }

    fn render_axis(&self, area: Rect, buf: &mut Buffer) {
        let width = area.width as usize;
        // a bar like `<----- 80 px ----->`
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
        .block(
            Block::new()
                .title(Title::from("Flex Layouts ".bold()))
                .title(" Use h l or ◄ ► to change tab and j k or ▲ ▼  to scroll"),
        )
        .highlight_style(Style::default().bold())
        .select(self.selected_example.selected())
        .divider(" ")
        .padding("", "")
        .render(area, buf);
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
            StretchLast => "  StretchLast  ".bg(stretch_last).into(),
            Stretch => "  Stretch  ".bg(stretch).into(),
            Start => "  Start  ".bg(start).into(),
            Center => "  Center  ".bg(center).into(),
            End => "  End  ".bg(end).into(),
            SpaceAround => " SpaceAround  ".bg(space_around).into(),
            SpaceBetween => "  SpaceBetween  ".bg(space_between).into(),
        }
    }
}

impl Widget for App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [tabs_and_axis_area, demo_area] =
            area.split(&Layout::vertical([Fixed(6), Proportional(0)]));

        // render demo content into a separate buffer so all examples fit
        let mut demo_buf = Buffer::empty(Rect::new(
            0,
            0,
            buf.area.width,
            N_EXAMPLES_PER_TAB * EXAMPLE_HEIGHT,
        ));

        self.selected_example.render(demo_buf.area, &mut demo_buf);

        // render tabs into a separate buffer
        let mut tabs_and_axis_buf = Buffer::empty(tabs_and_axis_area);
        self.render_tabs_and_axis(tabs_and_axis_area, &mut tabs_and_axis_buf);

        // Assemble both buffers
        // NOTE: You shouldn't do this in a production app
        buf.content = tabs_and_axis_buf.content;
        buf.content.append(
            &mut demo_buf
                .content
                .into_iter()
                .skip((buf.area.width * self.scroll_offset) as usize)
                .take(demo_area.area() as usize)
                .collect(),
        );
        buf.resize(buf.area);
    }
}

#[derive(Default, Debug, Copy, Clone)]
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
            vec![Fixed(20), Min(20), Max(20)],
            vec![Length(20), Percentage(20), Ratio(1, 5), Proportional(1)],
            vec![Length(20), Fixed(20), Percentage(20)],
            vec![Fixed(20), Percentage(20), Length(20)],
            vec![Percentage(20), Length(20), Fixed(20)],
            vec![Length(20), Length(15)],
            vec![Length(20), Fixed(20)],
            vec![Min(20), Max(20)],
            vec![Max(20)],
            vec![Min(20), Max(20), Length(20), Fixed(20)],
            vec![Proportional(0), Proportional(0)],
            vec![Proportional(1), Proportional(1), Length(20), Fixed(20)],
            vec![
                Min(10),
                Proportional(3),
                Proportional(2),
                Length(15),
                Fixed(15),
            ],
        ];

        for (area, constraints) in areas.iter().zip(examples) {
            Example::new(constraints).flex(flex).render(*area, buf);
        }
    }
}

struct Example {
    constraints: Vec<Constraint>,
    flex: Flex,
}

impl Example {
    fn new<C>(constraints: C) -> Self
    where
        C: Into<Vec<Constraint>>,
    {
        Self {
            constraints: constraints.into(),
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
        let blocks = Layout::horizontal(&self.constraints)
            .flex(self.flex)
            .split(area);

        for (block, constraint) in blocks.iter().zip(&self.constraints) {
            let [block, _] = block.split(&Layout::vertical([
                Fixed(ILLUSTRATION_HEIGHT),
                Fixed(SPACER_HEIGHT),
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

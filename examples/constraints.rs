use std::{error::Error, io};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{layout::Constraint::*, prelude::*, widgets::*};

const SPACER_HEIGHT: u16 = 1;
const ILLUSTRATION_HEIGHT: u16 = 3;
const EXAMPLE_HEIGHT: u16 = ILLUSTRATION_HEIGHT + SPACER_HEIGHT;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Each line in the example is a layout
    // There is on average 4 row per example
    // 4 row * 7 example = 28
    // Plus additional layout for tabs ...
    // Examples might also grow in a very near future
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
    let mut app = App::default();
    app.update_max_scroll_offset();

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
    fn update_max_scroll_offset(&mut self) {
        self.max_scroll_offset = (self.selected_example.get_example_count() - 1) * EXAMPLE_HEIGHT;
    }
    fn next(&mut self) {
        self.selected_example = self.selected_example.next();
        self.update_max_scroll_offset();
        self.scroll_offset = 0;
    }
    fn previous(&mut self) {
        self.selected_example = self.selected_example.previous();
        self.update_max_scroll_offset();
        self.scroll_offset = 0;
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
        // ┌Constraints───────────────────────────────────────────────────────────────────┐
        // │  Fixed  │  Length  │  Percentage  │  Ratio  │  Proportional  │  Min  │  Max  │
        // └──────────────────────────────────────────────────────────────────────────────┘
        Tabs::new(
            [
                ExampleSelection::Fixed,
                ExampleSelection::Length,
                ExampleSelection::Percentage,
                ExampleSelection::Ratio,
                ExampleSelection::Proportional,
                ExampleSelection::Min,
                ExampleSelection::Max,
            ]
            .iter()
            .map(|e| format!("{:?}", e)),
        )
        .block(
            Block::bordered()
                .title("Constraints ".bold())
                .title(" Use h l or ◄ ► to change tab and j k or ▲ ▼  to scroll".bold()),
        )
        .highlight_style(Style::default().yellow().bold())
        .select(self.selected_example.selected())
        .padding("  ", "  ")
        .render(area, buf);
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
            self.selected_example.get_example_count() * EXAMPLE_HEIGHT + tabs_and_axis_area.height,
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
    Fixed,
    Length,
    Percentage,
    Ratio,
    Proportional,
    Min,
    Max,
}

impl ExampleSelection {
    fn previous(&self) -> Self {
        use ExampleSelection::*;
        match *self {
            Fixed => Fixed,
            Length => Fixed,
            Percentage => Length,
            Ratio => Percentage,
            Proportional => Ratio,
            Min => Proportional,
            Max => Min,
        }
    }

    fn next(&self) -> Self {
        use ExampleSelection::*;
        match *self {
            Fixed => Length,
            Length => Percentage,
            Percentage => Ratio,
            Ratio => Proportional,
            Proportional => Min,
            Min => Max,
            Max => Max,
        }
    }

    fn selected(&self) -> usize {
        use ExampleSelection::*;
        match self {
            Fixed => 0,
            Length => 1,
            Percentage => 2,
            Ratio => 3,
            Proportional => 4,
            Min => 5,
            Max => 6,
        }
    }

    fn get_example_count(&self) -> u16 {
        use ExampleSelection::*;
        match self {
            Fixed => 2,
            Length => 4,
            Percentage => 5,
            Ratio => 4,
            Proportional => 2,
            Min => 5,
            Max => 5,
        }
    }
}

impl Widget for ExampleSelection {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            ExampleSelection::Fixed => self.render_fixed_example(area, buf),
            ExampleSelection::Length => self.render_length_example(area, buf),
            ExampleSelection::Percentage => self.render_percentage_example(area, buf),
            ExampleSelection::Ratio => self.render_ratio_example(area, buf),
            ExampleSelection::Proportional => self.render_proportional_example(area, buf),
            ExampleSelection::Min => self.render_min_example(area, buf),
            ExampleSelection::Max => self.render_max_example(area, buf),
        }
    }
}

impl ExampleSelection {
    fn render_fixed_example(&self, area: Rect, buf: &mut Buffer) {
        let [example1, example2, _] = area.split(&Layout::vertical([Fixed(EXAMPLE_HEIGHT); 3]));

        Example::new([Fixed(40), Proportional(0)]).render(example1, buf);

        Example::new([Fixed(20), Fixed(20), Proportional(0)]).render(example2, buf);
    }

    fn render_length_example(&self, area: Rect, buf: &mut Buffer) {
        let [example1, example2, example3, example4, _] =
            area.split(&Layout::vertical([Fixed(EXAMPLE_HEIGHT); 5]));

        Example::new([Length(20), Fixed(20)]).render(example1, buf);

        Example::new([Length(20), Length(20)]).render(example2, buf);

        Example::new([Length(20), Min(20)]).render(example3, buf);

        Example::new([Length(20), Max(20)]).render(example4, buf);
    }

    fn render_percentage_example(&self, area: Rect, buf: &mut Buffer) {
        let [example1, example2, example3, example4, example5, _] =
            area.split(&Layout::vertical([Fixed(EXAMPLE_HEIGHT); 6]));

        Example::new([Percentage(75), Proportional(0)]).render(example1, buf);

        Example::new([Percentage(25), Proportional(0)]).render(example2, buf);

        Example::new([Percentage(50), Min(20)]).render(example3, buf);

        Example::new([Percentage(0), Max(0)]).render(example4, buf);

        Example::new([Percentage(0), Proportional(0)]).render(example5, buf);
    }

    fn render_ratio_example(&self, area: Rect, buf: &mut Buffer) {
        let [example1, example2, example3, example4, _] =
            area.split(&Layout::vertical([Fixed(EXAMPLE_HEIGHT); 5]));

        Example::new([Ratio(1, 2); 2]).render(example1, buf);

        Example::new([Ratio(1, 4); 4]).render(example2, buf);

        Example::new([Ratio(1, 2), Ratio(1, 3), Ratio(1, 4)]).render(example3, buf);

        Example::new([Ratio(1, 2), Percentage(25), Length(10)]).render(example4, buf);
    }

    fn render_proportional_example(&self, area: Rect, buf: &mut Buffer) {
        let [example1, example2, _] = area.split(&Layout::vertical([Fixed(EXAMPLE_HEIGHT); 3]));

        Example::new([Proportional(1), Proportional(2), Proportional(3)]).render(example1, buf);

        Example::new([Proportional(1), Percentage(50), Proportional(1)]).render(example2, buf);
    }

    fn render_min_example(&self, area: Rect, buf: &mut Buffer) {
        let [example1, example2, example3, example4, example5, _] =
            area.split(&Layout::vertical([Fixed(EXAMPLE_HEIGHT); 6]));

        Example::new([Percentage(100), Min(0)]).render(example1, buf);

        Example::new([Percentage(100), Min(20)]).render(example2, buf);

        Example::new([Percentage(100), Min(40)]).render(example3, buf);

        Example::new([Percentage(100), Min(60)]).render(example4, buf);

        Example::new([Percentage(100), Min(80)]).render(example5, buf);
    }

    fn render_max_example(&self, area: Rect, buf: &mut Buffer) {
        let [example1, example2, example3, example4, example5, _] =
            area.split(&Layout::vertical([Fixed(EXAMPLE_HEIGHT); 6]));

        Example::new([Percentage(0), Max(0)]).render(example1, buf);

        Example::new([Percentage(0), Max(20)]).render(example2, buf);

        Example::new([Percentage(0), Max(40)]).render(example3, buf);

        Example::new([Percentage(0), Max(60)]).render(example4, buf);

        Example::new([Percentage(0), Max(80)]).render(example5, buf);
    }
}

struct Example {
    constraints: Vec<Constraint>,
}

impl Example {
    fn new<C>(constraints: C) -> Self
    where
        C: Into<Vec<Constraint>>,
    {
        Self {
            constraints: constraints.into(),
        }
    }
}

impl Widget for Example {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [area, _] = area.split(&Layout::vertical([
            Fixed(ILLUSTRATION_HEIGHT),
            Fixed(SPACER_HEIGHT),
        ]));
        let blocks = Layout::horizontal(&self.constraints).split(area);

        for (block, constraint) in blocks.iter().zip(&self.constraints) {
            let text = format!("{} px", block.width);
            let fg = match constraint {
                Constraint::Ratio(_, _) => Color::Indexed(1),
                Constraint::Percentage(_) => Color::Indexed(2),
                Constraint::Max(_) => Color::Indexed(3),
                Constraint::Min(_) => Color::Indexed(4),
                Constraint::Length(_) => Color::Indexed(5),
                Constraint::Fixed(_) => Color::Indexed(6),
                Constraint::Proportional(_) => Color::Indexed(7),
            };
            self.illustration(*constraint, text, fg).render(*block, buf);
        }
    }
}

impl Example {
    fn illustration(&self, constraint: Constraint, text: String, fg: Color) -> Paragraph {
        Paragraph::new(format!(" {constraint} ").fg(fg))
            .style(Color::DarkGray)
            .alignment(Alignment::Center)
            .block(
                Block::bordered()
                    .style(Style::default().fg(Color::DarkGray))
                    .title(
                        block::Title::from(format!("{text}"))
                            .alignment(Alignment::Right)
                            .position(block::Position::Bottom),
                    ),
            )
    }
}

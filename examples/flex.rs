use std::{error::Error, io};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Constraint::*, Flex},
    prelude::*,
    widgets::{block::Title, *},
};

const EXAMPLE_HEIGHT: u16 = 5;
const N_EXAMPLES_PER_TAB: u16 = 11;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

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

    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        Tabs::new(
            [
                ExampleSelection::Stretch,
                ExampleSelection::StretchLast,
                ExampleSelection::Start,
                ExampleSelection::Center,
                ExampleSelection::End,
                ExampleSelection::SpaceAround,
                ExampleSelection::SpaceBetween,
            ]
            .iter()
            .map(|e| format!("{:?}", e)),
        )
        .block(
            Block::bordered()
                .title(Title::from("Flex Layouts ".bold()))
                .title(" Use h l or ◄ ► to change tab and j k or ▲ ▼  to scroll".bold()),
        )
        .highlight_style(Style::default().yellow().bold())
        .select(self.selected_example.selected())
        .padding(" ", " ")
        .render(area, buf);
    }
}

impl Widget for App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [tabs_area, demo_area] = area.split(&Layout::vertical([Fixed(3), Proportional(0)]));

        // render demo content into a separate buffer so all examples fit
        let mut demo_buf = Buffer::empty(Rect::new(
            0,
            0,
            buf.area.width,
            N_EXAMPLES_PER_TAB * EXAMPLE_HEIGHT,
        ));

        self.selected_example.render(demo_buf.area, &mut demo_buf);

        // render tabs into a separate buffer
        let mut tabs_buf = Buffer::empty(tabs_area);
        self.render_tabs(tabs_area, &mut tabs_buf);

        // Assemble both buffers
        // NOTE: You shouldn't do this in a production app
        buf.content = tabs_buf.content;
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
    Stretch,
    StretchLast,
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
            Stretch => Stretch,
            StretchLast => Stretch,
            Start => StretchLast,
            Center => Start,
            End => Center,
            SpaceAround => End,
            SpaceBetween => SpaceAround,
        }
    }

    fn next(&self) -> Self {
        use ExampleSelection::*;
        match *self {
            Stretch => StretchLast,
            StretchLast => Start,
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
            Stretch => 0,
            StretchLast => 1,
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
            ExampleSelection::Stretch => self.render_example(area, buf, Flex::Stretch),
            ExampleSelection::StretchLast => self.render_example(area, buf, Flex::StretchLast),
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
        let [legend, area] = area.split(&Layout::vertical([Ratio(1, 3); 2]));
        let blocks = Layout::horizontal(&self.constraints)
            .flex(self.flex)
            .split(area);

        self.legend(legend.width as usize).render(legend, buf);

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
    fn legend(&self, width: usize) -> Paragraph {
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
    }

    fn illustration(&self, constraint: Constraint, text: String, fg: Color) -> Paragraph {
        Paragraph::new(format!("{:?}", constraint))
            .alignment(Alignment::Center)
            .block(
                Block::bordered()
                    .style(Style::default().fg(fg))
                    .title(block::Title::from(text).alignment(Alignment::Center)),
            )
    }
}

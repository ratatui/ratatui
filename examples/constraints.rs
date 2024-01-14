use std::{error::Error, io};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use itertools::Itertools;
use ratatui::{layout::Constraint::*, prelude::*, widgets::*};

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
    let mut selection = ExampleSelection::Fixed;
    loop {
        terminal.draw(|f| f.render_widget(selection, f.size()))?;

        if let Event::Key(key) = event::read()? {
            use KeyCode::*;
            match key.code {
                Char('q') => break Ok(()),
                Char('j') | Char('l') | Down | Right => {
                    selection = selection.next();
                }
                Char('k') | Char('h') | Up | Left => {
                    selection = selection.previous();
                }
                _ => (),
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum ExampleSelection {
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
}

impl Widget for ExampleSelection {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [tabs, area] = area.split(&Layout::vertical([Fixed(3), Proportional(0)]));
        let [area, _] = area.split(&Layout::horizontal([Fixed(80), Proportional(0)]));

        self.render_tabs(tabs, buf);

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
        .block(Block::bordered().title("Constraints"))
        .highlight_style(Style::default().yellow())
        .select(self.selected())
        .padding("  ", "  ")
        .render(area, buf);
    }

    fn render_fixed_example(&self, area: Rect, buf: &mut Buffer) {
        let [example1, example2, _] = area.split(&Layout::vertical([Fixed(8); 3]));

        // Fixed(40), Proportional(0)
        //
        // <---------------------50 px---------------------->
        //
        // ┌──────────────────────────────────────┐┌────────┐
        // │                 40 px                ││  10 px │
        // └──────────────────────────────────────┘└────────┘
        Example::new([Fixed(40), Proportional(0)]).render(example1, buf);

        // Fixed(20), Fixed(20), Proportional(0)
        //
        // <---------------------50 px---------------------->
        //
        // ┌──────────────────┐┌──────────────────┐┌────────┐
        // │       20 px      ││       20 px      ││  10 px │
        // └──────────────────┘└──────────────────┘└────────┘
        Example::new([Fixed(20), Fixed(20), Proportional(0)]).render(example2, buf);
    }

    fn render_length_example(&self, area: Rect, buf: &mut Buffer) {
        let [example1, example2, example3, example4, _] =
            area.split(&Layout::vertical([Fixed(8); 5]));

        // Length(20), Fixed(20)
        //
        // <---------------------50 px---------------------->
        //
        // ┌────────────────────────────┐┌──────────────────┐
        // │            30 px           ││       20 px      │
        // └────────────────────────────┘└──────────────────┘
        Example::new([Length(20), Fixed(20)]).render(example1, buf);

        // Length(20), Length(20)
        //
        // <---------------------50 px---------------------->
        //
        // ┌──────────────────┐┌────────────────────────────┐
        // │       20 px      ││            30 px           │
        // └──────────────────┘└────────────────────────────┘
        Example::new([Length(20), Length(20)]).render(example2, buf);

        // Length(20), Min(20)
        //
        // <---------------------50 px---------------------->
        //
        // ┌──────────────────┐┌────────────────────────────┐
        // │       20 px      ││            30 px           │
        // └──────────────────┘└────────────────────────────┘
        Example::new([Length(20), Min(20)]).render(example3, buf);

        // Length(20), Max(20)
        //
        // <---------------------50 px---------------------->
        //
        // ┌────────────────────────────┐┌──────────────────┐
        // │            30 px           ││       20 px      │
        // └────────────────────────────┘└──────────────────┘
        Example::new([Length(20), Max(20)]).render(example4, buf);
    }

    fn render_percentage_example(&self, area: Rect, buf: &mut Buffer) {
        let [example1, example2, example3, example4, example5, _] =
            area.split(&Layout::vertical([Fixed(8); 6]));

        // Percentage(75), Proportional(0)
        //
        // <---------------------50 px---------------------->
        //
        // ┌────────────────────────────────────────────────┐
        // │                      50 px                     │
        // └────────────────────────────────────────────────┘
        //
        Example::new([Percentage(75), Proportional(0)]).render(example1, buf);

        // Percentage(25), Proportional(0)
        //
        // <---------------------50 px---------------------->
        //
        // ┌────────────────────────────────────────────────┐
        // │                      50 px                     │
        // └────────────────────────────────────────────────┘
        Example::new([Percentage(25), Proportional(0)]).render(example2, buf);

        // Percentage(50), Min(20)
        //
        // <---------------------50 px---------------------->
        //
        // ┌───────────────────────┐┌───────────────────────┐
        // │         25 px         ││         25 px         │
        // └───────────────────────┘└───────────────────────┘
        Example::new([Percentage(50), Min(20)]).render(example3, buf);

        // Percentage(0), Max(0)
        //
        // <---------------------50 px---------------------->
        //
        // ┌────────────────────────────────────────────────┐
        // │                      50 px                     │
        // └────────────────────────────────────────────────┘
        Example::new([Percentage(0), Max(0)]).render(example4, buf);

        // Percentage(0), Proportional(0)
        //
        // <---------------------50 px---------------------->
        //
        // ┌────────────────────────────────────────────────┐
        // │                      50 px                     │
        // └────────────────────────────────────────────────┘
        Example::new([Percentage(0), Proportional(0)]).render(example5, buf);
    }

    fn render_ratio_example(&self, area: Rect, buf: &mut Buffer) {
        let [example1, example2, example3, example4, _] =
            area.split(&Layout::vertical([Fixed(8); 5]));

        // Ratio(1, 2), Ratio(1, 2)
        //
        // <---------------------50 px---------------------->
        //
        // ┌───────────────────────┐┌───────────────────────┐
        // │         25 px         ││         25 px         │
        // └───────────────────────┘└───────────────────────┘
        Example::new([Ratio(1, 2); 2]).render(example1, buf);

        // Ratio(1, 4), Ratio(1, 4), Ratio(1, 4), Ratio(1, 4)
        //
        // <---------------------50 px---------------------->
        //
        // ┌───────────┐┌──────────┐┌───────────┐┌──────────┐
        // │   13 px   ││   12 px  ││   13 px   ││   12 px  │
        // └───────────┘└──────────┘└───────────┘└──────────┘
        Example::new([Ratio(1, 4); 4]).render(example2, buf);

        // Ratio(1, 2), Ratio(1, 3), Ratio(1, 4)
        //
        // <---------------------50 px---------------------->
        //
        // ┌───────────────────────┐┌───────────────┐┌──────┐
        // │         25 px         ││     17 px     ││ 8 px │
        // └───────────────────────┘└───────────────┘└──────┘
        Example::new([Ratio(1, 2), Ratio(1, 3), Ratio(1, 4)]).render(example3, buf);

        // Ratio(1, 2), Percentage(25), Length(10)
        //
        // <---------------------50 px---------------------->
        //
        // ┌───────────────────────┐┌───────────┐┌──────────┐
        // │         25 px         ││   13 px   ││   12 px  │
        // └───────────────────────┘└───────────┘└──────────┘
        Example::new([Ratio(1, 2), Percentage(25), Length(10)]).render(example4, buf);
    }

    fn render_proportional_example(&self, area: Rect, buf: &mut Buffer) {
        let [example1, example2, _] = area.split(&Layout::vertical([Fixed(8); 3]));

        // Proportional(1), Proportional(2), Proportional(3)
        //
        // <---------------------50 px---------------------->
        //
        // ┌──────┐┌───────────────┐┌───────────────────────┐
        // │ 8 px ││     17 px     ││         25 px         │
        // └──────┘└───────────────┘└───────────────────────┘
        Example::new([Proportional(1), Proportional(2), Proportional(3)]).render(example1, buf);

        // Proportional(1), Percentage(50), Proportional(1)
        //
        // <---------------------50 px---------------------->
        //
        // ┌───────────┐┌───────────────────────┐┌──────────┐
        // │   13 px   ││         25 px         ││   12 px  │
        // └───────────┘└───────────────────────┘└──────────┘
        Example::new([Proportional(1), Percentage(50), Proportional(1)]).render(example2, buf);
    }

    fn render_min_example(&self, area: Rect, buf: &mut Buffer) {
        let [example1, example2, example3, example4, example5, _] =
            area.split(&Layout::vertical([Fixed(8); 6]));
        // Percentage(100), Min(0)
        //
        // <---------------------50 px---------------------->
        //
        // ┌────────────────────────────────────────────────┐
        // │                      50 px                     │
        // └────────────────────────────────────────────────┘
        Example::new([Percentage(100), Min(0)]).render(example1, buf);

        // Percentage(100), Min(20)
        //
        // <---------------------50 px---------------------->
        //
        // ┌────────────────────────────┐┌──────────────────┐
        // │            30 px           ││       20 px      │
        // └────────────────────────────┘└──────────────────┘
        Example::new([Percentage(100), Min(20)]).render(example2, buf);

        // Percentage(100), Min(40)
        //
        // <---------------------50 px---------------------->
        //
        // ┌────────┐┌──────────────────────────────────────┐
        // │  10 px ││                 40 px                │
        // └────────┘└──────────────────────────────────────┘
        Example::new([Percentage(100), Min(40)]).render(example3, buf);

        // Percentage(100), Min(60)
        //
        // <---------------------50 px---------------------->
        //
        // ┌────────────────────────────────────────────────┐
        // │                      50 px                     │
        // └────────────────────────────────────────────────┘
        Example::new([Percentage(100), Min(60)]).render(example4, buf);

        // Percentage(100), Min(80)
        //
        // <---------------------50 px---------------------->
        //
        // ┌────────────────────────────────────────────────┐
        // │                      50 px                     │
        // └────────────────────────────────────────────────┘
        Example::new([Percentage(100), Min(80)]).render(example5, buf);
    }

    fn render_max_example(&self, area: Rect, buf: &mut Buffer) {
        let [example1, example2, example3, example4, example5, _] =
            area.split(&Layout::vertical([Fixed(8); 6]));

        // Percentage(0), Max(0)
        //
        // <---------------------50 px---------------------->
        //
        // ┌────────────────────────────────────────────────┐
        // │                      50 px                     │
        // └────────────────────────────────────────────────┘
        Example::new([Percentage(0), Max(0)]).render(example1, buf);

        //
        // Percentage(0), Max(20)
        //
        // <---------------------50 px---------------------->
        //
        // ┌────────────────────────────┐┌──────────────────┐
        // │            30 px           ││       20 px      │
        // └────────────────────────────┘└──────────────────┘
        Example::new([Percentage(0), Max(20)]).render(example2, buf);

        // Percentage(0), Max(40)
        //
        // <---------------------50 px---------------------->
        //
        // ┌────────┐┌──────────────────────────────────────┐
        // │  10 px ││                 40 px                │
        // └────────┘└──────────────────────────────────────┘
        Example::new([Percentage(0), Max(40)]).render(example3, buf);

        // Percentage(0), Max(60)
        //
        // <---------------------50 px---------------------->
        //
        // ┌────────────────────────────────────────────────┐
        // │                      50 px                     │
        // └────────────────────────────────────────────────┘
        Example::new([Percentage(0), Max(60)]).render(example4, buf);

        // Percentage(0), Max(80)
        //
        // <---------------------50 px---------------------->
        //
        // ┌────────────────────────────────────────────────┐
        // │                      50 px                     │
        // └────────────────────────────────────────────────┘
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
        let [title, legend, area] = area.split(&Layout::vertical([Ratio(1, 3); 3]));
        let blocks = Layout::horizontal(&self.constraints).split(area);

        self.heading().render(title, buf);

        self.legend(legend.width as usize).render(legend, buf);

        for (i, block) in blocks.iter().enumerate() {
            let text = format!("{} px", block.width);
            let fg = Color::Indexed(i as u8 + 1);
            self.illustration(text, fg).render(*block, buf);
        }
    }
}

impl Example {
    fn heading(&self) -> Paragraph {
        // Renders the following
        //
        // Fixed(40), Proportional(0)
        let spans = self.constraints.iter().enumerate().map(|(i, c)| {
            let color = Color::Indexed(i as u8 + 1);
            Span::styled(format!("{:?}", c), color)
        });
        let heading =
            Line::from(Itertools::intersperse(spans, Span::raw(", ")).collect::<Vec<Span>>());
        Paragraph::new(heading).block(Block::default().padding(Padding::vertical(1)))
    }

    fn legend(&self, width: usize) -> Paragraph {
        // a bar like `<----- 80 px ----->`
        let width_label = format!("{} px", width);
        let width_bar = format!(
            "<{width_label:-^width$}>",
            width = width - width_label.len() / 2
        );
        Paragraph::new(width_bar.dark_gray()).alignment(Alignment::Center)
    }

    fn illustration(&self, text: String, fg: Color) -> Paragraph {
        // Renders the following
        //
        // ┌─────────┐┌─────────┐
        // │  40 px  ││  40 px  │
        // └─────────┘└─────────┘
        Paragraph::new(text)
            .alignment(Alignment::Center)
            .block(Block::bordered().style(Style::default().fg(fg)))
    }
}

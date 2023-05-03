use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Borders, Gauge, List, ListItem, ListState, Paragraph, Widget,
        WidgetWrapper,
    },
    Terminal,
};
use std::{error::Error, io, time::Duration};

const TICK_RATE: Duration = Duration::from_millis(250);

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;
    let app = App::new(&mut terminal, TICK_RATE);
    app.run()?;
    restore_terminal(terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(
    mut terminal: Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

struct App<'a, B>
where
    B: Backend,
{
    terminal: &'a mut Terminal<B>,
    tick_rate: Duration,
    list: WidgetList<'a>,
}

impl<'a, B> App<'a, B>
where
    B: Backend,
{
    pub fn new(terminal: &'a mut Terminal<B>, tick_rate: Duration) -> Self {
        let list = WidgetList::with_items(vec![
            WidgetListItem::String("string"),
            WidgetListItem::Raw("raw"),
            WidgetListItem::Styled("styled", Style::default().fg(Color::Red)),
            WidgetListItem::Text(Text::from("text")),
            WidgetListItem::Paragraph(
                Paragraph::new("paragraph").block(
                    Block::default()
                        .title("paragraph block")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Red)),
                ),
            ),
            WidgetListItem::Block(
                Block::default()
                    .title("block")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .border_style(Style::default().fg(Color::Red)),
            ),
            WidgetListItem::Book(BookWidget::new(
                "The Rust Programming Language",
                "Steve Klabnik and Carol Nichols",
                "https://doc.rust-lang.org/stable/book/",
            )),
            WidgetListItem::Gauge(50),
        ]);
        Self {
            terminal,
            tick_rate,
            list,
        }
    }
    pub fn run(mut self) -> Result<(), Box<dyn Error>> {
        loop {
            self.draw()?;
            if !crossterm::event::poll(self.tick_rate)? {
                continue;
            }
            match crossterm::event::read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => self.list.next(),
                    KeyCode::Up => self.list.previous(),
                    KeyCode::Esc => self.list.unselect(),
                    _ => {}
                },
                _ => {}
            }
        }
        Ok(())
    }

    fn draw(&mut self) -> Result<(), Box<dyn Error>> {
        self.terminal.draw(|frame| {
            let items = self
                .list
                .items
                .iter()
                .map(|i| match i {
                    // These are just testing the backwards compatibility of the ListItem
                    WidgetListItem::String(s) => ListItem::new(*s),
                    WidgetListItem::Raw(s) => ListItem::new(Text::raw(*s)),
                    WidgetListItem::Styled(s, style) => ListItem::new(Text::styled(*s, *style)),
                    WidgetListItem::Text(t) => ListItem::new(t.clone()),
                    // We don't recommend this approach, but it is possible to store a widget in
                    // a list item and then render it in the render closure
                    WidgetListItem::Paragraph(p) => {
                        ListItem::new(WidgetWrapper::new(p.clone(), 3, 10))
                    }
                    WidgetListItem::Block(b) => ListItem::new(WidgetWrapper::new(b.clone(), 3, 10)),
                    WidgetListItem::Book(b) => ListItem::new(WidgetWrapper::new(b.clone(), 5, 10)),
                    // Creating the widget in the render closure is the recommended approach
                    WidgetListItem::Gauge(g) => {
                        let guage = Gauge::default()
                            .block(Block::default().borders(Borders::ALL).title("Progress"))
                            .gauge_style(
                                Style::default()
                                    .fg(Color::White)
                                    .bg(Color::Black)
                                    .add_modifier(Modifier::ITALIC),
                            )
                            .percent(*g);
                        ListItem::new(WidgetWrapper::new(guage, 3, 10))
                    }
                })
                .collect::<Vec<ListItem>>();
            let list = List::new(items).highlight_symbol(">>").block(
                Block::default()
                    .title("Widget List Example")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            );
            frame.render_stateful_widget(list, frame.size(), &mut self.list.state);
        })?;
        Ok(())
    }
}

struct WidgetList<'a> {
    pub state: ListState,
    pub items: Vec<WidgetListItem<'a>>,
}

enum WidgetListItem<'a> {
    String(&'a str),
    Raw(&'a str),
    Styled(&'a str, Style),
    Text(Text<'a>),
    Paragraph(Paragraph<'a>),
    Block(Block<'a>),
    Book(BookWidget<'a>),
    Gauge(u16),
}

struct BookWidget<'a> {
    title: &'a str,
    author: &'a str,
    url: &'a str,
}

impl<'a> BookWidget<'a> {
    pub fn new(title: &'a str, author: &'a str, url: &'a str) -> Self {
        Self { title, author, url }
    }
}

impl Widget for &BookWidget<'_> {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        let contents = vec![
            Line::from(vec![
                Span::raw("Title: "),
                Span::styled(self.title, Style::default()),
            ]),
            Line::from(vec![
                Span::raw("Author: "),
                Span::styled(self.author, Style::default()),
            ]),
            Line::from(vec![
                Span::raw("URL: "),
                Span::styled(
                    self.url,
                    Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::UNDERLINED),
                ),
            ]),
        ];
        Paragraph::new(contents)
            .block(
                Block::default()
                    .title(self.title)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .render(area, buf);
    }
}

impl<'a> WidgetList<'a> {
    fn with_items(items: Vec<WidgetListItem<'a>>) -> Self {
        Self {
            state: ListState::default(),
            items: items,
        }
    }

    /// select next item wrapping around
    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => (i + 1) % self.items.len(),
            None => 0,
        };
        self.state.select(Some(i));
    }

    // select previous item wrapping around
    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => (i + self.items.len() - 1) % self.items.len(),
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

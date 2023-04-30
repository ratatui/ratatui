use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        BarChart, Block, Borders, Gauge, ListItem, Paragraph, WidgetList, WidgetListItem3,
        WidgetListState, Wrap,
    },
    Frame, Terminal,
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

struct StatefulList<T> {
    state: WidgetListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: WidgetListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    /*fn unselect(&mut self) {
        self.state.select(None);
    }*/
}

const LIST_COUNT: usize = 7;
const EDITABLE_PARAGRAPH_LIST_INDEX: usize = 6;

enum WidgetListItemData<'a> {
    Paragraph((&'a str, usize)),
    Gauge(&'a str),
    BarChart(Vec<(&'a str, u64)>),
}

type WidgetCustomItem<'a> = WidgetListItem3<Paragraph<'a>, Gauge<'a>, BarChart<'a>>;

macro_rules! is_selected {
    ($list:expr, $index:expr) => {
        if let Some(selected) = $list.state.selected() {
            $index == selected
        } else {
            false
        }
    };
}

/// This struct holds the current state of the app. In particular, it has the `items` field which is a wrapper
/// around `ListState`. Keeping track of the items state let us render the associated widget with its state
/// and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events.
/// Check the drawing logic for items on how to specify the highlighting style for selected items.
struct App<'a> {
    selected_list: usize,
    paragraphs: StatefulList<(&'a str, usize)>,
    texts: StatefulList<(&'a str, usize)>,
    gauges: StatefulList<&'a str>,
    barcharts: StatefulList<(&'a str, u64)>,
    widget_items: StatefulList<WidgetListItemData<'a>>,
    widget_items_filled: StatefulList<WidgetListItemData<'a>>,
    editable_paragraphs: StatefulList<String>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            paragraphs: StatefulList::with_items(vec![
                ("Item0", 6),
                ("Item1", 6),
                ("Item2", 1),
                ("Item3", 3),
                ("Item4", 1),
                ("Item5", 2),
                ("Item6", 1),
                ("Item7", 3),
                ("Item8", 1),
                ("Item9", 6),
                ("Item10", 1),
                ("Item11", 3),
                ("Item12", 1),
                ("Item13", 2),
                ("Item14", 1),
                ("Item15", 1),
                ("Item16", 4),
                ("Item17", 1),
                ("Item18", 5),
                ("Item19", 4),
                ("Item20", 1),
                ("Item21", 2),
                ("Item22", 1),
                ("Item23", 3),
                ("Item24", 1),
            ]),
            texts: StatefulList::with_items(vec![
                ("Item0", 6),
                ("Item1", 6),
                ("Item2", 1),
                ("Item3", 3),
                ("Item4", 1),
                ("Item5", 2),
                ("Item6", 1),
                ("Item7", 3),
                ("Item8", 1),
                ("Item9", 6),
                ("Item10", 1),
                ("Item11", 3),
                ("Item12", 1),
                ("Item13", 2),
                ("Item14", 1),
                ("Item15", 1),
                ("Item16", 4),
                ("Item17", 1),
                ("Item18", 5),
                ("Item19", 4),
                ("Item20", 1),
                ("Item21", 2),
                ("Item22", 1),
                ("Item23", 3),
                ("Item24", 1),
            ]),
            gauges: StatefulList::with_items(vec![
                "Event1", "Event2", "Event3", "Event4", "Event5", "Event6", "Event7", "Event8",
                "Event9", "Event10", "Event11", "Event12",
            ]),
            barcharts: StatefulList::with_items(vec![
                ("B1", 9),
                ("B2", 12),
                ("B3", 5),
                ("B4", 8),
                ("B5", 2),
                ("B6", 4),
                ("B7", 5),
                ("B8", 9),
                ("B9", 14),
                ("B10", 15),
                ("B11", 1),
                ("B12", 0),
                ("B13", 4),
                ("B14", 6),
            ]),
            widget_items: StatefulList::with_items(vec![
                WidgetListItemData::Paragraph(("title1", 3)),
                WidgetListItemData::Gauge("Gauge1"),
                WidgetListItemData::BarChart(vec![
                    ("B1", 9),
                    ("B2", 12),
                    ("B3", 5),
                    ("B4", 8),
                    ("B5", 2),
                    ("B6", 4),
                    ("B7", 5),
                    ("B8", 9),
                    ("B9", 14),
                    ("B10", 15),
                ]),
                WidgetListItemData::Paragraph(("title1", 6)),
                WidgetListItemData::Gauge("Gauge7"),
            ]),
            widget_items_filled: StatefulList::with_items(vec![
                WidgetListItemData::Paragraph(("title1", 3)),
                WidgetListItemData::Gauge("Gauge1"),
                WidgetListItemData::BarChart(vec![
                    ("B1", 9),
                    ("B2", 12),
                    ("B3", 5),
                    ("B4", 8),
                    ("B5", 2),
                    ("B6", 4),
                    ("B7", 5),
                    ("B8", 9),
                    ("B9", 14),
                    ("B10", 15),
                ]),
                WidgetListItemData::Paragraph(("title1", 6)),
                WidgetListItemData::Gauge("Gauge7"),
            ]),
            editable_paragraphs: StatefulList::with_items(vec![
                "write to edit this text 0".to_string(),
                "write to edit this text 1".to_string(),
                "write to edit this text 2".to_string(),
                "write to edit this text 3".to_string(),
                "write to edit this text 4".to_string(),
                "write to edit this text 5".to_string(),
            ]),
            selected_list: 0,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            if app.selected_list == EDITABLE_PARAGRAPH_LIST_INDEX {
                                if let Some(selected) = app.editable_paragraphs.state.selected() {
                                    app.editable_paragraphs.items[selected].push('q');
                                } else {
                                    return Ok(());
                                }
                            } else {
                                return Ok(());
                            }
                        }
                        KeyCode::Left => app.selected_list = app.selected_list.saturating_sub(1),
                        KeyCode::Right => {
                            if app.selected_list < LIST_COUNT - 1 {
                                app.selected_list += 1
                            }
                        }
                        KeyCode::Down => match app.selected_list {
                            0 => app.paragraphs.next(),
                            1 => app.texts.next(),
                            2 => app.gauges.next(),
                            3 => app.barcharts.next(),
                            4 => app.widget_items.next(),
                            5 => app.widget_items_filled.next(),
                            6 => app.editable_paragraphs.next(),
                            _ => {}
                        },
                        KeyCode::Up => match app.selected_list {
                            0 => app.paragraphs.previous(),
                            1 => app.texts.previous(),
                            2 => app.gauges.previous(),
                            3 => app.barcharts.previous(),
                            4 => app.widget_items.previous(),
                            5 => app.widget_items_filled.previous(),
                            6 => app.editable_paragraphs.previous(),
                            _ => {}
                        },
                        KeyCode::Char(ch) => {
                            if app.selected_list == EDITABLE_PARAGRAPH_LIST_INDEX {
                                if let Some(selected) = app.editable_paragraphs.state.selected() {
                                    app.editable_paragraphs.items[selected].push(ch);
                                }
                            }
                        }
                        KeyCode::Backspace => {
                            if app.selected_list == EDITABLE_PARAGRAPH_LIST_INDEX {
                                if let Some(selected) = app.editable_paragraphs.state.selected() {
                                    app.editable_paragraphs.items[selected].pop();
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Create two chunks with equal horizontal screen space
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let row0_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(vertical_chunks[0]);

    let row1_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(vertical_chunks[1]);

    create_paragraph_list(
        f,
        &mut app.paragraphs,
        row0_chunks[0],
        app.selected_list == 0,
    );
    create_text_list(f, &mut app.texts, row0_chunks[1], app.selected_list == 1);
    create_gauge_list(f, &mut app.gauges, row0_chunks[2], app.selected_list == 2);
    create_barchart_list(
        f,
        &mut app.barcharts,
        row0_chunks[3],
        app.selected_list == 3,
    );
    create_widget_item_list(
        f,
        &mut app.widget_items,
        row1_chunks[0],
        app.selected_list == 4,
        false,
    );
    create_widget_item_list(
        f,
        &mut app.widget_items_filled,
        row1_chunks[1],
        app.selected_list == 5,
        true,
    );

    create_editable_paragraph_list(
        f,
        &mut app.editable_paragraphs,
        row1_chunks[2],
        app.selected_list == EDITABLE_PARAGRAPH_LIST_INDEX,
    );
}

fn get_border_style(is_selected: bool) -> Style {
    if is_selected {
        Style::default().fg(Color::Red)
    } else {
        Style::default()
    }
}

fn create_paragraph<'a>(is_selected: bool, data: &'a (&str, usize)) -> Paragraph<'a> {
    let mut lines = vec![Line::from(data.0)];
    for index in 0..data.1 {
        lines.push(Line::from(Span::styled(
            format!(
                "[{}/{}] Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                index + 1,
                data.1
            ),
            Style::default().add_modifier(Modifier::ITALIC),
        )));
    }

    let mut style = Style::default().fg(Color::Black).bg(Color::White);
    if is_selected {
        style = style.bg(Color::LightGreen).add_modifier(Modifier::BOLD)
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .style(style)
}

fn create_paragraph_list<B: Backend>(
    f: &mut Frame<B>,
    list: &mut StatefulList<(&str, usize)>,
    area: Rect,
    is_selected: bool,
) {
    // Iterate through all elements in the `items` app and append some debug text to it.
    let items: Vec<Paragraph> = list
        .items
        .iter()
        .enumerate()
        .map(|(index, data)| create_paragraph(is_selected!(list, index), data))
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = WidgetList::new(items)
        .spacing(1)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("paragraphs")
                .border_style(get_border_style(is_selected)),
        )
        .highlight_symbol(">> ");

    // We can now render the item list
    f.render_stateful_widget(items, area, &mut list.state);
}

fn create_text_list<B: Backend>(
    f: &mut Frame<B>,
    list: &mut StatefulList<(&str, usize)>,
    area: Rect,
    is_selected: bool,
) {
    // Iterate through all elements in the `items` app and append some debug text to it.
    let items: Vec<ListItem> = list
        .items
        .iter()
        .enumerate()
        .map(|(index, data)| {
            let mut lines = vec![Line::from(data.0)];
            for index in 0..data.1 {
                lines.push(Line::from(Span::styled(
                    format!(
                        "[{}/{}] Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                        index + 1,
                        data.1
                    ),
                    Style::default().add_modifier(Modifier::ITALIC),
                )));
            }

            let mut style = Style::default().fg(Color::Black).bg(Color::White);
            if let Some(selected) = list.state.selected() {
                if index == selected {
                    style = style.bg(Color::LightGreen).add_modifier(Modifier::BOLD)
                }
            }

            ListItem::new(lines).style(style)
        })
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = WidgetList::new(items)
        .spacing(1)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("texts")
                .border_style(get_border_style(is_selected)),
        )
        .highlight_symbol(">> ")
        .repeat_highlight_symbol(true);

    // We can now render the item list
    f.render_stateful_widget(items, area, &mut list.state);
}

fn create_gauge<'a, T>(title: T, is_selected: bool, create_block: bool) -> Gauge<'a>
where
    T: Into<Line<'a>>,
{
    let mut gauge = Gauge::default()
        .gauge_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::ITALIC),
        )
        .percent(50);

    if is_selected || create_block {
        gauge = gauge
            .percent(100)
            .block(Block::default().title(title).borders(Borders::ALL));

        if is_selected {
            gauge = gauge.percent(100).gauge_style(
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::ITALIC),
            )
        }
    }

    gauge
}

fn create_gauge_list<B: Backend>(
    f: &mut Frame<B>,
    list: &mut StatefulList<&str>,
    area: Rect,
    is_selected: bool,
) {
    let gauges: Vec<Gauge> = list
        .items
        .iter()
        .enumerate()
        .map(|(index, &title)| create_gauge(title, is_selected!(list, index), false))
        .collect();

    let item_len = gauges.len();
    let gauge_list = WidgetList::new(gauges)
        .spacing(1)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("gauges")
                .border_style(get_border_style(is_selected)),
        )
        .item_heights(vec![Some(Constraint::Max(1)); item_len]); // Shrink
    f.render_stateful_widget(gauge_list, area, &mut list.state);
}

fn create_barchart<'a>(title: String, is_selected: bool, data: &'a [(&str, u64)]) -> BarChart<'a> {
    let mut barchart = BarChart::default()
        .block(Block::default().title(title).borders(Borders::ALL))
        .data(data)
        .bar_style(Style::default().fg(Color::Yellow))
        .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));

    if is_selected {
        barchart = barchart.bar_style(Style::default().fg(Color::LightGreen));
    }
    barchart
}

fn create_barchart_list<B: Backend>(
    f: &mut Frame<B>,
    list: &mut StatefulList<(&str, u64)>,
    area: Rect,
    is_selected: bool,
) {
    let charts: Vec<BarChart> = (0..15)
        .map(|index| {
            create_barchart(
                format!("Data{}", index),
                is_selected!(list, index),
                &list.items,
            )
        })
        .collect();

    let item_len = charts.len();
    let chart_list = WidgetList::new(charts)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Barcharts")
                .border_style(get_border_style(is_selected)),
        )
        .item_heights(vec![Some(Constraint::Max(5)); item_len]);
    f.render_stateful_widget(chart_list, area, &mut list.state);
}

fn create_widget_item_list<B: Backend>(
    f: &mut Frame<B>,
    list: &mut StatefulList<WidgetListItemData>,
    area: Rect,
    is_selected: bool,
    fill_content: bool,
) {
    let (charts, constraints): (Vec<WidgetCustomItem>, Vec<Option<Constraint>>) = list
        .items
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let is_selected = is_selected!(list, index);

            match item {
                WidgetListItemData::Paragraph(d) => (
                    WidgetCustomItem::One(create_paragraph(is_selected, d)),
                    None,
                ),
                WidgetListItemData::Gauge(d) => (
                    WidgetCustomItem::Two(create_gauge(*d, is_selected, true)),
                    Some(Constraint::Max(1)), // equivalent to Shrink
                ),
                WidgetListItemData::BarChart(d) => (
                    WidgetCustomItem::Three(create_barchart("Chart".to_string(), is_selected, d)),
                    Some(Constraint::Max(5)),
                ),
            }
        })
        .unzip();

    let mixed_list = WidgetList::new(charts)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Mixed")
                .border_style(get_border_style(is_selected)),
        )
        .item_heights(if fill_content {
            // Fill the view
            vec![Some(Constraint::Min(area.height.saturating_sub(2))); constraints.len()]
        } else {
            constraints
        });

    f.render_stateful_widget(mixed_list, area, &mut list.state);
}

fn create_editable_paragraph(is_selected: bool, data: &str) -> Paragraph {
    let lines = vec![if is_selected {
        Line::from(vec![
            Span::raw(data),
            Span::styled("█", Style::default().fg(Color::Red)),
        ])
    } else {
        Line::from(data)
    }];

    let mut style = Style::default().fg(Color::Black).bg(Color::White);
    if is_selected {
        style = style.bg(Color::LightGreen).add_modifier(Modifier::BOLD);
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .block(Block::default().borders(Borders::all()))
        .style(style)
}

fn create_editable_paragraph_list<B: Backend>(
    f: &mut Frame<B>,
    list: &mut StatefulList<String>,
    area: Rect,
    is_selected: bool,
) {
    // Iterate through all elements in the `items` app and append some debug text to it.
    let items: Vec<Paragraph> = list
        .items
        .iter()
        .enumerate()
        .map(|(index, data)| create_editable_paragraph(is_selected!(list, index), data))
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = WidgetList::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("editable paragraphs")
            .border_style(get_border_style(is_selected)),
    );

    // We can now render the item list
    f.render_stateful_widget(items, area, &mut list.state);
}

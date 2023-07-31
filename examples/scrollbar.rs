use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, symbols::scrollbar, widgets::*};

#[derive(Default)]
struct App {
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,
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
    let app = App::default();
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
        println!("{err:?}");
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
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('j') => {
                        app.vertical_scroll = app.vertical_scroll.saturating_add(1);
                        app.vertical_scroll_state = app
                            .vertical_scroll_state
                            .position(app.vertical_scroll as u16);
                    }
                    KeyCode::Char('k') => {
                        app.vertical_scroll = app.vertical_scroll.saturating_sub(1);
                        app.vertical_scroll_state = app
                            .vertical_scroll_state
                            .position(app.vertical_scroll as u16);
                    }
                    KeyCode::Char('h') => {
                        app.horizontal_scroll = app.horizontal_scroll.saturating_sub(1);
                        app.horizontal_scroll_state = app
                            .horizontal_scroll_state
                            .position(app.horizontal_scroll as u16);
                    }
                    KeyCode::Char('l') => {
                        app.horizontal_scroll = app.horizontal_scroll.saturating_add(1);
                        app.horizontal_scroll_state = app
                            .horizontal_scroll_state
                            .position(app.horizontal_scroll as u16);
                    }
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    // Words made "loooong" to demonstrate line breaking.
    let s = "Veeeeeeeeeeeeeeeery    loooooooooooooooooong   striiiiiiiiiiiiiiiiiiiiiiiiiing.   ";
    let mut long_line = s.repeat(usize::from(size.width) / s.len() + 4);
    long_line.push('\n');

    let block = Block::default().black();
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(size);

    let text = vec![
        Line::from("This is a line "),
        Line::from("This is a line   ".red()),
        Line::from("This is a line".on_dark_gray()),
        Line::from("This is a longer line".crossed_out()),
        Line::from(long_line.reset()),
        Line::from("This is a line".reset()),
        Line::from(vec![
            Span::raw("Masked text: "),
            Span::styled(
                Masked::new("password", '*'),
                Style::default().fg(Color::Red),
            ),
        ]),
        Line::from("This is a line "),
        Line::from("This is a line   ".red()),
        Line::from("This is a line".on_dark_gray()),
        Line::from("This is a longer line".crossed_out()),
        Line::from(long_line.reset()),
        Line::from("This is a line".reset()),
        Line::from(vec![
            Span::raw("Masked text: "),
            Span::styled(
                Masked::new("password", '*'),
                Style::default().fg(Color::Red),
            ),
        ]),
    ];
    app.vertical_scroll_state = app.vertical_scroll_state.content_length(text.len() as u16);
    app.horizontal_scroll_state = app
        .horizontal_scroll_state
        .content_length(long_line.len() as u16);

    let create_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            .gray()
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    };

    let title = Block::default()
        .title("Use h j k l to scroll ◄ ▲ ▼ ►")
        .title_alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let paragraph = Paragraph::new(text.clone())
        .gray()
        .block(create_block("Vertical scrollbar with arrows"))
        .scroll((app.vertical_scroll as u16, 0));
    f.render_widget(paragraph, chunks[1]);
    f.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        chunks[1],
        &mut app.vertical_scroll_state,
    );

    let paragraph = Paragraph::new(text.clone())
        .gray()
        .block(create_block(
            "Vertical scrollbar without arrows, without track symbol and mirrored",
        ))
        .scroll((app.vertical_scroll as u16, 0));
    f.render_widget(paragraph, chunks[2]);
    f.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalLeft)
            .symbols(scrollbar::VERTICAL)
            .begin_symbol(None)
            .track_symbol(None)
            .end_symbol(None),
        chunks[2].inner(&Margin {
            vertical: 1,
            horizontal: 0,
        }),
        &mut app.vertical_scroll_state,
    );

    let paragraph = Paragraph::new(text.clone())
        .gray()
        .block(create_block(
            "Horizontal scrollbar with only begin arrow & custom thumb symbol",
        ))
        .scroll((0, app.horizontal_scroll as u16));
    f.render_widget(paragraph, chunks[3]);
    f.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::HorizontalBottom)
            .thumb_symbol("🬋")
            .end_symbol(None),
        chunks[3].inner(&Margin {
            vertical: 0,
            horizontal: 1,
        }),
        &mut app.horizontal_scroll_state,
    );

    let paragraph = Paragraph::new(text.clone())
        .gray()
        .block(create_block(
            "Horizontal scrollbar without arrows & custom thumb and track symbol",
        ))
        .scroll((0, app.horizontal_scroll as u16));
    f.render_widget(paragraph, chunks[4]);
    f.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::HorizontalBottom)
            .thumb_symbol("░")
            .track_symbol(Some("─")),
        chunks[4].inner(&Margin {
            vertical: 0,
            horizontal: 1,
        }),
        &mut app.horizontal_scroll_state,
    );
}

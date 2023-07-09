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
use ratatui::{prelude::*, widgets::*};

struct App {
    scroll: u16,
}

impl App {
    fn new() -> App {
        App { scroll: 0 }
    }

    fn on_tick(&mut self) {
        self.scroll += 1;
        self.scroll %= 10;
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
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
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
        Line::from("This is a line".on_blue()),
        Line::from("This is a longer line".crossed_out()),
        Line::from(long_line.on_green()),
        Line::from("This is a line".green().italic()),
        Line::from(vec![
            "Masked text: ".into(),
            Span::styled(
                Masked::new("password", '*'),
                Style::default().fg(Color::Red),
            ),
        ]),
    ];

    let create_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Gray))
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    };

    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().fg(Color::Gray))
        .block(create_block("Default alignment (Left), no wrap"));
    f.render_widget(paragraph, chunks[0]);

    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().fg(Color::Gray))
        .block(create_block("Default alignment (Left), with wrap"))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[1]);

    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().fg(Color::Gray))
        .block(create_block("Right alignment, with wrap"))
        .alignment(Alignment::Right)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[2]);

    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::Gray))
        .block(create_block("Center alignment, with wrap, with scroll"))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .scroll((app.scroll, 0));
    f.render_widget(paragraph, chunks[3]);
}

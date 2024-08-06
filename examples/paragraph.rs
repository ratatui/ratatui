//! # [Ratatui] Paragraph example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui-org/ratatui
//! [examples]: https://github.com/ratatui-org/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui-org/ratatui/blob/main/examples/README.md

use std::{
    io::{self},
    time::{Duration, Instant},
};

use crossterm::event::KeyEventKind;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    text::{Line, Masked, Span},
    widgets::{Block, Paragraph, Widget, Wrap},
};

use self::common::{init_terminal, install_hooks, restore_terminal, Tui};

fn main() -> color_eyre::Result<()> {
    install_hooks()?;
    let mut terminal = init_terminal()?;
    let mut app = App::new();
    app.run(&mut terminal)?;
    restore_terminal()?;
    Ok(())
}

#[derive(Debug)]
struct App {
    should_exit: bool,
    scroll: u16,
    last_tick: Instant,
}

impl App {
    /// The duration between each tick.
    const TICK_RATE: Duration = Duration::from_millis(250);

    /// Create a new instance of the app.
    fn new() -> Self {
        Self {
            should_exit: false,
            scroll: 0,
            last_tick: Instant::now(),
        }
    }

    /// Run the app until the user exits.
    fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        while !self.should_exit {
            self.draw(terminal)?;
            self.handle_events()?;
            if self.last_tick.elapsed() >= Self::TICK_RATE {
                self.on_tick();
                self.last_tick = Instant::now();
            }
        }
        Ok(())
    }

    /// Draw the app to the terminal.
    fn draw(&mut self, terminal: &mut Tui) -> io::Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.area()))?;
        Ok(())
    }

    /// Handle events from the terminal.
    fn handle_events(&mut self) -> io::Result<()> {
        let timeout = Self::TICK_RATE.saturating_sub(self.last_tick.elapsed());
        while event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    self.should_exit = true;
                }
            }
        }
        Ok(())
    }

    /// Update the app state on each tick.
    fn on_tick(&mut self) {
        self.scroll = (self.scroll + 1) % 10;
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let areas = Layout::vertical([Constraint::Max(9); 4]).split(area);
        Paragraph::new(create_lines(area))
            .block(title_block("Default alignment (Left), no wrap"))
            .gray()
            .render(areas[0], buf);
        Paragraph::new(create_lines(area))
            .block(title_block("Default alignment (Left), with wrap"))
            .gray()
            .wrap(Wrap { trim: true })
            .render(areas[1], buf);
        Paragraph::new(create_lines(area))
            .block(title_block("Right alignment, with wrap"))
            .gray()
            .right_aligned()
            .wrap(Wrap { trim: true })
            .render(areas[2], buf);
        Paragraph::new(create_lines(area))
            .block(title_block("Center alignment, with wrap, with scroll"))
            .gray()
            .centered()
            .wrap(Wrap { trim: true })
            .scroll((self.scroll, 0))
            .render(areas[3], buf);
    }
}

/// Create a bordered block with a title.
fn title_block(title: &str) -> Block {
    Block::bordered()
        .gray()
        .title(title.bold().into_centered_line())
}

/// Create some lines to display in the paragraph.
fn create_lines(area: Rect) -> Vec<Line<'static>> {
    let short_line = "A long line to demonstrate line wrapping. ";
    let long_line = short_line.repeat(usize::from(area.width) / short_line.len() + 4);
    let mut styled_spans = vec![];
    for span in [
        "Styled".blue(),
        "Spans".red().on_white(),
        "Bold".bold(),
        "Italic".italic(),
        "Underlined".underlined(),
        "Strikethrough".crossed_out(),
    ] {
        styled_spans.push(span);
        styled_spans.push(" ".into());
    }
    vec![
        Line::raw("Unstyled Line"),
        Line::raw("Styled Line").black().on_red().bold().italic(),
        Line::from(styled_spans),
        Line::from(long_line.green().italic()),
        Line::from_iter([
            "Masked text: ".into(),
            Span::styled(Masked::new("my secret password", '*'), Color::Red),
        ]),
    ]
}

/// A module for common functionality used in the examples.
mod common {
    use std::{
        io::{self, stdout, Stdout},
        panic,
    };

    use color_eyre::{
        config::{EyreHook, HookBuilder, PanicHook},
        eyre,
    };
    use crossterm::ExecutableCommand;
    use ratatui::{
        backend::CrosstermBackend,
        crossterm::terminal::{
            disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        },
        Terminal,
    };

    // A simple alias for the terminal type used in this example.
    pub type Tui = Terminal<CrosstermBackend<Stdout>>;

    /// Initialize the terminal and enter alternate screen mode.
    pub fn init_terminal() -> io::Result<Tui> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout());
        Terminal::new(backend)
    }

    /// Restore the terminal to its original state.
    pub fn restore_terminal() -> io::Result<()> {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    /// Installs hooks for panic and error handling.
    ///
    /// Makes the app resilient to panics and errors by restoring the terminal before printing the
    /// panic or error message. This prevents error messages from being messed up by the terminal
    /// state.
    pub fn install_hooks() -> color_eyre::Result<()> {
        let (panic_hook, eyre_hook) = HookBuilder::default().into_hooks();
        install_panic_hook(panic_hook);
        install_error_hook(eyre_hook)?;
        Ok(())
    }

    /// Install a panic hook that restores the terminal before printing the panic.
    fn install_panic_hook(panic_hook: PanicHook) {
        let panic_hook = panic_hook.into_panic_hook();
        panic::set_hook(Box::new(move |panic_info| {
            let _ = restore_terminal();
            panic_hook(panic_info);
        }));
    }

    /// Install an error hook that restores the terminal before printing the error.
    fn install_error_hook(eyre_hook: EyreHook) -> color_eyre::Result<()> {
        let eyre_hook = eyre_hook.into_eyre_hook();
        eyre::set_hook(Box::new(move |error| {
            let _ = restore_terminal();
            eyre_hook(error)
        }))?;
        Ok(())
    }
}

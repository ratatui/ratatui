//! # [Ratatui] Widgets implementation examples
//!
//! This example demonstrates various ways to implement widget traits in Ratatui on a type, a
//! reference, and a mutable reference. It also shows how to use the `WidgetRef` trait to render
//! boxed widgets.
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
    iter,
    time::{Duration, Instant},
};

use color_eyre::Result;
use common::Terminal;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Widget, WidgetRef},
};

fn main() -> Result<()> {
    common::install_hooks()?;
    let tui = common::init_terminal()?;
    let app = App::default();
    app.run(tui)?;
    common::restore_terminal()?;
    Ok(())
}

#[derive(Default)]
struct App {
    should_quit: bool,
    timer: Timer,
    squares: Squares,
    green_square: RightAlignedSquare,
}

impl App {
    fn run(mut self, mut tui: Terminal) -> Result<()> {
        while !self.should_quit {
            self.draw(&mut tui)?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, tui: &mut Terminal) -> Result<()> {
        tui.draw(|frame| frame.render_widget(self, frame.size()))?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        // Handle events at least 50 frames per second (gifs are usually 50fps)
        let timeout = Duration::from_secs_f64(1.0 / 50.0);
        if !event::poll(timeout)? {
            return Ok(());
        }
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                _ => {}
            }
        }
        Ok(())
    }
}

/// Implement the `Widget` trait on a mutable reference to the `App` type.
///
/// This allows the `App` type to be rendered as a widget. The `App` type owns several other widgets
/// that are rendered as part of the app. The `Widget` trait is implemented on a mutable reference
/// to the `App` type, which allows this to be rendered without consuming the `App` type, and allows
/// the sub-widgets to be mutatable.
impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let constraints = Constraint::from_lengths([1, 1, 2, 1]);
        let [greeting, timer, squares, position] = Layout::vertical(constraints).areas(area);

        Greeting::default().render(greeting, buf);
        self.timer.render(timer, buf);
        self.squares.render(squares, buf);

        self.green_square.render(squares, buf);

        // display the position of the green square. This is updated automatically when the green
        // square is rendered.
        let green_square_position = format!(
            "Green square is at ({},{})",
            self.green_square.last_x, self.green_square.last_y
        );
        green_square_position.render(position, buf);
    }
}

/// An ephemeral greeting widget.
///
/// This widget is implemented on the type itself, which means that it is consumed when it is
/// rendered. This is useful for widgets that are cheap to create, don't need to be reused, and
/// don't need to store any state between renders. This is the simplest way to implement a widget in
/// Ratatui, but in most cases, it is better to implement the `Widget` trait on a reference to the
/// type, as shown in the other examples below.
///
/// This was the way most widgets were implemented in Ratatui before `Widget` was implemented on
/// references in [PR #903] (merged in Ratatui 0.26.0).
///
/// [PR #903]: https://github.com/ratatui-org/ratatui/pull/903
struct Greeting {
    message: String,
}

impl Default for Greeting {
    fn default() -> Self {
        Self {
            message: "Hello, world!".to_string(),
        }
    }
}

impl Widget for Greeting {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Span::raw(&self.message).render(area, buf);
    }
}

/// A timer widget that displays the elapsed time since the timer was started.
///
/// This widget is implemented on a reference to the type, which means that it can be reused and
/// doesn't need to be consumed when it is rendered. This is useful for widgets that need to store
/// state and be updated over time.
///
/// This approach was probably always available in Ratatui, but it wasn't widely used until `Widget`
/// was implemented on references in [PR #903] (merged in Ratatui 0.26.0). This is because all the
/// built-in widgets would consume themselves when rendered.
#[derive(Debug)]
struct Timer {
    start: Instant,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            start: Instant::now(),
        }
    }
}

impl Widget for &Timer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let elapsed = self.start.elapsed().as_secs_f32();
        let message = format!("Elapsed: {elapsed:.1?}s");
        message.render(area, buf);
    }
}

/// A widget that contains a list of squares.
///
/// This implements the `Widget` trait on a reference to the type. It contains a list of boxed
/// widgets that implement the `WidgetRef` trait. This is useful for widgets that contain a list of
/// other widgets that can be different types.
///
/// `RedSquare` and `BlueSquare` are widgets that render a red and blue square, respectively. They
/// implement the `WidgetRef` trait, which allows them to be rendered as boxed widgets (It's not
/// possible to use Widget for this as the widgets cannot generally be moved out of the box).
struct Squares {
    squares: Vec<Box<dyn WidgetRef>>,
}

struct RedSquare;

struct BlueSquare;

impl Default for Squares {
    fn default() -> Self {
        let red_square: Box<dyn WidgetRef> = Box::new(RedSquare);
        let blue_square: Box<dyn WidgetRef> = Box::new(BlueSquare);
        Self {
            squares: vec![red_square, blue_square],
        }
    }
}

impl Widget for &Squares {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let constraints = iter::repeat(Constraint::Length(4)).take(self.squares.len());
        let areas = Layout::horizontal(constraints).split(area);
        for (widget, area) in self.squares.iter().zip(areas.iter()) {
            widget.render_ref(*area, buf);
        }
    }
}

impl WidgetRef for RedSquare {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        fill(area, buf, "█", Color::Red);
    }
}

impl WidgetRef for BlueSquare {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        fill(area, buf, "█", Color::Blue);
    }
}

/// A widget that renders a green square aligned to the right of the area.
///
/// This widget is implemented on a mutable reference to the type, which means that it can store
/// state and update it when it is rendered. This is useful for widgets that need to store the
/// result of some calculation that can only be done when the widget is rendered.
///
/// The x and y coordinates of the square are stored in the widget and updated when the widget is
/// rendered. This allows the square to be aligned to the right of the area. These coordinates could
/// be used to perform hit testing (e.g. checking if a mouse click is inside the square). This app
/// just displays the coordinates as a string.
///
/// This approach was probably always available in Ratatui, but it wasn't widely used either. This
/// is an alternative to implementing the `StatefulWidget` trait, for situations where you want to
/// store the state in the widget itself instead of a separate struct.
#[derive(Default)]
struct RightAlignedSquare {
    last_x: u16,
    last_y: u16,
}

impl Widget for &mut RightAlignedSquare {
    /// Render a green square aligned to the right of the area.
    ///
    /// Updates the x and y coordinates to record the position of the square.
    fn render(self, area: Rect, buf: &mut Buffer) {
        const WIDTH: u16 = 4;
        self.last_x = area.right() - WIDTH;
        self.last_y = area.y;
        let area = Rect {
            x: self.last_x,
            width: WIDTH,
            ..area
        };
        fill(area, buf, "█", Color::Green);
    }
}

/// Fill the area with the specified symbol and style.
///
/// This probably should be a method on the `Buffer` type, but it is defined here for simplicity.
/// <https://github.com/ratatui-org/ratatui/issues/1146>
fn fill<S: Into<Style>>(area: Rect, buf: &mut Buffer, symbol: &str, style: S) {
    let style = style.into();
    for y in area.top()..area.bottom() {
        for x in area.left()..area.right() {
            buf.get_mut(x, y).set_symbol(symbol).set_style(style);
        }
    }
}

/// Contains functions common to all examples
mod common {
    use std::{
        io::{self, stdout, Stdout},
        panic,
    };

    use color_eyre::{
        config::{EyreHook, HookBuilder, PanicHook},
        eyre::{self},
    };
    use crossterm::{
        execute,
        terminal::{
            disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
            LeaveAlternateScreen,
        },
    };
    use ratatui::backend::CrosstermBackend;

    // A type alias to simplify the usage of the terminal and make it easier to change the backend
    // or choice of writer.
    pub type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

    /// Initialize the terminal by enabling raw mode and entering the alternate screen.
    ///
    /// This function should be called before the program starts to ensure that the terminal is in
    /// the correct state for the application.
    pub fn init_terminal() -> io::Result<Terminal> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout());
        Terminal::new(backend)
    }

    /// Restore the terminal by leaving the alternate screen and disabling raw mode.
    ///
    /// This function should be called before the program exits to ensure that the terminal is
    /// restored to its original state.
    pub fn restore_terminal() -> io::Result<()> {
        disable_raw_mode()?;
        execute!(
            stdout(),
            LeaveAlternateScreen,
            Clear(ClearType::FromCursorDown),
        )
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

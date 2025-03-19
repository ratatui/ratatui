/// A Ratatui example that demonstrates how to handle panics in your application.
///
/// Prior to Ratatui 0.28.1, a panic hook had to be manually set up to ensure that the terminal
/// was reset when a panic occurred. This was necessary because a panic would interrupt the
/// normal control flow and leave the terminal in a distorted state.
///
/// Starting with Ratatui 0.28.1, the panic hook is automatically set up by the new
/// `ratatui::init` function, so you no longer need to manually set up the panic hook. This
/// example now demonstrates how the panic hook acts when it is enabled by default.
///
/// When exiting normally or when handling `Result::Err`, we can reset the terminal manually at
/// the end of `main` just before we print the error.
///
/// Because a panic interrupts the normal control flow, manually resetting the terminal at the
/// end of `main` won't do us any good. Instead, we need to make sure to set up a panic hook
/// that first resets the terminal before handling the panic. This both reuses the standard
/// panic hook to ensure a consistent panic handling UX and properly resets the terminal to not
/// distort the output.
///
/// That's why this example is set up to show both situations, with and without the panic hook,
/// to see the difference.
///
/// For more information on how to set this up manually, see the [Color Eyre recipe] in the
/// Ratatui website.
///
/// This example runs with the Ratatui library code in the branch that you are currently
/// reading. See the [`latest`] branch for the code which works with the most recent Ratatui
/// release.
///
/// [`latest`]: https://github.com/ratatui/ratatui/tree/latest
/// [Color Eyre recipe]: https://ratatui.rs/recipes/apps/color-eyre
use color_eyre::{eyre::bail, Result};
use crossterm::event::{self, Event, KeyCode};
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph};
use ratatui::{DefaultTerminal, Frame};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}
struct App {
    hook_enabled: bool,
}

impl App {
    const fn new() -> Self {
        Self { hook_enabled: true }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('p') => panic!("intentional demo panic"),
                    KeyCode::Char('e') => bail!("intentional demo error"),
                    KeyCode::Char('h') => {
                        let _ = std::panic::take_hook();
                        self.hook_enabled = false;
                    }
                    KeyCode::Char('q') => return Ok(()),
                    _ => {}
                }
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let text = vec![
            if self.hook_enabled {
                Line::from("HOOK IS CURRENTLY **ENABLED**")
            } else {
                Line::from("HOOK IS CURRENTLY **DISABLED**")
            },
            Line::from(""),
            Line::from("Press `p` to cause a panic"),
            Line::from("Press `e` to cause an error"),
            Line::from("Press `h` to disable the panic hook"),
            Line::from("Press `q` to quit"),
            Line::from(""),
            Line::from("When your app panics without a panic hook, you will likely have to"),
            Line::from("reset your terminal afterwards with the `reset` command"),
            Line::from(""),
            Line::from("Try first with the panic handler enabled, and then with it disabled"),
            Line::from("to see the difference"),
        ];

        let paragraph = Paragraph::new(text)
            .block(Block::bordered().title("Panic Handler Demo"))
            .centered();

        frame.render_widget(paragraph, frame.area());
    }
}

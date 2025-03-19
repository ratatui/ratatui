/// A minimal example of a Ratatui application.
///
/// This is a bare minimum example. There are many approaches to running an application loop,
/// so this is not meant to be prescriptive. See the [examples] folder for more complete
/// examples. In particular, the [hello-world] example is a good starting point.
///
/// This example runs with the Ratatui library code in the branch that you are currently
/// reading. See the [`latest`] branch for the code which works with the most recent Ratatui
/// release.
///
/// [`latest`]: https://github.com/ratatui/ratatui/tree/latest
/// [examples]: https://github.com/ratatui/ratatui/blob/main/examples
/// [hello-world]: https://github.com/ratatui/ratatui/blob/main/examples/apps/hello-world
use crossterm::event::{self, Event};
use ratatui::text::Text;
use ratatui::Frame;

fn main() {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello World!");
    frame.render_widget(text, frame.area());
}

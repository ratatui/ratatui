// use crossterm::{
//     event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };
// use ratatui::{
//     backend::{Backend, CrosstermBackend},
//     layout::{Constraint, Direction, Layout},
//     style::{Color, Style},
//     widgets::{Block, Borders, Histogram},
//     Frame, Terminal,
// };
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
//
// use rand::{rngs::ThreadRng, thread_rng, Rng};
//
// struct App {
//     data: Vec<u64>,
//     size: usize,
//     rng: ThreadRng,
// }
//
// impl App {
//     fn new(size: usize) -> App {
//         let data = vec![0; size];
//         App {
//             data,
//             rng: thread_rng(),
//             size,
//         }
//     }
//
//     fn on_tick(&mut self) {
//         for i in 0..self.size {
//             self.data[i] = self.rng.gen_range(0..100);
//         }
//     }
// }
//
fn main() -> Result<(), Box<dyn Error>> {
    //     // setup terminal
    //     enable_raw_mode()?;
    //     let mut stdout = io::stdout();
    //     execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    //     let backend = CrosstermBackend::new(stdout);
    //     let mut terminal = Terminal::new(backend)?;
    //
    //     // create app and run it
    //     let tick_rate = Duration::from_millis(250);
    //     let app = App::new(100);
    //     let res = run_app(&mut terminal, app, tick_rate);
    //
    //     // restore terminal
    //     disable_raw_mode()?;
    //     execute!(
    //         terminal.backend_mut(),
    //         LeaveAlternateScreen,
    //         DisableMouseCapture
    //     )?;
    //     terminal.show_cursor()?;
    //
    //     if let Err(err) = res {
    //         println!("{:?}", err)
    //     }
    //
    Ok(())
}
//
// fn run_app<B: Backend>(
//     terminal: &mut Terminal<B>,
//     mut app: App,
//     tick_rate: Duration,
// ) -> io::Result<()> { let mut last_tick = Instant::now(); loop { terminal.draw(|f| ui(f, &app))?;
//
//         let timeout = tick_rate
//             .checked_sub(last_tick.elapsed())
//             .unwrap_or_else(|| Duration::from_secs(0));
//         if crossterm::event::poll(timeout)? {
//             if let Event::Key(key) = event::read()? {
//                 if let KeyCode::Char('q') = key.code {
//                     return Ok(());
//                 }
//             }
//         }
//         if last_tick.elapsed() >= tick_rate {
//             app.on_tick();
//             last_tick = Instant::now();
//         }
//     }
// }
//
// fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
//     let chunks = Layout::default()
//         .direction(Direction::Vertical)
//         .margin(2)
//         .constraints([Constraint::Percentage(100)].as_ref())
//         .split(f.size());
//     let histogram = Histogram::default()
//         .block(Block::default().title("Data1").borders(Borders::ALL))
//         .data(&app.data, 10)
//         .bar_style(Style::default().fg(Color::Yellow))
//         .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));
//     f.render_widget(histogram, chunks[0]);
// }

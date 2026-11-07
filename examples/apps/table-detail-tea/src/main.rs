//! A Ratatui example that demonstrates the Elm architecture (TEA) with a table and a detail view.
//!
//! TEA splits an app into a model, the messages that change it, an `update` function that applies
//! a message to the model, and a `view` function that renders the model. Here that is [`Model`],
//! [`Message`], [`Model::update`] and [`Model::view`].
//!
//! `update` never performs side effects. It returns a [`Command`] (Elm's `Cmd`) describing the
//! effect, and the runtime in [`run`] performs it and feeds any result back in as another message.
//! Loading the teas works this way: a background thread stands in for a slow source such as a
//! network request. Its result comes back as a `Result`, so a failed load gets its own state in
//! [`Model`], even though this stand-in never fails.
//!
//! Elm's subscriptions are the sources of messages besides commands. Terminal input is always on,
//! and [`Model::tick_rate`] asks for ticks only while the spinner is showing, so an idle app only
//! wakes up for input.
//!
//! ```text
//! Terminal input / ticks ─┐
//!                         ├--> Message --> update --> Model --> view --> Screen
//! Command result ─────────┘                  │
//!                                            └--> Command --> Runtime --> Command result
//! ```
//!
//! Unlike Elm, `update` changes the model through `&mut self` instead of returning a new one, so
//! the model doesn't have to be rebuilt for every message. The data still flows one way: the only
//! thing `view` writes back is the table's scroll offset.
//!
//! With an async runtime, only the runtime changes: [`run`] becomes a `select!` over the terminal
//! events, the command results and an interval while [`Model::tick_rate`] asks for one, and the
//! commands become spawned tasks. [`Model`] and [`Message`] stay as they are.
//!
//! This example runs with the Ratatui library code in the branch that you are currently reading.
//! See the [`latest`] branch for the code which works with the most recent Ratatui release.
//!
//! [`latest`]: https://github.com/ratatui/ratatui/tree/latest

use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::time::{Duration, Instant};
use std::{io, thread};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, Padding, Paragraph, Row, Table, TableState, Wrap};
use ratatui::{DefaultTerminal, Frame};

const TICK_RATE: Duration = Duration::from_millis(80);
const LOAD_TIME: Duration = Duration::from_secs(2);
const SPINNER: [char; 10] = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(run)
}

/// The whole state of the app. Each variant holds only what exists in that state, so there is no
/// selection to get wrong while the teas are still loading.
enum Model {
    Loading { spinner: usize },
    Loaded { teas: Vec<Tea>, table: TableState },
    Failed { error: String },
}

/// Everything that can happen to the model.
enum Message {
    GotTeas(Result<Vec<Tea>, String>),
    Tick,
    SelectNext,
    SelectPrevious,
    Reload,
    Quit,
}

/// A side effect that `update` asks the runtime to perform.
enum Command {
    LoadTeas,
    Quit,
}

impl Model {
    const fn init() -> (Self, Option<Command>) {
        (Self::Loading { spinner: 0 }, Some(Command::LoadTeas))
    }

    fn update(&mut self, message: Message) -> Option<Command> {
        match (message, &mut *self) {
            (Message::Quit, _) => return Some(Command::Quit),
            (Message::Reload, Self::Loaded { .. } | Self::Failed { .. }) => {
                let (model, command) = Self::init();
                *self = model;
                return command;
            }
            (Message::GotTeas(Ok(teas)), Self::Loading { .. }) => {
                let table = TableState::default().with_selected((!teas.is_empty()).then_some(0));
                *self = Self::Loaded { teas, table };
            }
            (Message::GotTeas(Err(error)), Self::Loading { .. }) => *self = Self::Failed { error },
            (Message::Tick, Self::Loading { spinner }) => *spinner += 1,
            (Message::SelectNext, Self::Loaded { teas, table }) => {
                let last = teas.len().saturating_sub(1);
                table.select(table.selected().map(|index| (index + 1).min(last)));
            }
            (Message::SelectPrevious, Self::Loaded { table, .. }) => {
                table.select(table.selected().map(|index| index.saturating_sub(1)));
            }
            // A reload while loading would start a second loader, and the rest have nothing to
            // change in the state they arrive in
            (
                Message::Reload | Message::SelectNext | Message::SelectPrevious,
                Self::Loading { .. },
            )
            | (Message::SelectNext | Message::SelectPrevious, Self::Failed { .. })
            | (Message::GotTeas(_) | Message::Tick, Self::Loaded { .. } | Self::Failed { .. }) => {}
        }
        None
    }

    /// Elm's `subscriptions`: the spinner ticks only while loading.
    const fn tick_rate(&self) -> Option<Duration> {
        match self {
            Self::Loading { .. } => Some(TICK_RATE),
            Self::Loaded { .. } | Self::Failed { .. } => None,
        }
    }

    /// Takes `&mut self` because rendering the table stores its scroll offset in [`TableState`].
    fn view(&mut self, frame: &mut Frame) {
        let block = Block::bordered().title(" Teas ").title_bottom(self.help());
        match self {
            Self::Loading { spinner } => {
                let spinner = SPINNER[*spinner % SPINNER.len()];
                let text = format!("{spinner} Loading the tea menu");
                frame.render_widget(Paragraph::new(text).block(block), frame.area());
            }
            Self::Loaded { teas, table } => {
                let layout = Layout::vertical([Constraint::Fill(1); 2]);
                let [table_area, detail_area] = frame.area().layout(&layout);
                let rows = teas.iter().map(Row::from);
                let widths = [
                    Constraint::Length(18),
                    Constraint::Length(8),
                    Constraint::Fill(1),
                ];
                let tea_table = Table::new(rows, widths)
                    .header(Row::new(Tea::COLUMNS).bold())
                    .row_highlight_style(Style::new().reversed())
                    .block(block);
                frame.render_stateful_widget(tea_table, table_area, table);

                let selected = table.selected().and_then(|index| teas.get(index));
                frame.render_widget(view_detail(selected), detail_area);
            }
            Self::Failed { error } => {
                let text = format!("Couldn't load the tea menu: {error}");
                let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
                frame.render_widget(paragraph, frame.area());
            }
        }
    }

    /// The keys that do something in the current state.
    const fn help(&self) -> &'static str {
        match self {
            Self::Loading { .. } => " (q) quit ",
            Self::Loaded { .. } => " (↑/↓) select | (r) reload | (q) quit ",
            Self::Failed { .. } => " (r) retry | (q) quit ",
        }
    }
}

fn view_detail(tea: Option<&Tea>) -> Paragraph<'static> {
    let Some(tea) = tea else {
        return Paragraph::new("No tea selected").block(Block::bordered());
    };
    let text = format!(
        "Brew at {} for {}.\n\n{}",
        tea.temperature, tea.steep, tea.note
    );
    let block = Block::bordered()
        .title(format!(" {} ", tea.name))
        .padding(Padding::uniform(1));
    Paragraph::new(text).block(block).wrap(Wrap { trim: true })
}

impl Message {
    fn from_event(event: &Event) -> Option<Self> {
        match event.as_key_press_event()?.code {
            KeyCode::Char('j') | KeyCode::Down => Some(Self::SelectNext),
            KeyCode::Char('k') | KeyCode::Up => Some(Self::SelectPrevious),
            KeyCode::Char('r') => Some(Self::Reload),
            KeyCode::Char('q') | KeyCode::Esc => Some(Self::Quit),
            _ => None,
        }
    }
}

/// What wakes the runtime up, besides ticks.
enum RuntimeEvent {
    Message(Message),
    /// The terminal changed size, which needs a redraw but doesn't change the model.
    Resize,
    InputFailed(io::Error),
}

/// The runtime: performs the last command, draws the model and waits for the next event.
fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let (sender, events) = mpsc::channel();
    spawn_input_reader(sender.clone());
    let (mut model, mut command) = Model::init();
    let mut next_tick = None;
    loop {
        match command.take() {
            Some(Command::LoadTeas) => spawn_tea_loader(sender.clone()),
            Some(Command::Quit) => return Ok(()),
            None => {}
        }
        terminal.draw(|frame| model.view(frame))?;
        match next_event(&events, model.tick_rate(), &mut next_tick)? {
            RuntimeEvent::Message(message) => command = model.update(message),
            RuntimeEvent::Resize => {}
            RuntimeEvent::InputFailed(error) => return Err(error.into()),
        }
    }
}

/// Waits for the next event. While the model asks for ticks, a due tick goes before any queued
/// event, so ticks keep coming however busy the input is, like Elm's `Time.every`.
fn next_event(
    events: &Receiver<RuntimeEvent>,
    tick_rate: Option<Duration>,
    next_tick: &mut Option<Instant>,
) -> Result<RuntimeEvent> {
    let Some(rate) = tick_rate else {
        *next_tick = None;
        return Ok(events.recv()?);
    };
    let deadline = *next_tick.get_or_insert_with(|| Instant::now() + rate);
    let now = Instant::now();
    if now < deadline {
        match events.recv_timeout(deadline - now) {
            Err(RecvTimeoutError::Timeout) => {}
            received => return Ok(received?),
        }
    }
    *next_tick = None;
    Ok(RuntimeEvent::Message(Message::Tick))
}

/// Sends terminal input to the runtime for as long as the app runs.
fn spawn_input_reader(sender: Sender<RuntimeEvent>) {
    thread::spawn(move || {
        if let Err(error) = forward_input(&sender) {
            sender.send(RuntimeEvent::InputFailed(error)).ok();
        }
    });
}

fn forward_input(sender: &Sender<RuntimeEvent>) -> io::Result<()> {
    loop {
        let input = match event::read()? {
            Event::Resize(..) => Some(RuntimeEvent::Resize),
            other => Message::from_event(&other).map(RuntimeEvent::Message),
        };
        if let Some(input) = input
            && sender.send(input).is_err()
        {
            return Ok(());
        }
    }
}

/// Performs [`Command::LoadTeas`]. The result comes back as [`Message::GotTeas`].
fn spawn_tea_loader(sender: Sender<RuntimeEvent>) {
    thread::spawn(move || {
        thread::sleep(LOAD_TIME);
        let teas = Message::GotTeas(Ok(TEAS.to_vec()));
        sender.send(RuntimeEvent::Message(teas)).ok();
    });
}

#[derive(Clone, Copy)]
struct Tea {
    name: &'static str,
    kind: &'static str,
    origin: &'static str,
    temperature: &'static str,
    steep: &'static str,
    note: &'static str,
}

impl Tea {
    const COLUMNS: [&str; 3] = ["Name", "Kind", "Origin"];
}

impl From<&Tea> for Row<'static> {
    fn from(tea: &Tea) -> Self {
        Self::new([tea.name, tea.kind, tea.origin])
    }
}

const TEAS: [Tea; 14] = [
    Tea {
        name: "Sencha",
        kind: "Green",
        origin: "Japan",
        temperature: "75 °C",
        steep: "1-2 minutes",
        note: "Japan's everyday green tea, steamed rather than pan-fired.",
    },
    Tea {
        name: "Gyokuro",
        kind: "Green",
        origin: "Japan",
        temperature: "60 °C",
        steep: "2 minutes",
        note: "Shaded for weeks before picking, which makes it sweet and savoury.",
    },
    Tea {
        name: "Longjing",
        kind: "Green",
        origin: "Zhejiang, China",
        temperature: "80 °C",
        steep: "2-3 minutes",
        note: "Also called Dragon Well. Pan-fired into flat, smooth leaves.",
    },
    Tea {
        name: "Bai Mudan",
        kind: "White",
        origin: "Fujian, China",
        temperature: "85 °C",
        steep: "3-4 minutes",
        note: "White Peony: buds and young leaves, only withered and dried.",
    },
    Tea {
        name: "Tieguanyin",
        kind: "Oolong",
        origin: "Fujian, China",
        temperature: "95 °C",
        steep: "1-3 minutes",
        note: "A rolled, floral oolong that holds up to many infusions.",
    },
    Tea {
        name: "Da Hong Pao",
        kind: "Oolong",
        origin: "Fujian, China",
        temperature: "95 °C",
        steep: "1-3 minutes",
        note: "A roasted rock oolong from the Wuyi mountains.",
    },
    Tea {
        name: "Dongfang Meiren",
        kind: "Oolong",
        origin: "Hsinchu, Taiwan",
        temperature: "85 °C",
        steep: "3 minutes",
        note: "Also called Oriental Beauty. Leafhopper bites give it a honey taste.",
    },
    Tea {
        name: "Darjeeling",
        kind: "Black",
        origin: "India",
        temperature: "90 °C",
        steep: "3 minutes",
        note: "Grown in the Himalayan foothills. The first flush is light and floral.",
    },
    Tea {
        name: "Assam",
        kind: "Black",
        origin: "India",
        temperature: "100 °C",
        steep: "3-5 minutes",
        note: "Strong and malty, a common base for breakfast blends.",
    },
    Tea {
        name: "Ostfriesentee",
        kind: "Black",
        origin: "East Frisia, Germany",
        temperature: "100 °C",
        steep: "3-5 minutes",
        note: "A strong Assam blend, poured over rock sugar and topped with unstirred cream.",
    },
    Tea {
        name: "Keemun",
        kind: "Black",
        origin: "Anhui, China",
        temperature: "90 °C",
        steep: "3-4 minutes",
        note: "A smooth Chinese black tea, often used in breakfast blends too.",
    },
    Tea {
        name: "Lapsang Souchong",
        kind: "Black",
        origin: "Fujian, China",
        temperature: "95 °C",
        steep: "3 minutes",
        note: "Dried over pine fires, which gives it a smoky taste.",
    },
    Tea {
        name: "Pu-erh",
        kind: "Dark",
        origin: "Yunnan, China",
        temperature: "100 °C",
        steep: "1-2 minutes",
        note: "Fermented and aged, often pressed into cakes.",
    },
    Tea {
        name: "Rooibos",
        kind: "Herbal",
        origin: "South Africa",
        temperature: "100 °C",
        steep: "5 minutes",
        note: "Not tea at all but a caffeine-free bush, steeped the same way.",
    },
];

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::time::Instant;

    use color_eyre::Result;

    use super::{Command, Message, Model, RuntimeEvent, TEAS, TICK_RATE, next_event};

    #[test]
    fn quit_asks_the_runtime_to_quit() {
        let (mut model, _) = Model::init();
        assert!(matches!(model.update(Message::Quit), Some(Command::Quit)));
    }

    #[test]
    fn failed_load_shows_the_error() {
        let (mut model, _) = Model::init();
        model.update(Message::GotTeas(Err(String::from("offline"))));
        assert!(matches!(model, Model::Failed { error } if error == "offline"));
    }

    #[test]
    fn due_tick_goes_before_queued_events() -> Result<()> {
        let (sender, events) = mpsc::channel();
        sender.send(RuntimeEvent::Message(Message::SelectNext))?;
        let mut next_tick = Some(Instant::now());
        let event = next_event(&events, Some(TICK_RATE), &mut next_tick)?;
        assert!(matches!(event, RuntimeEvent::Message(Message::Tick)));
        Ok(())
    }

    #[test]
    fn reload_after_a_failed_load_starts_a_new_one() {
        let mut model = Model::Failed {
            error: String::from("offline"),
        };
        assert!(matches!(
            model.update(Message::Reload),
            Some(Command::LoadTeas)
        ));
        assert!(matches!(model, Model::Loading { .. }));
    }

    #[test]
    fn reload_while_loading_is_ignored() {
        let (mut model, _) = Model::init();
        assert!(model.update(Message::Reload).is_none());
    }

    #[test]
    fn selection_stops_at_the_last_tea() {
        let mut model = Model::Loading { spinner: 0 };
        model.update(Message::GotTeas(Ok(TEAS[..2].to_vec())));
        for _ in 0..3 {
            model.update(Message::SelectNext);
        }
        assert!(matches!(model, Model::Loaded { table, .. } if table.selected() == Some(1)));
    }

    #[test]
    fn empty_menu_has_no_selection() {
        let mut model = Model::Loading { spinner: 0 };
        model.update(Message::GotTeas(Ok(Vec::new())));
        model.update(Message::SelectNext);
        assert!(matches!(model, Model::Loaded { table, .. } if table.selected().is_none()));
    }
}

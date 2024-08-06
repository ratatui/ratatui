//! # [Ratatui] Async example
//!
//! This example demonstrates how to use Ratatui with widgets that fetch data asynchronously. It
//! uses the `octocrab` crate to fetch a list of pull requests from the GitHub API. You will need an
//! environment variable named `GITHUB_TOKEN` with a valid GitHub personal access token. The token
//! does not need any special permissions.
//!
//! <https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token>
//! <https://github.com/settings/tokens/new> to create a new token (select classic, and no scopes)
//!
//! This example does not cover message passing between threads, it only demonstrates how to manage
//! shared state between the main thread and a background task, which acts mostly as a one-shot
//! fetcher. For more complex scenarios, you may need to use channels or other synchronization
//! primitives.
//!
//! A simple app might have multiple widgets that fetch data from different sources, and each widget
//! would have its own background task to fetch the data. The main thread would then render the
//! widgets with the latest data.
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
    sync::{Arc, RwLock},
    time::Duration,
};

use color_eyre::{eyre::Context, Result, Section};
use futures::StreamExt;
use octocrab::{
    params::{pulls::Sort, Direction},
    OctocrabBuilder, Page,
};
use ratatui::{
    crossterm::event::{Event, EventStream, KeyCode},
    layout::Offset,
    prelude::{Buffer, Constraint, Line, Modifier, Rect, Stylize},
    widgets::{
        Block, BorderType, HighlightSpacing, Row, StatefulWidget, Table, TableState, Widget,
    },
};

use self::terminal::Terminal;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    init_octocrab()?;
    let terminal = terminal::init()?;
    let app_result = App::default().run(terminal).await;
    terminal::restore();
    app_result
}

fn init_octocrab() -> Result<()> {
    let token = std::env::var("GITHUB_TOKEN")
        .wrap_err("The GITHUB_TOKEN environment variable was not found")
        .suggestion(
            "Go to https://github.com/settings/tokens/new to create a token, and re-run:
            GITHUB_TOKEN=ghp_... cargo run --example async --features crossterm",
        )?;
    let crab = OctocrabBuilder::new().personal_token(token).build()?;
    octocrab::initialise(crab);
    Ok(())
}

#[derive(Debug, Default)]
struct App {
    should_quit: bool,
    pulls: PullRequestsWidget,
}

impl App {
    const FRAMES_PER_SECOND: f32 = 60.0;

    pub async fn run(mut self, mut terminal: Terminal) -> Result<()> {
        self.pulls.run();

        let mut interval =
            tokio::time::interval(Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND));
        let mut events = EventStream::new();

        while !self.should_quit {
            tokio::select! {
                _ = interval.tick() => self.draw(&mut terminal)?,
                Some(Ok(event)) = events.next() =>  self.handle_event(&event),
            }
        }
        Ok(())
    }

    fn draw(&self, terminal: &mut Terminal) -> Result<()> {
        terminal.draw(|frame| {
            let area = frame.area();
            frame.render_widget(
                Line::from("ratatui async example").centered().cyan().bold(),
                area,
            );
            let area = area.offset(Offset { x: 0, y: 1 }).intersection(area);
            frame.render_widget(&self.pulls, area);
        })?;
        Ok(())
    }

    fn handle_event(&mut self, event: &Event) {
        if let Event::Key(event) = event {
            match event.code {
                KeyCode::Char('q') => self.should_quit = true,
                KeyCode::Char('j') => self.pulls.scroll_down(),
                KeyCode::Char('k') => self.pulls.scroll_up(),
                _ => {}
            }
        }
    }
}

/// A widget that displays a list of pull requests.
///
/// This is an async widget that fetches the list of pull requests from the GitHub API. It contains
/// an inner `Arc<RwLock<PullRequests>>` that holds the state of the widget. Cloning the widget
/// will clone the Arc, so you can pass it around to other threads, and this is used to spawn a
/// background task to fetch the pull requests.
#[derive(Debug, Clone, Default)]
struct PullRequestsWidget {
    inner: Arc<RwLock<PullRequests>>,
    selected_index: usize, // no need to lock this since it's only accessed by the main thread
}

#[derive(Debug, Default)]
struct PullRequests {
    pulls: Vec<PullRequest>,
    loading_state: LoadingState,
}

#[derive(Debug, Clone)]
struct PullRequest {
    id: String,
    title: String,
    url: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
enum LoadingState {
    #[default]
    Idle,
    Loading,
    Loaded,
    Error(String),
}

impl PullRequestsWidget {
    /// Start fetching the pull requests in the background.
    ///
    /// This method spawns a background task that fetches the pull requests from the GitHub API.
    /// The result of the fetch is then passed to the `on_load` or `on_err` methods.
    fn run(&self) {
        let this = self.clone(); // clone the widget to pass to the background task
        tokio::spawn(this.fetch_pulls());
    }

    async fn fetch_pulls(self) {
        // this runs once, but you could also run this in a loop, using a channel that accepts
        // messages to refresh on demand, or with an interval timer to refresh every N seconds
        self.set_loading_state(LoadingState::Loading);
        match octocrab::instance()
            .pulls("ratatui-org", "ratatui")
            .list()
            .sort(Sort::Updated)
            .direction(Direction::Descending)
            .send()
            .await
        {
            Ok(page) => self.on_load(&page),
            Err(err) => self.on_err(&err),
        }
    }
    fn on_load(&self, page: &Page<OctoPullRequest>) {
        let prs = page.items.iter().map(Into::into);
        let mut inner = self.inner.write().unwrap();
        inner.loading_state = LoadingState::Loaded;
        inner.pulls.extend(prs);
    }

    fn on_err(&self, err: &octocrab::Error) {
        self.set_loading_state(LoadingState::Error(err.to_string()));
    }

    fn set_loading_state(&self, state: LoadingState) {
        self.inner.write().unwrap().loading_state = state;
    }

    fn scroll_down(&mut self) {
        self.selected_index = self.selected_index.saturating_add(1);
    }

    fn scroll_up(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(1);
    }
}

type OctoPullRequest = octocrab::models::pulls::PullRequest;

impl From<&OctoPullRequest> for PullRequest {
    fn from(pr: &OctoPullRequest) -> Self {
        Self {
            id: pr.number.to_string(),
            title: pr.title.as_ref().unwrap().to_string(),
            url: pr
                .html_url
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_default(),
        }
    }
}

impl Widget for &PullRequestsWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner = self.inner.read().unwrap();

        // a block with a right aligned title with the loading state
        let loading_state = Line::from(format!("{:?}", inner.loading_state)).right_aligned();
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Pull Requests")
            .title(loading_state);

        // a table with the list of pull requests
        let rows = inner.pulls.iter();
        let widths = [
            Constraint::Length(5),
            Constraint::Fill(1),
            Constraint::Max(49),
        ];
        let table = Table::new(rows, widths)
            .block(block)
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_symbol(">>")
            .highlight_style(Modifier::REVERSED);
        let mut table_state = TableState::new().with_selected(self.selected_index);

        StatefulWidget::render(table, area, buf, &mut table_state);
    }
}

impl From<&PullRequest> for Row<'_> {
    fn from(pr: &PullRequest) -> Self {
        let pr = pr.clone();
        Row::new(vec![pr.id, pr.title, pr.url])
    }
}

mod terminal {
    use std::io;

    use crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use ratatui::prelude::{CrosstermBackend, Terminal as RatatuiTerminal};

    /// A type alias for the terminal type used in this example.
    pub type Terminal = RatatuiTerminal<CrosstermBackend<io::Stdout>>;

    pub fn init() -> io::Result<Terminal> {
        set_panic_hook();
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(io::stdout());
        Terminal::new(backend)
    }

    fn set_panic_hook() {
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            restore();
            hook(info);
        }));
    }

    /// Restores the terminal to its original state.
    pub fn restore() {
        if let Err(err) = disable_raw_mode() {
            eprintln!("error disabling raw mode: {err}");
        }
        if let Err(err) = execute!(io::stdout(), LeaveAlternateScreen) {
            eprintln!("error leaving alternate screen: {err}");
        }
    }
}

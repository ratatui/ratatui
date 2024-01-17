use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use itertools::Itertools;
use ratatui::{prelude::*, widgets::*};
use style::palette::material;

const EXAMPLE_TABLE_COLORS: [material::AccentedPalette; 4] = [
    material::RED,
    material::GREEN,
    material::BLUE,
    material::ORANGE,
];
const INFO_TEXT: &str =
    "(q) to quit / (j, ↑) to go up / (k, ↓) to go down / (l, →) for next color / (h, ←) for previous color";

struct App {
    state: TableState,
    items: Vec<Vec<String>>,
    scroll_state: ScrollbarState,
    color_index: usize,
}

impl App {
    fn new() -> App {
        let items = generate_fake_names();
        App {
            state: TableState::default().with_selected(0),
            scroll_state: ScrollbarState::new((items.len() - 1) * 4),
            color_index: 0,
            items,
        }
    }
    pub fn next(&mut self) {
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
        self.scroll_state = self.scroll_state.position(i * 4);
    }

    pub fn previous(&mut self) {
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
        self.scroll_state = self.scroll_state.position(i * 4);
    }

    pub fn next_color(&mut self) {
        if self.color_index >= EXAMPLE_TABLE_COLORS.len() - 1 {
            self.color_index = 0;
        } else {
            self.color_index += 1;
        }
    }

    pub fn previous_color(&mut self) {
        if self.color_index == 0 {
            self.color_index = EXAMPLE_TABLE_COLORS.len() - 1;
        } else {
            self.color_index -= 1
        }
    }
}

fn generate_fake_names() -> Vec<Vec<String>> {
    use fakeit::{address, contact, name};

    (0..20)
        .map(|_| {
            let name = name::full();
            let address = format!(
                "{}\n{}, {} {}",
                address::street(),
                address::city(),
                address::state(),
                address::zip()
            );
            let email = contact::email();
            vec![name, address, email]
        })
        .sorted_by(|a, b| a[0].cmp(&b[0]))
        .collect_vec()
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    KeyCode::Right | KeyCode::Char('l') => app.next_color(),
                    KeyCode::Left | KeyCode::Char('h') => app.previous_color(),
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let rects = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(3)])
        .split(f.size());

    let curr_color = EXAMPLE_TABLE_COLORS[app.color_index];

    let buffer_bg = curr_color.c900;
    let header_bg = curr_color.c900;
    let header_fg = material::WHITE;
    let header_style = Style::default().fg(header_fg).bg(header_bg);
    let row_fg = material::GRAY.c100;
    let normal_row_color = curr_color.c600;
    let alt_row_color = curr_color.c400;
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let footer_border_color = curr_color.c400;

    let header = ["Name", "Address", "Email"]
        .iter()
        .cloned()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(1);
    let rows = app.items.iter().enumerate().map(|(i, item)| {
        let color = match i % 2 {
            0 => normal_row_color,
            _ => alt_row_color,
        };
        item.iter()
            .cloned()
            .map(|content| Cell::from(Text::from(format!("\n{}\n", content))))
            .collect::<Row>()
            .style(Style::new().fg(row_fg).bg(color))
            .height(4)
    });
    let bar = " █ ";
    let t = Table::new(
        rows,
        [
            Constraint::Length(25),
            Constraint::Min(40),
            Constraint::Min(30),
        ],
    )
    .header(header)
    .highlight_style(selected_style)
    .highlight_symbol(Text::from(vec![
        "".into(),
        bar.into(),
        bar.into(),
        "".into(),
    ]))
    .bg(buffer_bg)
    .highlight_spacing(HighlightSpacing::Always);
    f.render_stateful_widget(t, rects[0], &mut app.state);
    f.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None),
        rects[0].inner(&Margin {
            vertical: 1,
            horizontal: 1,
        }),
        &mut app.scroll_state,
    );

    let info_footer = Paragraph::new(Line::from(INFO_TEXT))
        .style(Style::new().fg(row_fg).bg(buffer_bg))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::new().fg(footer_border_color))
                .border_type(BorderType::Double),
        );
    f.render_widget(info_footer, rects[1]);
}

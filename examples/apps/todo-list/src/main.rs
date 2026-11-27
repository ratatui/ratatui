/// A Ratatui example that demonstrates how to create a todo list with selectable items.
///
/// This example runs with the Ratatui library code in the branch that you are currently
/// reading. See the [`latest`] branch for the code which works with the most recent Ratatui
/// release.
///
/// [`latest`]: https://github.com/ratatui/ratatui/tree/latest
use std::sync::LazyLock;

use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::palette::tailwind;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{
    Block, Borders, List, ListHighlightSpacing, ListItem, ListState, Padding, Paragraph,
    SelectionMode, StatefulWidget, Widget, Wrap,
};
use ratatui::{DefaultTerminal, symbols};

// Colors are resolved once at startup. On terminals that don't support 24-bit color
// (e.g. Apple Terminal.app before build 465) the truecolor tailwind palette looks bad,
// so we fall back to indexed colors. See issue #1972.
static THEME: LazyLock<Theme> = LazyLock::new(Theme::new);

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::default().run(terminal))
}

/// This struct holds the current state of the app. In particular, it has the `todo_list` field
/// which is a wrapper around `ListState`. Keeping track of the state lets us render the
/// associated widget with its state and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events. Check
/// the drawing logic for items on how to specify the highlighting style for selected items.
struct App {
    should_exit: bool,
    todo_list: TodoList,
}

struct TodoList {
    items: Vec<TodoItem>,
    state: ListState,
}

#[derive(Debug)]
struct TodoItem {
    todo: String,
    info: String,
    status: Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Status {
    Todo,
    Completed,
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_exit: false,
            todo_list: TodoList::from_iter([
                (
                    Status::Todo,
                    "Rewrite everything with Rust!",
                    "I can't hold my inner voice. He tells me to rewrite the complete universe with Rust",
                ),
                (
                    Status::Completed,
                    "Rewrite all of your tui apps with Ratatui",
                    "Yes, you heard that right. Go and replace your tui with Ratatui.",
                ),
                (
                    Status::Todo,
                    "Pet your cat",
                    "Minnak loves to be pet by you! Don't forget to pet and give some treats!",
                ),
                (
                    Status::Todo,
                    "Walk with your dog",
                    "Max is bored, go walk with him!",
                ),
                (
                    Status::Completed,
                    "Pay the bills",
                    "Pay the train subscription!!!",
                ),
                (
                    Status::Completed,
                    "Refactor list example",
                    "If you see this info that means I completed this task!",
                ),
            ]),
        }
    }
}

impl FromIterator<(Status, &'static str, &'static str)> for TodoList {
    fn from_iter<I: IntoIterator<Item = (Status, &'static str, &'static str)>>(iter: I) -> Self {
        let items = iter
            .into_iter()
            .map(|(status, todo, info)| TodoItem::new(status, todo, info))
            .collect::<Vec<_>>();
        // Choosing a mode enables item-count tracking on the first render.
        let state = ListState::default().with_selection_mode(SelectionMode::Clamp);
        Self { items, state }
    }
}

impl TodoItem {
    fn new(status: Status, todo: &str, info: &str) -> Self {
        Self {
            status,
            todo: todo.to_string(),
            info: info.to_string(),
        }
    }
}

impl App {
    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Some(key) = event::read()?.as_key_press_event() {
                self.handle_key(key);
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Char('h') | KeyCode::Left => self.select_none(),
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.select_last(),
            KeyCode::Char('t') => self.toggle_selection_mode(),
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
                self.toggle_status();
            }
            _ => {}
        }
    }

    const fn select_none(&mut self) {
        self.todo_list.state.select(None);
    }

    fn select_next(&mut self) {
        self.todo_list.state.select_next();
    }
    fn select_previous(&mut self) {
        self.todo_list.state.select_previous();
    }

    const fn select_first(&mut self) {
        self.todo_list.state.select_first();
    }

    const fn select_last(&mut self) {
        self.todo_list.state.select_last();
    }

    const fn toggle_selection_mode(&mut self) {
        let mode = self.todo_list.state.selection_mode_mut();
        *mode = match *mode {
            SelectionMode::Clamp => SelectionMode::Wrap,
            SelectionMode::Wrap => SelectionMode::Clamp,
        };
    }

    /// Changes the status of the selected list item
    fn toggle_status(&mut self) {
        if let Some(i) = self.todo_list.state.selected() {
            self.todo_list.items[i].status = match self.todo_list.items[i].status {
                Status::Completed => Status::Todo,
                Status::Todo => Status::Completed,
            }
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_layout = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(2),
        ]);
        let [header_area, content_area, footer_area] = area.layout(&main_layout);

        let content_layout = Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]);
        let [list_area, item_area] = content_area.layout(&content_layout);

        App::render_header(header_area, buf);
        self.render_footer(footer_area, buf);
        self.render_list(list_area, buf);
        self.render_selected_item(item_area, buf);
    }
}

/// Rendering logic for the app
impl App {
    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Ratatui Todo List Example")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        let mode = match self.todo_list.state.selection_mode() {
            SelectionMode::Clamp => "clamp",
            SelectionMode::Wrap => "wrap",
        };
        Paragraph::new(vec![
            Line::raw("Use ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom."),
            Line::raw(format!("t: toggle clamp/wrap ({mode}) | q/Esc: quit")),
        ])
        .centered()
        .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("TODO List").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(THEME.todo_header_style())
            .bg(THEME.normal_row_bg);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .todo_list
            .items
            .iter()
            .enumerate()
            .map(|(i, todo_item)| {
                let color = alternate_colors(i);
                ListItem::from(todo_item).bg(color)
            })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .block(block)
            .highlight_style(THEME.selected_style())
            .highlight_symbol(">")
            .highlight_spacing(ListHighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.todo_list.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        // We get the info depending on the item's state.
        let info = if let Some(i) = self.todo_list.state.selected() {
            match self.todo_list.items[i].status {
                Status::Completed => format!("✓ DONE: {}", self.todo_list.items[i].info),
                Status::Todo => format!("☐ TODO: {}", self.todo_list.items[i].info),
            }
        } else {
            "Nothing selected...".to_string()
        };

        // We show the list item's info under the list in this paragraph
        let block = Block::new()
            .title(Line::raw("TODO Info").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(THEME.todo_header_style())
            .bg(THEME.normal_row_bg)
            .padding(Padding::horizontal(1));

        // We can now render the item info
        Paragraph::new(info)
            .block(block)
            .fg(THEME.text_fg)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}

fn alternate_colors(i: usize) -> Color {
    if i.is_multiple_of(2) {
        THEME.normal_row_bg
    } else {
        THEME.alt_row_bg
    }
}

impl From<&TodoItem> for ListItem<'_> {
    fn from(value: &TodoItem) -> Self {
        let line = match value.status {
            Status::Todo => Line::styled(format!(" ☐ {}", value.todo), THEME.text_fg),
            Status::Completed => {
                Line::styled(format!(" ✓ {}", value.todo), THEME.completed_text_fg)
            }
        };
        ListItem::new(line)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Theme {
    header_fg: Color,
    header_bg: Color,
    normal_row_bg: Color,
    alt_row_bg: Color,
    selected_row_bg: Color,
    text_fg: Color,
    completed_text_fg: Color,
}

impl Theme {
    fn new() -> Self {
        let term_program = std::env::var("TERM_PROGRAM").ok();
        let term_program_version = std::env::var("TERM_PROGRAM_VERSION").ok();
        let is_true_color =
            Self::supports_true_color(term_program.as_deref(), term_program_version.as_deref());
        Self::from_true_color(is_true_color)
    }

    // Builds the theme for a terminal that either does or does not support 24-bit color.
    fn from_true_color(is_true_color: bool) -> Self {
        use tailwind::{BLUE, GREEN, SLATE};

        let color = |true_color, indexed_color| {
            if is_true_color {
                true_color
            } else {
                indexed_color
            }
        };

        Self {
            header_fg: color(SLATE.c100, Color::Indexed(255)),
            header_bg: color(BLUE.c800, Color::Indexed(25)),
            normal_row_bg: color(SLATE.c950, Color::Indexed(16)),
            alt_row_bg: color(SLATE.c900, Color::Indexed(17)),
            selected_row_bg: color(SLATE.c800, Color::Indexed(18)),
            text_fg: color(SLATE.c200, Color::Indexed(252)),
            completed_text_fg: color(GREEN.c500, Color::Indexed(41)),
        }
    }

    const fn todo_header_style(self) -> Style {
        Style::new().fg(self.header_fg).bg(self.header_bg)
    }

    const fn selected_style(self) -> Style {
        Style::new()
            .bg(self.selected_row_bg)
            .add_modifier(Modifier::BOLD)
    }

    // Checks whether truecolor (24-bit color) is supported by the current terminal.
    //
    // Terminals known *not* to support truecolor:
    // - Apple Terminal.app: all versions before build 465
    //
    // Environment variables used:
    // - `TERM_PROGRAM`: identifies the terminal application in use
    // - `TERM_PROGRAM_VERSION`: build number of that terminal application
    fn supports_true_color(term_program: Option<&str>, term_program_version: Option<&str>) -> bool {
        if term_program != Some("Apple_Terminal") {
            return true;
        }
        let build_number = term_program_version
            .and_then(|version| version.split('.').next())
            .and_then(|version| version.parse::<u32>().ok())
            .unwrap_or(0);
        build_number >= 465
    }
}

#[cfg(test)]
mod tests {
    use ratatui::style::{Color, Modifier, Style};

    use super::Theme;

    #[test]
    fn apple_terminal_before_build_465_does_not_support_true_color() {
        assert!(!Theme::supports_true_color(
            Some("Apple_Terminal"),
            Some("464")
        ));
        assert!(!Theme::supports_true_color(
            Some("Apple_Terminal"),
            Some("455.1")
        ));
        assert!(!Theme::supports_true_color(
            Some("Apple_Terminal"),
            Some("unknown")
        ));
        assert!(!Theme::supports_true_color(Some("Apple_Terminal"), None));
    }

    #[test]
    fn apple_terminal_build_465_and_later_supports_true_color() {
        assert!(Theme::supports_true_color(
            Some("Apple_Terminal"),
            Some("465")
        ));
        assert!(Theme::supports_true_color(
            Some("Apple_Terminal"),
            Some("465.1")
        ));
        assert!(Theme::supports_true_color(
            Some("Apple_Terminal"),
            Some("500")
        ));
    }

    #[test]
    fn other_terminals_support_true_color() {
        assert!(Theme::supports_true_color(Some("iTerm.app"), Some("3.5.0")));
        assert!(Theme::supports_true_color(Some("WezTerm"), None));
        assert!(Theme::supports_true_color(None, None));
    }

    #[test]
    fn theme_falls_back_to_indexed_colors_without_true_color() {
        let theme = Theme::from_true_color(false);
        assert_eq!(theme.header_fg, Color::Indexed(255));
        assert_eq!(theme.header_bg, Color::Indexed(25));
        assert_eq!(theme.normal_row_bg, Color::Indexed(16));
        assert_eq!(theme.alt_row_bg, Color::Indexed(17));
        assert_eq!(theme.selected_row_bg, Color::Indexed(18));
        assert_eq!(theme.text_fg, Color::Indexed(252));
        assert_eq!(theme.completed_text_fg, Color::Indexed(41));
        assert_eq!(
            theme.todo_header_style(),
            Style::new().fg(Color::Indexed(255)).bg(Color::Indexed(25))
        );
        assert_eq!(
            theme.selected_style(),
            Style::new()
                .bg(Color::Indexed(18))
                .add_modifier(Modifier::BOLD)
        );
    }

    #[test]
    fn theme_uses_truecolor_when_supported() {
        let theme = Theme::from_true_color(true);
        assert!(matches!(theme.header_fg, Color::Rgb(..)));
        assert!(matches!(theme.header_bg, Color::Rgb(..)));
        assert!(matches!(theme.normal_row_bg, Color::Rgb(..)));
        assert!(matches!(theme.alt_row_bg, Color::Rgb(..)));
        assert!(matches!(theme.selected_row_bg, Color::Rgb(..)));
        assert!(matches!(theme.text_fg, Color::Rgb(..)));
        assert!(matches!(theme.completed_text_fg, Color::Rgb(..)));
        assert_eq!(
            theme.todo_header_style(),
            Style::new().fg(theme.header_fg).bg(theme.header_bg)
        );
        assert_eq!(
            theme.selected_style(),
            Style::new()
                .bg(theme.selected_row_bg)
                .add_modifier(Modifier::BOLD)
        );
    }
}

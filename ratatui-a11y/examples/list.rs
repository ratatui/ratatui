//! A selectable list, wired up to AT-SPI end to end: `ListState::selected()`
//! drives the accessible focus/selection state, and actions an AT client
//! sends back (`Focus`, `Click`) drive `ListState` in return.
//!
//! Forked from ratatui's `todo-list` example; see that one for the plain
//! (non-accessible) version of this app.
//!
//! Try it with Orca running, or walk the tree by hand -- see the crate
//! README (and the `IsEnabled` gotcha in the crate docs).
use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::palette::tailwind::{BLUE, GREEN, SLATE};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{
    Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
    StatefulWidget, Widget, Wrap,
};
use ratatui::{DefaultTerminal, symbols};
use ratatui_a11y::{
    A11y, Action, NodeId, Role, TreeBuilder, TreeUpdate, list_nodes, node_id, text_nodes,
};

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::default().run(terminal))
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            ]),
        }
    }
}

impl FromIterator<(Status, &'static str, &'static str)> for TodoList {
    fn from_iter<I: IntoIterator<Item = (Status, &'static str, &'static str)>>(iter: I) -> Self {
        let items = iter
            .into_iter()
            .map(|(status, todo, info)| TodoItem {
                status,
                todo: todo.to_string(),
                info: info.to_string(),
            })
            .collect();
        Self {
            items,
            state: ListState::default(),
        }
    }
}

impl App {
    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let mut a11y = A11y::new(self.build_tree());
        a11y.set_focused(true);

        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            a11y.update(self.build_tree());

            for request in a11y.actions().collect::<Vec<_>>() {
                self.handle_action(&request);
            }

            if event::poll(std::time::Duration::from_millis(50))?
                && let Some(key) = event::read()?.as_key_press_event()
            {
                self.handle_key(key);
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
            KeyCode::Char('h') | KeyCode::Left => self.todo_list.state.select(None),
            KeyCode::Char('j') | KeyCode::Down => self.todo_list.state.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.todo_list.state.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.todo_list.state.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.todo_list.state.select_last(),
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => self.toggle_status(),
            _ => {}
        }
    }

    /// Applies an action an AT client (e.g. Orca) sent back over AT-SPI.
    fn handle_action(&mut self, request: &ratatui_a11y::ActionRequest) {
        let Some(index) = self.item_index_for(request.target_node) else {
            return;
        };
        if request.action == Action::Click {
            self.todo_list.state.select(Some(index));
            self.toggle_status();
        }
    }

    fn item_index_for(&self, id: NodeId) -> Option<usize> {
        (0..self.todo_list.items.len()).find(|&i| item_id(i) == id)
    }

    fn toggle_status(&mut self) {
        if let Some(i) = self.todo_list.state.selected() {
            self.todo_list.items[i].status = match self.todo_list.items[i].status {
                Status::Completed => Status::Todo,
                Status::Todo => Status::Completed,
            };
        }
    }

    /// Builds this frame's accessibility tree: a window containing the
    /// list, containing one node per item, plus a static-text node for the
    /// currently selected item's detail.
    fn build_tree(&self) -> TreeUpdate {
        let window_id = node_id("todo-window");

        let mut tree = TreeBuilder::new();

        let labels = self.todo_list.items.iter().map(|item| {
            format!(
                "{}, {}",
                item.todo,
                match item.status {
                    Status::Todo => "not done",
                    Status::Completed => "done",
                }
            )
        });
        let mut list = list_nodes(labels, self.todo_list.state.selected(), "todo-list");
        list.add_action_to_children(Action::Click);
        let list_root = list.root;
        for (id, node) in list.nodes_mut() {
            if *id == list_root {
                node.set_label("Todo items");
            }
        }
        let selected_item = list.selected;
        let list_id = tree.subtree(list);

        let info = match self.todo_list.state.selected() {
            Some(i) => self.todo_list.items[i].info.clone(),
            None => "Nothing selected".to_string(),
        };
        let info_id = tree.subtree(text_nodes(info, "todo-info"));

        tree.node(
            window_id,
            Role::Window,
            "Ratatui Todo List",
            [list_id, info_id],
        );
        tree.root(window_id);
        tree.focus(selected_item.unwrap_or(list_id));

        tree.build()
    }
}

fn item_id(index: usize) -> NodeId {
    node_id((&"todo-list", index))
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_layout = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]);
        let [header_area, content_area, footer_area] = area.layout(&main_layout);

        let content_layout = Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]);
        let [list_area, item_area] = content_area.layout(&content_layout);

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        self.render_list(list_area, buf);
        self.render_selected_item(item_area, buf);
    }
}

impl App {
    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Ratatui Todo List Example (accessible)")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
            .centered()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("TODO List").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

        let items: Vec<ListItem> = self
            .todo_list
            .items
            .iter()
            .enumerate()
            .map(|(i, todo_item)| {
                let bg = if i.is_multiple_of(2) {
                    NORMAL_ROW_BG
                } else {
                    ALT_ROW_BG_COLOR
                };
                let line = match todo_item.status {
                    Status::Todo => Line::styled(format!(" ☐ {}", todo_item.todo), TEXT_FG_COLOR),
                    Status::Completed => {
                        Line::styled(format!(" ✓ {}", todo_item.todo), COMPLETED_TEXT_FG_COLOR)
                    }
                };
                ListItem::new(line).bg(bg)
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.todo_list.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        let info = match self.todo_list.state.selected() {
            Some(i) => match self.todo_list.items[i].status {
                Status::Completed => format!("✓ DONE: {}", self.todo_list.items[i].info),
                Status::Todo => format!("☐ TODO: {}", self.todo_list.items[i].info),
            },
            None => "Nothing selected...".to_string(),
        };

        let block = Block::new()
            .title(Line::raw("TODO Info").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG)
            .padding(Padding::horizontal(1));

        Paragraph::new(info)
            .block(block)
            .fg(TEXT_FG_COLOR)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}

//! Tabs + table + gauge in one app, proving the data-shaped adapters
//! compose: each panel is built with its own adapter function and merged
//! into one tree with `TreeBuilder::subtree`.
//!
//! Left/Right switch tabs, Up/Down move the table selection on the Tasks
//! tab. Try it with Orca running (see the crate README for the `IsEnabled`
//! gotcha).
use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Cell, Gauge, Paragraph, Row, Table, TableState, Tabs};
use ratatui::{DefaultTerminal, Frame};
use ratatui_a11y::{
    A11y, Role, TreeBuilder, TreeUpdate, gauge_nodes, group_nodes, node_id, table_nodes,
    tabs_nodes, text_nodes,
};

const TAB_TITLES: [&str; 3] = ["Overview", "Tasks", "Progress"];
const OVERVIEW_TEXT: &str = "A small dashboard: pick a tab to see its content.";

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::default().run(terminal))
}

struct Task {
    name: &'static str,
    done: bool,
}

struct App {
    should_exit: bool,
    selected_tab: usize,
    tasks: Vec<Task>,
    table_state: TableState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_exit: false,
            selected_tab: 0,
            tasks: vec![
                Task {
                    name: "Design mockups",
                    done: true,
                },
                Task {
                    name: "Wire up backend",
                    done: false,
                },
                Task {
                    name: "Write tests",
                    done: false,
                },
            ],
            table_state: TableState::default().with_selected(Some(0)),
        }
    }
}

impl App {
    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let mut a11y = A11y::new(self.build_tree());
        a11y.set_focused(true);

        while !self.should_exit {
            terminal.draw(|frame| self.render(frame))?;
            a11y.update(self.build_tree());
            let _ = a11y.actions().count();

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
            KeyCode::Left | KeyCode::Char('h') => {
                self.selected_tab = self
                    .selected_tab
                    .checked_sub(1)
                    .unwrap_or(TAB_TITLES.len() - 1);
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.selected_tab = (self.selected_tab + 1) % TAB_TITLES.len();
            }
            KeyCode::Down | KeyCode::Char('j') if self.selected_tab == 1 => {
                let next = self
                    .table_state
                    .selected()
                    .map_or(0, |i| (i + 1) % self.tasks.len());
                self.table_state.select(Some(next));
            }
            KeyCode::Up | KeyCode::Char('k') if self.selected_tab == 1 => {
                let prev = self
                    .table_state
                    .selected()
                    .map_or(0, |i| i.checked_sub(1).unwrap_or(self.tasks.len() - 1));
                self.table_state.select(Some(prev));
            }
            _ => {}
        }
    }

    fn progress_ratio(&self) -> f64 {
        let done = self.tasks.iter().filter(|t| t.done).count();
        done as f64 / self.tasks.len() as f64
    }

    fn build_tree(&self) -> TreeUpdate {
        let window_id = node_id("dashboard-window");
        let mut tree = TreeBuilder::new();

        let mut tabs = tabs_nodes(TAB_TITLES, Some(self.selected_tab), "dashboard-tabs");
        let tabs_root = tabs.root;
        for (id, node) in tabs.nodes_mut() {
            if *id == tabs_root {
                node.set_label("Dashboard sections");
            }
        }
        let tabs_selected = tabs.selected;
        let tabs_id = tree.subtree(tabs);

        let mut content_selected = None;
        let content_id = match self.selected_tab {
            0 => tree.subtree(text_nodes(OVERVIEW_TEXT, "dashboard-overview")),
            1 => {
                let header = Some(["Task".to_string(), "Status".to_string()]);
                let rows = self.tasks.iter().map(|t| {
                    [
                        t.name.to_string(),
                        if t.done {
                            "done".to_string()
                        } else {
                            "not done".to_string()
                        },
                    ]
                });
                let table = table_nodes(
                    header,
                    rows,
                    self.table_state.selected(),
                    None,
                    "dashboard-tasks",
                );
                content_selected = table.selected;
                tree.subtree(group_nodes("Tasks", [table], "dashboard-tasks-group"))
            }
            _ => tree.subtree(gauge_nodes(
                Some("Overall progress"),
                self.progress_ratio(),
                "dashboard-progress",
            )),
        };

        tree.node(
            window_id,
            Role::Window,
            "Ratatui Dashboard",
            [tabs_id, content_id],
        );
        tree.root(window_id);
        tree.focus(content_selected.or(tabs_selected).unwrap_or(tabs_id));

        tree.build()
    }

    fn render(&mut self, frame: &mut Frame) {
        let [tabs_area, content_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(frame.area());

        let tabs = Tabs::new(TAB_TITLES).select(self.selected_tab);
        frame.render_widget(tabs, tabs_area);

        match self.selected_tab {
            0 => frame.render_widget(Paragraph::new(OVERVIEW_TEXT), content_area),
            1 => {
                let rows = self.tasks.iter().map(|t| {
                    Row::new([
                        Cell::from(t.name),
                        Cell::from(if t.done { "done" } else { "not done" }),
                    ])
                });
                let table = Table::new(rows, [Constraint::Fill(1), Constraint::Length(10)])
                    .header(Row::new(["Task", "Status"]))
                    .row_highlight_style(ratatui::style::Style::new().reversed())
                    .block(Block::new().borders(Borders::TOP).title(Line::raw("Tasks")));
                frame.render_stateful_widget(table, content_area, &mut self.table_state);
            }
            _ => {
                let gauge = Gauge::default()
                    .block(
                        Block::new()
                            .borders(Borders::TOP)
                            .title(Line::raw("Progress")),
                    )
                    .ratio(self.progress_ratio());
                frame.render_widget(gauge, content_area);
            }
        }
    }
}

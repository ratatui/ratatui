//! Queue pane with row targets and inline row actions.

use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui_layout::focus::{FocusState, FocusTarget, FocusTargets};

use crate::model::{Task, TaskId};
use crate::route::{FocusScope, RouteMap, Target};

/// Result of handling a queue-pane key.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum QueueAction {
    /// The key was handled without changing task data.
    Handled(&'static str),
    /// Select a row by index.
    Select(usize),
    /// Run the task with the given id.
    Run(TaskId),
    /// The queue did not handle the key.
    Unhandled,
}

/// Queue pane behavior.
#[derive(Debug, Default)]
pub struct QueuePane;

impl QueuePane {
    /// Renders the queue and returns route plus focus target data.
    pub fn render(
        frame: &mut Frame,
        area: Rect,
        tasks: &[Task],
        selected: usize,
        focused: Option<Target>,
    ) -> QueueFrame {
        let inner = inner_area(area);
        frame.render_widget(Block::new().borders(Borders::ALL).title("queue"), area);

        let mut route = RouteMap::new().target(Target::QueuePane, area, 0);
        let mut focus_targets = Vec::new();
        for (index, task) in tasks.iter().enumerate().take(inner.height as usize) {
            let row = Rect::new(inner.x, inner.y + index as u16, inner.width, 1);
            let action = Rect::new(row.right().saturating_sub(8), row.y, 8.min(row.width), 1);
            let label_width = row.width.saturating_sub(action.width.saturating_add(1));
            let label = Rect::new(row.x, row.y, label_width, 1);
            let row_target = Target::QueueRow(task.id);
            let run_target = Target::QueueRun(task.id);

            route = route
                .target(row_target, row, 1)
                .target(run_target, action, 2);
            focus_targets.push(FocusTarget::new(row_target, row, (index * 2) as u16));
            focus_targets.push(FocusTarget::new(run_target, action, (index * 2 + 1) as u16));

            Self::render_row(frame, task, selected == index, focused, label, action);
        }

        QueueFrame {
            route,
            scope: FocusScope::new(
                Target::QueuePane,
                FocusTargets::from_targets(focus_targets),
                false,
            ),
        }
    }

    /// Handles keys for queue rows and inline row buttons.
    pub fn handle_key(
        key: KeyCode,
        focus: &mut FocusState<Target>,
        scope: &FocusScope,
        tasks: &[Task],
        selected: usize,
    ) -> QueueAction {
        if !scope.owns(focus.focused()) {
            return QueueAction::Unhandled;
        }
        match key {
            KeyCode::Char('j') | KeyCode::Down => {
                let next = (selected + 1).min(tasks.len().saturating_sub(1));
                focus.focus(tasks.get(next).map(|task| Target::QueueRow(task.id)));
                QueueAction::Select(next)
            }
            KeyCode::Char('k') | KeyCode::Up => {
                let previous = selected.saturating_sub(1);
                focus.focus(tasks.get(previous).map(|task| Target::QueueRow(task.id)));
                QueueAction::Select(previous)
            }
            KeyCode::Left => Self::focus_row(focus, tasks, selected),
            KeyCode::Right => Self::focus_run(focus, tasks, selected),
            KeyCode::Enter => Self::activate(focus.focused(), tasks, selected),
            _ => QueueAction::Unhandled,
        }
    }

    fn render_row(
        frame: &mut Frame,
        task: &Task,
        selected: bool,
        focused: Option<Target>,
        label: Rect,
        action: Rect,
    ) {
        let row_focused = focused == Some(Target::QueueRow(task.id));
        let run_focused = focused == Some(Target::QueueRun(task.id));
        let row_style = if row_focused {
            Style::new()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else if selected {
            Style::new().fg(Color::Cyan)
        } else {
            Style::new()
        };
        let text = format!(
            "#{:02} {:28} {:8} {}",
            task.id.0,
            truncate(&task.title, 28),
            task.owner,
            task.state.label()
        );
        frame.render_widget(Paragraph::new(text).style(row_style), label);

        let run_style = if run_focused {
            Style::new()
                .fg(Color::Black)
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::new().fg(Color::Black).bg(Color::Green)
        };
        frame.render_widget(Paragraph::new(" run ").centered().style(run_style), action);
    }

    fn focus_row(focus: &mut FocusState<Target>, tasks: &[Task], selected: usize) -> QueueAction {
        focus.focus(tasks.get(selected).map(|task| Target::QueueRow(task.id)));
        QueueAction::Handled("queue scope focused the selected row")
    }

    fn focus_run(focus: &mut FocusState<Target>, tasks: &[Task], selected: usize) -> QueueAction {
        focus.focus(tasks.get(selected).map(|task| Target::QueueRun(task.id)));
        QueueAction::Handled("queue scope focused the inline action")
    }

    fn activate(focused: Option<Target>, tasks: &[Task], selected: usize) -> QueueAction {
        match focused {
            Some(Target::QueueRun(id)) => QueueAction::Run(id),
            Some(Target::QueueRow(id)) => tasks
                .iter()
                .position(|task| task.id == id)
                .map_or(QueueAction::Unhandled, QueueAction::Select),
            _ => tasks
                .get(selected)
                .map_or(QueueAction::Unhandled, |_| QueueAction::Select(selected)),
        }
    }
}

/// Facts produced by rendering the queue.
#[derive(Debug, Clone)]
pub struct QueueFrame {
    /// Parent-chain route data.
    pub route: RouteMap,
    /// Local queue focus scope.
    pub scope: FocusScope,
}

const fn inner_area(area: Rect) -> Rect {
    Rect::new(
        area.x + 1,
        area.y + 1,
        area.width.saturating_sub(2),
        area.height.saturating_sub(2),
    )
}

fn truncate(value: &str, width: usize) -> String {
    let mut output = value.chars().take(width).collect::<String>();
    if value.chars().count() > width {
        output.pop();
        output.push('.');
    }
    output
}

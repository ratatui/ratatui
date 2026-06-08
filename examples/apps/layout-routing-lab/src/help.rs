//! Help modal with a trapped local focus scope.

use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui_layout::{FocusState, FocusTarget, FocusTargets};

use crate::route::{FocusScope, RouteMap, Target};

/// Rendered data produced by the help modal.
#[derive(Debug, Clone)]
pub struct HelpFrame {
    /// Backdrop, dialog, and close-button route targets.
    pub route: RouteMap,
    /// Trapped close-button focus scope.
    pub scope: FocusScope,
}

/// Help modal behavior.
#[derive(Debug, Default)]
pub struct HelpModal;

impl HelpModal {
    /// Renders the modal and returns route and focus target data.
    pub fn render(frame: &mut Frame, area: Rect) -> HelpFrame {
        let dialog = centered(area, 60, 13);
        let close = Rect::new(
            dialog.right().saturating_sub(12),
            dialog.bottom().saturating_sub(2),
            9,
            1,
        );
        frame.render_widget(Clear, dialog);
        frame.render_widget(
            Block::new().borders(Borders::ALL).title("routing help"),
            dialog,
        );
        let body = Rect::new(
            dialog.x + 2,
            dialog.y + 2,
            dialog.width.saturating_sub(4),
            dialog.height.saturating_sub(5),
        );
        frame.render_widget(Paragraph::new(HELP_TEXT), body);
        frame.render_widget(
            Paragraph::new(" close ").centered().style(
                Style::new()
                    .fg(Color::Black)
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            ),
            close,
        );

        let route = RouteMap::new()
            .target(Target::HelpBackdrop, area, 20)
            .target(Target::HelpDialog, dialog, 30)
            .target(Target::HelpClose, close, 31);
        let scope = FocusScope::new(
            Target::HelpDialog,
            FocusTargets::from_targets([FocusTarget::new(Target::HelpClose, close, 0)]),
            true,
        );
        HelpFrame { route, scope }
    }

    /// Handles keys while the modal is open.
    pub fn handle_key(key: KeyCode, focus: &mut FocusState<Target>) -> bool {
        match key {
            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => true,
            KeyCode::Tab | KeyCode::BackTab => {
                focus.focus(Some(Target::HelpClose));
                false
            }
            _ => false,
        }
    }
}

const HELP_TEXT: &str = "\
Leaf targets see input first. If they do not handle it, the local scope can handle movement or \
fallback policy, then the page can handle global shortcuts.

Try:
  mouse: row background vs row run button
  drag: grab the splitter and move horizontally
  keys: type in fields, Tab inside the form, F6 across panes
  ?: this modal traps focus until closed";

fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let [vertical] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    let [horizontal] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(vertical);
    horizontal
}

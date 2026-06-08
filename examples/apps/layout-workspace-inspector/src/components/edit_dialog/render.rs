//! Rendering and frame-plan construction for the edit dialog.
//!
//! This module owns the visual part of the modal: clearing the overlay area, planning named rows,
//! rendering fields and buttons, and returning the focus, mouse, layout, and cursor requests produced by
//! that render pass.

use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui_layout::{
    Column, Container, ContainerLayout, CursorRequests, FrameSnapshot, FrameTargets, Padding,
    Region, Regions, Row,
};

use super::EditDialog;
use super::fields::{
    CONTROLS, DIALOG_HEIGHT, DIALOG_WIDTH, DialogControl, DialogControlKind, DialogRow,
};
use super::state::DialogState;
use crate::DIALOG_FOCUS;
use crate::ids::{DialogField, TargetId};
use crate::ui::centered;

#[allow(
    clippy::unused_self,
    reason = "render phase helpers stay as methods so the example reads by component"
)]
impl EditDialog {
    /// Renders the dialog overlay and returns its routed data.
    ///
    /// The dialog first draws an overlay shell, then values named rows for fields and buttons. It
    /// returns a frame snapshot containing layout regions, focus targets, mouse targets, and a cursor
    /// request so the next input event can interact with the same fields the user sees.
    pub(crate) fn render(
        &self,
        frame: &mut Frame,
        focused: Option<TargetId>,
    ) -> FrameSnapshot<TargetId> {
        let dialog = centered(frame.area(), DIALOG_WIDTH, DIALOG_HEIGHT);
        let container = self.layout(dialog);
        self.render_shell(frame, container.outer);

        let rows = self.rows(container.inner);
        self.render_fields(frame, &rows, &self.state, focused);
        self.frame_snapshot(container.outer, &rows, focused)
    }

    /// Calculates the padded dialog content area.
    ///
    /// Padding keeps the text fields away from the border. The outer area remains the overlay
    /// rectangle used for clearing and z-ordered routing.
    pub(super) fn layout(&self, area: Rect) -> ContainerLayout<()> {
        Container::<()>::new()
            .padding(Padding::new(2, 2, 2, 1))
            .layout(area)
    }

    /// Clears the dialog area and draws its border.
    ///
    /// `Clear` makes the modal visually replace the page content underneath. The frame snapshot's higher
    /// z values then make mouse routing agree with what was drawn.
    fn render_shell(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Clear, area);
        frame.render_widget(
            Block::new()
                .borders(Borders::ALL)
                .title("edit selected item")
                .border_style(Style::new().fg(Color::Cyan)),
            area,
        );
    }

    /// Plans the dialog rows with named field ids.
    ///
    /// Named ids keep rendering, focus traversal, mouse routing, and cursor placement aligned without
    /// depending on numeric row indices.
    pub(super) fn rows(&self, area: Rect) -> Regions<DialogField> {
        let field_rows = [
            (DialogRow::Title, Constraint::Length(1)),
            (DialogRow::Owner, Constraint::Length(1)),
            (DialogRow::Status, Constraint::Length(1)),
            (DialogRow::Buttons, Constraint::Length(1)),
        ];
        let rows = Column::named(field_rows).spacing(1).regions(area);

        let mut fields = Regions::new(area);
        for control in CONTROLS
            .iter()
            .copied()
            .filter(|control| control.row != DialogRow::Buttons)
        {
            fields.push(Region::new(
                control.field,
                rows.area_for(control.row).expect("dialog row"),
            ));
        }

        let button_area = rows.area_for(DialogRow::Buttons).expect("buttons row");
        let button_slots = CONTROLS
            .iter()
            .copied()
            .filter(|control| control.row == DialogRow::Buttons)
            .map(|control| (control.field, Constraint::Length(10)));
        let buttons = Row::named(button_slots).spacing(2).regions(button_area);
        fields.extend(buttons);
        fields
    }

    /// Renders every planned dialog row.
    ///
    /// This iterates over the same layout that will later become routed targets. Rendering and
    /// interaction therefore share one source of row geometry.
    fn render_fields(
        &self,
        frame: &mut Frame,
        rows: &Regions<DialogField>,
        dialog: &DialogState,
        focused: Option<TargetId>,
    ) {
        for region in rows.regions() {
            self.render_field(frame, region.id, region.area, dialog, focused);
        }
    }

    /// Builds layout, focus, mouse, and cursor requests for the dialog.
    ///
    /// Each row becomes a high-z layout region and mouse target. Focus targets use dialog-specific
    /// ordering so Tab can cycle through fields and buttons while the page remains behind the overlay.
    fn frame_snapshot(
        &self,
        outer: Rect,
        rows: &Regions<DialogField>,
        focused: Option<TargetId>,
    ) -> FrameSnapshot<TargetId> {
        FrameTargets::new(outer, DIALOG_FOCUS)
            .z(20)
            .build_focusable(rows.regions().iter().copied(), TargetId::Dialog)
            .cursor(self.cursor_plan(rows, focused))
    }

    /// Builds a cursor request for the focused editable field.
    ///
    /// The terminal cursor belongs to the frame, not to the widget. The dialog requests a cursor at
    /// the end of the active text field, and `App::render` applies the final cursor from the merged
    /// `CursorRequests`.
    fn cursor_plan(
        &self,
        rows: &Regions<DialogField>,
        focused: Option<TargetId>,
    ) -> CursorRequests {
        let mut cursor = CursorRequests::new();
        if let Some(field) = super::navigation::focused_text_field(focused)
            && let Some(region) = rows.region_for(field.dialog_field())
        {
            let state = self.field_state(field);
            let request =
                state.cursor_request_after_prefix(region.area, field.dialog_field().label_width());
            cursor.push(request);
        }
        cursor
    }

    /// Renders one dialog row as either an editable field or an action button.
    ///
    /// The field enum drives both behavior and presentation: text fields display labels and values,
    /// while save/cancel display button-like labels. Focus styling uses the routed dialog id.
    fn render_field(
        &self,
        frame: &mut Frame,
        field: DialogField,
        area: Rect,
        dialog: &DialogState,
        focused: Option<TargetId>,
    ) {
        let selected = focused == Some(TargetId::Dialog(field));
        let control = DialogControl::for_field(field);
        match control.kind {
            DialogControlKind::Button => self.render_button(frame, field, area, selected),
            DialogControlKind::Text(_) | DialogControlKind::Status => {
                self.render_text_field(frame, field, area, dialog, selected);
            }
        }
    }

    /// Renders an action button with its role-specific color.
    fn render_button(&self, frame: &mut Frame, field: DialogField, area: Rect, selected: bool) {
        let style = button_style(field, selected);
        frame.render_widget(Paragraph::new(field.label()).centered().style(style), area);
    }

    /// Renders an editable text field with stable label styling.
    ///
    /// Focus highlights only the editable value. The label remains visually steady so the user can
    /// scan field names while seeing exactly which text will receive typing.
    fn render_text_field(
        &self,
        frame: &mut Frame,
        field: DialogField,
        area: Rect,
        dialog: &DialogState,
        selected: bool,
    ) {
        let value_style = if selected {
            Style::new()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        } else {
            Style::new()
        };
        let text = Line::from(vec![
            Span::raw(format!("{}: ", field.label())),
            Span::styled(dialog.value(field), value_style),
        ]);
        frame.render_widget(Paragraph::new(text), area);
    }

    /// Creates an edit dialog directly from state for focused tests.
    #[cfg(test)]
    pub(super) fn from_state(state: DialogState) -> Self {
        let mut buttons = ratatui_layout::ButtonRow::new(super::fields::button_fields());
        buttons.focus_id(&DialogField::Cancel);
        Self {
            title_field: ratatui_layout::TextFieldState::at_end(&state.title),
            owner_field: ratatui_layout::TextFieldState::at_end(&state.owner),
            buttons,
            state,
        }
    }
}

/// Returns the button style for selected and idle states.
const fn button_style(field: DialogField, selected: bool) -> Style {
    match (field, selected) {
        (DialogField::Save, true) => Style::new()
            .fg(Color::Black)
            .bg(Color::LightGreen)
            .add_modifier(Modifier::BOLD),
        (DialogField::Cancel, true) => Style::new()
            .fg(Color::Black)
            .bg(Color::LightRed)
            .add_modifier(Modifier::BOLD),
        (DialogField::Save, false) => Style::new().fg(Color::Black).bg(Color::Green),
        (DialogField::Cancel, false) => Style::new().fg(Color::White).bg(Color::Red),
        _ => Style::new(),
    }
}

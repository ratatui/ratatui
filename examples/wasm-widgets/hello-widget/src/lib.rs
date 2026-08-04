use crate::exports::ratatui::widget::widget::{Cell, Event, Guest, Rect, WidgetError};

wit_bindgen::generate!({
    path: "../../../ratatui-wasm/wit/widget.wit",
    world: "wasm-widget",
});

struct HelloWidget;

impl Guest for HelloWidget {
    fn render(area: Rect, _state: Option<Vec<u8>>) -> Result<Vec<Cell>, WidgetError> {
        let mut cells = Vec::new();
        let text = "Hello from WASM";
        for (i, ch) in text.chars().enumerate() {
            let x = u16::try_from(i).unwrap_or(0);
            if x >= area.width {
                break;
            }
            cells.push(Cell {
                x,
                y: 0,
                symbol: ch.to_string(),
                fg: Some("#00ff00".to_string()),
                bg: Some("#000000".to_string()),
            });
        }
        Ok(cells)
    }

    fn handle_event(_event: Event) -> Result<bool, WidgetError> {
        Ok(false)
    }

    fn capabilities() -> Vec<String> {
        Vec::new()
    }
}

export!(HelloWidget);

use crate::exports::ratatui::widget::widget::{
    Cell, Color, Event, Guest, Rect, RenderCommand, RenderResult, WidgetError,
};

wit_bindgen::generate!({
    path: "../../../ratatui-wasm/wit/widget.wit",
    world: "wasm-widget",
});

struct HelloWidget;

impl Guest for HelloWidget {
    fn render(area: Rect, _state: Option<Vec<u8>>) -> Result<RenderResult, WidgetError> {
        let mut cells = Vec::new();
        let text = "Hello from WASM";
        for (i, ch) in text.chars().enumerate() {
            let x = u16::try_from(i).unwrap_or(0);
            if x >= area.width {
                break;
            }
            cells.push(RenderCommand::Cell(Cell {
                x,
                y: 0,
                symbol: ch.to_string(),
                fg: Some(Color::Rgb(crate::exports::ratatui::widget::widget::RgbColor {
                    r: 0,
                    g: 255,
                    b: 0,
                })),
                bg: Some(Color::Rgb(crate::exports::ratatui::widget::widget::RgbColor {
                    r: 0,
                    g: 0,
                    b: 0,
                })),
            }));
        }
        Ok(RenderResult {
            commands: cells,
            state: None,
        })
    }

    fn handle_event(_event: Event) -> Result<bool, WidgetError> {
        Ok(false)
    }

    fn capabilities() -> Vec<String> {
        Vec::new()
    }
}

export!(HelloWidget);

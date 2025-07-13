use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::widgets::StatefulWidget;
use ratatui_widgets::list::{List, ListState};
use ratatui_widgets::table::HighlightSpacing;

fn main() {
    // Test with HighlightSpacing::Always and no selection
    let list = List::new(["Item 0", "Item 1", "Item 2"])
        .highlight_symbol(">>")
        .highlight_spacing(HighlightSpacing::Always);
    let mut state = ListState::default();
    let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 5));
    
    StatefulWidget::render(list, buffer.area, &mut buffer, &mut state);
    
    println!("Buffer with HighlightSpacing::Always, no selection:");
    for line in buffer.content() {
        println!("{:?}", line);
    }
    
    // Test with selection
    state.select(Some(1));
    let mut buffer2 = Buffer::empty(Rect::new(0, 0, 10, 5));
    let list2 = List::new(["Item 0", "Item 1", "Item 2"])
        .highlight_symbol(">>")
        .highlight_spacing(HighlightSpacing::Always);
    
    StatefulWidget::render(list2, buffer2.area, &mut buffer2, &mut state);
    
    println!("\nBuffer with HighlightSpacing::Always, with selection:");
    for line in buffer2.content() {
        println!("{:?}", line);
    }
}

use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::widgets::StatefulWidget;
use ratatui_widgets::list::{List, ListState};
use ratatui_widgets::table::HighlightSpacing;

fn main() {
    println!("Testing HighlightSpacing::Always behavior...");
    
    // Test 1: No selection with Always
    let list = List::new(["Item 0", "Item 1", "Item 2"])
        .highlight_symbol(">>")
        .highlight_spacing(HighlightSpacing::Always);
    let mut state = ListState::default();
    let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 5));
    
    StatefulWidget::render(&list, buffer.area, &mut buffer, &mut state);
    
    println!("No selection:");
    for (i, line) in buffer.content().chunks(10).enumerate() {
        if i < 5 {
            let line_str: String = line.iter().map(|cell| cell.symbol()).collect();
            println!("'{}'", line_str);
        }
    }
    
    // Test 2: With selection
    state.select(Some(1));
    let mut buffer2 = Buffer::empty(Rect::new(0, 0, 10, 5));
    
    StatefulWidget::render(&list, buffer2.area, &mut buffer2, &mut state);
    
    println!("\nWith selection (item 1):");
    for (i, line) in buffer2.content().chunks(10).enumerate() {
        if i < 5 {
            let line_str: String = line.iter().map(|cell| cell.symbol()).collect();
            println!("'{}'", line_str);
        }
    }
    
    // Test 3: WhenSelected for comparison
    let list3 = List::new(["Item 0", "Item 1", "Item 2"])
        .highlight_symbol(">>")
        .highlight_spacing(HighlightSpacing::WhenSelected);
    let mut state3 = ListState::default();
    let mut buffer3 = Buffer::empty(Rect::new(0, 0, 10, 5));
    
    StatefulWidget::render(&list3, buffer3.area, &mut buffer3, &mut state3);
    
    println!("\nWhenSelected, no selection:");
    for (i, line) in buffer3.content().chunks(10).enumerate() {
        if i < 5 {
            let line_str: String = line.iter().map(|cell| cell.symbol()).collect();
            println!("'{}'", line_str);
        }
    }
}

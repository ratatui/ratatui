# Resizable Inline Viewport

This example demonstrates the new resizable inline viewport functionality in Ratatui. It shows how
to create an inline viewport with a fixed height that can be dynamically resized during runtime
using the `set_viewport_height()` method.

## Features

- **Dynamic Resizing**: Increase or decrease the viewport height in real-time
- **Interactive Controls**: Simple keyboard controls for resizing operations
- **Visual Feedback**: Live display of current height and runtime statistics
- **Message Log**: Track all resize operations with timestamps
- **Height Indicator**: Visual gauge showing current height as percentage of maximum

## Controls

- **`+` or `=`**: Increase viewport height (max: 20 lines)
- **`-` or `_`**: Decrease viewport height (min: 3 lines)
- **`r`**: Reset viewport to initial height (6 lines)
- **`i`**: Insert a line before the viewport using `insert_before()`
- **`q`**: Quit the application

## Key Concepts Demonstrated

1. **Inline Viewport Creation**: Using `Viewport::Inline(height)` to create a viewport that appears
   inline with terminal content
2. **Dynamic Resizing**: Using `terminal.set_viewport_height(new_height)` to change the viewport
   size after creation
3. **Content Insertion**: Using `terminal.insert_before(height, draw_fn)` to add content above the viewport
4. **Buffer Management**: The terminal automatically handles buffer reallocation and clearing when resizing
5. **Event Handling**: Responding to keyboard input to trigger resize and insertion operations

## Running the Example

```bash
cargo run --project inline-resizable
```

## Code Structure

- **AppState**: Manages application state including current height, messages, and statistics
- **Event Handling**: Separate thread for input and tick events
- **Render Function**: Displays the UI with status information, controls, message log, and height gauge
- **Resize Logic**: Handles viewport height changes with validation and feedback

## Implementation Notes

- The viewport height is constrained between 3 and 20 lines for demonstration purposes
- Each resize operation is logged to show the functionality in action
- The example uses a tick-based rendering system to update runtime statistics
- Auto-resize is handled for terminal window changes

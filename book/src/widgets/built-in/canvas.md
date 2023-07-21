# Canvas

Canvas details and examples ...

Discussion about [examples/canvas.rs](https://github.com/ratatui-org/ratatui/blob/main/examples/canvas.rs) ...

```rust
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("World"))
        .marker(app.marker)
        .paint(|ctx| {
            ctx.draw(&Map {
                color: Color::White,
                resolution: MapResolution::High,
            });
            ctx.print(app.x, -app.y, "You are here".yellow());
        })
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0]);
```

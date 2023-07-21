# Quickstart

The following example demonstrates the minimal amount of code necessary to setup a terminal and
render "Hello World!". The full code for this example which contains a little more detail is in
[hello_world.rs](./examples/hello_world.rs). For more guidance on how to create Ratatui apps, see
the [Docs](https://docs.rs/ratatui) and [Examples](#examples). There is also a starter template
available at [rust-tui-template](https://github.com/tui-rs-revival/rust-tui-template).

```rust
fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;
    run(&mut terminal)?;
    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    Ok(loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Hello World!");
            frame.render_widget(greeting, frame.size());
        })?;
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if KeyCode::Char('q') == key.code {
                    break;
                }
            }
        }
    })
}
```

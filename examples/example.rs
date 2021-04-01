mod util;

use crate::util::{
    event::{Event, Events},
    StatefulTree,
};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
    Terminal,
};

use tui_tree_widget::{Tree, TreeItem};

struct App<'a> {
    tree: StatefulTree<'a>,
}

impl<'a> App<'a> {
    fn new() -> Self {
        Self {
            tree: StatefulTree::with_items(vec![
                TreeItem::new_leaf("a"),
                TreeItem::new(
                    "b",
                    vec![
                        TreeItem::new_leaf("c"),
                        TreeItem::new("d", vec![TreeItem::new_leaf("e"), TreeItem::new_leaf("f")]),
                        TreeItem::new_leaf("g"),
                    ],
                ),
                TreeItem::new_leaf("h"),
            ]),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    // App
    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            let area = f.size();

            let items = Tree::new(app.tree.items.to_vec())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!("Tree Widget {:?}", app.tree.state)),
                )
                .highlight_style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");
            f.render_stateful_widget(items, area, &mut app.tree.state);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Left => {
                    app.tree.close();
                }
                Key::Right => {
                    app.tree.open();
                }
                Key::Down => {
                    app.tree.next();
                }
                Key::Up => {
                    app.tree.previous();
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}

//! State like [`ListState`], [`TableState`] and [`ScrollbarState`] can be serialized and
//! deserialized through serde. This allows saving your entire state to disk when the user exits the
//! the app, and restore it again upon re-opening the app.
//! This way, they get right back to where they were, without having to re-seek to their previous
//! position, if that's applicable for the app at hand.
//!
//! **Note**: For this pattern to work easily, you need to have some toplevel struct which stores
//! _only_ state and not any draw commands.
//!
//! **Note**: For many applications, it might be beneficial to instead keep your own state and
//! instead construct the state for widgets on the fly instead, if that allows you to express you
//! the semantic meaning of your state better or only fetch part of a dataset.

use ratatui::{backend::TestBackend, prelude::*, widgets::*};
use serde::{Deserialize, Serialize};

#[test]
fn state_serde_roundtrip() {
    // default unmodified case
    test_case(
        0,
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│awa                         │",
            "│banana                      │",
            "│Cats!!                      │",
            "│d20                         │",
            "│Echo                        │",
            "│Foxtrot                     │",
            "│Golf                        │",
            "│Hotel                       │",
            "└────────────────────────────┘",
        ]),
    );
    // scrolled, but not cut off
    test_case(
        1,
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│banana                      │",
            "│Cats!!                      │",
            "│d20                         │",
            "│Echo                        │",
            "│Foxtrot                     │",
            "│Golf                        │",
            "│Hotel                       │",
            "│IwI                         │",
            "└────────────────────────────┘",
        ]),
    );
    // scrolled and partly cut off
    test_case(
        4,
        Buffer::with_lines(vec![
            "┌────────────────────────────┐",
            "│Echo                        │",
            "│Foxtrot                     │",
            "│Golf                        │",
            "│Hotel                       │",
            "│IwI                         │",
            "│Juliett                     │",
            "│                            │",
            "│                            │",
            "└────────────────────────────┘",
        ]),
    );
}

fn test_case(scroll: usize, expected: Buffer) {
    // make sure the buffer is what we expect it to be
    // then "forget" the original state
    let serialized = {
        let mut state = State {
            list: ListState::default()
                .with_offset(scroll)
                .with_selected(Some(scroll)),
        };

        state.check(&expected);
        serde_json::to_string_pretty(&state).unwrap()
    };

    // at this point nothing is carried over except for
    // 1. the serialized state
    // 2. the expected target buffer

    // so reconstruct the state
    let mut state: State = serde_json::from_str(&serialized).unwrap();
    state.check(&expected)
}

#[derive(Serialize, Deserialize)]
struct State {
    list: ListState,
}

impl State {
    fn check(&mut self, expected: &Buffer) {
        // In an actual application, you'd want to create a backend once and store it somewhere
        let backend = TestBackend::new(30, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let list = List::new([
                    "awa", "banana", "Cats!!", "d20", "Echo", "Foxtrot", "Golf", "Hotel", "IwI",
                    "Juliett",
                ])
                .block(Block::default().borders(Borders::ALL));
                f.render_stateful_widget(list, f.size(), &mut self.list);
            })
            .unwrap();
        terminal.backend().assert_buffer(expected);
    }
}

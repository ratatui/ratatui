//! State like [`ListState`], [`TableState`] and [`ScrollbarState`] can be serialized and
//! deserialized through the [`serde`] and [`rkyv`] crates.
//!
//! This allows saving your entire state to disk when the user exits the app, and restore it again
//! upon re-opening the app. This way, they get right back to where they were, without having to
//! re-seek to their previous position, if that's applicable for the app at hand.
//!
//! It might also be useful to synchronize state with another machine, in which case you might want
//! to look at [CRDT]s as implemented by for example the [`yrs`] or [`crdts`] crates though.
//!
//! **Note**: For serialization to work easily, you need to have some toplevel struct which stores
//! _only_ state and not any draw commands.
//!
//! **Note**: For many applications, it might be beneficial to instead keep your own state and
//! instead construct the state for widgets on the fly instead, if that allows you to express you
//! the semantic meaning of your state better or only fetch part of a dataset.
//!
//! [`serde`]: https://crates.io/crates/serde
//! [`rkyv`]: https://crates.io/crates/rkyv
//! [CRDT]: https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type
//! [`yrs`]: https://crates.io/crates/yrs
//! [`crdts`]: https://crates.io/crates/crdts

// not too happy about the redundancy in these tests,
// but if that helps readability then it's ok i guess /shrug

// regenerate the rkyv smoke refs in this with python, where `a` is the list emitted by the failure:
// print(str(list(map(lambda x: hex(x), a))).replace("'", ""))

use ratatui::{backend::TestBackend, prelude::*, widgets::*};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
)]
#[archive(check_bytes)]
struct AppState {
    list_state: ListState,
    table_state: TableState,
    scrollbar_state: ScrollbarState,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            list_state: ListState::default(),
            table_state: TableState::default(),
            scrollbar_state: ScrollbarState::new(10),
        }
    }
}

impl AppState {
    fn select(&mut self, index: usize) {
        self.list_state.select(Some(index));
        self.table_state.select(Some(index));
        self.scrollbar_state = self.scrollbar_state.position(index);
    }

    #[track_caller]
    fn serialize_serde(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    #[track_caller]
    fn deserialize_serde(repr: &str) -> Self {
        serde_json::from_str(repr).unwrap()
    }

    #[track_caller]
    fn serialize_rkyv(&self) -> rkyv::util::AlignedVec {
        rkyv::to_bytes::<_, 512>(self).unwrap()
    }

    #[track_caller]
    fn deserialize_rkyv(repr: &[u8]) -> Self {
        rkyv::from_bytes(repr).unwrap()
    }
}

/// Renders the list to a TestBackend and asserts that the result matches the expected buffer.
#[track_caller]
fn assert_buffer(state: &mut AppState, expected: &Buffer) {
    let backend = TestBackend::new(21, 5);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|f| {
            let items = vec![
                "awa", "banana", "Cats!!", "d20", "Echo", "Foxtrot", "Golf", "Hotel", "IwI",
                "Juliett",
            ];

            use Constraint::*;
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Length(10), Length(10), Length(1)])
                .split(f.size());
            let list = List::new(items.clone())
                .highlight_symbol(">>")
                .block(Block::default().borders(Borders::RIGHT));
            f.render_stateful_widget(list, layout[0], &mut state.list_state);

            let table = Table::new(
                items.iter().map(|i| Row::new(vec![*i])),
                [Constraint::Length(10); 5],
            )
            .highlight_symbol(">>");
            f.render_stateful_widget(table, layout[1], &mut state.table_state);

            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
            f.render_stateful_widget(scrollbar, layout[2], &mut state.scrollbar_state);
        })
        .unwrap();
    terminal.backend().assert_buffer(&expected);
}

const DEFAULT_STATE_BUFFER: [&'static str; 5] = [
    "awa      │awa       ▲",
    "banana   │banana    █",
    "Cats!!   │Cats!!    ║",
    "d20      │d20       ║",
    "Echo     │Echo      ▼",
];

const DEFAULT_STATE_REPR_SERDE: &'static str = r#"{
  "list_state": {
    "offset": 0,
    "selected": null
  },
  "table_state": {
    "offset": 0,
    "selected": null
  },
  "scrollbar_state": {
    "content_length": 10,
    "position": 0,
    "viewport_content_length": 0
  }
}"#;
const DEFAULT_STATE_REPR_RKYV: [u8; 36] = [
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0xa, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];

#[test]
fn default_state_serialize() {
    let mut state = AppState::default();

    let expected = Buffer::with_lines(DEFAULT_STATE_BUFFER.to_vec());
    assert_buffer(&mut state, &expected);

    let ser_state = state.serialize_serde();
    assert_eq!(ser_state, DEFAULT_STATE_REPR_SERDE);

    let ser_state = state.serialize_rkyv();
    assert_eq!(ser_state.into_vec(), DEFAULT_STATE_REPR_RKYV.to_vec());
}

#[test]
fn default_state_deserialize() {
    let expected = Buffer::with_lines(DEFAULT_STATE_BUFFER.to_vec());

    let mut state = AppState::deserialize_serde(DEFAULT_STATE_REPR_SERDE);
    assert_buffer(&mut state, &expected);

    let mut state = AppState::deserialize_rkyv(&DEFAULT_STATE_REPR_RKYV);
    assert_buffer(&mut state, &expected);
}

const SELECTED_STATE_BUFFER: [&'static str; 5] = [
    "  awa    │  awa     ▲",
    ">>banana │>>banana  █",
    "  Cats!! │  Cats!!  ║",
    "  d20    │  d20     ║",
    "  Echo   │  Echo    ▼",
];
const SELECTED_STATE_REPR_SERDE: &'static str = r#"{
  "list_state": {
    "offset": 0,
    "selected": 1
  },
  "table_state": {
    "offset": 0,
    "selected": 1
  },
  "scrollbar_state": {
    "content_length": 10,
    "position": 1,
    "viewport_content_length": 0
  }
}"#;
const SELECTED_STATE_REPR_RKYV: [u8; 36] = [
    0x1, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0xa, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];

#[test]
fn selected_state_serialize() {
    let mut state = AppState::default();
    state.select(1);

    let expected = Buffer::with_lines(SELECTED_STATE_BUFFER.to_vec());
    assert_buffer(&mut state, &expected);

    let ser_state = state.serialize_serde();
    assert_eq!(ser_state, SELECTED_STATE_REPR_SERDE);

    let ser_state = state.serialize_rkyv();
    assert_eq!(ser_state.into_vec(), SELECTED_STATE_REPR_RKYV.to_vec());
}

#[test]
fn selected_state_deserialize() {
    let expected = Buffer::with_lines(SELECTED_STATE_BUFFER.to_vec());

    let mut state = AppState::deserialize_serde(SELECTED_STATE_REPR_SERDE);
    assert_buffer(&mut state, &expected);

    // apparently something between rkyv/rustc is not playing well with const in this specific case
    // causing the const to be misaligned
    // but that doesn't happen if the variable is locally bound instead
    // so work around that for now, until it's fixed in rkyv or the like
    let repr = SELECTED_STATE_REPR_RKYV;
    let mut state = AppState::deserialize_rkyv(&repr);
    assert_buffer(&mut state, &expected);
}

const SCROLLED_STATE_BUFFER: [&'static str; 5] = [
    "  Echo   │  Echo    ▲",
    "  Foxtrot│  Foxtrot ║",
    "  Golf   │  Golf    ║",
    "  Hotel  │  Hotel   █",
    ">>IwI    │>>IwI     ▼",
];

const SCROLLED_STATE_REPR_SERDE: &'static str = r#"{
  "list_state": {
    "offset": 4,
    "selected": 8
  },
  "table_state": {
    "offset": 4,
    "selected": 8
  },
  "scrollbar_state": {
    "content_length": 10,
    "position": 8,
    "viewport_content_length": 0
  }
}"#;
const SCROLLED_STATE_REPR_RKYV: [u8; 36] = [
    0x1, 0x0, 0x0, 0x0, 0x8, 0x0, 0x0, 0x0, 0x4, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x8, 0x0, 0x0,
    0x0, 0x4, 0x0, 0x0, 0x0, 0xa, 0x0, 0x0, 0x0, 0x8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];

#[test]
fn scrolled_state_serialize() {
    let mut state = AppState::default();
    state.select(8);

    let expected = Buffer::with_lines(SCROLLED_STATE_BUFFER.to_vec());
    assert_buffer(&mut state, &expected);

    let ser_state = state.serialize_serde();
    assert_eq!(ser_state, SCROLLED_STATE_REPR_SERDE);

    let ser_state = state.serialize_rkyv();
    assert_eq!(ser_state.into_vec(), SCROLLED_STATE_REPR_RKYV.to_vec());
}

#[test]
fn scrolled_state_deserialize() {
    let expected = Buffer::with_lines(SCROLLED_STATE_BUFFER.to_vec());

    let mut state = AppState::deserialize_serde(SCROLLED_STATE_REPR_SERDE);
    assert_buffer(&mut state, &expected);

    let mut state = AppState::deserialize_rkyv(&SCROLLED_STATE_REPR_RKYV);
    assert_buffer(&mut state, &expected);
}

pub mod event;

use tui_tree_widget::{flatten, identifier, TreeItem, TreeState};

pub struct StatefulTree<'a> {
    pub state: TreeState,
    pub items: Vec<TreeItem<'a>>,
}

impl<'a> StatefulTree<'a> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            state: TreeState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<TreeItem<'a>>) -> Self {
        Self {
            state: TreeState::default(),
            items,
        }
    }

    fn move_up_down(&mut self, down: bool) {
        let visible = flatten(&self.state.get_all_opened(), &self.items);
        let current_identifier = self.state.selected();
        let current_index = visible
            .iter()
            .position(|o| o.identifier == current_identifier);
        let new_index = current_index.map_or(0, |current_index| {
            if down {
                current_index.saturating_add(1)
            } else {
                current_index.saturating_sub(1)
            }
            .min(visible.len() - 1)
        });
        let new_identifier = visible.get(new_index).unwrap().identifier.to_owned();
        self.state.select(new_identifier);
    }

    pub fn next(&mut self) {
        self.move_up_down(true);
    }

    pub fn previous(&mut self) {
        self.move_up_down(false);
    }

    pub fn close(&mut self) {
        let selected = self.state.selected();
        if !self.state.close(&selected) {
            let (head, _) = identifier::get_without_leaf(&selected);
            self.state.select(head);
        }
    }

    pub fn open(&mut self) {
        self.state.open(self.state.selected());
    }
}

use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex, PoisonError};

use accesskit::{ActionHandler, ActionRequest, ActivationHandler, DeactivationHandler, TreeUpdate};
use accesskit_unix::Adapter;

/// Wires a Ratatui app into the Linux AT-SPI accessibility bus.
///
/// See the crate docs for the `IsEnabled` gotcha: creating this does
/// nothing observable unless the desktop has accessibility turned on.
pub struct A11y {
    adapter: Adapter,
    last_tree: Arc<Mutex<TreeUpdate>>,
    actions: Receiver<ActionRequest>,
}

impl A11y {
    /// Creates the adapter with an initial tree. `initial` is what's served
    /// to AT clients that connect before the app's first [`Self::update`]
    /// call, so it should be a real, complete tree rather than a
    /// placeholder.
    pub fn new(initial: TreeUpdate) -> Self {
        let last_tree = Arc::new(Mutex::new(initial));
        let (tx, rx) = mpsc::channel();

        let adapter = Adapter::new(
            Activation {
                tree: Arc::clone(&last_tree),
            },
            Actions { tx },
            Deactivation,
        );

        Self {
            adapter,
            last_tree,
            actions: rx,
        }
    }

    /// Pushes a new tree snapshot. Call this after every render.
    /// This does no work at all when no AT client is attached: the closure that would
    /// clone and stash `tree` only runs if the platform side is actually listening.
    pub fn update(&mut self, tree: TreeUpdate) {
        let last_tree = Arc::clone(&self.last_tree);
        self.adapter.update_if_active(move || {
            *lock(&last_tree) = tree.clone();
            tree
        });
    }

    /// Tells the platform side whether the app currently holds terminal
    /// focus. Drive this from your backend's focus-gained/focus-lost events.
    pub fn set_focused(&mut self, is_focused: bool) {
        self.adapter.update_window_focus_state(is_focused);
    }

    /// Drains any actions AT clients have requested since the last poll.
    /// Call this once per event-loop iteration; it never blocks.
    pub fn actions(&self) -> impl Iterator<Item = ActionRequest> + '_ {
        self.actions.try_iter()
    }
}

/// A poisoned lock here just means a prior update panicked mid-write; the
/// stale-but-valid tree underneath is still fine to read or overwrite, so
/// recover instead of taking the whole app down over an accessibility
/// side-channel.
fn lock(tree: &Mutex<TreeUpdate>) -> std::sync::MutexGuard<'_, TreeUpdate> {
    tree.lock().unwrap_or_else(PoisonError::into_inner)
}

struct Activation {
    tree: Arc<Mutex<TreeUpdate>>,
}

impl ActivationHandler for Activation {
    fn request_initial_tree(&mut self) -> Option<TreeUpdate> {
        Some(lock(&self.tree).clone())
    }
}

struct Actions {
    tx: Sender<ActionRequest>,
}

impl ActionHandler for Actions {
    fn do_action(&mut self, request: ActionRequest) {
        // The receiver may already be gone at shutdown; nothing to do about that.
        let _ = self.tx.send(request);
    }
}

struct Deactivation;

impl DeactivationHandler for Deactivation {
    fn deactivate_accessibility(&mut self) {}
}

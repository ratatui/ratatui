//! Flatten app-owned tree state into a virtual list without giving the list tree semantics.
//!
//! Expansion, stable ids, and indentation belong to `App`. `VirtualList` only measures and renders
//! the visible rows by index, then returns a hit-testable layout for the current frame.

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Paragraph, Widget};
use ratatui_layout::list::{
    ListHeightCache, ListItemContext, ListItems, VirtualList, VirtualListState,
};
use ratatui_layout::participant::MeasureContext;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut app = App::default();
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| app.render(frame))?;
            if let Some(key) = event::read()?.as_key_press_event()
                && !app.handle_key(key.code)
            {
                break Ok(());
            }
        }
    })
}

#[derive(Default)]
struct App {
    state: VirtualListState,
    cache: ListHeightCache,
    expanded: Expanded,
}

impl App {
    fn render(&mut self, frame: &mut Frame) {
        let visible = self.visible_nodes();
        self.ensure_selection(&visible);
        let mut rows = TreeRows {
            visible,
            expanded: self.expanded,
        };
        VirtualList::new().render_cached(
            frame.area(),
            frame.buffer_mut(),
            &mut self.state,
            &mut rows,
            &mut self.cache,
        );
    }

    fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char(' ') | KeyCode::Enter => self.toggle_selected(),
            KeyCode::Char('q') | KeyCode::Esc => return Self::quit(),
            _ => {}
        }
        true
    }

    fn visible_nodes(&self) -> Vec<VisibleNode> {
        let mut visible = Vec::new();
        visible.push(VisibleNode {
            id: NodeId::Project,
            depth: 0,
        });
        if self.expanded.project {
            visible.push(VisibleNode {
                id: NodeId::Src,
                depth: 1,
            });
            if self.expanded.src {
                visible.push(VisibleNode {
                    id: NodeId::Lib,
                    depth: 2,
                });
                visible.push(VisibleNode {
                    id: NodeId::Main,
                    depth: 2,
                });
            }
            visible.push(VisibleNode {
                id: NodeId::Docs,
                depth: 1,
            });
            if self.expanded.docs {
                visible.push(VisibleNode {
                    id: NodeId::Readme,
                    depth: 2,
                });
                visible.push(VisibleNode {
                    id: NodeId::Guide,
                    depth: 2,
                });
            }
        }
        visible
    }

    const fn ensure_selection(&mut self, visible: &[VisibleNode]) {
        if visible.is_empty() {
            self.state.select(None);
            return;
        }

        if self.state.selected().is_none() {
            self.state.select(Some(0));
        }
    }

    fn select_next(&mut self) {
        let len = self.visible_nodes().len();
        self.state.select_relative(1, len);
    }

    fn select_previous(&mut self) {
        let len = self.visible_nodes().len();
        self.state.select_relative(-1, len);
    }

    fn toggle_selected(&mut self) {
        let visible = self.visible_nodes();
        let Some(index) = self.state.selected() else {
            return;
        };
        let Some(node) = visible.get(index).copied() else {
            return;
        };
        self.expanded.toggle(node.id);
        self.cache.clear();
    }

    const fn quit() -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy)]
struct Expanded {
    project: bool,
    src: bool,
    docs: bool,
}

impl Default for Expanded {
    fn default() -> Self {
        Self {
            project: true,
            src: false,
            docs: false,
        }
    }
}

impl Expanded {
    const fn toggle(&mut self, id: NodeId) {
        match id {
            NodeId::Project => self.project = !self.project,
            NodeId::Src => self.src = !self.src,
            NodeId::Docs => self.docs = !self.docs,
            NodeId::Lib | NodeId::Main | NodeId::Readme | NodeId::Guide => {}
        }
    }

    const fn is_expanded(self, id: NodeId) -> bool {
        match id {
            NodeId::Project => self.project,
            NodeId::Src => self.src,
            NodeId::Docs => self.docs,
            NodeId::Lib | NodeId::Main | NodeId::Readme | NodeId::Guide => false,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum NodeId {
    Project,
    Src,
    Lib,
    Main,
    Docs,
    Readme,
    Guide,
}

impl NodeId {
    const fn label(self) -> &'static str {
        match self {
            Self::Project => "ratatui-layout",
            Self::Src => "src",
            Self::Lib => "lib.rs",
            Self::Main => "main.rs",
            Self::Docs => "docs",
            Self::Readme => "README.md",
            Self::Guide => "layout.md",
        }
    }

    const fn expandable(self) -> bool {
        matches!(self, Self::Project | Self::Src | Self::Docs)
    }
}

#[derive(Debug, Clone, Copy)]
struct VisibleNode {
    id: NodeId,
    depth: u16,
}

struct TreeRows {
    visible: Vec<VisibleNode>,
    expanded: Expanded,
}

impl ListItems for TreeRows {
    fn len(&self) -> usize {
        self.visible.len()
    }

    fn height_for_width(&self, _: usize, _: u16, _: MeasureContext) -> u16 {
        1
    }

    fn render_item(&mut self, index: usize, area: Rect, buf: &mut Buffer, ctx: ListItemContext) {
        let node = self.visible[index];
        let style = if ctx.render.state.selected {
            Style::new().add_modifier(Modifier::REVERSED)
        } else {
            Style::new()
        };
        let marker = if self.expanded.is_expanded(node.id) {
            "-"
        } else if node.id.expandable() {
            "+"
        } else {
            " "
        };
        let text = format!(
            "{}{} {}",
            "  ".repeat(node.depth as usize),
            marker,
            node.id.label()
        );
        Paragraph::new(text).style(style).render(area, buf);
    }
}

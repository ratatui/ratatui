//! Out-of-band data attached to buffer cells, such as hyperlinks.
//!
//! This module is only available with the `cell-extensions` feature.
//!
//! A [`Cell`](crate::buffer::Cell) holds one grapheme cluster and its style. Some terminal
//! features don't fit that model because they are written as escape sequences *around* the
//! symbol rather than as part of it. [OSC 8 hyperlinks] are the main example: the link has to be
//! opened before the first linked cell is printed and closed after the last one.
//!
//! Extensions store that data next to the symbol, in a [`CellExt`]. Backends write it around the
//! symbol as run state, the same way they handle colors, so it never affects the symbol's text or
//! width. Extensions take part in cell equality, so a cell whose only change is its extension is
//! redrawn by the next diff.
//!
//! [OSC 8 hyperlinks]: https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda

// Extensions are shared between cells with `Arc`, which needs atomic pointers. Fail with a clear
// message instead of an unresolved import on targets without them (e.g. thumbv6m-none-eabi).
#[cfg(not(target_has_atomic = "ptr"))]
compile_error!(
    "the `cell-extensions` feature requires a target with atomic pointer support \
     (`target_has_atomic = \"ptr\"`)"
);

#[cfg(target_has_atomic = "ptr")]
use alloc::sync::Arc;
use core::fmt;
use core::hash::{Hash, Hasher};

use compact_str::{CompactString, format_compact};

use crate::layout::Rect;

/// An [OSC 8] terminal hyperlink.
///
/// A hyperlink has a target URI and an optional `id`. Terminals use the `id` to treat cells that
/// are not printed in one contiguous run, such as a link that wraps onto a second line or a link
/// redrawn by a partial diff, as a single link for hover highlighting.
///
/// Both values are sanitized on construction so they can't inject escape sequences: control
/// characters (C0, DEL and C1) are removed from the URI and the `id`, and `;` and `:` (the OSC 8
/// parameter separators) are also removed from the `id`.
///
/// Attach a hyperlink to cells with [`Buffer::set_hyperlink`], the span and line helpers such as
/// [`Buffer::set_line_with_hyperlink`], or [`Cell::set_hyperlink`].
///
/// # Example
///
/// ```
/// use ratatui_core::buffer::Hyperlink;
///
/// let link = Hyperlink::new("https://ratatui.rs").with_id("docs");
/// assert_eq!(link.uri(), "https://ratatui.rs");
/// assert_eq!(link.id(), Some("docs"));
/// assert_eq!(
///     link.open_sequence().to_string(),
///     "\x1b]8;id=docs;https://ratatui.rs\x1b\\"
/// );
/// assert_eq!(Hyperlink::CLOSE_SEQUENCE, "\x1b]8;;\x1b\\");
/// ```
///
/// [OSC 8]: https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda
/// [`Buffer::set_hyperlink`]: crate::buffer::Buffer::set_hyperlink
/// [`Buffer::set_line_with_hyperlink`]: crate::buffer::Buffer::set_line_with_hyperlink
/// [`Cell::set_hyperlink`]: crate::buffer::Cell::set_hyperlink
#[derive(Clone, Eq)]
pub struct Hyperlink {
    uri: CompactString,
    id: Option<CompactString>,
}

impl Hyperlink {
    /// The escape sequence that closes the current hyperlink: `ESC ] 8 ; ; ESC \`.
    pub const CLOSE_SEQUENCE: &'static str = "\x1b]8;;\x1b\\";

    /// Creates a hyperlink to the given URI, removing any control characters from it.
    ///
    /// The URI is written to the terminal as is (minus control characters). Per the OSC 8
    /// specification it should be a URI with any characters outside printable ASCII
    /// percent-encoded.
    pub fn new<T: AsRef<str>>(uri: T) -> Self {
        Self {
            uri: sanitize(uri.as_ref(), char::is_control),
            id: None,
        }
    }

    /// Sets the `id` parameter of the hyperlink.
    ///
    /// Control characters, `;` and `:` are removed. An `id` that is empty after sanitizing is
    /// treated as no `id`.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn with_id<T: AsRef<str>>(mut self, id: T) -> Self {
        let id = sanitize(id.as_ref(), |c| c.is_control() || c == ';' || c == ':');
        self.id = (!id.is_empty()).then_some(id);
        self
    }

    /// Returns the target URI of the hyperlink.
    #[must_use]
    pub fn uri(&self) -> &str {
        &self.uri
    }

    /// Returns the `id` parameter of the hyperlink, if any.
    #[must_use]
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Returns a value that formats as the OSC 8 sequence opening this hyperlink:
    /// `ESC ] 8 ; [id=<id>] ; <uri> ESC \`.
    ///
    /// Backends write this before the first cell of a run of linked cells and
    /// [`CLOSE_SEQUENCE`](Self::CLOSE_SEQUENCE) after the last one.
    #[must_use]
    pub const fn open_sequence(&self) -> OpenSequence<'_> {
        OpenSequence(self)
    }

    /// Returns a copy of this hyperlink with an `id` derived from its URI and `area`, unless it
    /// already has one.
    ///
    /// Used by the [`Buffer`](crate::buffer::Buffer) helpers so that a link applied to an area is
    /// grouped into one link even when it is printed in several runs.
    pub(crate) fn with_default_id(&self, area: Rect) -> Self {
        if self.id.is_some() {
            return self.clone();
        }
        let mut hasher = Fnv1a::default();
        self.uri.hash(&mut hasher);
        area.hash(&mut hasher);
        Self {
            uri: self.uri.clone(),
            id: Some(format_compact!("{:016x}", hasher.finish())),
        }
    }
}

impl PartialEq for Hyperlink {
    fn eq(&self, other: &Self) -> bool {
        core::ptr::eq(self, other) || (self.uri == other.uri && self.id == other.id)
    }
}

impl Hash for Hyperlink {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uri.hash(state);
        self.id.hash(state);
    }
}

impl fmt::Debug for Hyperlink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("Hyperlink");
        debug.field("uri", &self.uri.as_str());
        if let Some(id) = &self.id {
            debug.field("id", &id.as_str());
        }
        debug.finish()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Hyperlink {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        HyperlinkRepr {
            uri: self.uri.clone(),
            id: self.id.clone(),
        }
        .serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Hyperlink {
    /// Deserializes a hyperlink, sanitizing it the same way as [`Hyperlink::new`] and
    /// [`Hyperlink::with_id`].
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let repr = HyperlinkRepr::deserialize(deserializer)?;
        let link = Self::new(repr.uri);
        Ok(match repr.id {
            Some(id) => link.with_id(id),
            None => link,
        })
    }
}

#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
struct HyperlinkRepr {
    uri: CompactString,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    id: Option<CompactString>,
}

/// Formats as the OSC 8 sequence that opens a [`Hyperlink`].
///
/// Returned by [`Hyperlink::open_sequence`].
#[derive(Clone, Copy, Debug)]
pub struct OpenSequence<'a>(&'a Hyperlink);

impl fmt::Display for OpenSequence<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\x1b]8;")?;
        if let Some(id) = &self.0.id {
            f.write_str("id=")?;
            f.write_str(id)?;
        }
        f.write_str(";")?;
        f.write_str(&self.0.uri)?;
        f.write_str("\x1b\\")
    }
}

/// Out-of-band data attached to a [`Cell`](crate::buffer::Cell).
///
/// Backends write extensions around the cell's symbol as run state. Extensions never affect the
/// symbol's text or width. Currently the only extension is a [`Hyperlink`].
///
/// A `CellExt` is a cheap to clone, reference-counted handle, so applying the same extension to
/// many cells shares a single allocation, and comparing two cells that share it costs a pointer
/// comparison. Values are immutable: [`with_hyperlink`](Self::with_hyperlink) returns a new value
/// (copying the data only when it is shared).
///
/// # Example
///
/// ```
/// use ratatui_core::buffer::{Cell, CellExt, Hyperlink};
///
/// let ext = CellExt::from(Hyperlink::new("https://ratatui.rs"));
/// let mut cell = Cell::new("r");
/// cell.set_ext(Some(ext.clone()));
/// assert_eq!(cell.ext(), Some(&ext));
/// assert_eq!(
///     cell.hyperlink().map(Hyperlink::uri),
///     Some("https://ratatui.rs")
/// );
/// ```
#[derive(Clone, Default)]
pub struct CellExt(Arc<CellExtData>);

/// The data behind a [`CellExt`]. Private so new kinds of extensions can be added without
/// breaking changes.
#[derive(Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct CellExtData {
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    hyperlink: Option<Hyperlink>,
}

impl CellExt {
    /// Creates an empty extension.
    ///
    /// [`Cell::set_ext`](crate::buffer::Cell::set_ext) stores an empty extension as `None`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `true` if the extension holds no data.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.hyperlink.is_none()
    }

    /// Returns the hyperlink, if any.
    #[must_use]
    pub fn hyperlink(&self) -> Option<&Hyperlink> {
        self.0.hyperlink.as_ref()
    }

    /// Returns the extension with its hyperlink replaced by `hyperlink`.
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn with_hyperlink(mut self, hyperlink: Option<Hyperlink>) -> Self {
        if self.0.hyperlink != hyperlink {
            Arc::make_mut(&mut self.0).hyperlink = hyperlink;
        }
        self
    }

    /// Returns `true` if both values point to the same allocation.
    pub(crate) fn ptr_eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl PartialEq for CellExt {
    /// Compares two extensions, checking whether they share an allocation before comparing their
    /// data.
    fn eq(&self, other: &Self) -> bool {
        self.ptr_eq(other) || self.0 == other.0
    }
}

impl Eq for CellExt {}

impl Hash for CellExt {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl fmt::Debug for CellExt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("CellExt");
        if let Some(hyperlink) = &self.0.hyperlink {
            debug.field("hyperlink", hyperlink);
        }
        debug.finish()
    }
}

impl From<Hyperlink> for CellExt {
    fn from(hyperlink: Hyperlink) -> Self {
        Self::new().with_hyperlink(Some(hyperlink))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for CellExt {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for CellExt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        CellExtData::deserialize(deserializer).map(|data| Self(Arc::new(data)))
    }
}

/// Returns `s` without the characters matching `remove`, without allocating when nothing is
/// removed and `s` fits inline.
fn sanitize(s: &str, remove: impl Fn(char) -> bool) -> CompactString {
    if s.contains(&remove) {
        s.chars().filter(|&c| !remove(c)).collect()
    } else {
        CompactString::new(s)
    }
}

/// A 64-bit FNV-1a hasher. Used to derive stable hyperlink ids without depending on `std`'s
/// randomly seeded hasher.
struct Fnv1a(u64);

impl Default for Fnv1a {
    fn default() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }
}

impl Hasher for Fnv1a {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.0 ^= u64::from(byte);
            self.0 = self.0.wrapping_mul(0x0100_0000_01b3);
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use alloc::vec::Vec;
    use alloc::{format, vec};

    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::buffer::{Buffer, BufferDiff, Cell};
    use crate::style::{Color, Style};
    use crate::text::{Line, Span};

    fn link(uri: &str) -> Hyperlink {
        Hyperlink::new(uri)
    }

    #[test]
    fn hyperlink_new() {
        let link = Hyperlink::new("https://ratatui.rs");
        assert_eq!(link.uri(), "https://ratatui.rs");
        assert_eq!(link.id(), None);
    }

    #[test]
    fn hyperlink_sanitizes_uri() {
        let link = Hyperlink::new("https://a\x1b]8;;evil\x07.com/\u{9c}x\u{7f}y\n");
        assert_eq!(link.uri(), "https://a]8;;evil.com/xy");
    }

    #[test]
    fn hyperlink_sanitizes_id() {
        let link = Hyperlink::new("https://ratatui.rs").with_id("a;b:c\x1b\\d\u{9c}");
        assert_eq!(link.id(), Some("abc\\d"));
        let link = Hyperlink::new("https://ratatui.rs").with_id(";:\x1b");
        assert_eq!(link.id(), None);
    }

    #[test]
    fn hyperlink_sequences() {
        assert_eq!(
            link("https://ratatui.rs").open_sequence().to_string(),
            "\x1b]8;;https://ratatui.rs\x1b\\"
        );
        assert_eq!(
            link("https://ratatui.rs")
                .with_id("x")
                .open_sequence()
                .to_string(),
            "\x1b]8;id=x;https://ratatui.rs\x1b\\"
        );
        assert_eq!(Hyperlink::CLOSE_SEQUENCE, "\x1b]8;;\x1b\\");
    }

    #[test]
    fn hyperlink_eq() {
        assert_eq!(link("a"), link("a"));
        assert_ne!(link("a"), link("b"));
        assert_ne!(link("a"), link("a").with_id("x"));
        assert_eq!(link("a").with_id("x"), link("a").with_id("x"));
    }

    #[test]
    fn hyperlink_debug() {
        assert_eq!(format!("{:?}", link("a")), r#"Hyperlink { uri: "a" }"#);
        assert_eq!(
            format!("{:?}", link("a").with_id("x")),
            r#"Hyperlink { uri: "a", id: "x" }"#
        );
    }

    #[test]
    fn default_id_is_stable_and_respects_explicit_id() {
        let area = Rect::new(1, 2, 3, 4);
        let a = link("a").with_default_id(area);
        assert_eq!(a, link("a").with_default_id(area));
        assert_eq!(a.id().map(str::len), Some(16));
        assert_ne!(a, link("b").with_default_id(area));
        assert_ne!(a, link("a").with_default_id(Rect::new(1, 2, 3, 5)));
        assert_eq!(link("a").with_id("x").with_default_id(area).id(), Some("x"));
    }

    #[test]
    fn cell_ext_eq() {
        let a = CellExt::from(link("a"));
        assert!(a.ptr_eq(&a.clone()));
        assert_eq!(a, a.clone());
        // value-equal but separately allocated
        let b = CellExt::from(link("a"));
        assert!(!a.ptr_eq(&b));
        assert_eq!(a, b);
        assert_ne!(a, CellExt::from(link("b")));
        assert_ne!(a, CellExt::new());
    }

    #[test]
    fn cell_ext_with_hyperlink() {
        let ext = CellExt::new();
        assert!(ext.is_empty());
        assert_eq!(ext.hyperlink(), None);
        let ext = ext.with_hyperlink(Some(link("a")));
        assert!(!ext.is_empty());
        assert_eq!(ext.hyperlink(), Some(&link("a")));
        // copy on write: the shared original is unchanged
        let shared = ext.clone();
        let ext = ext.with_hyperlink(None);
        assert!(ext.is_empty());
        assert_eq!(shared.hyperlink(), Some(&link("a")));
    }

    #[test]
    fn cell_ext_debug() {
        assert_eq!(format!("{:?}", CellExt::new()), "CellExt");
        assert_eq!(
            format!("{:?}", CellExt::from(link("a"))),
            r#"CellExt { hyperlink: Hyperlink { uri: "a" } }"#
        );
    }

    #[test]
    fn cell_hyperlink() {
        let mut cell = Cell::new("a");
        assert_eq!(cell.hyperlink(), None);
        assert_eq!(cell.ext(), None);
        cell.set_hyperlink(Some(&link("a")));
        assert_eq!(cell.hyperlink(), Some(&link("a")));
        cell.set_hyperlink(None);
        assert_eq!(cell.ext(), None, "an empty extension is stored as None");
        cell.set_ext(Some(CellExt::new()));
        assert_eq!(cell.ext(), None);
    }

    #[test]
    fn cell_eq_and_hash_include_ext() {
        use core::hash::BuildHasher;
        let hasher = hashbrown::DefaultHashBuilder::default();

        let plain = Cell::new("a");
        let mut linked = Cell::new("a");
        linked.set_hyperlink(Some(&link("a")));
        assert_ne!(plain, linked);
        assert_ne!(hasher.hash_one(&plain), hasher.hash_one(&linked));

        let mut other = Cell::new("a");
        other.set_hyperlink(Some(&link("a")));
        assert_eq!(linked, other);
        assert_eq!(hasher.hash_one(&linked), hasher.hash_one(&other));
    }

    #[test]
    fn cell_set_style_keeps_ext() {
        let mut cell = Cell::new("a");
        cell.set_hyperlink(Some(&link("a")));
        cell.set_style(Style::new().fg(Color::Red))
            .set_bg(Color::Blue);
        assert_eq!(cell.hyperlink(), Some(&link("a")));
    }

    #[test]
    fn cell_writing_content_clears_ext() {
        use crate::symbols::merge::MergeStrategy;
        let linked = || {
            let mut cell = Cell::new("─");
            cell.set_hyperlink(Some(&link("a")));
            cell
        };
        assert_eq!(linked().set_symbol("b").hyperlink(), None);
        assert_eq!(linked().set_char('b').hyperlink(), None);
        assert_eq!(
            linked().merge_symbol("│", MergeStrategy::Exact).hyperlink(),
            None
        );
        let mut cell = linked();
        cell.reset();
        assert_eq!(cell, Cell::EMPTY);
        let mut cell = linked();
        cell.append_symbol("\u{301}");
        assert_eq!(cell.hyperlink(), Some(&link("a")));
    }

    #[test]
    fn buffer_set_hyperlink_shares_one_allocation() {
        let mut buf = Buffer::with_lines(["abcd", "efgh"]);
        let area = Rect::new(1, 0, 2, 2);
        buf.set_hyperlink(area, Some(&link("a")));
        let linked: Vec<_> = buf
            .content
            .iter()
            .enumerate()
            .filter_map(|(i, cell)| cell.ext().map(|ext| (buf.pos_of(i), ext)))
            .collect();
        let positions: Vec<_> = linked.iter().map(|(pos, _)| *pos).collect();
        assert_eq!(positions, [(1, 0), (2, 0), (1, 1), (2, 1)]);
        assert!(linked.iter().all(|(_, ext)| ext.ptr_eq(linked[0].1)));
        let expected = link("a").with_default_id(area);
        assert_eq!(linked[0].1.hyperlink(), Some(&expected));
        // removing
        buf.set_hyperlink(Rect::new(2, 0, 1, 2), None);
        assert_eq!(buf[(2, 0)].ext(), None);
        assert_eq!(buf[(1, 0)].hyperlink(), Some(&expected));
    }

    #[test]
    fn buffer_set_hyperlink_clips_to_buffer() {
        let mut buf = Buffer::with_lines(["ab"]);
        buf.set_hyperlink(Rect::new(1, 0, 10, 10), Some(&link("a").with_id("x")));
        assert_eq!(buf[(0, 0)].hyperlink(), None);
        assert_eq!(buf[(1, 0)].hyperlink(), Some(&link("a").with_id("x")));
    }

    #[test]
    fn buffer_set_ext() {
        let mut buf = Buffer::with_lines(["ab"]);
        let ext = CellExt::from(link("a"));
        buf.set_ext(Rect::new(0, 0, 1, 1), Some(&ext));
        assert_eq!(buf[(0, 0)].ext(), Some(&ext));
        assert_eq!(buf[(1, 0)].ext(), None);
        buf.set_ext(buf.area, None);
        assert_eq!(buf[(0, 0)].ext(), None);
    }

    #[test]
    fn buffer_set_span_with_hyperlink() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 1));
        let link = link("a").with_id("x");
        let end = buf.set_span_with_hyperlink(1, 0, &Span::raw("日本x"), 4, &link);
        assert_eq!(end, (5, 0));
        let linked: Vec<_> = (0..10).map(|x| buf[(x, 0)].hyperlink().is_some()).collect();
        assert_eq!(
            linked,
            [
                false, true, true, true, true, false, false, false, false, false
            ]
        );
    }

    #[test]
    fn buffer_set_line_with_hyperlink() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 1));
        let line = Line::from(vec![Span::raw("ab"), Span::raw("cd")]);
        let end = buf.set_line_with_hyperlink(0, 0, &line, 3, &link("a"));
        assert_eq!(end, (3, 0));
        assert!(buf[(2, 0)].hyperlink().is_some());
        assert!(buf[(3, 0)].hyperlink().is_none());
        // a line that writes nothing links nothing
        buf.set_line_with_hyperlink(5, 0, &Line::default(), 3, &link("b"));
        assert!(buf.content.iter().skip(3).all(|cell| cell.ext().is_none()));
    }

    #[test]
    fn buffer_merge_and_clone_keep_links() {
        let mut a = Buffer::with_lines(["ab"]);
        let mut b = Buffer::empty(Rect::new(0, 1, 2, 1));
        b.set_string(0, 1, "cd", Style::new());
        b.set_hyperlink(b.area, Some(&link("b").with_id("b")));
        a.merge(&b);
        assert_eq!(a[(1, 1)].hyperlink(), Some(&link("b").with_id("b")));
        assert_eq!(a.clone(), a);
        a.reset();
        assert!(a.content.iter().all(|cell| cell.ext().is_none()));
    }

    #[test]
    fn buffer_debug_shows_hyperlink_runs() {
        let mut buf = Buffer::with_lines(["abcd", "efgh"]);
        buf.set_hyperlink(Rect::new(1, 0, 2, 2), Some(&link("a").with_id("x")));
        buf.set_hyperlink(Rect::new(3, 1, 1, 1), Some(&link("b").with_id("y")));
        #[cfg(feature = "underline-color")]
        let style = "x: 0, y: 0, fg: Reset, bg: Reset, underline: Reset, modifier: NONE,";
        #[cfg(not(feature = "underline-color"))]
        let style = "x: 0, y: 0, fg: Reset, bg: Reset, modifier: NONE,";
        let expected = format!(
            indoc!(
                r#"
                Buffer {{
                    area: Rect {{ x: 0, y: 0, width: 4, height: 2 }},
                    content: [
                        "abcd",
                        "efgh",
                    ],
                    styles: [
                        {}
                    ],
                    hyperlinks: [
                        x: 1, y: 0, width: 2, Hyperlink {{ uri: "a", id: "x" }},
                        x: 1, y: 1, width: 2, Hyperlink {{ uri: "a", id: "x" }},
                        x: 3, y: 1, width: 1, Hyperlink {{ uri: "b", id: "y" }},
                    ]
                }}"#
            ),
            style
        );
        assert_eq!(format!("{buf:?}"), expected);
    }

    #[test]
    fn buffer_debug_without_hyperlinks_is_unchanged() {
        let buf = Buffer::with_lines(["ab"]);
        assert!(format!("{buf:?}").ends_with("    ]\n}"));
        assert!(!format!("{buf:?}").contains("hyperlinks"));
    }

    #[test]
    fn diff_yields_link_only_changes() {
        let prev = Buffer::with_lines(["abcd"]);
        let mut next = prev.clone();
        next.set_hyperlink(Rect::new(1, 0, 2, 1), Some(&link("a")));
        let diff: Vec<_> = BufferDiff::new(&prev, &next)
            .map(|(x, y, cell)| (x, y, cell.hyperlink().is_some()))
            .collect();
        assert_eq!(diff, [(1, 0, true), (2, 0, true)]);

        // and back again
        let diff: Vec<_> = BufferDiff::new(&next, &prev)
            .map(|(x, y, cell)| (x, y, cell.hyperlink().is_some()))
            .collect();
        assert_eq!(diff, [(1, 0, false), (2, 0, false)]);

        // value-equal links in separate allocations are not a change
        let mut same = prev.clone();
        same.set_hyperlink(Rect::new(1, 0, 2, 1), Some(&link("a")));
        assert_eq!(BufferDiff::new(&next, &same).count(), 0);
    }

    /// A wide glyph whose only change is its link is redrawn without its trailing cell: the
    /// terminal attaches the link to the whole glyph when it is printed.
    #[test]
    fn diff_wide_glyph_link_only_change() {
        let prev = Buffer::with_lines(["日x"]);
        let mut next = prev.clone();
        next.set_hyperlink(Rect::new(0, 0, 2, 1), Some(&link("a")));
        let diff: Vec<_> = BufferDiff::new(&prev, &next)
            .map(|(x, y, cell)| (x, y, cell.symbol()))
            .collect();
        assert_eq!(diff, [(0, 0, "日")]);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_round_trip() {
        let mut buf = Buffer::with_lines(["ab"]);
        buf.set_hyperlink(Rect::new(0, 0, 1, 1), Some(&link("a").with_id("x")));
        let json = serde_json::to_string(&buf).unwrap();
        assert!(json.contains(r#""ext":{"hyperlink":{"uri":"a","id":"x"}}"#));
        assert_eq!(
            json.matches("ext").count(),
            1,
            "empty extensions are skipped"
        );
        let back: Buffer = serde_json::from_str(&json).unwrap();
        assert_eq!(back, buf);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_sanitizes_hyperlinks() {
        let link: Hyperlink = serde_json::from_str(r#"{"uri":"a\u001b\\b","id":"x;y"}"#).unwrap();
        assert_eq!(link, Hyperlink::new("a\\b").with_id("xy"));
    }
}

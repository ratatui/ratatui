# Migrating from TUI

[Ratatui](https://github.com/tui-rs-revival/ratatui) is a fork of [tui-rs](https://github.com/fdehau/tui-rs/), created to continue maintenance of the project.

Several options are available to migrate apps and libs:
- Use ratatui as a drop in replacement aliased as tui
- Replace ratatui fully
- Support both tui and ratatui (useful for libraries)

## Drop in replacement

The simplest approach to migrating to ratatui is to use it as drop in replacement for tui and updating the terminal libraries used (crossterm / termion). E.g.:

```toml
tui = { package = "ratatui", version = "0.21.0", features = ["crossterm"] }
crossterm = { version = "0.26.1" }
```

Or:

```toml
tui = { package = "ratatui", version = "0.21.0", default-features = false, features = ["termion"] }
termion = { version = "2.0" }
```

## Fully replace Tui with Ratatui

Most new code would instead use the following. To take this approach to migration requires find and replace `tui::`->`ratatui::` on the entire codebase.

```toml
ratatui = { version = "0.21.0" }
crossterm = { version = "0.26.1" }
```

## Support both tui and ratatui

For more complex scenarios where a library (or in some cases an app) needs to support both ratatui and maintain existing support for tui, it may be feasible to use feature flags to select which library to use. See [tui-logger](https://github.com/gin66/tui-logger) for an example of this approach.

## Version comparison

The changes between versions are documented in the project's [CHANGELOG](https://github.com/tui-rs-revival/ratatui/blob/main/CHANGELOG.md)

<details>
<summary>
Public API differences between ratatui 0.21.0 and ratatui 0.22.0:
</summary>

```diff
Removed items from the public API
=================================
-pub fn ratatui::layout::Layout::direction(self, direction: ratatui::layout::Direction) -> ratatui::layout::Layout
-pub fn ratatui::layout::Layout::horizontal_margin(self, horizontal: u16) -> ratatui::layout::Layout
-pub fn ratatui::layout::Layout::margin(self, margin: u16) -> ratatui::layout::Layout
-pub fn ratatui::layout::Layout::vertical_margin(self, vertical: u16) -> ratatui::layout::Layout
-pub unsafe const fn ratatui::style::Modifier::from_bits_unchecked(bits: u16) -> Self
-impl core::cmp::Ord for ratatui::style::Modifier
-pub fn ratatui::style::Modifier::cmp(&self, other: &ratatui::style::Modifier) -> core::cmp::Ordering
-impl core::cmp::PartialOrd<ratatui::style::Modifier> for ratatui::style::Modifier
-pub fn ratatui::style::Modifier::partial_cmp(&self, other: &ratatui::style::Modifier) -> core::option::Option<core::cmp::Ordering>
-impl core::hash::Hash for ratatui::style::Modifier
-pub fn ratatui::style::Modifier::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
-pub fn ratatui::style::Style::add_modifier(self, modifier: ratatui::style::Modifier) -> ratatui::style::Style
-pub fn ratatui::style::Style::remove_modifier(self, modifier: ratatui::style::Modifier) -> ratatui::style::Style
-impl ratatui::widgets::BorderType
-pub fn ratatui::widgets::BorderType::line_symbols(border_type: ratatui::widgets::BorderType) -> ratatui::symbols::line::Set
-impl core::clone::Clone for ratatui::widgets::BorderType
-pub fn ratatui::widgets::BorderType::clone(&self) -> ratatui::widgets::BorderType
-impl core::cmp::Eq for ratatui::widgets::BorderType
-impl core::cmp::PartialEq<ratatui::widgets::BorderType> for ratatui::widgets::BorderType
-pub fn ratatui::widgets::BorderType::eq(&self, other: &ratatui::widgets::BorderType) -> bool
-impl core::fmt::Debug for ratatui::widgets::BorderType
-pub fn ratatui::widgets::BorderType::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
-impl core::marker::Copy for ratatui::widgets::BorderType
-impl core::marker::StructuralEq for ratatui::widgets::BorderType
-impl core::marker::StructuralPartialEq for ratatui::widgets::BorderType
-impl core::marker::Send for ratatui::widgets::BorderType
-impl core::marker::Sync for ratatui::widgets::BorderType
-impl core::marker::Unpin for ratatui::widgets::BorderType
-impl core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::BorderType
-impl core::panic::unwind_safe::UnwindSafe for ratatui::widgets::BorderType
-impl<T, U> core::convert::Into<U> for ratatui::widgets::BorderType where U: core::convert::From<T>
-pub fn ratatui::widgets::BorderType::into(self) -> U
-impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::BorderType where U: core::convert::Into<T>
-pub type ratatui::widgets::BorderType::Error = core::convert::Infallible
-pub fn ratatui::widgets::BorderType::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
-impl<T, U> core::convert::TryInto<U> for ratatui::widgets::BorderType where U: core::convert::TryFrom<T>
-pub type ratatui::widgets::BorderType::Error = <U as core::convert::TryFrom<T>>::Error
-pub fn ratatui::widgets::BorderType::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
-impl<T> alloc::borrow::ToOwned for ratatui::widgets::BorderType where T: core::clone::Clone
-pub type ratatui::widgets::BorderType::Owned = T
-pub fn ratatui::widgets::BorderType::clone_into(&self, target: &mut T)
-pub fn ratatui::widgets::BorderType::to_owned(&self) -> T
-impl<T> core::any::Any for ratatui::widgets::BorderType where T: 'static + core::marker::Sized
-pub fn ratatui::widgets::BorderType::type_id(&self) -> core::any::TypeId
-impl<T> core::borrow::Borrow<T> for ratatui::widgets::BorderType where T: core::marker::Sized
-pub fn ratatui::widgets::BorderType::borrow(&self) -> &T
-impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::BorderType where T: core::marker::Sized
-pub fn ratatui::widgets::BorderType::borrow_mut(&mut self) -> &mut T
-impl<T> core::convert::From<T> for ratatui::widgets::BorderType
-pub fn ratatui::widgets::BorderType::from(t: T) -> T
-pub fn ratatui::widgets::BarChart<'a>::block(self, block: ratatui::widgets::Block<'a>) -> ratatui::widgets::BarChart<'a>
-pub fn ratatui::widgets::BarChart<'a>::data(self, data: &'a [(&'a str, u64)]) -> ratatui::widgets::BarChart<'a>
-impl<'a> ratatui::widgets::Block<'a>
-pub fn ratatui::widgets::Block<'a>::border_style(self, style: ratatui::style::Style) -> ratatui::widgets::Block<'a>
-pub fn ratatui::widgets::Block<'a>::border_type(self, border_type: ratatui::widgets::BorderType) -> ratatui::widgets::Block<'a>
-pub fn ratatui::widgets::Block<'a>::borders(self, flag: ratatui::widgets::Borders) -> ratatui::widgets::Block<'a>
-pub fn ratatui::widgets::Block<'a>::inner(&self, area: ratatui::layout::Rect) -> ratatui::layout::Rect
-pub fn ratatui::widgets::Block<'a>::padding(self, padding: ratatui::widgets::Padding) -> ratatui::widgets::Block<'a>
-pub fn ratatui::widgets::Block<'a>::style(self, style: ratatui::style::Style) -> ratatui::widgets::Block<'a>
-pub fn ratatui::widgets::Block<'a>::title<T>(self, title: T) -> ratatui::widgets::Block<'a> where T: core::convert::Into<ratatui::text::Line<'a>>
-pub fn ratatui::widgets::Block<'a>::title_alignment(self, alignment: ratatui::layout::Alignment) -> ratatui::widgets::Block<'a>
-pub fn ratatui::widgets::Block<'a>::title_on_bottom(self) -> ratatui::widgets::Block<'a>
-pub fn ratatui::widgets::Block<'a>::title_style(self, style: ratatui::style::Style) -> ratatui::widgets::Block<'a>
-impl<'a> core::default::Default for ratatui::widgets::Block<'a>
-pub fn ratatui::widgets::Block<'a>::default() -> ratatui::widgets::Block<'a>
-impl<'a> ratatui::widgets::Widget for ratatui::widgets::Block<'a>
-impl<'a> ratatui::widgets::Widget for ratatui::widgets::Block<'a>
-pub fn ratatui::widgets::Block<'a>::render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer)
-pub fn ratatui::widgets::Block<'a>::render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer)
-impl<'a> core::clone::Clone for ratatui::widgets::Block<'a>
-pub fn ratatui::widgets::Block<'a>::clone(&self) -> ratatui::widgets::Block<'a>
-impl<'a> core::cmp::Eq for ratatui::widgets::Block<'a>
-impl<'a> core::cmp::PartialEq<ratatui::widgets::Block<'a>> for ratatui::widgets::Block<'a>
-pub fn ratatui::widgets::Block<'a>::eq(&self, other: &ratatui::widgets::Block<'a>) -> bool
-impl<'a> core::fmt::Debug for ratatui::widgets::Block<'a>
-pub fn ratatui::widgets::Block<'a>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
-impl<'a> core::marker::StructuralEq for ratatui::widgets::Block<'a>
-impl<'a> core::marker::StructuralPartialEq for ratatui::widgets::Block<'a>
-impl<'a> core::marker::Send for ratatui::widgets::Block<'a>
-impl<'a> core::marker::Sync for ratatui::widgets::Block<'a>
-impl<'a> core::marker::Unpin for ratatui::widgets::Block<'a>
-impl<'a> core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::Block<'a>
-impl<'a> core::panic::unwind_safe::UnwindSafe for ratatui::widgets::Block<'a>
-impl<T, U> core::convert::Into<U> for ratatui::widgets::Block<'a> where U: core::convert::From<T>
-pub fn ratatui::widgets::Block<'a>::into(self) -> U
-impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::Block<'a> where U: core::convert::Into<T>
-pub type ratatui::widgets::Block<'a>::Error = core::convert::Infallible
-pub fn ratatui::widgets::Block<'a>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
-impl<T, U> core::convert::TryInto<U> for ratatui::widgets::Block<'a> where U: core::convert::TryFrom<T>
-pub type ratatui::widgets::Block<'a>::Error = <U as core::convert::TryFrom<T>>::Error
-pub fn ratatui::widgets::Block<'a>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
-impl<T> alloc::borrow::ToOwned for ratatui::widgets::Block<'a> where T: core::clone::Clone
-pub type ratatui::widgets::Block<'a>::Owned = T
-pub fn ratatui::widgets::Block<'a>::clone_into(&self, target: &mut T)
-pub fn ratatui::widgets::Block<'a>::to_owned(&self) -> T
-impl<T> core::any::Any for ratatui::widgets::Block<'a> where T: 'static + core::marker::Sized
-pub fn ratatui::widgets::Block<'a>::type_id(&self) -> core::any::TypeId
-impl<T> core::borrow::Borrow<T> for ratatui::widgets::Block<'a> where T: core::marker::Sized
-pub fn ratatui::widgets::Block<'a>::borrow(&self) -> &T
-impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::Block<'a> where T: core::marker::Sized
-pub fn ratatui::widgets::Block<'a>::borrow_mut(&mut self) -> &mut T
-impl<T> core::convert::From<T> for ratatui::widgets::Block<'a>
-pub fn ratatui::widgets::Block<'a>::from(t: T) -> T
-pub unsafe const fn ratatui::widgets::Borders::from_bits_unchecked(bits: u8) -> Self
-impl core::cmp::Ord for ratatui::widgets::Borders
-pub fn ratatui::widgets::Borders::cmp(&self, other: &ratatui::widgets::Borders) -> core::cmp::Ordering
-impl core::cmp::PartialOrd<ratatui::widgets::Borders> for ratatui::widgets::Borders
-pub fn ratatui::widgets::Borders::partial_cmp(&self, other: &ratatui::widgets::Borders) -> core::option::Option<core::cmp::Ordering>
-impl core::hash::Hash for ratatui::widgets::Borders
-pub fn ratatui::widgets::Borders::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
-impl ratatui::widgets::Padding
-pub fn ratatui::widgets::Padding::horizontal(value: u16) -> Self
-pub fn ratatui::widgets::Padding::new(left: u16, right: u16, top: u16, bottom: u16) -> Self
-pub fn ratatui::widgets::Padding::uniform(value: u16) -> Self
-pub fn ratatui::widgets::Padding::vertical(value: u16) -> Self
-pub fn ratatui::widgets::Padding::zero() -> Self
-impl core::clone::Clone for ratatui::widgets::Padding
-pub fn ratatui::widgets::Padding::clone(&self) -> ratatui::widgets::Padding
-impl core::cmp::Eq for ratatui::widgets::Padding
-impl core::cmp::PartialEq<ratatui::widgets::Padding> for ratatui::widgets::Padding
-pub fn ratatui::widgets::Padding::eq(&self, other: &ratatui::widgets::Padding) -> bool
-impl core::fmt::Debug for ratatui::widgets::Padding
-pub fn ratatui::widgets::Padding::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
-impl core::hash::Hash for ratatui::widgets::Padding
-pub fn ratatui::widgets::Padding::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
-impl core::marker::StructuralEq for ratatui::widgets::Padding
-impl core::marker::StructuralPartialEq for ratatui::widgets::Padding
-impl core::marker::Send for ratatui::widgets::Padding
-impl core::marker::Sync for ratatui::widgets::Padding
-impl core::marker::Unpin for ratatui::widgets::Padding
-impl core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::Padding
-impl core::panic::unwind_safe::UnwindSafe for ratatui::widgets::Padding
-impl<T, U> core::convert::Into<U> for ratatui::widgets::Padding where U: core::convert::From<T>
-pub fn ratatui::widgets::Padding::into(self) -> U
-impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::Padding where U: core::convert::Into<T>
-pub type ratatui::widgets::Padding::Error = core::convert::Infallible
-pub fn ratatui::widgets::Padding::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
-impl<T, U> core::convert::TryInto<U> for ratatui::widgets::Padding where U: core::convert::TryFrom<T>
-pub type ratatui::widgets::Padding::Error = <U as core::convert::TryFrom<T>>::Error
-pub fn ratatui::widgets::Padding::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
-impl<T> alloc::borrow::ToOwned for ratatui::widgets::Padding where T: core::clone::Clone
-pub type ratatui::widgets::Padding::Owned = T
-pub fn ratatui::widgets::Padding::clone_into(&self, target: &mut T)
-pub fn ratatui::widgets::Padding::to_owned(&self) -> T
-impl<T> core::any::Any for ratatui::widgets::Padding where T: 'static + core::marker::Sized
-pub fn ratatui::widgets::Padding::type_id(&self) -> core::any::TypeId
-impl<T> core::borrow::Borrow<T> for ratatui::widgets::Padding where T: core::marker::Sized
-pub fn ratatui::widgets::Padding::borrow(&self) -> &T
-impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::Padding where T: core::marker::Sized
-pub fn ratatui::widgets::Padding::borrow_mut(&mut self) -> &mut T
-impl<T> core::convert::From<T> for ratatui::widgets::Padding
-pub fn ratatui::widgets::Padding::from(t: T) -> T

Changed items in the public API
===============================
-pub fn ratatui::backend::Backend::append_lines(&mut self, n: u16) -> std::io::error::Result<()>
+pub fn ratatui::backend::Backend::append_lines(&mut self, _n: u16) -> std::io::error::Result<()>
-pub fn ratatui::layout::Rect::area(self) -> u16
+pub const fn ratatui::layout::Rect::area(self) -> u16
-pub fn ratatui::layout::Rect::bottom(self) -> u16
+pub const fn ratatui::layout::Rect::bottom(self) -> u16
-pub fn ratatui::layout::Rect::intersects(self, other: ratatui::layout::Rect) -> bool
+pub const fn ratatui::layout::Rect::intersects(self, other: ratatui::layout::Rect) -> bool
-pub fn ratatui::layout::Rect::left(self) -> u16
+pub const fn ratatui::layout::Rect::left(self) -> u16
-pub fn ratatui::layout::Rect::right(self) -> u16
+pub const fn ratatui::layout::Rect::right(self) -> u16
-pub fn ratatui::layout::Rect::top(self) -> u16
+pub const fn ratatui::layout::Rect::top(self) -> u16
-pub struct ratatui::style::Modifier
+pub struct ratatui::style::Modifier(_)
-pub fn ratatui::widgets::canvas::Canvas<'a, F>::block(self, block: ratatui::widgets::Block<'a>) -> ratatui::widgets::canvas::Canvas<'a, F>
+pub fn ratatui::widgets::canvas::Canvas<'a, F>::block(self, block: ratatui::widgets::block::Block<'a>) -> ratatui::widgets::canvas::Canvas<'a, F>
-pub struct ratatui::widgets::Borders
+pub struct ratatui::widgets::Borders(_)
-pub fn ratatui::widgets::Chart<'a>::block(self, block: ratatui::widgets::Block<'a>) -> ratatui::widgets::Chart<'a>
+pub fn ratatui::widgets::Chart<'a>::block(self, block: ratatui::widgets::block::Block<'a>) -> ratatui::widgets::Chart<'a>
-pub fn ratatui::widgets::Gauge<'a>::block(self, block: ratatui::widgets::Block<'a>) -> ratatui::widgets::Gauge<'a>
+pub fn ratatui::widgets::Gauge<'a>::block(self, block: ratatui::widgets::block::Block<'a>) -> ratatui::widgets::Gauge<'a>
-pub fn ratatui::widgets::LineGauge<'a>::block(self, block: ratatui::widgets::Block<'a>) -> Self
+pub fn ratatui::widgets::LineGauge<'a>::block(self, block: ratatui::widgets::block::Block<'a>) -> Self
-pub fn ratatui::widgets::List<'a>::block(self, block: ratatui::widgets::Block<'a>) -> ratatui::widgets::List<'a>
+pub fn ratatui::widgets::List<'a>::block(self, block: ratatui::widgets::block::Block<'a>) -> ratatui::widgets::List<'a>
-pub fn ratatui::widgets::Paragraph<'a>::block(self, block: ratatui::widgets::Block<'a>) -> ratatui::widgets::Paragraph<'a>
+pub fn ratatui::widgets::Paragraph<'a>::block(self, block: ratatui::widgets::block::Block<'a>) -> ratatui::widgets::Paragraph<'a>
-pub fn ratatui::widgets::Sparkline<'a>::block(self, block: ratatui::widgets::Block<'a>) -> ratatui::widgets::Sparkline<'a>
+pub fn ratatui::widgets::Sparkline<'a>::block(self, block: ratatui::widgets::block::Block<'a>) -> ratatui::widgets::Sparkline<'a>
-pub fn ratatui::widgets::Table<'a>::block(self, block: ratatui::widgets::Block<'a>) -> Self
+pub fn ratatui::widgets::Table<'a>::block(self, block: ratatui::widgets::block::Block<'a>) -> Self
-pub fn ratatui::widgets::Tabs<'a>::block(self, block: ratatui::widgets::Block<'a>) -> ratatui::widgets::Tabs<'a>
+pub fn ratatui::widgets::Tabs<'a>::block(self, block: ratatui::widgets::block::Block<'a>) -> ratatui::widgets::Tabs<'a>

Added items to the public API
=============================
+impl core::clone::Clone for ratatui::backend::ClearType
+pub fn ratatui::backend::ClearType::clone(&self) -> ratatui::backend::ClearType
+impl core::cmp::Eq for ratatui::backend::ClearType
+impl core::cmp::PartialEq<ratatui::backend::ClearType> for ratatui::backend::ClearType
+pub fn ratatui::backend::ClearType::eq(&self, other: &ratatui::backend::ClearType) -> bool
+impl core::fmt::Debug for ratatui::backend::ClearType
+pub fn ratatui::backend::ClearType::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::marker::Copy for ratatui::backend::ClearType
+impl core::marker::StructuralEq for ratatui::backend::ClearType
+impl core::marker::StructuralPartialEq for ratatui::backend::ClearType
+impl core::marker::Send for ratatui::backend::ClearType
+impl core::marker::Sync for ratatui::backend::ClearType
+impl core::marker::Unpin for ratatui::backend::ClearType
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::backend::ClearType
+impl core::panic::unwind_safe::UnwindSafe for ratatui::backend::ClearType
+impl<T, U> core::convert::Into<U> for ratatui::backend::ClearType where U: core::convert::From<T>
+pub fn ratatui::backend::ClearType::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::backend::ClearType where U: core::convert::Into<T>
+pub type ratatui::backend::ClearType::Error = core::convert::Infallible
+pub fn ratatui::backend::ClearType::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::backend::ClearType where U: core::convert::TryFrom<T>
+pub type ratatui::backend::ClearType::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::backend::ClearType::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::backend::ClearType where T: core::clone::Clone
+pub type ratatui::backend::ClearType::Owned = T
+pub fn ratatui::backend::ClearType::clone_into(&self, target: &mut T)
+pub fn ratatui::backend::ClearType::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::backend::ClearType where T: 'static + core::marker::Sized
+pub fn ratatui::backend::ClearType::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::backend::ClearType where T: core::marker::Sized
+pub fn ratatui::backend::ClearType::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::backend::ClearType where T: core::marker::Sized
+pub fn ratatui::backend::ClearType::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::backend::ClearType
+pub fn ratatui::backend::ClearType::from(t: T) -> T
+impl<W> ratatui::backend::CrosstermBackend<W> where W: std::io::Write
+impl<W> ratatui::backend::CrosstermBackend<W> where W: std::io::Write
+pub fn ratatui::backend::CrosstermBackend<W>::new(buffer: W) -> ratatui::backend::CrosstermBackend<W>
+pub fn ratatui::backend::CrosstermBackend<W>::new(buffer: W) -> ratatui::backend::CrosstermBackend<W>
+impl<W> ratatui::backend::Backend for ratatui::backend::CrosstermBackend<W> where W: std::io::Write
+impl<W> ratatui::backend::Backend for ratatui::backend::CrosstermBackend<W> where W: std::io::Write
+impl<W> ratatui::backend::Backend for ratatui::backend::CrosstermBackend<W> where W: std::io::Write
+impl<W> ratatui::backend::Backend for ratatui::backend::CrosstermBackend<W> where W: std::io::Write
+pub fn ratatui::backend::CrosstermBackend<W>::append_lines(&mut self, n: u16) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::append_lines(&mut self, n: u16) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::append_lines(&mut self, n: u16) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::append_lines(&mut self, n: u16) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::clear(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::clear(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::clear(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::clear(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::clear_region(&mut self, clear_type: ratatui::backend::ClearType) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::clear_region(&mut self, clear_type: ratatui::backend::ClearType) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::clear_region(&mut self, clear_type: ratatui::backend::ClearType) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::clear_region(&mut self, clear_type: ratatui::backend::ClearType) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::draw<'a, I>(&mut self, content: I) -> std::io::error::Result<()> where I: core::iter::traits::iterator::Iterator<Item = (u16, u16, &'a ratatui::buffer::Cell)>
+pub fn ratatui::backend::CrosstermBackend<W>::draw<'a, I>(&mut self, content: I) -> std::io::error::Result<()> where I: core::iter::traits::iterator::Iterator<Item = (u16, u16, &'a ratatui::buffer::Cell)>
+pub fn ratatui::backend::CrosstermBackend<W>::draw<'a, I>(&mut self, content: I) -> std::io::error::Result<()> where I: core::iter::traits::iterator::Iterator<Item = (u16, u16, &'a ratatui::buffer::Cell)>
+pub fn ratatui::backend::CrosstermBackend<W>::draw<'a, I>(&mut self, content: I) -> std::io::error::Result<()> where I: core::iter::traits::iterator::Iterator<Item = (u16, u16, &'a ratatui::buffer::Cell)>
+pub fn ratatui::backend::CrosstermBackend<W>::flush(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::flush(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::flush(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::flush(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::flush(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::flush(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::get_cursor(&mut self) -> std::io::error::Result<(u16, u16)>
+pub fn ratatui::backend::CrosstermBackend<W>::get_cursor(&mut self) -> std::io::error::Result<(u16, u16)>
+pub fn ratatui::backend::CrosstermBackend<W>::get_cursor(&mut self) -> std::io::error::Result<(u16, u16)>
+pub fn ratatui::backend::CrosstermBackend<W>::get_cursor(&mut self) -> std::io::error::Result<(u16, u16)>
+pub fn ratatui::backend::CrosstermBackend<W>::hide_cursor(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::hide_cursor(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::hide_cursor(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::hide_cursor(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::set_cursor(&mut self, x: u16, y: u16) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::set_cursor(&mut self, x: u16, y: u16) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::set_cursor(&mut self, x: u16, y: u16) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::set_cursor(&mut self, x: u16, y: u16) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::show_cursor(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::show_cursor(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::show_cursor(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::show_cursor(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::size(&self) -> std::io::error::Result<ratatui::layout::Rect>
+pub fn ratatui::backend::CrosstermBackend<W>::size(&self) -> std::io::error::Result<ratatui::layout::Rect>
+pub fn ratatui::backend::CrosstermBackend<W>::size(&self) -> std::io::error::Result<ratatui::layout::Rect>
+pub fn ratatui::backend::CrosstermBackend<W>::size(&self) -> std::io::error::Result<ratatui::layout::Rect>
+impl<W> std::io::Write for ratatui::backend::CrosstermBackend<W> where W: std::io::Write
+impl<W> std::io::Write for ratatui::backend::CrosstermBackend<W> where W: std::io::Write
+pub fn ratatui::backend::CrosstermBackend<W>::write(&mut self, buf: &[u8]) -> std::io::error::Result<usize>
+pub fn ratatui::backend::CrosstermBackend<W>::write(&mut self, buf: &[u8]) -> std::io::error::Result<usize>
+impl<W> core::marker::Send for ratatui::backend::CrosstermBackend<W> where W: core::marker::Send
+impl<W> core::marker::Send for ratatui::backend::CrosstermBackend<W> where W: core::marker::Send
+impl<W> core::marker::Sync for ratatui::backend::CrosstermBackend<W> where W: core::marker::Sync
+impl<W> core::marker::Sync for ratatui::backend::CrosstermBackend<W> where W: core::marker::Sync
+impl<W> core::marker::Unpin for ratatui::backend::CrosstermBackend<W> where W: core::marker::Unpin
+impl<W> core::marker::Unpin for ratatui::backend::CrosstermBackend<W> where W: core::marker::Unpin
+impl<W> core::panic::unwind_safe::RefUnwindSafe for ratatui::backend::CrosstermBackend<W> where W: core::panic::unwind_safe::RefUnwindSafe
+impl<W> core::panic::unwind_safe::RefUnwindSafe for ratatui::backend::CrosstermBackend<W> where W: core::panic::unwind_safe::RefUnwindSafe
+impl<W> core::panic::unwind_safe::UnwindSafe for ratatui::backend::CrosstermBackend<W> where W: core::panic::unwind_safe::UnwindSafe
+impl<W> core::panic::unwind_safe::UnwindSafe for ratatui::backend::CrosstermBackend<W> where W: core::panic::unwind_safe::UnwindSafe
+impl<T, U> core::convert::Into<U> for ratatui::backend::CrosstermBackend<W> where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::backend::CrosstermBackend<W> where U: core::convert::From<T>
+pub fn ratatui::backend::CrosstermBackend<W>::into(self) -> U
+pub fn ratatui::backend::CrosstermBackend<W>::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::backend::CrosstermBackend<W> where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::backend::CrosstermBackend<W> where U: core::convert::Into<T>
+pub type ratatui::backend::CrosstermBackend<W>::Error = core::convert::Infallible
+pub type ratatui::backend::CrosstermBackend<W>::Error = core::convert::Infallible
+pub fn ratatui::backend::CrosstermBackend<W>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::backend::CrosstermBackend<W>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::backend::CrosstermBackend<W> where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::backend::CrosstermBackend<W> where U: core::convert::TryFrom<T>
+pub type ratatui::backend::CrosstermBackend<W>::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::backend::CrosstermBackend<W>::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::backend::CrosstermBackend<W>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::backend::CrosstermBackend<W>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> core::any::Any for ratatui::backend::CrosstermBackend<W> where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::backend::CrosstermBackend<W> where T: 'static + core::marker::Sized
+pub fn ratatui::backend::CrosstermBackend<W>::type_id(&self) -> core::any::TypeId
+pub fn ratatui::backend::CrosstermBackend<W>::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::backend::CrosstermBackend<W> where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::backend::CrosstermBackend<W> where T: core::marker::Sized
+pub fn ratatui::backend::CrosstermBackend<W>::borrow(&self) -> &T
+pub fn ratatui::backend::CrosstermBackend<W>::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::backend::CrosstermBackend<W> where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::backend::CrosstermBackend<W> where T: core::marker::Sized
+pub fn ratatui::backend::CrosstermBackend<W>::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::backend::CrosstermBackend<W>::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::backend::CrosstermBackend<W>
+impl<T> core::convert::From<T> for ratatui::backend::CrosstermBackend<W>
+pub fn ratatui::backend::CrosstermBackend<W>::from(t: T) -> T
+pub fn ratatui::backend::CrosstermBackend<W>::from(t: T) -> T
+impl<T> crossterm::command::ExecutableCommand for ratatui::backend::CrosstermBackend<W> where T: std::io::Write + core::marker::Sized
+impl<T> crossterm::command::ExecutableCommand for ratatui::backend::CrosstermBackend<W> where T: std::io::Write + core::marker::Sized
+pub fn ratatui::backend::CrosstermBackend<W>::execute(&mut self, command: impl crossterm::command::Command) -> core::result::Result<&mut T, std::io::error::Error>
+pub fn ratatui::backend::CrosstermBackend<W>::execute(&mut self, command: impl crossterm::command::Command) -> core::result::Result<&mut T, std::io::error::Error>
+impl<T> crossterm::command::QueueableCommand for ratatui::backend::CrosstermBackend<W> where T: std::io::Write + core::marker::Sized
+impl<T> crossterm::command::QueueableCommand for ratatui::backend::CrosstermBackend<W> where T: std::io::Write + core::marker::Sized
+pub fn ratatui::backend::CrosstermBackend<W>::queue(&mut self, command: impl crossterm::command::Command) -> core::result::Result<&mut T, std::io::error::Error>
+pub fn ratatui::backend::CrosstermBackend<W>::queue(&mut self, command: impl crossterm::command::Command) -> core::result::Result<&mut T, std::io::error::Error>
+impl<W> crossterm::command::SynchronizedUpdate for ratatui::backend::CrosstermBackend<W> where W: std::io::Write + core::marker::Sized
+impl<W> crossterm::command::SynchronizedUpdate for ratatui::backend::CrosstermBackend<W> where W: std::io::Write + core::marker::Sized
+pub fn ratatui::backend::CrosstermBackend<W>::sync_update<T>(&mut self, operations: impl core::ops::function::FnOnce(&mut W) -> T) -> core::result::Result<T, std::io::error::Error>
+pub fn ratatui::backend::CrosstermBackend<W>::sync_update<T>(&mut self, operations: impl core::ops::function::FnOnce(&mut W) -> T) -> core::result::Result<T, std::io::error::Error>
+impl ratatui::backend::TestBackend
+pub fn ratatui::backend::TestBackend::assert_buffer(&self, expected: &ratatui::buffer::Buffer)
+pub fn ratatui::backend::TestBackend::buffer(&self) -> &ratatui::buffer::Buffer
+pub fn ratatui::backend::TestBackend::new(width: u16, height: u16) -> ratatui::backend::TestBackend
+pub fn ratatui::backend::TestBackend::resize(&mut self, width: u16, height: u16)
+impl core::fmt::Display for ratatui::backend::TestBackend
+pub fn ratatui::backend::TestBackend::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::backend::TestBackend::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl ratatui::backend::Backend for ratatui::backend::TestBackend
+impl ratatui::backend::Backend for ratatui::backend::TestBackend
+impl ratatui::backend::Backend for ratatui::backend::TestBackend
+pub fn ratatui::backend::TestBackend::clear(&mut self) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::backend::TestBackend::clear(&mut self) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::backend::TestBackend::clear(&mut self) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::backend::TestBackend::draw<'a, I>(&mut self, content: I) -> core::result::Result<(), std::io::error::Error> where I: core::iter::traits::iterator::Iterator<Item = (u16, u16, &'a ratatui::buffer::Cell)>
+pub fn ratatui::backend::TestBackend::draw<'a, I>(&mut self, content: I) -> core::result::Result<(), std::io::error::Error> where I: core::iter::traits::iterator::Iterator<Item = (u16, u16, &'a ratatui::buffer::Cell)>
+pub fn ratatui::backend::TestBackend::draw<'a, I>(&mut self, content: I) -> core::result::Result<(), std::io::error::Error> where I: core::iter::traits::iterator::Iterator<Item = (u16, u16, &'a ratatui::buffer::Cell)>
+pub fn ratatui::backend::TestBackend::flush(&mut self) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::backend::TestBackend::flush(&mut self) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::backend::TestBackend::flush(&mut self) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::backend::TestBackend::get_cursor(&mut self) -> core::result::Result<(u16, u16), std::io::error::Error>
+pub fn ratatui::backend::TestBackend::get_cursor(&mut self) -> core::result::Result<(u16, u16), std::io::error::Error>
+pub fn ratatui::backend::TestBackend::get_cursor(&mut self) -> core::result::Result<(u16, u16), std::io::error::Error>
+pub fn ratatui::backend::TestBackend::hide_cursor(&mut self) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::backend::TestBackend::hide_cursor(&mut self) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::backend::TestBackend::hide_cursor(&mut self) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::backend::TestBackend::set_cursor(&mut self, x: u16, y: u16) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::backend::TestBackend::set_cursor(&mut self, x: u16, y: u16) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::backend::TestBackend::set_cursor(&mut self, x: u16, y: u16) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::backend::TestBackend::show_cursor(&mut self) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::backend::TestBackend::show_cursor(&mut self) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::backend::TestBackend::show_cursor(&mut self) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::backend::TestBackend::size(&self) -> core::result::Result<ratatui::layout::Rect, std::io::error::Error>
+pub fn ratatui::backend::TestBackend::size(&self) -> core::result::Result<ratatui::layout::Rect, std::io::error::Error>
+pub fn ratatui::backend::TestBackend::size(&self) -> core::result::Result<ratatui::layout::Rect, std::io::error::Error>
+impl core::fmt::Debug for ratatui::backend::TestBackend
+impl core::marker::Send for ratatui::backend::TestBackend
+impl core::marker::Sync for ratatui::backend::TestBackend
+impl core::marker::Unpin for ratatui::backend::TestBackend
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::backend::TestBackend
+impl core::panic::unwind_safe::UnwindSafe for ratatui::backend::TestBackend
+impl<T, U> core::convert::Into<U> for ratatui::backend::TestBackend where U: core::convert::From<T>
+pub fn ratatui::backend::TestBackend::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::backend::TestBackend where U: core::convert::Into<T>
+pub type ratatui::backend::TestBackend::Error = core::convert::Infallible
+pub fn ratatui::backend::TestBackend::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::backend::TestBackend where U: core::convert::TryFrom<T>
+pub type ratatui::backend::TestBackend::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::backend::TestBackend::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::string::ToString for ratatui::backend::TestBackend where T: core::fmt::Display + core::marker::Sized
+pub fn ratatui::backend::TestBackend::to_string(&self) -> alloc::string::String
+impl<T> core::any::Any for ratatui::backend::TestBackend where T: 'static + core::marker::Sized
+pub fn ratatui::backend::TestBackend::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::backend::TestBackend where T: core::marker::Sized
+pub fn ratatui::backend::TestBackend::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::backend::TestBackend where T: core::marker::Sized
+pub fn ratatui::backend::TestBackend::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::backend::TestBackend
+pub fn ratatui::backend::TestBackend::from(t: T) -> T
+impl ratatui::buffer::Buffer
+impl ratatui::buffer::Buffer
+pub fn ratatui::buffer::Buffer::area(&self) -> &ratatui::layout::Rect
+pub fn ratatui::buffer::Buffer::area(&self) -> &ratatui::layout::Rect
+pub fn ratatui::buffer::Buffer::content(&self) -> &[ratatui::buffer::Cell]
+pub fn ratatui::buffer::Buffer::content(&self) -> &[ratatui::buffer::Cell]
+pub fn ratatui::buffer::Buffer::diff<'a>(&self, other: &'a ratatui::buffer::Buffer) -> alloc::vec::Vec<(u16, u16, &'a ratatui::buffer::Cell)>
+pub fn ratatui::buffer::Buffer::diff<'a>(&self, other: &'a ratatui::buffer::Buffer) -> alloc::vec::Vec<(u16, u16, &'a ratatui::buffer::Cell)>
+pub fn ratatui::buffer::Buffer::empty(area: ratatui::layout::Rect) -> ratatui::buffer::Buffer
+pub fn ratatui::buffer::Buffer::empty(area: ratatui::layout::Rect) -> ratatui::buffer::Buffer
+pub fn ratatui::buffer::Buffer::filled(area: ratatui::layout::Rect, cell: &ratatui::buffer::Cell) -> ratatui::buffer::Buffer
+pub fn ratatui::buffer::Buffer::filled(area: ratatui::layout::Rect, cell: &ratatui::buffer::Cell) -> ratatui::buffer::Buffer
+pub fn ratatui::buffer::Buffer::get(&self, x: u16, y: u16) -> &ratatui::buffer::Cell
+pub fn ratatui::buffer::Buffer::get(&self, x: u16, y: u16) -> &ratatui::buffer::Cell
+pub fn ratatui::buffer::Buffer::get_mut(&mut self, x: u16, y: u16) -> &mut ratatui::buffer::Cell
+pub fn ratatui::buffer::Buffer::get_mut(&mut self, x: u16, y: u16) -> &mut ratatui::buffer::Cell
+pub fn ratatui::buffer::Buffer::index_of(&self, x: u16, y: u16) -> usize
+pub fn ratatui::buffer::Buffer::index_of(&self, x: u16, y: u16) -> usize
+pub fn ratatui::buffer::Buffer::merge(&mut self, other: &ratatui::buffer::Buffer)
+pub fn ratatui::buffer::Buffer::merge(&mut self, other: &ratatui::buffer::Buffer)
+pub fn ratatui::buffer::Buffer::pos_of(&self, i: usize) -> (u16, u16)
+pub fn ratatui::buffer::Buffer::pos_of(&self, i: usize) -> (u16, u16)
+pub fn ratatui::buffer::Buffer::reset(&mut self)
+pub fn ratatui::buffer::Buffer::reset(&mut self)
+pub fn ratatui::buffer::Buffer::resize(&mut self, area: ratatui::layout::Rect)
+pub fn ratatui::buffer::Buffer::resize(&mut self, area: ratatui::layout::Rect)
+pub fn ratatui::buffer::Buffer::set_background(&mut self, area: ratatui::layout::Rect, color: ratatui::style::Color)
+pub fn ratatui::buffer::Buffer::set_background(&mut self, area: ratatui::layout::Rect, color: ratatui::style::Color)
+pub fn ratatui::buffer::Buffer::set_line(&mut self, x: u16, y: u16, line: &ratatui::text::Line<'_>, width: u16) -> (u16, u16)
+pub fn ratatui::buffer::Buffer::set_line(&mut self, x: u16, y: u16, line: &ratatui::text::Line<'_>, width: u16) -> (u16, u16)
+pub fn ratatui::buffer::Buffer::set_span(&mut self, x: u16, y: u16, span: &ratatui::text::Span<'_>, width: u16) -> (u16, u16)
+pub fn ratatui::buffer::Buffer::set_span(&mut self, x: u16, y: u16, span: &ratatui::text::Span<'_>, width: u16) -> (u16, u16)
+pub fn ratatui::buffer::Buffer::set_spans(&mut self, x: u16, y: u16, spans: &ratatui::text::Spans<'_>, width: u16) -> (u16, u16)
+pub fn ratatui::buffer::Buffer::set_spans(&mut self, x: u16, y: u16, spans: &ratatui::text::Spans<'_>, width: u16) -> (u16, u16)
+pub fn ratatui::buffer::Buffer::set_string<S>(&mut self, x: u16, y: u16, string: S, style: ratatui::style::Style) where S: core::convert::AsRef<str>
+pub fn ratatui::buffer::Buffer::set_string<S>(&mut self, x: u16, y: u16, string: S, style: ratatui::style::Style) where S: core::convert::AsRef<str>
+pub fn ratatui::buffer::Buffer::set_stringn<S>(&mut self, x: u16, y: u16, string: S, width: usize, style: ratatui::style::Style) -> (u16, u16) where S: core::convert::AsRef<str>
+pub fn ratatui::buffer::Buffer::set_stringn<S>(&mut self, x: u16, y: u16, string: S, width: usize, style: ratatui::style::Style) -> (u16, u16) where S: core::convert::AsRef<str>
+pub fn ratatui::buffer::Buffer::set_style(&mut self, area: ratatui::layout::Rect, style: ratatui::style::Style)
+pub fn ratatui::buffer::Buffer::set_style(&mut self, area: ratatui::layout::Rect, style: ratatui::style::Style)
+pub fn ratatui::buffer::Buffer::with_lines<S>(lines: alloc::vec::Vec<S>) -> ratatui::buffer::Buffer where S: core::convert::AsRef<str>
+pub fn ratatui::buffer::Buffer::with_lines<S>(lines: alloc::vec::Vec<S>) -> ratatui::buffer::Buffer where S: core::convert::AsRef<str>
+impl core::fmt::Debug for ratatui::buffer::Buffer
+impl core::fmt::Debug for ratatui::buffer::Buffer
+pub fn ratatui::buffer::Buffer::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::buffer::Buffer::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::clone::Clone for ratatui::buffer::Buffer
+impl core::clone::Clone for ratatui::buffer::Buffer
+pub fn ratatui::buffer::Buffer::clone(&self) -> ratatui::buffer::Buffer
+pub fn ratatui::buffer::Buffer::clone(&self) -> ratatui::buffer::Buffer
+impl core::cmp::Eq for ratatui::buffer::Buffer
+impl core::cmp::Eq for ratatui::buffer::Buffer
+impl core::cmp::PartialEq<ratatui::buffer::Buffer> for ratatui::buffer::Buffer
+impl core::cmp::PartialEq<ratatui::buffer::Buffer> for ratatui::buffer::Buffer
+pub fn ratatui::buffer::Buffer::eq(&self, other: &ratatui::buffer::Buffer) -> bool
+pub fn ratatui::buffer::Buffer::eq(&self, other: &ratatui::buffer::Buffer) -> bool
+impl core::default::Default for ratatui::buffer::Buffer
+impl core::default::Default for ratatui::buffer::Buffer
+pub fn ratatui::buffer::Buffer::default() -> ratatui::buffer::Buffer
+pub fn ratatui::buffer::Buffer::default() -> ratatui::buffer::Buffer
+impl core::marker::StructuralEq for ratatui::buffer::Buffer
+impl core::marker::StructuralEq for ratatui::buffer::Buffer
+impl core::marker::StructuralPartialEq for ratatui::buffer::Buffer
+impl core::marker::StructuralPartialEq for ratatui::buffer::Buffer
+impl core::marker::Send for ratatui::buffer::Buffer
+impl core::marker::Send for ratatui::buffer::Buffer
+impl core::marker::Sync for ratatui::buffer::Buffer
+impl core::marker::Sync for ratatui::buffer::Buffer
+impl core::marker::Unpin for ratatui::buffer::Buffer
+impl core::marker::Unpin for ratatui::buffer::Buffer
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::buffer::Buffer
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::buffer::Buffer
+impl core::panic::unwind_safe::UnwindSafe for ratatui::buffer::Buffer
+impl core::panic::unwind_safe::UnwindSafe for ratatui::buffer::Buffer
+impl<T, U> core::convert::Into<U> for ratatui::buffer::Buffer where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::buffer::Buffer where U: core::convert::From<T>
+pub fn ratatui::buffer::Buffer::into(self) -> U
+pub fn ratatui::buffer::Buffer::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::buffer::Buffer where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::buffer::Buffer where U: core::convert::Into<T>
+pub type ratatui::buffer::Buffer::Error = core::convert::Infallible
+pub type ratatui::buffer::Buffer::Error = core::convert::Infallible
+pub fn ratatui::buffer::Buffer::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::buffer::Buffer::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::buffer::Buffer where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::buffer::Buffer where U: core::convert::TryFrom<T>
+pub type ratatui::buffer::Buffer::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::buffer::Buffer::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::buffer::Buffer::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::buffer::Buffer::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::buffer::Buffer where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::buffer::Buffer where T: core::clone::Clone
+pub type ratatui::buffer::Buffer::Owned = T
+pub type ratatui::buffer::Buffer::Owned = T
+pub fn ratatui::buffer::Buffer::clone_into(&self, target: &mut T)
+pub fn ratatui::buffer::Buffer::clone_into(&self, target: &mut T)
+pub fn ratatui::buffer::Buffer::to_owned(&self) -> T
+pub fn ratatui::buffer::Buffer::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::buffer::Buffer where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::buffer::Buffer where T: 'static + core::marker::Sized
+pub fn ratatui::buffer::Buffer::type_id(&self) -> core::any::TypeId
+pub fn ratatui::buffer::Buffer::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::buffer::Buffer where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::buffer::Buffer where T: core::marker::Sized
+pub fn ratatui::buffer::Buffer::borrow(&self) -> &T
+pub fn ratatui::buffer::Buffer::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::buffer::Buffer where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::buffer::Buffer where T: core::marker::Sized
+pub fn ratatui::buffer::Buffer::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::buffer::Buffer::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::buffer::Buffer
+impl<T> core::convert::From<T> for ratatui::buffer::Buffer
+pub fn ratatui::buffer::Buffer::from(t: T) -> T
+pub fn ratatui::buffer::Buffer::from(t: T) -> T
+pub ratatui::buffer::Cell::underline_color: ratatui::style::Color
+impl ratatui::buffer::Cell
+pub fn ratatui::buffer::Cell::reset(&mut self)
+pub fn ratatui::buffer::Cell::set_bg(&mut self, color: ratatui::style::Color) -> &mut ratatui::buffer::Cell
+pub fn ratatui::buffer::Cell::set_char(&mut self, ch: char) -> &mut ratatui::buffer::Cell
+pub fn ratatui::buffer::Cell::set_fg(&mut self, color: ratatui::style::Color) -> &mut ratatui::buffer::Cell
+pub fn ratatui::buffer::Cell::set_style(&mut self, style: ratatui::style::Style) -> &mut ratatui::buffer::Cell
+pub fn ratatui::buffer::Cell::set_symbol(&mut self, symbol: &str) -> &mut ratatui::buffer::Cell
+pub fn ratatui::buffer::Cell::style(&self) -> ratatui::style::Style
+impl core::default::Default for ratatui::buffer::Cell
+pub fn ratatui::buffer::Cell::default() -> ratatui::buffer::Cell
+impl core::clone::Clone for ratatui::buffer::Cell
+pub fn ratatui::buffer::Cell::clone(&self) -> ratatui::buffer::Cell
+impl core::cmp::Eq for ratatui::buffer::Cell
+impl core::cmp::PartialEq<ratatui::buffer::Cell> for ratatui::buffer::Cell
+pub fn ratatui::buffer::Cell::eq(&self, other: &ratatui::buffer::Cell) -> bool
+impl core::fmt::Debug for ratatui::buffer::Cell
+pub fn ratatui::buffer::Cell::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::marker::StructuralEq for ratatui::buffer::Cell
+impl core::marker::StructuralPartialEq for ratatui::buffer::Cell
+impl core::marker::Send for ratatui::buffer::Cell
+impl core::marker::Sync for ratatui::buffer::Cell
+impl core::marker::Unpin for ratatui::buffer::Cell
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::buffer::Cell
+impl core::panic::unwind_safe::UnwindSafe for ratatui::buffer::Cell
+impl<T, U> core::convert::Into<U> for ratatui::buffer::Cell where U: core::convert::From<T>
+pub fn ratatui::buffer::Cell::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::buffer::Cell where U: core::convert::Into<T>
+pub type ratatui::buffer::Cell::Error = core::convert::Infallible
+pub fn ratatui::buffer::Cell::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::buffer::Cell where U: core::convert::TryFrom<T>
+pub type ratatui::buffer::Cell::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::buffer::Cell::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::buffer::Cell where T: core::clone::Clone
+pub type ratatui::buffer::Cell::Owned = T
+pub fn ratatui::buffer::Cell::clone_into(&self, target: &mut T)
+pub fn ratatui::buffer::Cell::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::buffer::Cell where T: 'static + core::marker::Sized
+pub fn ratatui::buffer::Cell::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::buffer::Cell where T: core::marker::Sized
+pub fn ratatui::buffer::Cell::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::buffer::Cell where T: core::marker::Sized
+pub fn ratatui::buffer::Cell::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::buffer::Cell
+pub fn ratatui::buffer::Cell::from(t: T) -> T
+impl core::clone::Clone for ratatui::layout::Alignment
+impl core::clone::Clone for ratatui::layout::Alignment
+pub fn ratatui::layout::Alignment::clone(&self) -> ratatui::layout::Alignment
+pub fn ratatui::layout::Alignment::clone(&self) -> ratatui::layout::Alignment
+impl core::cmp::Eq for ratatui::layout::Alignment
+impl core::cmp::Eq for ratatui::layout::Alignment
+impl core::cmp::PartialEq<ratatui::layout::Alignment> for ratatui::layout::Alignment
+impl core::cmp::PartialEq<ratatui::layout::Alignment> for ratatui::layout::Alignment
+pub fn ratatui::layout::Alignment::eq(&self, other: &ratatui::layout::Alignment) -> bool
+pub fn ratatui::layout::Alignment::eq(&self, other: &ratatui::layout::Alignment) -> bool
+impl core::fmt::Debug for ratatui::layout::Alignment
+impl core::fmt::Debug for ratatui::layout::Alignment
+pub fn ratatui::layout::Alignment::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::layout::Alignment::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::hash::Hash for ratatui::layout::Alignment
+impl core::hash::Hash for ratatui::layout::Alignment
+impl core::hash::Hash for ratatui::layout::Alignment
+pub fn ratatui::layout::Alignment::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
+pub fn ratatui::layout::Alignment::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
+pub fn ratatui::layout::Alignment::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
+impl core::marker::Copy for ratatui::layout::Alignment
+impl core::marker::Copy for ratatui::layout::Alignment
+impl core::marker::StructuralEq for ratatui::layout::Alignment
+impl core::marker::StructuralEq for ratatui::layout::Alignment
+impl core::marker::StructuralPartialEq for ratatui::layout::Alignment
+impl core::marker::StructuralPartialEq for ratatui::layout::Alignment
+impl core::marker::Send for ratatui::layout::Alignment
+impl core::marker::Send for ratatui::layout::Alignment
+impl core::marker::Sync for ratatui::layout::Alignment
+impl core::marker::Sync for ratatui::layout::Alignment
+impl core::marker::Unpin for ratatui::layout::Alignment
+impl core::marker::Unpin for ratatui::layout::Alignment
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::layout::Alignment
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::layout::Alignment
+impl core::panic::unwind_safe::UnwindSafe for ratatui::layout::Alignment
+impl core::panic::unwind_safe::UnwindSafe for ratatui::layout::Alignment
+impl<T, U> core::convert::Into<U> for ratatui::layout::Alignment where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::layout::Alignment where U: core::convert::From<T>
+pub fn ratatui::layout::Alignment::into(self) -> U
+pub fn ratatui::layout::Alignment::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::layout::Alignment where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::layout::Alignment where U: core::convert::Into<T>
+pub type ratatui::layout::Alignment::Error = core::convert::Infallible
+pub type ratatui::layout::Alignment::Error = core::convert::Infallible
+pub fn ratatui::layout::Alignment::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::layout::Alignment::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::layout::Alignment where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::layout::Alignment where U: core::convert::TryFrom<T>
+pub type ratatui::layout::Alignment::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::layout::Alignment::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::layout::Alignment::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::layout::Alignment::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::layout::Alignment where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::layout::Alignment where T: core::clone::Clone
+pub type ratatui::layout::Alignment::Owned = T
+pub type ratatui::layout::Alignment::Owned = T
+pub fn ratatui::layout::Alignment::clone_into(&self, target: &mut T)
+pub fn ratatui::layout::Alignment::clone_into(&self, target: &mut T)
+pub fn ratatui::layout::Alignment::to_owned(&self) -> T
+pub fn ratatui::layout::Alignment::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::layout::Alignment where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::layout::Alignment where T: 'static + core::marker::Sized
+pub fn ratatui::layout::Alignment::type_id(&self) -> core::any::TypeId
+pub fn ratatui::layout::Alignment::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::layout::Alignment where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::layout::Alignment where T: core::marker::Sized
+pub fn ratatui::layout::Alignment::borrow(&self) -> &T
+pub fn ratatui::layout::Alignment::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::layout::Alignment where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::layout::Alignment where T: core::marker::Sized
+pub fn ratatui::layout::Alignment::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::layout::Alignment::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::layout::Alignment
+impl<T> core::convert::From<T> for ratatui::layout::Alignment
+pub fn ratatui::layout::Alignment::from(t: T) -> T
+pub fn ratatui::layout::Alignment::from(t: T) -> T
+impl ratatui::layout::Constraint
+impl ratatui::layout::Constraint
+pub fn ratatui::layout::Constraint::apply(&self, length: u16) -> u16
+pub fn ratatui::layout::Constraint::apply(&self, length: u16) -> u16
+impl core::clone::Clone for ratatui::layout::Constraint
+impl core::clone::Clone for ratatui::layout::Constraint
+pub fn ratatui::layout::Constraint::clone(&self) -> ratatui::layout::Constraint
+pub fn ratatui::layout::Constraint::clone(&self) -> ratatui::layout::Constraint
+impl core::cmp::Eq for ratatui::layout::Constraint
+impl core::cmp::Eq for ratatui::layout::Constraint
+impl core::cmp::PartialEq<ratatui::layout::Constraint> for ratatui::layout::Constraint
+impl core::cmp::PartialEq<ratatui::layout::Constraint> for ratatui::layout::Constraint
+pub fn ratatui::layout::Constraint::eq(&self, other: &ratatui::layout::Constraint) -> bool
+pub fn ratatui::layout::Constraint::eq(&self, other: &ratatui::layout::Constraint) -> bool
+impl core::fmt::Debug for ratatui::layout::Constraint
+impl core::fmt::Debug for ratatui::layout::Constraint
+pub fn ratatui::layout::Constraint::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::layout::Constraint::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::hash::Hash for ratatui::layout::Constraint
+impl core::hash::Hash for ratatui::layout::Constraint
+pub fn ratatui::layout::Constraint::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
+pub fn ratatui::layout::Constraint::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
+impl core::marker::Copy for ratatui::layout::Constraint
+impl core::marker::Copy for ratatui::layout::Constraint
+impl core::marker::StructuralEq for ratatui::layout::Constraint
+impl core::marker::StructuralEq for ratatui::layout::Constraint
+impl core::marker::StructuralPartialEq for ratatui::layout::Constraint
+impl core::marker::StructuralPartialEq for ratatui::layout::Constraint
+impl core::marker::Send for ratatui::layout::Constraint
+impl core::marker::Send for ratatui::layout::Constraint
+impl core::marker::Sync for ratatui::layout::Constraint
+impl core::marker::Sync for ratatui::layout::Constraint
+impl core::marker::Unpin for ratatui::layout::Constraint
+impl core::marker::Unpin for ratatui::layout::Constraint
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::layout::Constraint
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::layout::Constraint
+impl core::panic::unwind_safe::UnwindSafe for ratatui::layout::Constraint
+impl core::panic::unwind_safe::UnwindSafe for ratatui::layout::Constraint
+impl<T, U> core::convert::Into<U> for ratatui::layout::Constraint where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::layout::Constraint where U: core::convert::From<T>
+pub fn ratatui::layout::Constraint::into(self) -> U
+pub fn ratatui::layout::Constraint::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::layout::Constraint where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::layout::Constraint where U: core::convert::Into<T>
+pub type ratatui::layout::Constraint::Error = core::convert::Infallible
+pub type ratatui::layout::Constraint::Error = core::convert::Infallible
+pub fn ratatui::layout::Constraint::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::layout::Constraint::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::layout::Constraint where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::layout::Constraint where U: core::convert::TryFrom<T>
+pub type ratatui::layout::Constraint::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::layout::Constraint::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::layout::Constraint::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::layout::Constraint::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::layout::Constraint where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::layout::Constraint where T: core::clone::Clone
+pub type ratatui::layout::Constraint::Owned = T
+pub type ratatui::layout::Constraint::Owned = T
+pub fn ratatui::layout::Constraint::clone_into(&self, target: &mut T)
+pub fn ratatui::layout::Constraint::clone_into(&self, target: &mut T)
+pub fn ratatui::layout::Constraint::to_owned(&self) -> T
+pub fn ratatui::layout::Constraint::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::layout::Constraint where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::layout::Constraint where T: 'static + core::marker::Sized
+pub fn ratatui::layout::Constraint::type_id(&self) -> core::any::TypeId
+pub fn ratatui::layout::Constraint::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::layout::Constraint where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::layout::Constraint where T: core::marker::Sized
+pub fn ratatui::layout::Constraint::borrow(&self) -> &T
+pub fn ratatui::layout::Constraint::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::layout::Constraint where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::layout::Constraint where T: core::marker::Sized
+pub fn ratatui::layout::Constraint::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::layout::Constraint::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::layout::Constraint
+impl<T> core::convert::From<T> for ratatui::layout::Constraint
+pub fn ratatui::layout::Constraint::from(t: T) -> T
+pub fn ratatui::layout::Constraint::from(t: T) -> T
+impl core::clone::Clone for ratatui::layout::Corner
+impl core::clone::Clone for ratatui::layout::Corner
+pub fn ratatui::layout::Corner::clone(&self) -> ratatui::layout::Corner
+pub fn ratatui::layout::Corner::clone(&self) -> ratatui::layout::Corner
+impl core::cmp::Eq for ratatui::layout::Corner
+impl core::cmp::Eq for ratatui::layout::Corner
+impl core::cmp::PartialEq<ratatui::layout::Corner> for ratatui::layout::Corner
+impl core::cmp::PartialEq<ratatui::layout::Corner> for ratatui::layout::Corner
+pub fn ratatui::layout::Corner::eq(&self, other: &ratatui::layout::Corner) -> bool
+pub fn ratatui::layout::Corner::eq(&self, other: &ratatui::layout::Corner) -> bool
+impl core::fmt::Debug for ratatui::layout::Corner
+impl core::fmt::Debug for ratatui::layout::Corner
+pub fn ratatui::layout::Corner::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::layout::Corner::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::hash::Hash for ratatui::layout::Corner
+impl core::hash::Hash for ratatui::layout::Corner
+pub fn ratatui::layout::Corner::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
+pub fn ratatui::layout::Corner::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
+impl core::marker::Copy for ratatui::layout::Corner
+impl core::marker::Copy for ratatui::layout::Corner
+impl core::marker::StructuralEq for ratatui::layout::Corner
+impl core::marker::StructuralEq for ratatui::layout::Corner
+impl core::marker::StructuralPartialEq for ratatui::layout::Corner
+impl core::marker::StructuralPartialEq for ratatui::layout::Corner
+impl core::marker::Send for ratatui::layout::Corner
+impl core::marker::Send for ratatui::layout::Corner
+impl core::marker::Sync for ratatui::layout::Corner
+impl core::marker::Sync for ratatui::layout::Corner
+impl core::marker::Unpin for ratatui::layout::Corner
+impl core::marker::Unpin for ratatui::layout::Corner
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::layout::Corner
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::layout::Corner
+impl core::panic::unwind_safe::UnwindSafe for ratatui::layout::Corner
+impl core::panic::unwind_safe::UnwindSafe for ratatui::layout::Corner
+impl<T, U> core::convert::Into<U> for ratatui::layout::Corner where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::layout::Corner where U: core::convert::From<T>
+pub fn ratatui::layout::Corner::into(self) -> U
+pub fn ratatui::layout::Corner::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::layout::Corner where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::layout::Corner where U: core::convert::Into<T>
+pub type ratatui::layout::Corner::Error = core::convert::Infallible
+pub type ratatui::layout::Corner::Error = core::convert::Infallible
+pub fn ratatui::layout::Corner::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::layout::Corner::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::layout::Corner where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::layout::Corner where U: core::convert::TryFrom<T>
+pub type ratatui::layout::Corner::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::layout::Corner::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::layout::Corner::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::layout::Corner::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::layout::Corner where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::layout::Corner where T: core::clone::Clone
+pub type ratatui::layout::Corner::Owned = T
+pub type ratatui::layout::Corner::Owned = T
+pub fn ratatui::layout::Corner::clone_into(&self, target: &mut T)
+pub fn ratatui::layout::Corner::clone_into(&self, target: &mut T)
+pub fn ratatui::layout::Corner::to_owned(&self) -> T
+pub fn ratatui::layout::Corner::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::layout::Corner where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::layout::Corner where T: 'static + core::marker::Sized
+pub fn ratatui::layout::Corner::type_id(&self) -> core::any::TypeId
+pub fn ratatui::layout::Corner::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::layout::Corner where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::layout::Corner where T: core::marker::Sized
+pub fn ratatui::layout::Corner::borrow(&self) -> &T
+pub fn ratatui::layout::Corner::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::layout::Corner where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::layout::Corner where T: core::marker::Sized
+pub fn ratatui::layout::Corner::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::layout::Corner::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::layout::Corner
+impl<T> core::convert::From<T> for ratatui::layout::Corner
+pub fn ratatui::layout::Corner::from(t: T) -> T
+pub fn ratatui::layout::Corner::from(t: T) -> T
+impl core::clone::Clone for ratatui::layout::Direction
+impl core::clone::Clone for ratatui::layout::Direction
+pub fn ratatui::layout::Direction::clone(&self) -> ratatui::layout::Direction
+pub fn ratatui::layout::Direction::clone(&self) -> ratatui::layout::Direction
+impl core::cmp::Eq for ratatui::layout::Direction
+impl core::cmp::Eq for ratatui::layout::Direction
+impl core::cmp::PartialEq<ratatui::layout::Direction> for ratatui::layout::Direction
+impl core::cmp::PartialEq<ratatui::layout::Direction> for ratatui::layout::Direction
+pub fn ratatui::layout::Direction::eq(&self, other: &ratatui::layout::Direction) -> bool
+pub fn ratatui::layout::Direction::eq(&self, other: &ratatui::layout::Direction) -> bool
+impl core::fmt::Debug for ratatui::layout::Direction
+impl core::fmt::Debug for ratatui::layout::Direction
+pub fn ratatui::layout::Direction::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::layout::Direction::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::hash::Hash for ratatui::layout::Direction
+impl core::hash::Hash for ratatui::layout::Direction
+pub fn ratatui::layout::Direction::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
+pub fn ratatui::layout::Direction::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
+impl core::marker::StructuralEq for ratatui::layout::Direction
+impl core::marker::StructuralEq for ratatui::layout::Direction
+impl core::marker::StructuralPartialEq for ratatui::layout::Direction
+impl core::marker::StructuralPartialEq for ratatui::layout::Direction
+impl core::marker::Send for ratatui::layout::Direction
+impl core::marker::Send for ratatui::layout::Direction
+impl core::marker::Sync for ratatui::layout::Direction
+impl core::marker::Sync for ratatui::layout::Direction
+impl core::marker::Unpin for ratatui::layout::Direction
+impl core::marker::Unpin for ratatui::layout::Direction
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::layout::Direction
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::layout::Direction
+impl core::panic::unwind_safe::UnwindSafe for ratatui::layout::Direction
+impl core::panic::unwind_safe::UnwindSafe for ratatui::layout::Direction
+impl<T, U> core::convert::Into<U> for ratatui::layout::Direction where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::layout::Direction where U: core::convert::From<T>
+pub fn ratatui::layout::Direction::into(self) -> U
+pub fn ratatui::layout::Direction::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::layout::Direction where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::layout::Direction where U: core::convert::Into<T>
+pub type ratatui::layout::Direction::Error = core::convert::Infallible
+pub type ratatui::layout::Direction::Error = core::convert::Infallible
+pub fn ratatui::layout::Direction::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::layout::Direction::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::layout::Direction where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::layout::Direction where U: core::convert::TryFrom<T>
+pub type ratatui::layout::Direction::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::layout::Direction::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::layout::Direction::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::layout::Direction::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::layout::Direction where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::layout::Direction where T: core::clone::Clone
+pub type ratatui::layout::Direction::Owned = T
+pub type ratatui::layout::Direction::Owned = T
+pub fn ratatui::layout::Direction::clone_into(&self, target: &mut T)
+pub fn ratatui::layout::Direction::clone_into(&self, target: &mut T)
+pub fn ratatui::layout::Direction::to_owned(&self) -> T
+pub fn ratatui::layout::Direction::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::layout::Direction where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::layout::Direction where T: 'static + core::marker::Sized
+pub fn ratatui::layout::Direction::type_id(&self) -> core::any::TypeId
+pub fn ratatui::layout::Direction::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::layout::Direction where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::layout::Direction where T: core::marker::Sized
+pub fn ratatui::layout::Direction::borrow(&self) -> &T
+pub fn ratatui::layout::Direction::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::layout::Direction where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::layout::Direction where T: core::marker::Sized
+pub fn ratatui::layout::Direction::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::layout::Direction::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::layout::Direction
+impl<T> core::convert::From<T> for ratatui::layout::Direction
+pub fn ratatui::layout::Direction::from(t: T) -> T
+pub fn ratatui::layout::Direction::from(t: T) -> T
+impl ratatui::layout::Layout
+impl ratatui::layout::Layout
+pub fn ratatui::layout::Layout::constraints<C>(self, constraints: C) -> ratatui::layout::Layout where C: core::convert::Into<alloc::vec::Vec<ratatui::layout::Constraint>>
+pub fn ratatui::layout::Layout::constraints<C>(self, constraints: C) -> ratatui::layout::Layout where C: core::convert::Into<alloc::vec::Vec<ratatui::layout::Constraint>>
+pub const fn ratatui::layout::Layout::direction(self, direction: ratatui::layout::Direction) -> ratatui::layout::Layout
+pub const fn ratatui::layout::Layout::direction(self, direction: ratatui::layout::Direction) -> ratatui::layout::Layout
+pub const fn ratatui::layout::Layout::direction(self, direction: ratatui::layout::Direction) -> ratatui::layout::Layout
+pub const fn ratatui::layout::Layout::horizontal_margin(self, horizontal: u16) -> ratatui::layout::Layout
+pub const fn ratatui::layout::Layout::horizontal_margin(self, horizontal: u16) -> ratatui::layout::Layout
+pub const fn ratatui::layout::Layout::horizontal_margin(self, horizontal: u16) -> ratatui::layout::Layout
+pub const fn ratatui::layout::Layout::margin(self, margin: u16) -> ratatui::layout::Layout
+pub const fn ratatui::layout::Layout::margin(self, margin: u16) -> ratatui::layout::Layout
+pub const fn ratatui::layout::Layout::margin(self, margin: u16) -> ratatui::layout::Layout
+pub const fn ratatui::layout::Layout::new() -> ratatui::layout::Layout
+pub const fn ratatui::layout::Layout::new() -> ratatui::layout::Layout
+pub const fn ratatui::layout::Layout::new() -> ratatui::layout::Layout
+pub fn ratatui::layout::Layout::split(&self, area: ratatui::layout::Rect) -> alloc::rc::Rc<[ratatui::layout::Rect]>
+pub fn ratatui::layout::Layout::split(&self, area: ratatui::layout::Rect) -> alloc::rc::Rc<[ratatui::layout::Rect]>
+pub const fn ratatui::layout::Layout::vertical_margin(self, vertical: u16) -> ratatui::layout::Layout
+pub const fn ratatui::layout::Layout::vertical_margin(self, vertical: u16) -> ratatui::layout::Layout
+pub const fn ratatui::layout::Layout::vertical_margin(self, vertical: u16) -> ratatui::layout::Layout
+impl core::default::Default for ratatui::layout::Layout
+impl core::default::Default for ratatui::layout::Layout
+pub fn ratatui::layout::Layout::default() -> ratatui::layout::Layout
+pub fn ratatui::layout::Layout::default() -> ratatui::layout::Layout
+impl core::clone::Clone for ratatui::layout::Layout
+impl core::clone::Clone for ratatui::layout::Layout
+pub fn ratatui::layout::Layout::clone(&self) -> ratatui::layout::Layout
+pub fn ratatui::layout::Layout::clone(&self) -> ratatui::layout::Layout
+impl core::cmp::Eq for ratatui::layout::Layout
+impl core::cmp::Eq for ratatui::layout::Layout
+impl core::cmp::PartialEq<ratatui::layout::Layout> for ratatui::layout::Layout
+impl core::cmp::PartialEq<ratatui::layout::Layout> for ratatui::layout::Layout
+pub fn ratatui::layout::Layout::eq(&self, other: &ratatui::layout::Layout) -> bool
+pub fn ratatui::layout::Layout::eq(&self, other: &ratatui::layout::Layout) -> bool
+impl core::fmt::Debug for ratatui::layout::Layout
+impl core::fmt::Debug for ratatui::layout::Layout
+pub fn ratatui::layout::Layout::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::layout::Layout::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::hash::Hash for ratatui::layout::Layout
+impl core::hash::Hash for ratatui::layout::Layout
+pub fn ratatui::layout::Layout::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
+pub fn ratatui::layout::Layout::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
+impl core::marker::StructuralEq for ratatui::layout::Layout
+impl core::marker::StructuralEq for ratatui::layout::Layout
+impl core::marker::StructuralPartialEq for ratatui::layout::Layout
+impl core::marker::StructuralPartialEq for ratatui::layout::Layout
+impl core::marker::Send for ratatui::layout::Layout
+impl core::marker::Send for ratatui::layout::Layout
+impl core::marker::Sync for ratatui::layout::Layout
+impl core::marker::Sync for ratatui::layout::Layout
+impl core::marker::Unpin for ratatui::layout::Layout
+impl core::marker::Unpin for ratatui::layout::Layout
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::layout::Layout
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::layout::Layout
+impl core::panic::unwind_safe::UnwindSafe for ratatui::layout::Layout
+impl core::panic::unwind_safe::UnwindSafe for ratatui::layout::Layout
+impl<T, U> core::convert::Into<U> for ratatui::layout::Layout where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::layout::Layout where U: core::convert::From<T>
+pub fn ratatui::layout::Layout::into(self) -> U
+pub fn ratatui::layout::Layout::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::layout::Layout where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::layout::Layout where U: core::convert::Into<T>
+pub type ratatui::layout::Layout::Error = core::convert::Infallible
+pub type ratatui::layout::Layout::Error = core::convert::Infallible
+pub fn ratatui::layout::Layout::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::layout::Layout::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::layout::Layout where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::layout::Layout where U: core::convert::TryFrom<T>
+pub type ratatui::layout::Layout::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::layout::Layout::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::layout::Layout::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::layout::Layout::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::layout::Layout where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::layout::Layout where T: core::clone::Clone
+pub type ratatui::layout::Layout::Owned = T
+pub type ratatui::layout::Layout::Owned = T
+pub fn ratatui::layout::Layout::clone_into(&self, target: &mut T)
+pub fn ratatui::layout::Layout::clone_into(&self, target: &mut T)
+pub fn ratatui::layout::Layout::to_owned(&self) -> T
+pub fn ratatui::layout::Layout::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::layout::Layout where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::layout::Layout where T: 'static + core::marker::Sized
+pub fn ratatui::layout::Layout::type_id(&self) -> core::any::TypeId
+pub fn ratatui::layout::Layout::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::layout::Layout where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::layout::Layout where T: core::marker::Sized
+pub fn ratatui::layout::Layout::borrow(&self) -> &T
+pub fn ratatui::layout::Layout::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::layout::Layout where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::layout::Layout where T: core::marker::Sized
+pub fn ratatui::layout::Layout::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::layout::Layout::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::layout::Layout
+impl<T> core::convert::From<T> for ratatui::layout::Layout
+pub fn ratatui::layout::Layout::from(t: T) -> T
+pub fn ratatui::layout::Layout::from(t: T) -> T
+impl core::clone::Clone for ratatui::layout::Margin
+impl core::clone::Clone for ratatui::layout::Margin
+pub fn ratatui::layout::Margin::clone(&self) -> ratatui::layout::Margin
+pub fn ratatui::layout::Margin::clone(&self) -> ratatui::layout::Margin
+impl core::cmp::Eq for ratatui::layout::Margin
+impl core::cmp::Eq for ratatui::layout::Margin
+impl core::cmp::PartialEq<ratatui::layout::Margin> for ratatui::layout::Margin
+impl core::cmp::PartialEq<ratatui::layout::Margin> for ratatui::layout::Margin
+pub fn ratatui::layout::Margin::eq(&self, other: &ratatui::layout::Margin) -> bool
+pub fn ratatui::layout::Margin::eq(&self, other: &ratatui::layout::Margin) -> bool
+impl core::fmt::Debug for ratatui::layout::Margin
+impl core::fmt::Debug for ratatui::layout::Margin
+pub fn ratatui::layout::Margin::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::layout::Margin::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::hash::Hash for ratatui::layout::Margin
+impl core::hash::Hash for ratatui::layout::Margin
+pub fn ratatui::layout::Margin::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
+pub fn ratatui::layout::Margin::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
+impl core::marker::StructuralEq for ratatui::layout::Margin
+impl core::marker::StructuralEq for ratatui::layout::Margin
+impl core::marker::StructuralPartialEq for ratatui::layout::Margin
+impl core::marker::StructuralPartialEq for ratatui::layout::Margin
+impl core::marker::Send for ratatui::layout::Margin
+impl core::marker::Send for ratatui::layout::Margin
+impl core::marker::Sync for ratatui::layout::Margin
+impl core::marker::Sync for ratatui::layout::Margin
+impl core::marker::Unpin for ratatui::layout::Margin
+impl core::marker::Unpin for ratatui::layout::Margin
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::layout::Margin
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::layout::Margin
+impl core::panic::unwind_safe::UnwindSafe for ratatui::layout::Margin
+impl core::panic::unwind_safe::UnwindSafe for ratatui::layout::Margin
+impl<T, U> core::convert::Into<U> for ratatui::layout::Margin where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::layout::Margin where U: core::convert::From<T>
+pub fn ratatui::layout::Margin::into(self) -> U
+pub fn ratatui::layout::Margin::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::layout::Margin where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::layout::Margin where U: core::convert::Into<T>
+pub type ratatui::layout::Margin::Error = core::convert::Infallible
+pub type ratatui::layout::Margin::Error = core::convert::Infallible
+pub fn ratatui::layout::Margin::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::layout::Margin::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::layout::Margin where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::layout::Margin where U: core::convert::TryFrom<T>
+pub type ratatui::layout::Margin::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::layout::Margin::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::layout::Margin::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::layout::Margin::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::layout::Margin where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::layout::Margin where T: core::clone::Clone
+pub type ratatui::layout::Margin::Owned = T
+pub type ratatui::layout::Margin::Owned = T
+pub fn ratatui::layout::Margin::clone_into(&self, target: &mut T)
+pub fn ratatui::layout::Margin::clone_into(&self, target: &mut T)
+pub fn ratatui::layout::Margin::to_owned(&self) -> T
+pub fn ratatui::layout::Margin::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::layout::Margin where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::layout::Margin where T: 'static + core::marker::Sized
+pub fn ratatui::layout::Margin::type_id(&self) -> core::any::TypeId
+pub fn ratatui::layout::Margin::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::layout::Margin where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::layout::Margin where T: core::marker::Sized
+pub fn ratatui::layout::Margin::borrow(&self) -> &T
+pub fn ratatui::layout::Margin::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::layout::Margin where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::layout::Margin where T: core::marker::Sized
+pub fn ratatui::layout::Margin::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::layout::Margin::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::layout::Margin
+impl<T> core::convert::From<T> for ratatui::layout::Margin
+pub fn ratatui::layout::Margin::from(t: T) -> T
+pub fn ratatui::layout::Margin::from(t: T) -> T
+impl ratatui::layout::Rect
+impl ratatui::layout::Rect
+pub const fn ratatui::layout::Rect::area(self) -> u16
+pub const fn ratatui::layout::Rect::area(self) -> u16
+pub const fn ratatui::layout::Rect::bottom(self) -> u16
+pub const fn ratatui::layout::Rect::bottom(self) -> u16
+pub fn ratatui::layout::Rect::inner(self, margin: &ratatui::layout::Margin) -> ratatui::layout::Rect
+pub fn ratatui::layout::Rect::inner(self, margin: &ratatui::layout::Margin) -> ratatui::layout::Rect
+pub fn ratatui::layout::Rect::intersection(self, other: ratatui::layout::Rect) -> ratatui::layout::Rect
+pub fn ratatui::layout::Rect::intersection(self, other: ratatui::layout::Rect) -> ratatui::layout::Rect
+pub const fn ratatui::layout::Rect::intersects(self, other: ratatui::layout::Rect) -> bool
+pub const fn ratatui::layout::Rect::intersects(self, other: ratatui::layout::Rect) -> bool
+pub const fn ratatui::layout::Rect::left(self) -> u16
+pub const fn ratatui::layout::Rect::left(self) -> u16
+pub fn ratatui::layout::Rect::new(x: u16, y: u16, width: u16, height: u16) -> ratatui::layout::Rect
+pub fn ratatui::layout::Rect::new(x: u16, y: u16, width: u16, height: u16) -> ratatui::layout::Rect
+pub const fn ratatui::layout::Rect::right(self) -> u16
+pub const fn ratatui::layout::Rect::right(self) -> u16
+pub const fn ratatui::layout::Rect::top(self) -> u16
+pub const fn ratatui::layout::Rect::top(self) -> u16
+pub fn ratatui::layout::Rect::union(self, other: ratatui::layout::Rect) -> ratatui::layout::Rect
+pub fn ratatui::layout::Rect::union(self, other: ratatui::layout::Rect) -> ratatui::layout::Rect
+impl core::clone::Clone for ratatui::layout::Rect
+impl core::clone::Clone for ratatui::layout::Rect
+pub fn ratatui::layout::Rect::clone(&self) -> ratatui::layout::Rect
+pub fn ratatui::layout::Rect::clone(&self) -> ratatui::layout::Rect
+impl core::cmp::Eq for ratatui::layout::Rect
+impl core::cmp::Eq for ratatui::layout::Rect
+impl core::cmp::PartialEq<ratatui::layout::Rect> for ratatui::layout::Rect
+impl core::cmp::PartialEq<ratatui::layout::Rect> for ratatui::layout::Rect
+pub fn ratatui::layout::Rect::eq(&self, other: &ratatui::layout::Rect) -> bool
+pub fn ratatui::layout::Rect::eq(&self, other: &ratatui::layout::Rect) -> bool
+impl core::default::Default for ratatui::layout::Rect
+impl core::default::Default for ratatui::layout::Rect
+pub fn ratatui::layout::Rect::default() -> ratatui::layout::Rect
+pub fn ratatui::layout::Rect::default() -> ratatui::layout::Rect
+impl core::fmt::Debug for ratatui::layout::Rect
+impl core::fmt::Debug for ratatui::layout::Rect
+pub fn ratatui::layout::Rect::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::layout::Rect::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::hash::Hash for ratatui::layout::Rect
+impl core::hash::Hash for ratatui::layout::Rect
+pub fn ratatui::layout::Rect::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
+pub fn ratatui::layout::Rect::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
+impl core::marker::Copy for ratatui::layout::Rect
+impl core::marker::Copy for ratatui::layout::Rect
+impl core::marker::StructuralEq for ratatui::layout::Rect
+impl core::marker::StructuralEq for ratatui::layout::Rect
+impl core::marker::StructuralPartialEq for ratatui::layout::Rect
+impl core::marker::StructuralPartialEq for ratatui::layout::Rect
+impl core::marker::Send for ratatui::layout::Rect
+impl core::marker::Send for ratatui::layout::Rect
+impl core::marker::Sync for ratatui::layout::Rect
+impl core::marker::Sync for ratatui::layout::Rect
+impl core::marker::Unpin for ratatui::layout::Rect
+impl core::marker::Unpin for ratatui::layout::Rect
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::layout::Rect
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::layout::Rect
+impl core::panic::unwind_safe::UnwindSafe for ratatui::layout::Rect
+impl core::panic::unwind_safe::UnwindSafe for ratatui::layout::Rect
+impl<T, U> core::convert::Into<U> for ratatui::layout::Rect where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::layout::Rect where U: core::convert::From<T>
+pub fn ratatui::layout::Rect::into(self) -> U
+pub fn ratatui::layout::Rect::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::layout::Rect where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::layout::Rect where U: core::convert::Into<T>
+pub type ratatui::layout::Rect::Error = core::convert::Infallible
+pub type ratatui::layout::Rect::Error = core::convert::Infallible
+pub fn ratatui::layout::Rect::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::layout::Rect::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::layout::Rect where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::layout::Rect where U: core::convert::TryFrom<T>
+pub type ratatui::layout::Rect::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::layout::Rect::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::layout::Rect::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::layout::Rect::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::layout::Rect where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::layout::Rect where T: core::clone::Clone
+pub type ratatui::layout::Rect::Owned = T
+pub type ratatui::layout::Rect::Owned = T
+pub fn ratatui::layout::Rect::clone_into(&self, target: &mut T)
+pub fn ratatui::layout::Rect::clone_into(&self, target: &mut T)
+pub fn ratatui::layout::Rect::to_owned(&self) -> T
+pub fn ratatui::layout::Rect::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::layout::Rect where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::layout::Rect where T: 'static + core::marker::Sized
+pub fn ratatui::layout::Rect::type_id(&self) -> core::any::TypeId
+pub fn ratatui::layout::Rect::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::layout::Rect where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::layout::Rect where T: core::marker::Sized
+pub fn ratatui::layout::Rect::borrow(&self) -> &T
+pub fn ratatui::layout::Rect::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::layout::Rect where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::layout::Rect where T: core::marker::Sized
+pub fn ratatui::layout::Rect::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::layout::Rect::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::layout::Rect
+impl<T> core::convert::From<T> for ratatui::layout::Rect
+pub fn ratatui::layout::Rect::from(t: T) -> T
+pub fn ratatui::layout::Rect::from(t: T) -> T
+pub mod ratatui::prelude
+pub mod ratatui::prelude::backend
+pub enum ratatui::prelude::backend::ClearType
+pub ratatui::prelude::backend::ClearType::AfterCursor
+pub ratatui::prelude::backend::ClearType::All
+pub ratatui::prelude::backend::ClearType::BeforeCursor
+pub ratatui::prelude::backend::ClearType::CurrentLine
+pub ratatui::prelude::backend::ClearType::UntilNewLine
+pub struct ratatui::prelude::backend::CrosstermBackend<W: std::io::Write>
+pub struct ratatui::prelude::backend::TestBackend
+pub trait ratatui::prelude::backend::Backend
+pub fn ratatui::prelude::backend::Backend::append_lines(&mut self, _n: u16) -> std::io::error::Result<()>
+pub fn ratatui::prelude::backend::Backend::clear(&mut self) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::prelude::backend::Backend::clear_region(&mut self, clear_type: ratatui::backend::ClearType) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::prelude::backend::Backend::draw<'a, I>(&mut self, content: I) -> core::result::Result<(), std::io::error::Error> where I: core::iter::traits::iterator::Iterator<Item = (u16, u16, &'a ratatui::buffer::Cell)>
+pub fn ratatui::prelude::backend::Backend::flush(&mut self) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::prelude::backend::Backend::get_cursor(&mut self) -> core::result::Result<(u16, u16), std::io::error::Error>
+pub fn ratatui::prelude::backend::Backend::hide_cursor(&mut self) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::prelude::backend::Backend::set_cursor(&mut self, x: u16, y: u16) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::prelude::backend::Backend::show_cursor(&mut self) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::prelude::backend::Backend::size(&self) -> core::result::Result<ratatui::layout::Rect, std::io::error::Error>
+pub mod ratatui::prelude::buffer
+pub struct ratatui::prelude::buffer::Buffer
+pub ratatui::prelude::buffer::Buffer::area: ratatui::layout::Rect
+pub ratatui::prelude::buffer::Buffer::content: alloc::vec::Vec<ratatui::buffer::Cell>
+pub struct ratatui::prelude::buffer::Cell
+pub ratatui::prelude::buffer::Cell::bg: ratatui::style::Color
+pub ratatui::prelude::buffer::Cell::fg: ratatui::style::Color
+pub ratatui::prelude::buffer::Cell::modifier: ratatui::style::Modifier
+pub ratatui::prelude::buffer::Cell::symbol: alloc::string::String
+pub ratatui::prelude::buffer::Cell::underline_color: ratatui::style::Color
+pub mod ratatui::prelude::layout
+pub enum ratatui::prelude::layout::Alignment
+pub ratatui::prelude::layout::Alignment::Center
+pub ratatui::prelude::layout::Alignment::Left
+pub ratatui::prelude::layout::Alignment::Right
+pub enum ratatui::prelude::layout::Constraint
+pub ratatui::prelude::layout::Constraint::Length(u16)
+pub ratatui::prelude::layout::Constraint::Max(u16)
+pub ratatui::prelude::layout::Constraint::Min(u16)
+pub ratatui::prelude::layout::Constraint::Percentage(u16)
+pub ratatui::prelude::layout::Constraint::Ratio(u32, u32)
+pub enum ratatui::prelude::layout::Corner
+pub ratatui::prelude::layout::Corner::BottomLeft
+pub ratatui::prelude::layout::Corner::BottomRight
+pub ratatui::prelude::layout::Corner::TopLeft
+pub ratatui::prelude::layout::Corner::TopRight
+pub enum ratatui::prelude::layout::Direction
+pub ratatui::prelude::layout::Direction::Horizontal
+pub ratatui::prelude::layout::Direction::Vertical
+pub struct ratatui::prelude::layout::Layout
+pub struct ratatui::prelude::layout::Margin
+pub ratatui::prelude::layout::Margin::horizontal: u16
+pub ratatui::prelude::layout::Margin::vertical: u16
+pub struct ratatui::prelude::layout::Rect
+pub ratatui::prelude::layout::Rect::height: u16
+pub ratatui::prelude::layout::Rect::width: u16
+pub ratatui::prelude::layout::Rect::x: u16
+pub ratatui::prelude::layout::Rect::y: u16
+pub mod ratatui::prelude::style
+pub enum ratatui::prelude::style::Color
+pub ratatui::prelude::style::Color::Black
+pub ratatui::prelude::style::Color::Blue
+pub ratatui::prelude::style::Color::Cyan
+pub ratatui::prelude::style::Color::DarkGray
+pub ratatui::prelude::style::Color::Gray
+pub ratatui::prelude::style::Color::Green
+pub ratatui::prelude::style::Color::Indexed(u8)
+pub ratatui::prelude::style::Color::LightBlue
+pub ratatui::prelude::style::Color::LightCyan
+pub ratatui::prelude::style::Color::LightGreen
+pub ratatui::prelude::style::Color::LightMagenta
+pub ratatui::prelude::style::Color::LightRed
+pub ratatui::prelude::style::Color::LightYellow
+pub ratatui::prelude::style::Color::Magenta
+pub ratatui::prelude::style::Color::Red
+pub ratatui::prelude::style::Color::Reset
+pub ratatui::prelude::style::Color::Rgb(u8, u8, u8)
+pub ratatui::prelude::style::Color::White
+pub ratatui::prelude::style::Color::Yellow
+impl core::convert::From<ratatui::style::Color> for crossterm::style::types::color::Color
+impl core::convert::From<ratatui::style::Color> for crossterm::style::types::color::Color
+pub fn crossterm::style::types::color::Color::from(color: ratatui::style::Color) -> Self
+pub fn crossterm::style::types::color::Color::from(color: ratatui::style::Color) -> Self
+impl core::str::traits::FromStr for ratatui::style::Color
+impl core::str::traits::FromStr for ratatui::style::Color
+pub type ratatui::style::Color::Err = ratatui::style::ParseColorError
+pub type ratatui::style::Color::Err = ratatui::style::ParseColorError
+pub fn ratatui::style::Color::from_str(s: &str) -> core::result::Result<Self, Self::Err>
+pub fn ratatui::style::Color::from_str(s: &str) -> core::result::Result<Self, Self::Err>
+impl core::clone::Clone for ratatui::style::Color
+impl core::clone::Clone for ratatui::style::Color
+pub fn ratatui::style::Color::clone(&self) -> ratatui::style::Color
+pub fn ratatui::style::Color::clone(&self) -> ratatui::style::Color
+impl core::cmp::Eq for ratatui::style::Color
+impl core::cmp::Eq for ratatui::style::Color
+impl core::cmp::PartialEq<ratatui::style::Color> for ratatui::style::Color
+impl core::cmp::PartialEq<ratatui::style::Color> for ratatui::style::Color
+pub fn ratatui::style::Color::eq(&self, other: &ratatui::style::Color) -> bool
+pub fn ratatui::style::Color::eq(&self, other: &ratatui::style::Color) -> bool
+impl core::fmt::Debug for ratatui::style::Color
+impl core::fmt::Debug for ratatui::style::Color
+pub fn ratatui::style::Color::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::style::Color::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::marker::Copy for ratatui::style::Color
+impl core::marker::Copy for ratatui::style::Color
+impl core::marker::StructuralEq for ratatui::style::Color
+impl core::marker::StructuralEq for ratatui::style::Color
+impl core::marker::StructuralPartialEq for ratatui::style::Color
+impl core::marker::StructuralPartialEq for ratatui::style::Color
+impl core::marker::Send for ratatui::style::Color
+impl core::marker::Send for ratatui::style::Color
+impl core::marker::Sync for ratatui::style::Color
+impl core::marker::Sync for ratatui::style::Color
+impl core::marker::Unpin for ratatui::style::Color
+impl core::marker::Unpin for ratatui::style::Color
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::style::Color
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::style::Color
+impl core::panic::unwind_safe::UnwindSafe for ratatui::style::Color
+impl core::panic::unwind_safe::UnwindSafe for ratatui::style::Color
+impl<T, U> core::convert::Into<U> for ratatui::style::Color where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::style::Color where U: core::convert::From<T>
+pub fn ratatui::style::Color::into(self) -> U
+pub fn ratatui::style::Color::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::style::Color where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::style::Color where U: core::convert::Into<T>
+pub type ratatui::style::Color::Error = core::convert::Infallible
+pub type ratatui::style::Color::Error = core::convert::Infallible
+pub fn ratatui::style::Color::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::style::Color::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::style::Color where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::style::Color where U: core::convert::TryFrom<T>
+pub type ratatui::style::Color::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::style::Color::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::style::Color::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::style::Color::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::style::Color where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::style::Color where T: core::clone::Clone
+pub type ratatui::style::Color::Owned = T
+pub type ratatui::style::Color::Owned = T
+pub fn ratatui::style::Color::clone_into(&self, target: &mut T)
+pub fn ratatui::style::Color::clone_into(&self, target: &mut T)
+pub fn ratatui::style::Color::to_owned(&self) -> T
+pub fn ratatui::style::Color::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::style::Color where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::style::Color where T: 'static + core::marker::Sized
+pub fn ratatui::style::Color::type_id(&self) -> core::any::TypeId
+pub fn ratatui::style::Color::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::style::Color where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::style::Color where T: core::marker::Sized
+pub fn ratatui::style::Color::borrow(&self) -> &T
+pub fn ratatui::style::Color::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::style::Color where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::style::Color where T: core::marker::Sized
+pub fn ratatui::style::Color::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::style::Color::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::style::Color
+impl<T> core::convert::From<T> for ratatui::style::Color
+pub fn ratatui::style::Color::from(t: T) -> T
+pub fn ratatui::style::Color::from(t: T) -> T
+pub struct ratatui::prelude::style::Modifier(_)
+impl ratatui::style::Modifier
+impl ratatui::style::Modifier
+impl ratatui::style::Modifier
+impl ratatui::style::Modifier
+impl ratatui::style::Modifier
+impl ratatui::style::Modifier
+impl ratatui::style::Modifier
+impl ratatui::style::Modifier
+pub const ratatui::style::Modifier::BOLD: Self
+pub const ratatui::style::Modifier::BOLD: Self
+pub const ratatui::style::Modifier::CROSSED_OUT: Self
+pub const ratatui::style::Modifier::CROSSED_OUT: Self
+pub const ratatui::style::Modifier::DIM: Self
+pub const ratatui::style::Modifier::DIM: Self
+pub const ratatui::style::Modifier::HIDDEN: Self
+pub const ratatui::style::Modifier::HIDDEN: Self
+pub const ratatui::style::Modifier::ITALIC: Self
+pub const ratatui::style::Modifier::ITALIC: Self
+pub const ratatui::style::Modifier::RAPID_BLINK: Self
+pub const ratatui::style::Modifier::RAPID_BLINK: Self
+pub const ratatui::style::Modifier::REVERSED: Self
+pub const ratatui::style::Modifier::REVERSED: Self
+pub const ratatui::style::Modifier::SLOW_BLINK: Self
+pub const ratatui::style::Modifier::SLOW_BLINK: Self
+pub const ratatui::style::Modifier::UNDERLINED: Self
+pub const ratatui::style::Modifier::UNDERLINED: Self
+pub const fn ratatui::style::Modifier::all() -> Self
+pub const fn ratatui::style::Modifier::all() -> Self
+pub const fn ratatui::style::Modifier::bits(&self) -> u16
+pub const fn ratatui::style::Modifier::bits(&self) -> u16
+pub const fn ratatui::style::Modifier::complement(self) -> Self
+pub const fn ratatui::style::Modifier::complement(self) -> Self
+pub const fn ratatui::style::Modifier::contains(&self, other: Self) -> bool
+pub const fn ratatui::style::Modifier::contains(&self, other: Self) -> bool
+pub const fn ratatui::style::Modifier::difference(self, other: Self) -> Self
+pub const fn ratatui::style::Modifier::difference(self, other: Self) -> Self
+pub const fn ratatui::style::Modifier::empty() -> Self
+pub const fn ratatui::style::Modifier::empty() -> Self
+pub const fn ratatui::style::Modifier::from_bits(bits: u16) -> core::option::Option<Self>
+pub const fn ratatui::style::Modifier::from_bits(bits: u16) -> core::option::Option<Self>
+pub const fn ratatui::style::Modifier::from_bits_retain(bits: u16) -> Self
+pub const fn ratatui::style::Modifier::from_bits_retain(bits: u16) -> Self
+pub const fn ratatui::style::Modifier::from_bits_retain(bits: u16) -> Self
+pub const fn ratatui::style::Modifier::from_bits_truncate(bits: u16) -> Self
+pub const fn ratatui::style::Modifier::from_bits_truncate(bits: u16) -> Self
+pub fn ratatui::style::Modifier::from_name(name: &str) -> core::option::Option<Self>
+pub fn ratatui::style::Modifier::from_name(name: &str) -> core::option::Option<Self>
+pub fn ratatui::style::Modifier::from_name(name: &str) -> core::option::Option<Self>
+pub fn ratatui::style::Modifier::insert(&mut self, other: Self)
+pub fn ratatui::style::Modifier::insert(&mut self, other: Self)
+pub const fn ratatui::style::Modifier::intersection(self, other: Self) -> Self
+pub const fn ratatui::style::Modifier::intersection(self, other: Self) -> Self
+pub const fn ratatui::style::Modifier::intersects(&self, other: Self) -> bool
+pub const fn ratatui::style::Modifier::intersects(&self, other: Self) -> bool
+pub const fn ratatui::style::Modifier::is_all(&self) -> bool
+pub const fn ratatui::style::Modifier::is_all(&self) -> bool
+pub const fn ratatui::style::Modifier::is_empty(&self) -> bool
+pub const fn ratatui::style::Modifier::is_empty(&self) -> bool
+pub fn ratatui::style::Modifier::remove(&mut self, other: Self)
+pub fn ratatui::style::Modifier::remove(&mut self, other: Self)
+pub fn ratatui::style::Modifier::set(&mut self, other: Self, value: bool)
+pub fn ratatui::style::Modifier::set(&mut self, other: Self, value: bool)
+pub const fn ratatui::style::Modifier::symmetric_difference(self, other: Self) -> Self
+pub const fn ratatui::style::Modifier::symmetric_difference(self, other: Self) -> Self
+pub fn ratatui::style::Modifier::toggle(&mut self, other: Self)
+pub fn ratatui::style::Modifier::toggle(&mut self, other: Self)
+pub const fn ratatui::style::Modifier::union(self, other: Self) -> Self
+pub const fn ratatui::style::Modifier::union(self, other: Self) -> Self
+pub const fn ratatui::style::Modifier::iter(&self) -> bitflags::iter::Iter<ratatui::style::Modifier>
+pub const fn ratatui::style::Modifier::iter(&self) -> bitflags::iter::Iter<ratatui::style::Modifier>
+pub const fn ratatui::style::Modifier::iter(&self) -> bitflags::iter::Iter<ratatui::style::Modifier>
+pub const fn ratatui::style::Modifier::iter_names(&self) -> bitflags::iter::IterNames<ratatui::style::Modifier>
+pub const fn ratatui::style::Modifier::iter_names(&self) -> bitflags::iter::IterNames<ratatui::style::Modifier>
+pub const fn ratatui::style::Modifier::iter_names(&self) -> bitflags::iter::IterNames<ratatui::style::Modifier>
+impl bitflags::traits::Flags for ratatui::style::Modifier
+impl bitflags::traits::Flags for ratatui::style::Modifier
+impl bitflags::traits::Flags for ratatui::style::Modifier
+pub type ratatui::style::Modifier::Bits = u16
+pub type ratatui::style::Modifier::Bits = u16
+pub type ratatui::style::Modifier::Bits = u16
+pub const ratatui::style::Modifier::FLAGS: &'static [bitflags::traits::Flag<ratatui::style::Modifier>]
+pub const ratatui::style::Modifier::FLAGS: &'static [bitflags::traits::Flag<ratatui::style::Modifier>]
+pub const ratatui::style::Modifier::FLAGS: &'static [bitflags::traits::Flag<ratatui::style::Modifier>]
+pub fn ratatui::style::Modifier::bits(&self) -> u16
+pub fn ratatui::style::Modifier::bits(&self) -> u16
+pub fn ratatui::style::Modifier::bits(&self) -> u16
+pub fn ratatui::style::Modifier::from_bits_retain(bits: u16) -> ratatui::style::Modifier
+pub fn ratatui::style::Modifier::from_bits_retain(bits: u16) -> ratatui::style::Modifier
+pub fn ratatui::style::Modifier::from_bits_retain(bits: u16) -> ratatui::style::Modifier
+impl bitflags::traits::PublicFlags for ratatui::style::Modifier
+impl bitflags::traits::PublicFlags for ratatui::style::Modifier
+impl bitflags::traits::PublicFlags for ratatui::style::Modifier
+pub type ratatui::style::Modifier::Internal = InternalBitFlags
+pub type ratatui::style::Modifier::Internal = InternalBitFlags
+pub type ratatui::style::Modifier::Internal = InternalBitFlags
+pub type ratatui::style::Modifier::Primitive = u16
+pub type ratatui::style::Modifier::Primitive = u16
+pub type ratatui::style::Modifier::Primitive = u16
+impl core::fmt::Binary for ratatui::style::Modifier
+impl core::fmt::Binary for ratatui::style::Modifier
+pub fn ratatui::style::Modifier::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::style::Modifier::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::style::Modifier::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::style::Modifier::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::style::Modifier::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::style::Modifier::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::style::Modifier::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::style::Modifier::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::style::Modifier::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::style::Modifier::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::fmt::Debug for ratatui::style::Modifier
+impl core::fmt::Debug for ratatui::style::Modifier
+impl core::fmt::LowerHex for ratatui::style::Modifier
+impl core::fmt::LowerHex for ratatui::style::Modifier
+impl core::fmt::Octal for ratatui::style::Modifier
+impl core::fmt::Octal for ratatui::style::Modifier
+impl core::fmt::UpperHex for ratatui::style::Modifier
+impl core::fmt::UpperHex for ratatui::style::Modifier
+impl core::iter::traits::collect::Extend<ratatui::style::Modifier> for ratatui::style::Modifier
+impl core::iter::traits::collect::Extend<ratatui::style::Modifier> for ratatui::style::Modifier
+pub fn ratatui::style::Modifier::extend<T: core::iter::traits::collect::IntoIterator<Item = Self>>(&mut self, iterator: T)
+pub fn ratatui::style::Modifier::extend<T: core::iter::traits::collect::IntoIterator<Item = Self>>(&mut self, iterator: T)
+impl core::iter::traits::collect::FromIterator<ratatui::style::Modifier> for ratatui::style::Modifier
+impl core::iter::traits::collect::FromIterator<ratatui::style::Modifier> for ratatui::style::Modifier
+pub fn ratatui::style::Modifier::from_iter<T: core::iter::traits::collect::IntoIterator<Item = Self>>(iterator: T) -> Self
+pub fn ratatui::style::Modifier::from_iter<T: core::iter::traits::collect::IntoIterator<Item = Self>>(iterator: T) -> Self
+impl core::iter::traits::collect::IntoIterator for ratatui::style::Modifier
+impl core::iter::traits::collect::IntoIterator for ratatui::style::Modifier
+impl core::iter::traits::collect::IntoIterator for ratatui::style::Modifier
+pub type ratatui::style::Modifier::IntoIter = bitflags::iter::Iter<ratatui::style::Modifier>
+pub type ratatui::style::Modifier::IntoIter = bitflags::iter::Iter<ratatui::style::Modifier>
+pub type ratatui::style::Modifier::IntoIter = bitflags::iter::Iter<ratatui::style::Modifier>
+pub type ratatui::style::Modifier::Item = ratatui::style::Modifier
+pub type ratatui::style::Modifier::Item = ratatui::style::Modifier
+pub type ratatui::style::Modifier::Item = ratatui::style::Modifier
+pub fn ratatui::style::Modifier::into_iter(self) -> Self::IntoIter
+pub fn ratatui::style::Modifier::into_iter(self) -> Self::IntoIter
+pub fn ratatui::style::Modifier::into_iter(self) -> Self::IntoIter
+impl core::ops::arith::Sub<ratatui::style::Modifier> for ratatui::style::Modifier
+impl core::ops::arith::Sub<ratatui::style::Modifier> for ratatui::style::Modifier
+pub type ratatui::style::Modifier::Output = ratatui::style::Modifier
+pub type ratatui::style::Modifier::Output = ratatui::style::Modifier
+pub type ratatui::style::Modifier::Output = ratatui::style::Modifier
+pub type ratatui::style::Modifier::Output = ratatui::style::Modifier
+pub type ratatui::style::Modifier::Output = ratatui::style::Modifier
+pub type ratatui::style::Modifier::Output = ratatui::style::Modifier
+pub type ratatui::style::Modifier::Output = ratatui::style::Modifier
+pub type ratatui::style::Modifier::Output = ratatui::style::Modifier
+pub type ratatui::style::Modifier::Output = ratatui::style::Modifier
+pub type ratatui::style::Modifier::Output = ratatui::style::Modifier
+pub fn ratatui::style::Modifier::sub(self, other: Self) -> Self
+pub fn ratatui::style::Modifier::sub(self, other: Self) -> Self
+impl core::ops::arith::SubAssign<ratatui::style::Modifier> for ratatui::style::Modifier
+impl core::ops::arith::SubAssign<ratatui::style::Modifier> for ratatui::style::Modifier
+pub fn ratatui::style::Modifier::sub_assign(&mut self, other: Self)
+pub fn ratatui::style::Modifier::sub_assign(&mut self, other: Self)
+impl core::ops::bit::BitAnd<ratatui::style::Modifier> for ratatui::style::Modifier
+impl core::ops::bit::BitAnd<ratatui::style::Modifier> for ratatui::style::Modifier
+pub fn ratatui::style::Modifier::bitand(self, other: Self) -> Self
+pub fn ratatui::style::Modifier::bitand(self, other: Self) -> Self
+impl core::ops::bit::BitAndAssign<ratatui::style::Modifier> for ratatui::style::Modifier
+impl core::ops::bit::BitAndAssign<ratatui::style::Modifier> for ratatui::style::Modifier
+pub fn ratatui::style::Modifier::bitand_assign(&mut self, other: Self)
+pub fn ratatui::style::Modifier::bitand_assign(&mut self, other: Self)
+impl core::ops::bit::BitOr<ratatui::style::Modifier> for ratatui::style::Modifier
+impl core::ops::bit::BitOr<ratatui::style::Modifier> for ratatui::style::Modifier
+pub fn ratatui::style::Modifier::bitor(self, other: ratatui::style::Modifier) -> Self
+pub fn ratatui::style::Modifier::bitor(self, other: ratatui::style::Modifier) -> Self
+impl core::ops::bit::BitOrAssign<ratatui::style::Modifier> for ratatui::style::Modifier
+impl core::ops::bit::BitOrAssign<ratatui::style::Modifier> for ratatui::style::Modifier
+pub fn ratatui::style::Modifier::bitor_assign(&mut self, other: Self)
+pub fn ratatui::style::Modifier::bitor_assign(&mut self, other: Self)
+impl core::ops::bit::BitXor<ratatui::style::Modifier> for ratatui::style::Modifier
+impl core::ops::bit::BitXor<ratatui::style::Modifier> for ratatui::style::Modifier
+pub fn ratatui::style::Modifier::bitxor(self, other: Self) -> Self
+pub fn ratatui::style::Modifier::bitxor(self, other: Self) -> Self
+impl core::ops::bit::BitXorAssign<ratatui::style::Modifier> for ratatui::style::Modifier
+impl core::ops::bit::BitXorAssign<ratatui::style::Modifier> for ratatui::style::Modifier
+pub fn ratatui::style::Modifier::bitxor_assign(&mut self, other: Self)
+pub fn ratatui::style::Modifier::bitxor_assign(&mut self, other: Self)
+impl core::ops::bit::Not for ratatui::style::Modifier
+impl core::ops::bit::Not for ratatui::style::Modifier
+pub fn ratatui::style::Modifier::not(self) -> Self
+pub fn ratatui::style::Modifier::not(self) -> Self
+impl core::clone::Clone for ratatui::style::Modifier
+impl core::clone::Clone for ratatui::style::Modifier
+pub fn ratatui::style::Modifier::clone(&self) -> ratatui::style::Modifier
+pub fn ratatui::style::Modifier::clone(&self) -> ratatui::style::Modifier
+impl core::cmp::Eq for ratatui::style::Modifier
+impl core::cmp::Eq for ratatui::style::Modifier
+impl core::cmp::PartialEq<ratatui::style::Modifier> for ratatui::style::Modifier
+impl core::cmp::PartialEq<ratatui::style::Modifier> for ratatui::style::Modifier
+pub fn ratatui::style::Modifier::eq(&self, other: &ratatui::style::Modifier) -> bool
+pub fn ratatui::style::Modifier::eq(&self, other: &ratatui::style::Modifier) -> bool
+impl core::marker::Copy for ratatui::style::Modifier
+impl core::marker::Copy for ratatui::style::Modifier
+impl core::marker::StructuralEq for ratatui::style::Modifier
+impl core::marker::StructuralEq for ratatui::style::Modifier
+impl core::marker::StructuralPartialEq for ratatui::style::Modifier
+impl core::marker::StructuralPartialEq for ratatui::style::Modifier
+impl core::marker::Send for ratatui::style::Modifier
+impl core::marker::Send for ratatui::style::Modifier
+impl core::marker::Sync for ratatui::style::Modifier
+impl core::marker::Sync for ratatui::style::Modifier
+impl core::marker::Unpin for ratatui::style::Modifier
+impl core::marker::Unpin for ratatui::style::Modifier
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::style::Modifier
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::style::Modifier
+impl core::panic::unwind_safe::UnwindSafe for ratatui::style::Modifier
+impl core::panic::unwind_safe::UnwindSafe for ratatui::style::Modifier
+impl<B> bitflags::traits::BitFlags for ratatui::style::Modifier where B: bitflags::traits::Flags
+impl<B> bitflags::traits::BitFlags for ratatui::style::Modifier where B: bitflags::traits::Flags
+impl<B> bitflags::traits::BitFlags for ratatui::style::Modifier where B: bitflags::traits::Flags
+pub type ratatui::style::Modifier::Iter = bitflags::iter::Iter<B>
+pub type ratatui::style::Modifier::Iter = bitflags::iter::Iter<B>
+pub type ratatui::style::Modifier::Iter = bitflags::iter::Iter<B>
+pub type ratatui::style::Modifier::IterNames = bitflags::iter::IterNames<B>
+pub type ratatui::style::Modifier::IterNames = bitflags::iter::IterNames<B>
+pub type ratatui::style::Modifier::IterNames = bitflags::iter::IterNames<B>
+impl<T, U> core::convert::Into<U> for ratatui::style::Modifier where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::style::Modifier where U: core::convert::From<T>
+pub fn ratatui::style::Modifier::into(self) -> U
+pub fn ratatui::style::Modifier::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::style::Modifier where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::style::Modifier where U: core::convert::Into<T>
+pub type ratatui::style::Modifier::Error = core::convert::Infallible
+pub type ratatui::style::Modifier::Error = core::convert::Infallible
+pub fn ratatui::style::Modifier::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::style::Modifier::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::style::Modifier where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::style::Modifier where U: core::convert::TryFrom<T>
+pub type ratatui::style::Modifier::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::style::Modifier::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::style::Modifier::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::style::Modifier::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::style::Modifier where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::style::Modifier where T: core::clone::Clone
+pub type ratatui::style::Modifier::Owned = T
+pub type ratatui::style::Modifier::Owned = T
+pub fn ratatui::style::Modifier::clone_into(&self, target: &mut T)
+pub fn ratatui::style::Modifier::clone_into(&self, target: &mut T)
+pub fn ratatui::style::Modifier::to_owned(&self) -> T
+pub fn ratatui::style::Modifier::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::style::Modifier where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::style::Modifier where T: 'static + core::marker::Sized
+pub fn ratatui::style::Modifier::type_id(&self) -> core::any::TypeId
+pub fn ratatui::style::Modifier::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::style::Modifier where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::style::Modifier where T: core::marker::Sized
+pub fn ratatui::style::Modifier::borrow(&self) -> &T
+pub fn ratatui::style::Modifier::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::style::Modifier where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::style::Modifier where T: core::marker::Sized
+pub fn ratatui::style::Modifier::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::style::Modifier::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::style::Modifier
+impl<T> core::convert::From<T> for ratatui::style::Modifier
+pub fn ratatui::style::Modifier::from(t: T) -> T
+pub fn ratatui::style::Modifier::from(t: T) -> T
+pub struct ratatui::prelude::style::ParseColorError
+impl core::error::Error for ratatui::style::ParseColorError
+impl core::fmt::Display for ratatui::style::ParseColorError
+pub fn ratatui::style::ParseColorError::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::style::ParseColorError::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::clone::Clone for ratatui::style::ParseColorError
+impl core::clone::Clone for ratatui::style::ParseColorError
+pub fn ratatui::style::ParseColorError::clone(&self) -> ratatui::style::ParseColorError
+pub fn ratatui::style::ParseColorError::clone(&self) -> ratatui::style::ParseColorError
+impl core::cmp::Eq for ratatui::style::ParseColorError
+impl core::cmp::Eq for ratatui::style::ParseColorError
+impl core::cmp::PartialEq<ratatui::style::ParseColorError> for ratatui::style::ParseColorError
+impl core::cmp::PartialEq<ratatui::style::ParseColorError> for ratatui::style::ParseColorError
+pub fn ratatui::style::ParseColorError::eq(&self, other: &ratatui::style::ParseColorError) -> bool
+pub fn ratatui::style::ParseColorError::eq(&self, other: &ratatui::style::ParseColorError) -> bool
+impl core::fmt::Debug for ratatui::style::ParseColorError
+impl core::marker::Copy for ratatui::style::ParseColorError
+impl core::marker::Copy for ratatui::style::ParseColorError
+impl core::marker::StructuralEq for ratatui::style::ParseColorError
+impl core::marker::StructuralEq for ratatui::style::ParseColorError
+impl core::marker::StructuralPartialEq for ratatui::style::ParseColorError
+impl core::marker::StructuralPartialEq for ratatui::style::ParseColorError
+impl core::marker::Send for ratatui::style::ParseColorError
+impl core::marker::Sync for ratatui::style::ParseColorError
+impl core::marker::Unpin for ratatui::style::ParseColorError
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::style::ParseColorError
+impl core::panic::unwind_safe::UnwindSafe for ratatui::style::ParseColorError
+impl<E> core::any::Provider for ratatui::style::ParseColorError where E: core::error::Error + core::marker::Sized
+pub fn ratatui::style::ParseColorError::provide<'a>(&'a self, demand: &mut core::any::Demand<'a>)
+impl<T, U> core::convert::Into<U> for ratatui::style::ParseColorError where U: core::convert::From<T>
+pub fn ratatui::style::ParseColorError::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::style::ParseColorError where U: core::convert::Into<T>
+pub type ratatui::style::ParseColorError::Error = core::convert::Infallible
+pub fn ratatui::style::ParseColorError::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::style::ParseColorError where U: core::convert::TryFrom<T>
+pub type ratatui::style::ParseColorError::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::style::ParseColorError::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::style::ParseColorError where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::style::ParseColorError where T: core::clone::Clone
+pub type ratatui::style::ParseColorError::Owned = T
+pub type ratatui::style::ParseColorError::Owned = T
+pub fn ratatui::style::ParseColorError::clone_into(&self, target: &mut T)
+pub fn ratatui::style::ParseColorError::clone_into(&self, target: &mut T)
+pub fn ratatui::style::ParseColorError::to_owned(&self) -> T
+pub fn ratatui::style::ParseColorError::to_owned(&self) -> T
+impl<T> alloc::string::ToString for ratatui::style::ParseColorError where T: core::fmt::Display + core::marker::Sized
+pub fn ratatui::style::ParseColorError::to_string(&self) -> alloc::string::String
+impl<T> core::any::Any for ratatui::style::ParseColorError where T: 'static + core::marker::Sized
+pub fn ratatui::style::ParseColorError::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::style::ParseColorError where T: core::marker::Sized
+pub fn ratatui::style::ParseColorError::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::style::ParseColorError where T: core::marker::Sized
+pub fn ratatui::style::ParseColorError::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::style::ParseColorError
+pub fn ratatui::style::ParseColorError::from(t: T) -> T
+pub struct ratatui::prelude::style::Style
+pub ratatui::prelude::style::Style::add_modifier: ratatui::style::Modifier
+pub ratatui::prelude::style::Style::bg: core::option::Option<ratatui::style::Color>
+pub ratatui::prelude::style::Style::fg: core::option::Option<ratatui::style::Color>
+pub ratatui::prelude::style::Style::sub_modifier: ratatui::style::Modifier
+pub ratatui::prelude::style::Style::underline_color: core::option::Option<ratatui::style::Color>
+impl ratatui::style::Style
+impl ratatui::style::Style
+pub const fn ratatui::style::Style::add_modifier(self, modifier: ratatui::style::Modifier) -> ratatui::style::Style
+pub const fn ratatui::style::Style::add_modifier(self, modifier: ratatui::style::Modifier) -> ratatui::style::Style
+pub const fn ratatui::style::Style::add_modifier(self, modifier: ratatui::style::Modifier) -> ratatui::style::Style
+pub const fn ratatui::style::Style::bg(self, color: ratatui::style::Color) -> ratatui::style::Style
+pub const fn ratatui::style::Style::bg(self, color: ratatui::style::Color) -> ratatui::style::Style
+pub const fn ratatui::style::Style::fg(self, color: ratatui::style::Color) -> ratatui::style::Style
+pub const fn ratatui::style::Style::fg(self, color: ratatui::style::Color) -> ratatui::style::Style
+pub const fn ratatui::style::Style::new() -> ratatui::style::Style
+pub const fn ratatui::style::Style::new() -> ratatui::style::Style
+pub fn ratatui::style::Style::patch(self, other: ratatui::style::Style) -> ratatui::style::Style
+pub fn ratatui::style::Style::patch(self, other: ratatui::style::Style) -> ratatui::style::Style
+pub const fn ratatui::style::Style::remove_modifier(self, modifier: ratatui::style::Modifier) -> ratatui::style::Style
+pub const fn ratatui::style::Style::remove_modifier(self, modifier: ratatui::style::Modifier) -> ratatui::style::Style
+pub const fn ratatui::style::Style::remove_modifier(self, modifier: ratatui::style::Modifier) -> ratatui::style::Style
+pub const fn ratatui::style::Style::reset() -> ratatui::style::Style
+pub const fn ratatui::style::Style::reset() -> ratatui::style::Style
+pub const fn ratatui::style::Style::underline_color(self, color: ratatui::style::Color) -> ratatui::style::Style
+pub const fn ratatui::style::Style::underline_color(self, color: ratatui::style::Color) -> ratatui::style::Style
+pub const fn ratatui::style::Style::underline_color(self, color: ratatui::style::Color) -> ratatui::style::Style
+impl core::default::Default for ratatui::style::Style
+impl core::default::Default for ratatui::style::Style
+pub fn ratatui::style::Style::default() -> ratatui::style::Style
+pub fn ratatui::style::Style::default() -> ratatui::style::Style
+impl ratatui::style::Styled for ratatui::style::Style
+impl ratatui::style::Styled for ratatui::style::Style
+impl ratatui::style::Styled for ratatui::style::Style
+impl ratatui::style::Styled for ratatui::style::Style
+impl ratatui::style::Styled for ratatui::style::Style
+impl ratatui::style::Styled for ratatui::style::Style
+pub type ratatui::style::Style::Item = ratatui::style::Style
+pub type ratatui::style::Style::Item = ratatui::style::Style
+pub type ratatui::style::Style::Item = ratatui::style::Style
+pub type ratatui::style::Style::Item = ratatui::style::Style
+pub type ratatui::style::Style::Item = ratatui::style::Style
+pub type ratatui::style::Style::Item = ratatui::style::Style
+pub fn ratatui::style::Style::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::style::Style::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::style::Style::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::style::Style::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::style::Style::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::style::Style::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::style::Style::style(&self) -> ratatui::style::Style
+pub fn ratatui::style::Style::style(&self) -> ratatui::style::Style
+pub fn ratatui::style::Style::style(&self) -> ratatui::style::Style
+pub fn ratatui::style::Style::style(&self) -> ratatui::style::Style
+pub fn ratatui::style::Style::style(&self) -> ratatui::style::Style
+pub fn ratatui::style::Style::style(&self) -> ratatui::style::Style
+impl core::clone::Clone for ratatui::style::Style
+impl core::clone::Clone for ratatui::style::Style
+pub fn ratatui::style::Style::clone(&self) -> ratatui::style::Style
+pub fn ratatui::style::Style::clone(&self) -> ratatui::style::Style
+impl core::cmp::Eq for ratatui::style::Style
+impl core::cmp::Eq for ratatui::style::Style
+impl core::cmp::PartialEq<ratatui::style::Style> for ratatui::style::Style
+impl core::cmp::PartialEq<ratatui::style::Style> for ratatui::style::Style
+pub fn ratatui::style::Style::eq(&self, other: &ratatui::style::Style) -> bool
+pub fn ratatui::style::Style::eq(&self, other: &ratatui::style::Style) -> bool
+impl core::fmt::Debug for ratatui::style::Style
+impl core::fmt::Debug for ratatui::style::Style
+pub fn ratatui::style::Style::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::style::Style::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::marker::Copy for ratatui::style::Style
+impl core::marker::Copy for ratatui::style::Style
+impl core::marker::StructuralEq for ratatui::style::Style
+impl core::marker::StructuralEq for ratatui::style::Style
+impl core::marker::StructuralPartialEq for ratatui::style::Style
+impl core::marker::StructuralPartialEq for ratatui::style::Style
+impl core::marker::Send for ratatui::style::Style
+impl core::marker::Send for ratatui::style::Style
+impl core::marker::Sync for ratatui::style::Style
+impl core::marker::Sync for ratatui::style::Style
+impl core::marker::Unpin for ratatui::style::Style
+impl core::marker::Unpin for ratatui::style::Style
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::style::Style
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::style::Style
+impl core::panic::unwind_safe::UnwindSafe for ratatui::style::Style
+impl core::panic::unwind_safe::UnwindSafe for ratatui::style::Style
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for ratatui::style::Style where U: ratatui::style::Styled<Item = T>
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for ratatui::style::Style where U: ratatui::style::Styled<Item = T>
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for ratatui::style::Style where U: ratatui::style::Styled<Item = T>
+pub fn ratatui::style::Style::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::style::Style::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::style::Style::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::style::Style::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::style::Style::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::style::Style::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::style::Style::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn ratatui::style::Style::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn ratatui::style::Style::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn ratatui::style::Style::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::style::Style::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::style::Style::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::style::Style::reset(self) -> T
+pub fn ratatui::style::Style::reset(self) -> T
+pub fn ratatui::style::Style::reset(self) -> T
+impl<T, U> core::convert::Into<U> for ratatui::style::Style where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::style::Style where U: core::convert::From<T>
+pub fn ratatui::style::Style::into(self) -> U
+pub fn ratatui::style::Style::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::style::Style where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::style::Style where U: core::convert::Into<T>
+pub type ratatui::style::Style::Error = core::convert::Infallible
+pub type ratatui::style::Style::Error = core::convert::Infallible
+pub fn ratatui::style::Style::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::style::Style::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::style::Style where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::style::Style where U: core::convert::TryFrom<T>
+pub type ratatui::style::Style::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::style::Style::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::style::Style::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::style::Style::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::style::Style where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::style::Style where T: core::clone::Clone
+pub type ratatui::style::Style::Owned = T
+pub type ratatui::style::Style::Owned = T
+pub fn ratatui::style::Style::clone_into(&self, target: &mut T)
+pub fn ratatui::style::Style::clone_into(&self, target: &mut T)
+pub fn ratatui::style::Style::to_owned(&self) -> T
+pub fn ratatui::style::Style::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::style::Style where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::style::Style where T: 'static + core::marker::Sized
+pub fn ratatui::style::Style::type_id(&self) -> core::any::TypeId
+pub fn ratatui::style::Style::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::style::Style where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::style::Style where T: core::marker::Sized
+pub fn ratatui::style::Style::borrow(&self) -> &T
+pub fn ratatui::style::Style::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::style::Style where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::style::Style where T: core::marker::Sized
+pub fn ratatui::style::Style::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::style::Style::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::style::Style
+impl<T> core::convert::From<T> for ratatui::style::Style
+pub fn ratatui::style::Style::from(t: T) -> T
+pub fn ratatui::style::Style::from(t: T) -> T
+pub trait ratatui::prelude::style::Styled
+pub type ratatui::prelude::style::Styled::Item
+pub fn ratatui::prelude::style::Styled::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::prelude::style::Styled::style(&self) -> ratatui::style::Style
+impl<'a> ratatui::style::Styled for &'a str
+impl<'a> ratatui::style::Styled for &'a str
+impl<'a> ratatui::style::Styled for &'a str
+pub type &'a str::Item = ratatui::text::Span<'a>
+pub type &'a str::Item = ratatui::text::Span<'a>
+pub type &'a str::Item = ratatui::text::Span<'a>
+pub fn &'a str::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn &'a str::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn &'a str::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn &'a str::style(&self) -> ratatui::style::Style
+pub fn &'a str::style(&self) -> ratatui::style::Style
+pub fn &'a str::style(&self) -> ratatui::style::Style
+impl<'a> ratatui::style::Styled for ratatui::text::Span<'a>
+impl<'a> ratatui::style::Styled for ratatui::text::Span<'a>
+impl<'a> ratatui::style::Styled for ratatui::text::Span<'a>
+impl<'a> ratatui::style::Styled for ratatui::text::Span<'a>
+impl<'a> ratatui::style::Styled for ratatui::text::Span<'a>
+impl<'a> ratatui::style::Styled for ratatui::text::Span<'a>
+pub type ratatui::text::Span<'a>::Item = ratatui::text::Span<'a>
+pub type ratatui::text::Span<'a>::Item = ratatui::text::Span<'a>
+pub type ratatui::text::Span<'a>::Item = ratatui::text::Span<'a>
+pub type ratatui::text::Span<'a>::Item = ratatui::text::Span<'a>
+pub type ratatui::text::Span<'a>::Item = ratatui::text::Span<'a>
+pub type ratatui::text::Span<'a>::Item = ratatui::text::Span<'a>
+pub fn ratatui::text::Span<'a>::set_style(self, style: ratatui::style::Style) -> Self
+pub fn ratatui::text::Span<'a>::set_style(self, style: ratatui::style::Style) -> Self
+pub fn ratatui::text::Span<'a>::set_style(self, style: ratatui::style::Style) -> Self
+pub fn ratatui::text::Span<'a>::set_style(self, style: ratatui::style::Style) -> Self
+pub fn ratatui::text::Span<'a>::set_style(self, style: ratatui::style::Style) -> Self
+pub fn ratatui::text::Span<'a>::set_style(self, style: ratatui::style::Style) -> Self
+pub fn ratatui::text::Span<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::text::Span<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::text::Span<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::text::Span<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::text::Span<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::text::Span<'a>::style(&self) -> ratatui::style::Style
+impl<'a> ratatui::style::Styled for ratatui::text::StyledGrapheme<'a>
+impl<'a> ratatui::style::Styled for ratatui::text::StyledGrapheme<'a>
+impl<'a> ratatui::style::Styled for ratatui::text::StyledGrapheme<'a>
+impl<'a> ratatui::style::Styled for ratatui::text::StyledGrapheme<'a>
+impl<'a> ratatui::style::Styled for ratatui::text::StyledGrapheme<'a>
+pub type ratatui::text::StyledGrapheme<'a>::Item = ratatui::text::StyledGrapheme<'a>
+pub type ratatui::text::StyledGrapheme<'a>::Item = ratatui::text::StyledGrapheme<'a>
+pub type ratatui::text::StyledGrapheme<'a>::Item = ratatui::text::StyledGrapheme<'a>
+pub type ratatui::text::StyledGrapheme<'a>::Item = ratatui::text::StyledGrapheme<'a>
+pub type ratatui::text::StyledGrapheme<'a>::Item = ratatui::text::StyledGrapheme<'a>
+pub fn ratatui::text::StyledGrapheme<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::text::StyledGrapheme<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::text::StyledGrapheme<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::text::StyledGrapheme<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::text::StyledGrapheme<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::text::StyledGrapheme<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::text::StyledGrapheme<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::text::StyledGrapheme<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::text::StyledGrapheme<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::text::StyledGrapheme<'a>::style(&self) -> ratatui::style::Style
+impl<'a> ratatui::style::Styled for ratatui::widgets::Axis<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Axis<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Axis<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Axis<'a>
+pub type ratatui::widgets::Axis<'a>::Item = ratatui::widgets::Axis<'a>
+pub type ratatui::widgets::Axis<'a>::Item = ratatui::widgets::Axis<'a>
+pub type ratatui::widgets::Axis<'a>::Item = ratatui::widgets::Axis<'a>
+pub type ratatui::widgets::Axis<'a>::Item = ratatui::widgets::Axis<'a>
+pub fn ratatui::widgets::Axis<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Axis<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Axis<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Axis<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Axis<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Axis<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Axis<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Axis<'a>::style(&self) -> ratatui::style::Style
+impl<'a> ratatui::style::Styled for ratatui::widgets::BarChart<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::BarChart<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::BarChart<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::BarChart<'a>
+pub type ratatui::widgets::BarChart<'a>::Item = ratatui::widgets::BarChart<'a>
+pub type ratatui::widgets::BarChart<'a>::Item = ratatui::widgets::BarChart<'a>
+pub type ratatui::widgets::BarChart<'a>::Item = ratatui::widgets::BarChart<'a>
+pub type ratatui::widgets::BarChart<'a>::Item = ratatui::widgets::BarChart<'a>
+pub fn ratatui::widgets::BarChart<'a>::set_style(self, style: ratatui::style::Style) -> Self
+pub fn ratatui::widgets::BarChart<'a>::set_style(self, style: ratatui::style::Style) -> Self
+pub fn ratatui::widgets::BarChart<'a>::set_style(self, style: ratatui::style::Style) -> Self
+pub fn ratatui::widgets::BarChart<'a>::set_style(self, style: ratatui::style::Style) -> Self
+pub fn ratatui::widgets::BarChart<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::BarChart<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::BarChart<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::BarChart<'a>::style(&self) -> ratatui::style::Style
+impl<'a> ratatui::style::Styled for ratatui::widgets::Cell<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Cell<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Cell<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Cell<'a>
+pub type ratatui::widgets::Cell<'a>::Item = ratatui::widgets::Cell<'a>
+pub type ratatui::widgets::Cell<'a>::Item = ratatui::widgets::Cell<'a>
+pub type ratatui::widgets::Cell<'a>::Item = ratatui::widgets::Cell<'a>
+pub type ratatui::widgets::Cell<'a>::Item = ratatui::widgets::Cell<'a>
+pub fn ratatui::widgets::Cell<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Cell<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Cell<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Cell<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Cell<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Cell<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Cell<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Cell<'a>::style(&self) -> ratatui::style::Style
+impl<'a> ratatui::style::Styled for ratatui::widgets::Chart<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Chart<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Chart<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Chart<'a>
+pub type ratatui::widgets::Chart<'a>::Item = ratatui::widgets::Chart<'a>
+pub type ratatui::widgets::Chart<'a>::Item = ratatui::widgets::Chart<'a>
+pub type ratatui::widgets::Chart<'a>::Item = ratatui::widgets::Chart<'a>
+pub type ratatui::widgets::Chart<'a>::Item = ratatui::widgets::Chart<'a>
+pub fn ratatui::widgets::Chart<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Chart<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Chart<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Chart<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Chart<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Chart<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Chart<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Chart<'a>::style(&self) -> ratatui::style::Style
+impl<'a> ratatui::style::Styled for ratatui::widgets::Dataset<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Dataset<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Dataset<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Dataset<'a>
+pub type ratatui::widgets::Dataset<'a>::Item = ratatui::widgets::Dataset<'a>
+pub type ratatui::widgets::Dataset<'a>::Item = ratatui::widgets::Dataset<'a>
+pub type ratatui::widgets::Dataset<'a>::Item = ratatui::widgets::Dataset<'a>
+pub type ratatui::widgets::Dataset<'a>::Item = ratatui::widgets::Dataset<'a>
+pub fn ratatui::widgets::Dataset<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Dataset<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Dataset<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Dataset<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Dataset<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Dataset<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Dataset<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Dataset<'a>::style(&self) -> ratatui::style::Style
+impl<'a> ratatui::style::Styled for ratatui::widgets::Gauge<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Gauge<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Gauge<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Gauge<'a>
+pub type ratatui::widgets::Gauge<'a>::Item = ratatui::widgets::Gauge<'a>
+pub type ratatui::widgets::Gauge<'a>::Item = ratatui::widgets::Gauge<'a>
+pub type ratatui::widgets::Gauge<'a>::Item = ratatui::widgets::Gauge<'a>
+pub type ratatui::widgets::Gauge<'a>::Item = ratatui::widgets::Gauge<'a>
+pub fn ratatui::widgets::Gauge<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Gauge<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Gauge<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Gauge<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Gauge<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Gauge<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Gauge<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Gauge<'a>::style(&self) -> ratatui::style::Style
+impl<'a> ratatui::style::Styled for ratatui::widgets::LineGauge<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::LineGauge<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::LineGauge<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::LineGauge<'a>
+pub type ratatui::widgets::LineGauge<'a>::Item = ratatui::widgets::LineGauge<'a>
+pub type ratatui::widgets::LineGauge<'a>::Item = ratatui::widgets::LineGauge<'a>
+pub type ratatui::widgets::LineGauge<'a>::Item = ratatui::widgets::LineGauge<'a>
+pub type ratatui::widgets::LineGauge<'a>::Item = ratatui::widgets::LineGauge<'a>
+pub fn ratatui::widgets::LineGauge<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::LineGauge<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::LineGauge<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::LineGauge<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::LineGauge<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::LineGauge<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::LineGauge<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::LineGauge<'a>::style(&self) -> ratatui::style::Style
+impl<'a> ratatui::style::Styled for ratatui::widgets::List<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::List<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::List<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::List<'a>
+pub type ratatui::widgets::List<'a>::Item = ratatui::widgets::List<'a>
+pub type ratatui::widgets::List<'a>::Item = ratatui::widgets::List<'a>
+pub type ratatui::widgets::List<'a>::Item = ratatui::widgets::List<'a>
+pub type ratatui::widgets::List<'a>::Item = ratatui::widgets::List<'a>
+pub fn ratatui::widgets::List<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::List<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::List<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::List<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::List<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::List<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::List<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::List<'a>::style(&self) -> ratatui::style::Style
+impl<'a> ratatui::style::Styled for ratatui::widgets::ListItem<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::ListItem<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::ListItem<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::ListItem<'a>
+pub type ratatui::widgets::ListItem<'a>::Item = ratatui::widgets::ListItem<'a>
+pub type ratatui::widgets::ListItem<'a>::Item = ratatui::widgets::ListItem<'a>
+pub type ratatui::widgets::ListItem<'a>::Item = ratatui::widgets::ListItem<'a>
+pub type ratatui::widgets::ListItem<'a>::Item = ratatui::widgets::ListItem<'a>
+pub fn ratatui::widgets::ListItem<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::ListItem<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::ListItem<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::ListItem<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::ListItem<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::ListItem<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::ListItem<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::ListItem<'a>::style(&self) -> ratatui::style::Style
+impl<'a> ratatui::style::Styled for ratatui::widgets::Paragraph<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Paragraph<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Paragraph<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Paragraph<'a>
+pub type ratatui::widgets::Paragraph<'a>::Item = ratatui::widgets::Paragraph<'a>
+pub type ratatui::widgets::Paragraph<'a>::Item = ratatui::widgets::Paragraph<'a>
+pub type ratatui::widgets::Paragraph<'a>::Item = ratatui::widgets::Paragraph<'a>
+pub type ratatui::widgets::Paragraph<'a>::Item = ratatui::widgets::Paragraph<'a>
+pub fn ratatui::widgets::Paragraph<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Paragraph<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Paragraph<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Paragraph<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Paragraph<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Paragraph<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Paragraph<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Paragraph<'a>::style(&self) -> ratatui::style::Style
+impl<'a> ratatui::style::Styled for ratatui::widgets::Row<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Row<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Row<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Row<'a>
+pub type ratatui::widgets::Row<'a>::Item = ratatui::widgets::Row<'a>
+pub type ratatui::widgets::Row<'a>::Item = ratatui::widgets::Row<'a>
+pub type ratatui::widgets::Row<'a>::Item = ratatui::widgets::Row<'a>
+pub type ratatui::widgets::Row<'a>::Item = ratatui::widgets::Row<'a>
+pub fn ratatui::widgets::Row<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Row<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Row<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Row<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Row<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Row<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Row<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Row<'a>::style(&self) -> ratatui::style::Style
+impl<'a> ratatui::style::Styled for ratatui::widgets::Sparkline<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Sparkline<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Sparkline<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Sparkline<'a>
+pub type ratatui::widgets::Sparkline<'a>::Item = ratatui::widgets::Sparkline<'a>
+pub type ratatui::widgets::Sparkline<'a>::Item = ratatui::widgets::Sparkline<'a>
+pub type ratatui::widgets::Sparkline<'a>::Item = ratatui::widgets::Sparkline<'a>
+pub type ratatui::widgets::Sparkline<'a>::Item = ratatui::widgets::Sparkline<'a>
+pub fn ratatui::widgets::Sparkline<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Sparkline<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Sparkline<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Sparkline<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Sparkline<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Sparkline<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Sparkline<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Sparkline<'a>::style(&self) -> ratatui::style::Style
+impl<'a> ratatui::style::Styled for ratatui::widgets::Table<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Table<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Table<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Table<'a>
+pub type ratatui::widgets::Table<'a>::Item = ratatui::widgets::Table<'a>
+pub type ratatui::widgets::Table<'a>::Item = ratatui::widgets::Table<'a>
+pub type ratatui::widgets::Table<'a>::Item = ratatui::widgets::Table<'a>
+pub type ratatui::widgets::Table<'a>::Item = ratatui::widgets::Table<'a>
+pub fn ratatui::widgets::Table<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Table<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Table<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Table<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Table<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Table<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Table<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Table<'a>::style(&self) -> ratatui::style::Style
+impl<'a> ratatui::style::Styled for ratatui::widgets::Tabs<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Tabs<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Tabs<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::Tabs<'a>
+pub type ratatui::widgets::Tabs<'a>::Item = ratatui::widgets::Tabs<'a>
+pub type ratatui::widgets::Tabs<'a>::Item = ratatui::widgets::Tabs<'a>
+pub type ratatui::widgets::Tabs<'a>::Item = ratatui::widgets::Tabs<'a>
+pub type ratatui::widgets::Tabs<'a>::Item = ratatui::widgets::Tabs<'a>
+pub fn ratatui::widgets::Tabs<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Tabs<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Tabs<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Tabs<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::Tabs<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Tabs<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Tabs<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::Tabs<'a>::style(&self) -> ratatui::style::Style
+impl<'a> ratatui::style::Styled for ratatui::widgets::block::Block<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::block::Block<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::block::Block<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::block::Block<'a>
+impl<'a> ratatui::style::Styled for ratatui::widgets::block::Block<'a>
+pub type ratatui::widgets::block::Block<'a>::Item = ratatui::widgets::block::Block<'a>
+pub type ratatui::widgets::block::Block<'a>::Item = ratatui::widgets::block::Block<'a>
+pub type ratatui::widgets::block::Block<'a>::Item = ratatui::widgets::block::Block<'a>
+pub type ratatui::widgets::block::Block<'a>::Item = ratatui::widgets::block::Block<'a>
+pub type ratatui::widgets::block::Block<'a>::Item = ratatui::widgets::block::Block<'a>
+pub fn ratatui::widgets::block::Block<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::block::Block<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::block::Block<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::block::Block<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::block::Block<'a>::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::widgets::block::Block<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::block::Block<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::block::Block<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::block::Block<'a>::style(&self) -> ratatui::style::Style
+pub fn ratatui::widgets::block::Block<'a>::style(&self) -> ratatui::style::Style
+pub trait ratatui::prelude::style::Stylize<'a, T>: core::marker::Sized
+pub fn ratatui::prelude::style::Stylize::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::prelude::style::Stylize::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::prelude::style::Stylize::black(self) -> T
+pub fn ratatui::prelude::style::Stylize::blue(self) -> T
+pub fn ratatui::prelude::style::Stylize::bold(self) -> T
+pub fn ratatui::prelude::style::Stylize::crossed_out(self) -> T
+pub fn ratatui::prelude::style::Stylize::cyan(self) -> T
+pub fn ratatui::prelude::style::Stylize::dark_gray(self) -> T
+pub fn ratatui::prelude::style::Stylize::dim(self) -> T
+pub fn ratatui::prelude::style::Stylize::fg<S: core::convert::Into<ratatui::style::Color>>(self, color: S) -> T
+pub fn ratatui::prelude::style::Stylize::gray(self) -> T
+pub fn ratatui::prelude::style::Stylize::green(self) -> T
+pub fn ratatui::prelude::style::Stylize::hidden(self) -> T
+pub fn ratatui::prelude::style::Stylize::italic(self) -> T
+pub fn ratatui::prelude::style::Stylize::light_blue(self) -> T
+pub fn ratatui::prelude::style::Stylize::light_cyan(self) -> T
+pub fn ratatui::prelude::style::Stylize::light_green(self) -> T
+pub fn ratatui::prelude::style::Stylize::light_magenta(self) -> T
+pub fn ratatui::prelude::style::Stylize::light_red(self) -> T
+pub fn ratatui::prelude::style::Stylize::light_yellow(self) -> T
+pub fn ratatui::prelude::style::Stylize::magenta(self) -> T
+pub fn ratatui::prelude::style::Stylize::not_bold(self) -> T
+pub fn ratatui::prelude::style::Stylize::not_crossed_out(self) -> T
+pub fn ratatui::prelude::style::Stylize::not_dim(self) -> T
+pub fn ratatui::prelude::style::Stylize::not_hidden(self) -> T
+pub fn ratatui::prelude::style::Stylize::not_italic(self) -> T
+pub fn ratatui::prelude::style::Stylize::not_rapid_blink(self) -> T
+pub fn ratatui::prelude::style::Stylize::not_reversed(self) -> T
+pub fn ratatui::prelude::style::Stylize::not_slow_blink(self) -> T
+pub fn ratatui::prelude::style::Stylize::not_underlined(self) -> T
+pub fn ratatui::prelude::style::Stylize::on_black(self) -> T
+pub fn ratatui::prelude::style::Stylize::on_blue(self) -> T
+pub fn ratatui::prelude::style::Stylize::on_cyan(self) -> T
+pub fn ratatui::prelude::style::Stylize::on_dark_gray(self) -> T
+pub fn ratatui::prelude::style::Stylize::on_gray(self) -> T
+pub fn ratatui::prelude::style::Stylize::on_green(self) -> T
+pub fn ratatui::prelude::style::Stylize::on_light_blue(self) -> T
+pub fn ratatui::prelude::style::Stylize::on_light_cyan(self) -> T
+pub fn ratatui::prelude::style::Stylize::on_light_green(self) -> T
+pub fn ratatui::prelude::style::Stylize::on_light_magenta(self) -> T
+pub fn ratatui::prelude::style::Stylize::on_light_red(self) -> T
+pub fn ratatui::prelude::style::Stylize::on_light_yellow(self) -> T
+pub fn ratatui::prelude::style::Stylize::on_magenta(self) -> T
+pub fn ratatui::prelude::style::Stylize::on_red(self) -> T
+pub fn ratatui::prelude::style::Stylize::on_white(self) -> T
+pub fn ratatui::prelude::style::Stylize::on_yellow(self) -> T
+pub fn ratatui::prelude::style::Stylize::rapid_blink(self) -> T
+pub fn ratatui::prelude::style::Stylize::red(self) -> T
+pub fn ratatui::prelude::style::Stylize::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::prelude::style::Stylize::reset(self) -> T
+pub fn ratatui::prelude::style::Stylize::reversed(self) -> T
+pub fn ratatui::prelude::style::Stylize::slow_blink(self) -> T
+pub fn ratatui::prelude::style::Stylize::underlined(self) -> T
+pub fn ratatui::prelude::style::Stylize::white(self) -> T
+pub fn ratatui::prelude::style::Stylize::yellow(self) -> T
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for U where U: ratatui::style::Styled<Item = T>
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for U where U: ratatui::style::Styled<Item = T>
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for U where U: ratatui::style::Styled<Item = T>
+pub fn U::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn U::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn U::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn U::bg(self, color: ratatui::style::Color) -> T
+pub fn U::bg(self, color: ratatui::style::Color) -> T
+pub fn U::bg(self, color: ratatui::style::Color) -> T
+pub fn U::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn U::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn U::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn U::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn U::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn U::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn U::reset(self) -> T
+pub fn U::reset(self) -> T
+pub fn U::reset(self) -> T
+pub mod ratatui::prelude::symbols
+pub mod ratatui::prelude::symbols::bar
+pub struct ratatui::prelude::symbols::bar::Set
+pub ratatui::prelude::symbols::bar::Set::empty: &'static str
+pub ratatui::prelude::symbols::bar::Set::five_eighths: &'static str
+pub ratatui::prelude::symbols::bar::Set::full: &'static str
+pub ratatui::prelude::symbols::bar::Set::half: &'static str
+pub ratatui::prelude::symbols::bar::Set::one_eighth: &'static str
+pub ratatui::prelude::symbols::bar::Set::one_quarter: &'static str
+pub ratatui::prelude::symbols::bar::Set::seven_eighths: &'static str
+pub ratatui::prelude::symbols::bar::Set::three_eighths: &'static str
+pub ratatui::prelude::symbols::bar::Set::three_quarters: &'static str
+impl core::clone::Clone for ratatui::symbols::bar::Set
+pub fn ratatui::symbols::bar::Set::clone(&self) -> ratatui::symbols::bar::Set
+impl core::fmt::Debug for ratatui::symbols::bar::Set
+pub fn ratatui::symbols::bar::Set::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::marker::Send for ratatui::symbols::bar::Set
+impl core::marker::Sync for ratatui::symbols::bar::Set
+impl core::marker::Unpin for ratatui::symbols::bar::Set
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::symbols::bar::Set
+impl core::panic::unwind_safe::UnwindSafe for ratatui::symbols::bar::Set
+impl<T, U> core::convert::Into<U> for ratatui::symbols::bar::Set where U: core::convert::From<T>
+pub fn ratatui::symbols::bar::Set::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::symbols::bar::Set where U: core::convert::Into<T>
+pub type ratatui::symbols::bar::Set::Error = core::convert::Infallible
+pub fn ratatui::symbols::bar::Set::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::symbols::bar::Set where U: core::convert::TryFrom<T>
+pub type ratatui::symbols::bar::Set::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::symbols::bar::Set::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::symbols::bar::Set where T: core::clone::Clone
+pub type ratatui::symbols::bar::Set::Owned = T
+pub fn ratatui::symbols::bar::Set::clone_into(&self, target: &mut T)
+pub fn ratatui::symbols::bar::Set::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::symbols::bar::Set where T: 'static + core::marker::Sized
+pub fn ratatui::symbols::bar::Set::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::symbols::bar::Set where T: core::marker::Sized
+pub fn ratatui::symbols::bar::Set::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::symbols::bar::Set where T: core::marker::Sized
+pub fn ratatui::symbols::bar::Set::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::symbols::bar::Set
+pub fn ratatui::symbols::bar::Set::from(t: T) -> T
+pub const ratatui::prelude::symbols::bar::FIVE_EIGHTHS: &str
+pub const ratatui::prelude::symbols::bar::FULL: &str
+pub const ratatui::prelude::symbols::bar::HALF: &str
+pub const ratatui::prelude::symbols::bar::NINE_LEVELS: _
+pub const ratatui::prelude::symbols::bar::ONE_EIGHTH: &str
+pub const ratatui::prelude::symbols::bar::ONE_QUARTER: &str
+pub const ratatui::prelude::symbols::bar::SEVEN_EIGHTHS: &str
+pub const ratatui::prelude::symbols::bar::THREE_EIGHTHS: &str
+pub const ratatui::prelude::symbols::bar::THREE_LEVELS: _
+pub const ratatui::prelude::symbols::bar::THREE_QUARTERS: &str
+pub mod ratatui::prelude::symbols::block
+pub struct ratatui::prelude::symbols::block::Set
+pub ratatui::prelude::symbols::block::Set::empty: &'static str
+pub ratatui::prelude::symbols::block::Set::five_eighths: &'static str
+pub ratatui::prelude::symbols::block::Set::full: &'static str
+pub ratatui::prelude::symbols::block::Set::half: &'static str
+pub ratatui::prelude::symbols::block::Set::one_eighth: &'static str
+pub ratatui::prelude::symbols::block::Set::one_quarter: &'static str
+pub ratatui::prelude::symbols::block::Set::seven_eighths: &'static str
+pub ratatui::prelude::symbols::block::Set::three_eighths: &'static str
+pub ratatui::prelude::symbols::block::Set::three_quarters: &'static str
+impl core::clone::Clone for ratatui::symbols::block::Set
+pub fn ratatui::symbols::block::Set::clone(&self) -> ratatui::symbols::block::Set
+impl core::fmt::Debug for ratatui::symbols::block::Set
+pub fn ratatui::symbols::block::Set::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::marker::Send for ratatui::symbols::block::Set
+impl core::marker::Sync for ratatui::symbols::block::Set
+impl core::marker::Unpin for ratatui::symbols::block::Set
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::symbols::block::Set
+impl core::panic::unwind_safe::UnwindSafe for ratatui::symbols::block::Set
+impl<T, U> core::convert::Into<U> for ratatui::symbols::block::Set where U: core::convert::From<T>
+pub fn ratatui::symbols::block::Set::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::symbols::block::Set where U: core::convert::Into<T>
+pub type ratatui::symbols::block::Set::Error = core::convert::Infallible
+pub fn ratatui::symbols::block::Set::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::symbols::block::Set where U: core::convert::TryFrom<T>
+pub type ratatui::symbols::block::Set::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::symbols::block::Set::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::symbols::block::Set where T: core::clone::Clone
+pub type ratatui::symbols::block::Set::Owned = T
+pub fn ratatui::symbols::block::Set::clone_into(&self, target: &mut T)
+pub fn ratatui::symbols::block::Set::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::symbols::block::Set where T: 'static + core::marker::Sized
+pub fn ratatui::symbols::block::Set::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::symbols::block::Set where T: core::marker::Sized
+pub fn ratatui::symbols::block::Set::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::symbols::block::Set where T: core::marker::Sized
+pub fn ratatui::symbols::block::Set::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::symbols::block::Set
+pub fn ratatui::symbols::block::Set::from(t: T) -> T
+pub const ratatui::prelude::symbols::block::FIVE_EIGHTHS: &str
+pub const ratatui::prelude::symbols::block::FULL: &str
+pub const ratatui::prelude::symbols::block::HALF: &str
+pub const ratatui::prelude::symbols::block::NINE_LEVELS: _
+pub const ratatui::prelude::symbols::block::ONE_EIGHTH: &str
+pub const ratatui::prelude::symbols::block::ONE_QUARTER: &str
+pub const ratatui::prelude::symbols::block::SEVEN_EIGHTHS: &str
+pub const ratatui::prelude::symbols::block::THREE_EIGHTHS: &str
+pub const ratatui::prelude::symbols::block::THREE_LEVELS: _
+pub const ratatui::prelude::symbols::block::THREE_QUARTERS: &str
+pub mod ratatui::prelude::symbols::braille
+pub const ratatui::prelude::symbols::braille::BLANK: u16 = 10_240u16
+pub const ratatui::prelude::symbols::braille::DOTS: _
+pub mod ratatui::prelude::symbols::line
+pub struct ratatui::prelude::symbols::line::Set
+pub ratatui::prelude::symbols::line::Set::bottom_left: &'static str
+pub ratatui::prelude::symbols::line::Set::bottom_right: &'static str
+pub ratatui::prelude::symbols::line::Set::cross: &'static str
+pub ratatui::prelude::symbols::line::Set::horizontal: &'static str
+pub ratatui::prelude::symbols::line::Set::horizontal_down: &'static str
+pub ratatui::prelude::symbols::line::Set::horizontal_up: &'static str
+pub ratatui::prelude::symbols::line::Set::top_left: &'static str
+pub ratatui::prelude::symbols::line::Set::top_right: &'static str
+pub ratatui::prelude::symbols::line::Set::vertical: &'static str
+pub ratatui::prelude::symbols::line::Set::vertical_left: &'static str
+pub ratatui::prelude::symbols::line::Set::vertical_right: &'static str
+impl core::clone::Clone for ratatui::symbols::line::Set
+pub fn ratatui::symbols::line::Set::clone(&self) -> ratatui::symbols::line::Set
+impl core::fmt::Debug for ratatui::symbols::line::Set
+pub fn ratatui::symbols::line::Set::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::marker::Send for ratatui::symbols::line::Set
+impl core::marker::Sync for ratatui::symbols::line::Set
+impl core::marker::Unpin for ratatui::symbols::line::Set
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::symbols::line::Set
+impl core::panic::unwind_safe::UnwindSafe for ratatui::symbols::line::Set
+impl<T, U> core::convert::Into<U> for ratatui::symbols::line::Set where U: core::convert::From<T>
+pub fn ratatui::symbols::line::Set::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::symbols::line::Set where U: core::convert::Into<T>
+pub type ratatui::symbols::line::Set::Error = core::convert::Infallible
+pub fn ratatui::symbols::line::Set::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::symbols::line::Set where U: core::convert::TryFrom<T>
+pub type ratatui::symbols::line::Set::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::symbols::line::Set::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::symbols::line::Set where T: core::clone::Clone
+pub type ratatui::symbols::line::Set::Owned = T
+pub fn ratatui::symbols::line::Set::clone_into(&self, target: &mut T)
+pub fn ratatui::symbols::line::Set::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::symbols::line::Set where T: 'static + core::marker::Sized
+pub fn ratatui::symbols::line::Set::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::symbols::line::Set where T: core::marker::Sized
+pub fn ratatui::symbols::line::Set::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::symbols::line::Set where T: core::marker::Sized
+pub fn ratatui::symbols::line::Set::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::symbols::line::Set
+pub fn ratatui::symbols::line::Set::from(t: T) -> T
+pub const ratatui::prelude::symbols::line::BOTTOM_LEFT: &str
+pub const ratatui::prelude::symbols::line::BOTTOM_RIGHT: &str
+pub const ratatui::prelude::symbols::line::CROSS: &str
+pub const ratatui::prelude::symbols::line::DOUBLE: _
+pub const ratatui::prelude::symbols::line::DOUBLE_BOTTOM_LEFT: &str
+pub const ratatui::prelude::symbols::line::DOUBLE_BOTTOM_RIGHT: &str
+pub const ratatui::prelude::symbols::line::DOUBLE_CROSS: &str
+pub const ratatui::prelude::symbols::line::DOUBLE_HORIZONTAL: &str
+pub const ratatui::prelude::symbols::line::DOUBLE_HORIZONTAL_DOWN: &str
+pub const ratatui::prelude::symbols::line::DOUBLE_HORIZONTAL_UP: &str
+pub const ratatui::prelude::symbols::line::DOUBLE_TOP_LEFT: &str
+pub const ratatui::prelude::symbols::line::DOUBLE_TOP_RIGHT: &str
+pub const ratatui::prelude::symbols::line::DOUBLE_VERTICAL: &str
+pub const ratatui::prelude::symbols::line::DOUBLE_VERTICAL_LEFT: &str
+pub const ratatui::prelude::symbols::line::DOUBLE_VERTICAL_RIGHT: &str
+pub const ratatui::prelude::symbols::line::HORIZONTAL: &str
+pub const ratatui::prelude::symbols::line::HORIZONTAL_DOWN: &str
+pub const ratatui::prelude::symbols::line::HORIZONTAL_UP: &str
+pub const ratatui::prelude::symbols::line::NORMAL: _
+pub const ratatui::prelude::symbols::line::ROUNDED: _
+pub const ratatui::prelude::symbols::line::ROUNDED_BOTTOM_LEFT: &str
+pub const ratatui::prelude::symbols::line::ROUNDED_BOTTOM_RIGHT: &str
+pub const ratatui::prelude::symbols::line::ROUNDED_TOP_LEFT: &str
+pub const ratatui::prelude::symbols::line::ROUNDED_TOP_RIGHT: &str
+pub const ratatui::prelude::symbols::line::THICK: _
+pub const ratatui::prelude::symbols::line::THICK_BOTTOM_LEFT: &str
+pub const ratatui::prelude::symbols::line::THICK_BOTTOM_RIGHT: &str
+pub const ratatui::prelude::symbols::line::THICK_CROSS: &str
+pub const ratatui::prelude::symbols::line::THICK_HORIZONTAL: &str
+pub const ratatui::prelude::symbols::line::THICK_HORIZONTAL_DOWN: &str
+pub const ratatui::prelude::symbols::line::THICK_HORIZONTAL_UP: &str
+pub const ratatui::prelude::symbols::line::THICK_TOP_LEFT: &str
+pub const ratatui::prelude::symbols::line::THICK_TOP_RIGHT: &str
+pub const ratatui::prelude::symbols::line::THICK_VERTICAL: &str
+pub const ratatui::prelude::symbols::line::THICK_VERTICAL_LEFT: &str
+pub const ratatui::prelude::symbols::line::THICK_VERTICAL_RIGHT: &str
+pub const ratatui::prelude::symbols::line::TOP_LEFT: &str
+pub const ratatui::prelude::symbols::line::TOP_RIGHT: &str
+pub const ratatui::prelude::symbols::line::VERTICAL: &str
+pub const ratatui::prelude::symbols::line::VERTICAL_LEFT: &str
+pub const ratatui::prelude::symbols::line::VERTICAL_RIGHT: &str
+pub enum ratatui::prelude::symbols::Marker
+pub ratatui::prelude::symbols::Marker::Bar
+pub ratatui::prelude::symbols::Marker::Block
+pub ratatui::prelude::symbols::Marker::Braille
+pub ratatui::prelude::symbols::Marker::Dot
+impl core::clone::Clone for ratatui::symbols::Marker
+impl core::clone::Clone for ratatui::symbols::Marker
+pub fn ratatui::symbols::Marker::clone(&self) -> ratatui::symbols::Marker
+pub fn ratatui::symbols::Marker::clone(&self) -> ratatui::symbols::Marker
+impl core::fmt::Debug for ratatui::symbols::Marker
+impl core::fmt::Debug for ratatui::symbols::Marker
+pub fn ratatui::symbols::Marker::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::symbols::Marker::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::marker::Copy for ratatui::symbols::Marker
+impl core::marker::Copy for ratatui::symbols::Marker
+impl core::marker::Send for ratatui::symbols::Marker
+impl core::marker::Send for ratatui::symbols::Marker
+impl core::marker::Sync for ratatui::symbols::Marker
+impl core::marker::Sync for ratatui::symbols::Marker
+impl core::marker::Unpin for ratatui::symbols::Marker
+impl core::marker::Unpin for ratatui::symbols::Marker
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::symbols::Marker
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::symbols::Marker
+impl core::panic::unwind_safe::UnwindSafe for ratatui::symbols::Marker
+impl core::panic::unwind_safe::UnwindSafe for ratatui::symbols::Marker
+impl<T, U> core::convert::Into<U> for ratatui::symbols::Marker where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::symbols::Marker where U: core::convert::From<T>
+pub fn ratatui::symbols::Marker::into(self) -> U
+pub fn ratatui::symbols::Marker::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::symbols::Marker where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::symbols::Marker where U: core::convert::Into<T>
+pub type ratatui::symbols::Marker::Error = core::convert::Infallible
+pub type ratatui::symbols::Marker::Error = core::convert::Infallible
+pub fn ratatui::symbols::Marker::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::symbols::Marker::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::symbols::Marker where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::symbols::Marker where U: core::convert::TryFrom<T>
+pub type ratatui::symbols::Marker::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::symbols::Marker::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::symbols::Marker::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::symbols::Marker::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::symbols::Marker where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::symbols::Marker where T: core::clone::Clone
+pub type ratatui::symbols::Marker::Owned = T
+pub type ratatui::symbols::Marker::Owned = T
+pub fn ratatui::symbols::Marker::clone_into(&self, target: &mut T)
+pub fn ratatui::symbols::Marker::clone_into(&self, target: &mut T)
+pub fn ratatui::symbols::Marker::to_owned(&self) -> T
+pub fn ratatui::symbols::Marker::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::symbols::Marker where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::symbols::Marker where T: 'static + core::marker::Sized
+pub fn ratatui::symbols::Marker::type_id(&self) -> core::any::TypeId
+pub fn ratatui::symbols::Marker::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::symbols::Marker where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::symbols::Marker where T: core::marker::Sized
+pub fn ratatui::symbols::Marker::borrow(&self) -> &T
+pub fn ratatui::symbols::Marker::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::symbols::Marker where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::symbols::Marker where T: core::marker::Sized
+pub fn ratatui::symbols::Marker::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::symbols::Marker::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::symbols::Marker
+impl<T> core::convert::From<T> for ratatui::symbols::Marker
+pub fn ratatui::symbols::Marker::from(t: T) -> T
+pub fn ratatui::symbols::Marker::from(t: T) -> T
+pub const ratatui::prelude::symbols::DOT: &str
+pub mod ratatui::prelude::terminal
+pub enum ratatui::prelude::terminal::Viewport
+pub ratatui::prelude::terminal::Viewport::Fixed(ratatui::layout::Rect)
+pub ratatui::prelude::terminal::Viewport::Fullscreen
+pub ratatui::prelude::terminal::Viewport::Inline(u16)
+impl core::clone::Clone for ratatui::terminal::Viewport
+impl core::clone::Clone for ratatui::terminal::Viewport
+pub fn ratatui::terminal::Viewport::clone(&self) -> ratatui::terminal::Viewport
+pub fn ratatui::terminal::Viewport::clone(&self) -> ratatui::terminal::Viewport
+impl core::cmp::Eq for ratatui::terminal::Viewport
+impl core::cmp::Eq for ratatui::terminal::Viewport
+impl core::cmp::PartialEq<ratatui::terminal::Viewport> for ratatui::terminal::Viewport
+impl core::cmp::PartialEq<ratatui::terminal::Viewport> for ratatui::terminal::Viewport
+pub fn ratatui::terminal::Viewport::eq(&self, other: &ratatui::terminal::Viewport) -> bool
+pub fn ratatui::terminal::Viewport::eq(&self, other: &ratatui::terminal::Viewport) -> bool
+impl core::fmt::Debug for ratatui::terminal::Viewport
+impl core::fmt::Debug for ratatui::terminal::Viewport
+pub fn ratatui::terminal::Viewport::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::terminal::Viewport::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::marker::StructuralEq for ratatui::terminal::Viewport
+impl core::marker::StructuralEq for ratatui::terminal::Viewport
+impl core::marker::StructuralPartialEq for ratatui::terminal::Viewport
+impl core::marker::StructuralPartialEq for ratatui::terminal::Viewport
+impl core::marker::Send for ratatui::terminal::Viewport
+impl core::marker::Send for ratatui::terminal::Viewport
+impl core::marker::Sync for ratatui::terminal::Viewport
+impl core::marker::Sync for ratatui::terminal::Viewport
+impl core::marker::Unpin for ratatui::terminal::Viewport
+impl core::marker::Unpin for ratatui::terminal::Viewport
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::terminal::Viewport
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::terminal::Viewport
+impl core::panic::unwind_safe::UnwindSafe for ratatui::terminal::Viewport
+impl core::panic::unwind_safe::UnwindSafe for ratatui::terminal::Viewport
+impl<T, U> core::convert::Into<U> for ratatui::terminal::Viewport where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::terminal::Viewport where U: core::convert::From<T>
+pub fn ratatui::terminal::Viewport::into(self) -> U
+pub fn ratatui::terminal::Viewport::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::terminal::Viewport where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::terminal::Viewport where U: core::convert::Into<T>
+pub type ratatui::terminal::Viewport::Error = core::convert::Infallible
+pub type ratatui::terminal::Viewport::Error = core::convert::Infallible
+pub fn ratatui::terminal::Viewport::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::terminal::Viewport::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::terminal::Viewport where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::terminal::Viewport where U: core::convert::TryFrom<T>
+pub type ratatui::terminal::Viewport::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::terminal::Viewport::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::terminal::Viewport::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::terminal::Viewport::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::terminal::Viewport where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::terminal::Viewport where T: core::clone::Clone
+pub type ratatui::terminal::Viewport::Owned = T
+pub type ratatui::terminal::Viewport::Owned = T
+pub fn ratatui::terminal::Viewport::clone_into(&self, target: &mut T)
+pub fn ratatui::terminal::Viewport::clone_into(&self, target: &mut T)
+pub fn ratatui::terminal::Viewport::to_owned(&self) -> T
+pub fn ratatui::terminal::Viewport::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::terminal::Viewport where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::terminal::Viewport where T: 'static + core::marker::Sized
+pub fn ratatui::terminal::Viewport::type_id(&self) -> core::any::TypeId
+pub fn ratatui::terminal::Viewport::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::terminal::Viewport where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::terminal::Viewport where T: core::marker::Sized
+pub fn ratatui::terminal::Viewport::borrow(&self) -> &T
+pub fn ratatui::terminal::Viewport::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::terminal::Viewport where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::terminal::Viewport where T: core::marker::Sized
+pub fn ratatui::terminal::Viewport::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::terminal::Viewport::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::terminal::Viewport
+impl<T> core::convert::From<T> for ratatui::terminal::Viewport
+pub fn ratatui::terminal::Viewport::from(t: T) -> T
+pub fn ratatui::terminal::Viewport::from(t: T) -> T
+pub struct ratatui::prelude::terminal::CompletedFrame<'a>
+pub ratatui::prelude::terminal::CompletedFrame::area: ratatui::layout::Rect
+pub ratatui::prelude::terminal::CompletedFrame::buffer: &'a ratatui::buffer::Buffer
+impl<'a> core::marker::Send for ratatui::terminal::CompletedFrame<'a>
+impl<'a> core::marker::Sync for ratatui::terminal::CompletedFrame<'a>
+impl<'a> core::marker::Unpin for ratatui::terminal::CompletedFrame<'a>
+impl<'a> core::panic::unwind_safe::RefUnwindSafe for ratatui::terminal::CompletedFrame<'a>
+impl<'a> core::panic::unwind_safe::UnwindSafe for ratatui::terminal::CompletedFrame<'a>
+impl<T, U> core::convert::Into<U> for ratatui::terminal::CompletedFrame<'a> where U: core::convert::From<T>
+pub fn ratatui::terminal::CompletedFrame<'a>::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::terminal::CompletedFrame<'a> where U: core::convert::Into<T>
+pub type ratatui::terminal::CompletedFrame<'a>::Error = core::convert::Infallible
+pub fn ratatui::terminal::CompletedFrame<'a>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::terminal::CompletedFrame<'a> where U: core::convert::TryFrom<T>
+pub type ratatui::terminal::CompletedFrame<'a>::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::terminal::CompletedFrame<'a>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> core::any::Any for ratatui::terminal::CompletedFrame<'a> where T: 'static + core::marker::Sized
+pub fn ratatui::terminal::CompletedFrame<'a>::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::terminal::CompletedFrame<'a> where T: core::marker::Sized
+pub fn ratatui::terminal::CompletedFrame<'a>::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::terminal::CompletedFrame<'a> where T: core::marker::Sized
+pub fn ratatui::terminal::CompletedFrame<'a>::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::terminal::CompletedFrame<'a>
+pub fn ratatui::terminal::CompletedFrame<'a>::from(t: T) -> T
+pub struct ratatui::prelude::terminal::Frame<'a, B> where B: ratatui::backend::Backend + 'a
+impl<'a, B> ratatui::terminal::Frame<'a, B> where B: ratatui::backend::Backend
+impl<'a, B> ratatui::terminal::Frame<'a, B> where B: ratatui::backend::Backend
+pub fn ratatui::terminal::Frame<'a, B>::render_stateful_widget<W>(&mut self, widget: W, area: ratatui::layout::Rect, state: &mut <W as ratatui::widgets::StatefulWidget>::State) where W: ratatui::widgets::StatefulWidget
+pub fn ratatui::terminal::Frame<'a, B>::render_stateful_widget<W>(&mut self, widget: W, area: ratatui::layout::Rect, state: &mut <W as ratatui::widgets::StatefulWidget>::State) where W: ratatui::widgets::StatefulWidget
+pub fn ratatui::terminal::Frame<'a, B>::render_widget<W>(&mut self, widget: W, area: ratatui::layout::Rect) where W: ratatui::widgets::Widget
+pub fn ratatui::terminal::Frame<'a, B>::render_widget<W>(&mut self, widget: W, area: ratatui::layout::Rect) where W: ratatui::widgets::Widget
+pub fn ratatui::terminal::Frame<'a, B>::set_cursor(&mut self, x: u16, y: u16)
+pub fn ratatui::terminal::Frame<'a, B>::set_cursor(&mut self, x: u16, y: u16)
+pub fn ratatui::terminal::Frame<'a, B>::size(&self) -> ratatui::layout::Rect
+pub fn ratatui::terminal::Frame<'a, B>::size(&self) -> ratatui::layout::Rect
+impl<'a, B> core::marker::Send for ratatui::terminal::Frame<'a, B> where B: core::marker::Send
+impl<'a, B> core::marker::Send for ratatui::terminal::Frame<'a, B> where B: core::marker::Send
+impl<'a, B> core::marker::Sync for ratatui::terminal::Frame<'a, B> where B: core::marker::Sync
+impl<'a, B> core::marker::Sync for ratatui::terminal::Frame<'a, B> where B: core::marker::Sync
+impl<'a, B> core::marker::Unpin for ratatui::terminal::Frame<'a, B>
+impl<'a, B> core::marker::Unpin for ratatui::terminal::Frame<'a, B>
+impl<'a, B> core::panic::unwind_safe::RefUnwindSafe for ratatui::terminal::Frame<'a, B> where B: core::panic::unwind_safe::RefUnwindSafe
+impl<'a, B> core::panic::unwind_safe::RefUnwindSafe for ratatui::terminal::Frame<'a, B> where B: core::panic::unwind_safe::RefUnwindSafe
+impl<'a, B> !core::panic::unwind_safe::UnwindSafe for ratatui::terminal::Frame<'a, B>
+impl<'a, B> !core::panic::unwind_safe::UnwindSafe for ratatui::terminal::Frame<'a, B>
+impl<T, U> core::convert::Into<U> for ratatui::terminal::Frame<'a, B> where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::terminal::Frame<'a, B> where U: core::convert::From<T>
+pub fn ratatui::terminal::Frame<'a, B>::into(self) -> U
+pub fn ratatui::terminal::Frame<'a, B>::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::terminal::Frame<'a, B> where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::terminal::Frame<'a, B> where U: core::convert::Into<T>
+pub type ratatui::terminal::Frame<'a, B>::Error = core::convert::Infallible
+pub type ratatui::terminal::Frame<'a, B>::Error = core::convert::Infallible
+pub fn ratatui::terminal::Frame<'a, B>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::terminal::Frame<'a, B>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::terminal::Frame<'a, B> where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::terminal::Frame<'a, B> where U: core::convert::TryFrom<T>
+pub type ratatui::terminal::Frame<'a, B>::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::terminal::Frame<'a, B>::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::terminal::Frame<'a, B>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::terminal::Frame<'a, B>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> core::any::Any for ratatui::terminal::Frame<'a, B> where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::terminal::Frame<'a, B> where T: 'static + core::marker::Sized
+pub fn ratatui::terminal::Frame<'a, B>::type_id(&self) -> core::any::TypeId
+pub fn ratatui::terminal::Frame<'a, B>::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::terminal::Frame<'a, B> where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::terminal::Frame<'a, B> where T: core::marker::Sized
+pub fn ratatui::terminal::Frame<'a, B>::borrow(&self) -> &T
+pub fn ratatui::terminal::Frame<'a, B>::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::terminal::Frame<'a, B> where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::terminal::Frame<'a, B> where T: core::marker::Sized
+pub fn ratatui::terminal::Frame<'a, B>::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::terminal::Frame<'a, B>::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::terminal::Frame<'a, B>
+impl<T> core::convert::From<T> for ratatui::terminal::Frame<'a, B>
+pub fn ratatui::terminal::Frame<'a, B>::from(t: T) -> T
+pub fn ratatui::terminal::Frame<'a, B>::from(t: T) -> T
+pub struct ratatui::prelude::terminal::Terminal<B> where B: ratatui::backend::Backend
+impl<B> ratatui::terminal::Terminal<B> where B: ratatui::backend::Backend
+impl<B> ratatui::terminal::Terminal<B> where B: ratatui::backend::Backend
+pub fn ratatui::terminal::Terminal<B>::autoresize(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::terminal::Terminal<B>::autoresize(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::terminal::Terminal<B>::backend(&self) -> &B
+pub fn ratatui::terminal::Terminal<B>::backend(&self) -> &B
+pub fn ratatui::terminal::Terminal<B>::backend_mut(&mut self) -> &mut B
+pub fn ratatui::terminal::Terminal<B>::backend_mut(&mut self) -> &mut B
+pub fn ratatui::terminal::Terminal<B>::clear(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::terminal::Terminal<B>::clear(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::terminal::Terminal<B>::current_buffer_mut(&mut self) -> &mut ratatui::buffer::Buffer
+pub fn ratatui::terminal::Terminal<B>::current_buffer_mut(&mut self) -> &mut ratatui::buffer::Buffer
+pub fn ratatui::terminal::Terminal<B>::draw<F>(&mut self, f: F) -> std::io::error::Result<ratatui::terminal::CompletedFrame<'_>> where F: core::ops::function::FnOnce(&mut ratatui::terminal::Frame<'_, B>)
+pub fn ratatui::terminal::Terminal<B>::draw<F>(&mut self, f: F) -> std::io::error::Result<ratatui::terminal::CompletedFrame<'_>> where F: core::ops::function::FnOnce(&mut ratatui::terminal::Frame<'_, B>)
+pub fn ratatui::terminal::Terminal<B>::flush(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::terminal::Terminal<B>::flush(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::terminal::Terminal<B>::get_cursor(&mut self) -> std::io::error::Result<(u16, u16)>
+pub fn ratatui::terminal::Terminal<B>::get_cursor(&mut self) -> std::io::error::Result<(u16, u16)>
+pub fn ratatui::terminal::Terminal<B>::get_frame(&mut self) -> ratatui::terminal::Frame<'_, B>
+pub fn ratatui::terminal::Terminal<B>::get_frame(&mut self) -> ratatui::terminal::Frame<'_, B>
+pub fn ratatui::terminal::Terminal<B>::hide_cursor(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::terminal::Terminal<B>::hide_cursor(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::terminal::Terminal<B>::insert_before<F>(&mut self, height: u16, draw_fn: F) -> std::io::error::Result<()> where F: core::ops::function::FnOnce(&mut ratatui::buffer::Buffer)
+pub fn ratatui::terminal::Terminal<B>::insert_before<F>(&mut self, height: u16, draw_fn: F) -> std::io::error::Result<()> where F: core::ops::function::FnOnce(&mut ratatui::buffer::Buffer)
+pub fn ratatui::terminal::Terminal<B>::new(backend: B) -> std::io::error::Result<ratatui::terminal::Terminal<B>>
+pub fn ratatui::terminal::Terminal<B>::new(backend: B) -> std::io::error::Result<ratatui::terminal::Terminal<B>>
+pub fn ratatui::terminal::Terminal<B>::resize(&mut self, size: ratatui::layout::Rect) -> std::io::error::Result<()>
+pub fn ratatui::terminal::Terminal<B>::resize(&mut self, size: ratatui::layout::Rect) -> std::io::error::Result<()>
+pub fn ratatui::terminal::Terminal<B>::set_cursor(&mut self, x: u16, y: u16) -> std::io::error::Result<()>
+pub fn ratatui::terminal::Terminal<B>::set_cursor(&mut self, x: u16, y: u16) -> std::io::error::Result<()>
+pub fn ratatui::terminal::Terminal<B>::show_cursor(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::terminal::Terminal<B>::show_cursor(&mut self) -> std::io::error::Result<()>
+pub fn ratatui::terminal::Terminal<B>::size(&self) -> std::io::error::Result<ratatui::layout::Rect>
+pub fn ratatui::terminal::Terminal<B>::size(&self) -> std::io::error::Result<ratatui::layout::Rect>
+pub fn ratatui::terminal::Terminal<B>::swap_buffers(&mut self)
+pub fn ratatui::terminal::Terminal<B>::swap_buffers(&mut self)
+pub fn ratatui::terminal::Terminal<B>::swap_buffers(&mut self)
+pub fn ratatui::terminal::Terminal<B>::swap_buffers(&mut self)
+pub fn ratatui::terminal::Terminal<B>::with_options(backend: B, options: ratatui::terminal::TerminalOptions) -> std::io::error::Result<ratatui::terminal::Terminal<B>>
+pub fn ratatui::terminal::Terminal<B>::with_options(backend: B, options: ratatui::terminal::TerminalOptions) -> std::io::error::Result<ratatui::terminal::Terminal<B>>
+impl<B> core::ops::drop::Drop for ratatui::terminal::Terminal<B> where B: ratatui::backend::Backend
+impl<B> core::ops::drop::Drop for ratatui::terminal::Terminal<B> where B: ratatui::backend::Backend
+pub fn ratatui::terminal::Terminal<B>::drop(&mut self)
+pub fn ratatui::terminal::Terminal<B>::drop(&mut self)
+impl<B> core::fmt::Debug for ratatui::terminal::Terminal<B> where B: ratatui::backend::Backend + core::fmt::Debug
+impl<B> core::fmt::Debug for ratatui::terminal::Terminal<B> where B: ratatui::backend::Backend + core::fmt::Debug
+pub fn ratatui::terminal::Terminal<B>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::terminal::Terminal<B>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl<B> core::marker::Send for ratatui::terminal::Terminal<B> where B: core::marker::Send
+impl<B> core::marker::Send for ratatui::terminal::Terminal<B> where B: core::marker::Send
+impl<B> core::marker::Sync for ratatui::terminal::Terminal<B> where B: core::marker::Sync
+impl<B> core::marker::Sync for ratatui::terminal::Terminal<B> where B: core::marker::Sync
+impl<B> core::marker::Unpin for ratatui::terminal::Terminal<B> where B: core::marker::Unpin
+impl<B> core::marker::Unpin for ratatui::terminal::Terminal<B> where B: core::marker::Unpin
+impl<B> core::panic::unwind_safe::RefUnwindSafe for ratatui::terminal::Terminal<B> where B: core::panic::unwind_safe::RefUnwindSafe
+impl<B> core::panic::unwind_safe::RefUnwindSafe for ratatui::terminal::Terminal<B> where B: core::panic::unwind_safe::RefUnwindSafe
+impl<B> core::panic::unwind_safe::UnwindSafe for ratatui::terminal::Terminal<B> where B: core::panic::unwind_safe::UnwindSafe
+impl<B> core::panic::unwind_safe::UnwindSafe for ratatui::terminal::Terminal<B> where B: core::panic::unwind_safe::UnwindSafe
+impl<T, U> core::convert::Into<U> for ratatui::terminal::Terminal<B> where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::terminal::Terminal<B> where U: core::convert::From<T>
+pub fn ratatui::terminal::Terminal<B>::into(self) -> U
+pub fn ratatui::terminal::Terminal<B>::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::terminal::Terminal<B> where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::terminal::Terminal<B> where U: core::convert::Into<T>
+pub type ratatui::terminal::Terminal<B>::Error = core::convert::Infallible
+pub type ratatui::terminal::Terminal<B>::Error = core::convert::Infallible
+pub fn ratatui::terminal::Terminal<B>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::terminal::Terminal<B>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::terminal::Terminal<B> where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::terminal::Terminal<B> where U: core::convert::TryFrom<T>
+pub type ratatui::terminal::Terminal<B>::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::terminal::Terminal<B>::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::terminal::Terminal<B>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::terminal::Terminal<B>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> core::any::Any for ratatui::terminal::Terminal<B> where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::terminal::Terminal<B> where T: 'static + core::marker::Sized
+pub fn ratatui::terminal::Terminal<B>::type_id(&self) -> core::any::TypeId
+pub fn ratatui::terminal::Terminal<B>::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::terminal::Terminal<B> where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::terminal::Terminal<B> where T: core::marker::Sized
+pub fn ratatui::terminal::Terminal<B>::borrow(&self) -> &T
+pub fn ratatui::terminal::Terminal<B>::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::terminal::Terminal<B> where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::terminal::Terminal<B> where T: core::marker::Sized
+pub fn ratatui::terminal::Terminal<B>::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::terminal::Terminal<B>::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::terminal::Terminal<B>
+impl<T> core::convert::From<T> for ratatui::terminal::Terminal<B>
+pub fn ratatui::terminal::Terminal<B>::from(t: T) -> T
+pub fn ratatui::terminal::Terminal<B>::from(t: T) -> T
+pub struct ratatui::prelude::terminal::TerminalOptions
+pub ratatui::prelude::terminal::TerminalOptions::viewport: ratatui::terminal::Viewport
+impl core::clone::Clone for ratatui::terminal::TerminalOptions
+impl core::clone::Clone for ratatui::terminal::TerminalOptions
+pub fn ratatui::terminal::TerminalOptions::clone(&self) -> ratatui::terminal::TerminalOptions
+pub fn ratatui::terminal::TerminalOptions::clone(&self) -> ratatui::terminal::TerminalOptions
+impl core::cmp::Eq for ratatui::terminal::TerminalOptions
+impl core::cmp::Eq for ratatui::terminal::TerminalOptions
+impl core::cmp::PartialEq<ratatui::terminal::TerminalOptions> for ratatui::terminal::TerminalOptions
+impl core::cmp::PartialEq<ratatui::terminal::TerminalOptions> for ratatui::terminal::TerminalOptions
+pub fn ratatui::terminal::TerminalOptions::eq(&self, other: &ratatui::terminal::TerminalOptions) -> bool
+pub fn ratatui::terminal::TerminalOptions::eq(&self, other: &ratatui::terminal::TerminalOptions) -> bool
+impl core::fmt::Debug for ratatui::terminal::TerminalOptions
+impl core::fmt::Debug for ratatui::terminal::TerminalOptions
+pub fn ratatui::terminal::TerminalOptions::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::terminal::TerminalOptions::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::marker::StructuralEq for ratatui::terminal::TerminalOptions
+impl core::marker::StructuralEq for ratatui::terminal::TerminalOptions
+impl core::marker::StructuralPartialEq for ratatui::terminal::TerminalOptions
+impl core::marker::StructuralPartialEq for ratatui::terminal::TerminalOptions
+impl core::marker::Send for ratatui::terminal::TerminalOptions
+impl core::marker::Send for ratatui::terminal::TerminalOptions
+impl core::marker::Sync for ratatui::terminal::TerminalOptions
+impl core::marker::Sync for ratatui::terminal::TerminalOptions
+impl core::marker::Unpin for ratatui::terminal::TerminalOptions
+impl core::marker::Unpin for ratatui::terminal::TerminalOptions
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::terminal::TerminalOptions
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::terminal::TerminalOptions
+impl core::panic::unwind_safe::UnwindSafe for ratatui::terminal::TerminalOptions
+impl core::panic::unwind_safe::UnwindSafe for ratatui::terminal::TerminalOptions
+impl<T, U> core::convert::Into<U> for ratatui::terminal::TerminalOptions where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::terminal::TerminalOptions where U: core::convert::From<T>
+pub fn ratatui::terminal::TerminalOptions::into(self) -> U
+pub fn ratatui::terminal::TerminalOptions::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::terminal::TerminalOptions where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::terminal::TerminalOptions where U: core::convert::Into<T>
+pub type ratatui::terminal::TerminalOptions::Error = core::convert::Infallible
+pub type ratatui::terminal::TerminalOptions::Error = core::convert::Infallible
+pub fn ratatui::terminal::TerminalOptions::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::terminal::TerminalOptions::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::terminal::TerminalOptions where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::terminal::TerminalOptions where U: core::convert::TryFrom<T>
+pub type ratatui::terminal::TerminalOptions::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::terminal::TerminalOptions::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::terminal::TerminalOptions::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::terminal::TerminalOptions::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::terminal::TerminalOptions where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::terminal::TerminalOptions where T: core::clone::Clone
+pub type ratatui::terminal::TerminalOptions::Owned = T
+pub type ratatui::terminal::TerminalOptions::Owned = T
+pub fn ratatui::terminal::TerminalOptions::clone_into(&self, target: &mut T)
+pub fn ratatui::terminal::TerminalOptions::clone_into(&self, target: &mut T)
+pub fn ratatui::terminal::TerminalOptions::to_owned(&self) -> T
+pub fn ratatui::terminal::TerminalOptions::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::terminal::TerminalOptions where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::terminal::TerminalOptions where T: 'static + core::marker::Sized
+pub fn ratatui::terminal::TerminalOptions::type_id(&self) -> core::any::TypeId
+pub fn ratatui::terminal::TerminalOptions::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::terminal::TerminalOptions where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::terminal::TerminalOptions where T: core::marker::Sized
+pub fn ratatui::terminal::TerminalOptions::borrow(&self) -> &T
+pub fn ratatui::terminal::TerminalOptions::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::terminal::TerminalOptions where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::terminal::TerminalOptions where T: core::marker::Sized
+pub fn ratatui::terminal::TerminalOptions::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::terminal::TerminalOptions::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::terminal::TerminalOptions
+impl<T> core::convert::From<T> for ratatui::terminal::TerminalOptions
+pub fn ratatui::terminal::TerminalOptions::from(t: T) -> T
+pub fn ratatui::terminal::TerminalOptions::from(t: T) -> T
+pub mod ratatui::prelude::text
+pub struct ratatui::prelude::text::Line<'a>
+pub ratatui::prelude::text::Line::alignment: core::option::Option<ratatui::layout::Alignment>
+pub ratatui::prelude::text::Line::spans: alloc::vec::Vec<ratatui::text::Span<'a>>
+impl<'a> ratatui::text::Line<'a>
+impl<'a> ratatui::text::Line<'a>
+pub fn ratatui::text::Line<'a>::alignment(self, alignment: ratatui::layout::Alignment) -> Self
+pub fn ratatui::text::Line<'a>::alignment(self, alignment: ratatui::layout::Alignment) -> Self
+pub fn ratatui::text::Line<'a>::patch_style(&mut self, style: ratatui::style::Style)
+pub fn ratatui::text::Line<'a>::patch_style(&mut self, style: ratatui::style::Style)
+pub fn ratatui::text::Line<'a>::reset_style(&mut self)
+pub fn ratatui::text::Line<'a>::reset_style(&mut self)
+pub fn ratatui::text::Line<'a>::styled<T>(content: T, style: ratatui::style::Style) -> ratatui::text::Line<'a> where T: core::convert::Into<alloc::borrow::Cow<'a, str>>
+pub fn ratatui::text::Line<'a>::styled<T>(content: T, style: ratatui::style::Style) -> ratatui::text::Line<'a> where T: core::convert::Into<alloc::borrow::Cow<'a, str>>
+pub fn ratatui::text::Line<'a>::styled<T>(content: T, style: ratatui::style::Style) -> ratatui::text::Line<'a> where T: core::convert::Into<alloc::borrow::Cow<'a, str>>
+pub fn ratatui::text::Line<'a>::styled_graphemes(&'a self, base_style: ratatui::style::Style) -> impl core::iter::traits::iterator::Iterator<Item = ratatui::text::StyledGrapheme<'a>>
+pub fn ratatui::text::Line<'a>::styled_graphemes(&'a self, base_style: ratatui::style::Style) -> impl core::iter::traits::iterator::Iterator<Item = ratatui::text::StyledGrapheme<'a>>
+pub fn ratatui::text::Line<'a>::styled_graphemes(&'a self, base_style: ratatui::style::Style) -> impl core::iter::traits::iterator::Iterator<Item = ratatui::text::StyledGrapheme<'a>>
+pub fn ratatui::text::Line<'a>::width(&self) -> usize
+pub fn ratatui::text::Line<'a>::width(&self) -> usize
+impl<'a> core::convert::From<&'a str> for ratatui::text::Line<'a>
+impl<'a> core::convert::From<&'a str> for ratatui::text::Line<'a>
+pub fn ratatui::text::Line<'a>::from(s: &'a str) -> Self
+pub fn ratatui::text::Line<'a>::from(s: &'a str) -> Self
+impl<'a> core::convert::From<alloc::string::String> for ratatui::text::Line<'a>
+impl<'a> core::convert::From<alloc::string::String> for ratatui::text::Line<'a>
+pub fn ratatui::text::Line<'a>::from(s: alloc::string::String) -> Self
+pub fn ratatui::text::Line<'a>::from(s: alloc::string::String) -> Self
+impl<'a> core::convert::From<alloc::vec::Vec<ratatui::text::Span<'a>, alloc::alloc::Global>> for ratatui::text::Line<'a>
+impl<'a> core::convert::From<alloc::vec::Vec<ratatui::text::Span<'a>, alloc::alloc::Global>> for ratatui::text::Line<'a>
+pub fn ratatui::text::Line<'a>::from(spans: alloc::vec::Vec<ratatui::text::Span<'a>>) -> Self
+pub fn ratatui::text::Line<'a>::from(spans: alloc::vec::Vec<ratatui::text::Span<'a>>) -> Self
+impl<'a> core::convert::From<ratatui::text::Line<'a>> for alloc::string::String
+impl<'a> core::convert::From<ratatui::text::Line<'a>> for alloc::string::String
+pub fn alloc::string::String::from(line: ratatui::text::Line<'a>) -> alloc::string::String
+pub fn alloc::string::String::from(line: ratatui::text::Line<'a>) -> alloc::string::String
+impl<'a> core::convert::From<ratatui::text::Line<'a>> for ratatui::text::Text<'a>
+impl<'a> core::convert::From<ratatui::text::Line<'a>> for ratatui::text::Text<'a>
+impl<'a> core::convert::From<ratatui::text::Line<'a>> for ratatui::text::Text<'a>
+impl<'a> core::convert::From<ratatui::text::Line<'a>> for ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(line: ratatui::text::Line<'a>) -> ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(line: ratatui::text::Line<'a>) -> ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(line: ratatui::text::Line<'a>) -> ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(line: ratatui::text::Line<'a>) -> ratatui::text::Text<'a>
+impl<'a> core::convert::From<ratatui::text::Span<'a>> for ratatui::text::Line<'a>
+impl<'a> core::convert::From<ratatui::text::Span<'a>> for ratatui::text::Line<'a>
+impl<'a> core::convert::From<ratatui::text::Span<'a>> for ratatui::text::Line<'a>
+impl<'a> core::convert::From<ratatui::text::Span<'a>> for ratatui::text::Line<'a>
+pub fn ratatui::text::Line<'a>::from(span: ratatui::text::Span<'a>) -> Self
+pub fn ratatui::text::Line<'a>::from(span: ratatui::text::Span<'a>) -> Self
+pub fn ratatui::text::Line<'a>::from(span: ratatui::text::Span<'a>) -> Self
+pub fn ratatui::text::Line<'a>::from(span: ratatui::text::Span<'a>) -> Self
+impl<'a> core::convert::From<ratatui::text::Spans<'a>> for ratatui::text::Line<'a>
+impl<'a> core::convert::From<ratatui::text::Spans<'a>> for ratatui::text::Line<'a>
+impl<'a> core::convert::From<ratatui::text::Spans<'a>> for ratatui::text::Line<'a>
+pub fn ratatui::text::Line<'a>::from(value: ratatui::text::Spans<'a>) -> Self
+pub fn ratatui::text::Line<'a>::from(value: ratatui::text::Spans<'a>) -> Self
+pub fn ratatui::text::Line<'a>::from(value: ratatui::text::Spans<'a>) -> Self
+impl<'a> core::clone::Clone for ratatui::text::Line<'a>
+impl<'a> core::clone::Clone for ratatui::text::Line<'a>
+pub fn ratatui::text::Line<'a>::clone(&self) -> ratatui::text::Line<'a>
+pub fn ratatui::text::Line<'a>::clone(&self) -> ratatui::text::Line<'a>
+impl<'a> core::cmp::Eq for ratatui::text::Line<'a>
+impl<'a> core::cmp::Eq for ratatui::text::Line<'a>
+impl<'a> core::cmp::PartialEq<ratatui::text::Line<'a>> for ratatui::text::Line<'a>
+impl<'a> core::cmp::PartialEq<ratatui::text::Line<'a>> for ratatui::text::Line<'a>
+pub fn ratatui::text::Line<'a>::eq(&self, other: &ratatui::text::Line<'a>) -> bool
+pub fn ratatui::text::Line<'a>::eq(&self, other: &ratatui::text::Line<'a>) -> bool
+impl<'a> core::default::Default for ratatui::text::Line<'a>
+impl<'a> core::default::Default for ratatui::text::Line<'a>
+pub fn ratatui::text::Line<'a>::default() -> ratatui::text::Line<'a>
+pub fn ratatui::text::Line<'a>::default() -> ratatui::text::Line<'a>
+impl<'a> core::fmt::Debug for ratatui::text::Line<'a>
+impl<'a> core::fmt::Debug for ratatui::text::Line<'a>
+pub fn ratatui::text::Line<'a>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::text::Line<'a>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl<'a> core::marker::StructuralEq for ratatui::text::Line<'a>
+impl<'a> core::marker::StructuralEq for ratatui::text::Line<'a>
+impl<'a> core::marker::StructuralPartialEq for ratatui::text::Line<'a>
+impl<'a> core::marker::StructuralPartialEq for ratatui::text::Line<'a>
+impl<'a> core::marker::Send for ratatui::text::Line<'a>
+impl<'a> core::marker::Send for ratatui::text::Line<'a>
+impl<'a> core::marker::Sync for ratatui::text::Line<'a>
+impl<'a> core::marker::Sync for ratatui::text::Line<'a>
+impl<'a> core::marker::Unpin for ratatui::text::Line<'a>
+impl<'a> core::marker::Unpin for ratatui::text::Line<'a>
+impl<'a> core::panic::unwind_safe::RefUnwindSafe for ratatui::text::Line<'a>
+impl<'a> core::panic::unwind_safe::RefUnwindSafe for ratatui::text::Line<'a>
+impl<'a> core::panic::unwind_safe::UnwindSafe for ratatui::text::Line<'a>
+impl<'a> core::panic::unwind_safe::UnwindSafe for ratatui::text::Line<'a>
+impl<T, U> core::convert::Into<U> for ratatui::text::Line<'a> where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::text::Line<'a> where U: core::convert::From<T>
+pub fn ratatui::text::Line<'a>::into(self) -> U
+pub fn ratatui::text::Line<'a>::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::text::Line<'a> where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::text::Line<'a> where U: core::convert::Into<T>
+pub type ratatui::text::Line<'a>::Error = core::convert::Infallible
+pub type ratatui::text::Line<'a>::Error = core::convert::Infallible
+pub fn ratatui::text::Line<'a>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::text::Line<'a>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::text::Line<'a> where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::text::Line<'a> where U: core::convert::TryFrom<T>
+pub type ratatui::text::Line<'a>::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::text::Line<'a>::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::text::Line<'a>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::text::Line<'a>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::text::Line<'a> where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::text::Line<'a> where T: core::clone::Clone
+pub type ratatui::text::Line<'a>::Owned = T
+pub type ratatui::text::Line<'a>::Owned = T
+pub fn ratatui::text::Line<'a>::clone_into(&self, target: &mut T)
+pub fn ratatui::text::Line<'a>::clone_into(&self, target: &mut T)
+pub fn ratatui::text::Line<'a>::to_owned(&self) -> T
+pub fn ratatui::text::Line<'a>::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::text::Line<'a> where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::text::Line<'a> where T: 'static + core::marker::Sized
+pub fn ratatui::text::Line<'a>::type_id(&self) -> core::any::TypeId
+pub fn ratatui::text::Line<'a>::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::text::Line<'a> where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::text::Line<'a> where T: core::marker::Sized
+pub fn ratatui::text::Line<'a>::borrow(&self) -> &T
+pub fn ratatui::text::Line<'a>::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::text::Line<'a> where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::text::Line<'a> where T: core::marker::Sized
+pub fn ratatui::text::Line<'a>::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::text::Line<'a>::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::text::Line<'a>
+impl<T> core::convert::From<T> for ratatui::text::Line<'a>
+pub fn ratatui::text::Line<'a>::from(t: T) -> T
+pub fn ratatui::text::Line<'a>::from(t: T) -> T
+pub struct ratatui::prelude::text::Masked<'a>
+impl<'a> ratatui::text::Masked<'a>
+impl<'a> ratatui::text::Masked<'a>
+pub fn ratatui::text::Masked<'a>::mask_char(&self) -> char
+pub fn ratatui::text::Masked<'a>::mask_char(&self) -> char
+pub fn ratatui::text::Masked<'a>::new(s: impl core::convert::Into<alloc::borrow::Cow<'a, str>>, mask_char: char) -> Self
+pub fn ratatui::text::Masked<'a>::new(s: impl core::convert::Into<alloc::borrow::Cow<'a, str>>, mask_char: char) -> Self
+pub fn ratatui::text::Masked<'a>::value(&self) -> alloc::borrow::Cow<'a, str>
+pub fn ratatui::text::Masked<'a>::value(&self) -> alloc::borrow::Cow<'a, str>
+impl core::fmt::Debug for ratatui::text::Masked<'_>
+impl core::fmt::Debug for ratatui::text::Masked<'_>
+pub fn ratatui::text::Masked<'_>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::text::Masked<'_>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::text::Masked<'_>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::text::Masked<'_>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::fmt::Display for ratatui::text::Masked<'_>
+impl core::fmt::Display for ratatui::text::Masked<'_>
+impl<'a> core::convert::From<&'a ratatui::text::Masked<'_>> for ratatui::text::Text<'a>
+impl<'a> core::convert::From<&'a ratatui::text::Masked<'_>> for ratatui::text::Text<'a>
+impl<'a> core::convert::From<&'a ratatui::text::Masked<'_>> for ratatui::text::Text<'a>
+impl<'a> core::convert::From<&'a ratatui::text::Masked<'_>> for ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(masked: &'a ratatui::text::Masked<'_>) -> ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(masked: &'a ratatui::text::Masked<'_>) -> ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(masked: &'a ratatui::text::Masked<'_>) -> ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(masked: &'a ratatui::text::Masked<'_>) -> ratatui::text::Text<'a>
+impl<'a> core::convert::From<&'a ratatui::text::Masked<'a>> for alloc::borrow::Cow<'a, str>
+impl<'a> core::convert::From<&'a ratatui::text::Masked<'a>> for alloc::borrow::Cow<'a, str>
+pub fn alloc::borrow::Cow<'a, str>::from(masked: &'a ratatui::text::Masked<'_>) -> alloc::borrow::Cow<'a, str>
+pub fn alloc::borrow::Cow<'a, str>::from(masked: &'a ratatui::text::Masked<'_>) -> alloc::borrow::Cow<'a, str>
+impl<'a> core::convert::From<ratatui::text::Masked<'a>> for alloc::borrow::Cow<'a, str>
+impl<'a> core::convert::From<ratatui::text::Masked<'a>> for alloc::borrow::Cow<'a, str>
+pub fn alloc::borrow::Cow<'a, str>::from(masked: ratatui::text::Masked<'a>) -> alloc::borrow::Cow<'a, str>
+pub fn alloc::borrow::Cow<'a, str>::from(masked: ratatui::text::Masked<'a>) -> alloc::borrow::Cow<'a, str>
+impl<'a> core::convert::From<ratatui::text::Masked<'a>> for ratatui::text::Text<'a>
+impl<'a> core::convert::From<ratatui::text::Masked<'a>> for ratatui::text::Text<'a>
+impl<'a> core::convert::From<ratatui::text::Masked<'a>> for ratatui::text::Text<'a>
+impl<'a> core::convert::From<ratatui::text::Masked<'a>> for ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(masked: ratatui::text::Masked<'a>) -> ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(masked: ratatui::text::Masked<'a>) -> ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(masked: ratatui::text::Masked<'a>) -> ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(masked: ratatui::text::Masked<'a>) -> ratatui::text::Text<'a>
+impl<'a> core::clone::Clone for ratatui::text::Masked<'a>
+impl<'a> core::clone::Clone for ratatui::text::Masked<'a>
+pub fn ratatui::text::Masked<'a>::clone(&self) -> ratatui::text::Masked<'a>
+pub fn ratatui::text::Masked<'a>::clone(&self) -> ratatui::text::Masked<'a>
+impl<'a> core::cmp::Eq for ratatui::text::Masked<'a>
+impl<'a> core::cmp::Eq for ratatui::text::Masked<'a>
+impl<'a> core::cmp::Eq for ratatui::text::Masked<'a>
+impl<'a> core::cmp::PartialEq<ratatui::text::Masked<'a>> for ratatui::text::Masked<'a>
+impl<'a> core::cmp::PartialEq<ratatui::text::Masked<'a>> for ratatui::text::Masked<'a>
+impl<'a> core::cmp::PartialEq<ratatui::text::Masked<'a>> for ratatui::text::Masked<'a>
+pub fn ratatui::text::Masked<'a>::eq(&self, other: &ratatui::text::Masked<'a>) -> bool
+pub fn ratatui::text::Masked<'a>::eq(&self, other: &ratatui::text::Masked<'a>) -> bool
+pub fn ratatui::text::Masked<'a>::eq(&self, other: &ratatui::text::Masked<'a>) -> bool
+impl<'a> core::hash::Hash for ratatui::text::Masked<'a>
+impl<'a> core::hash::Hash for ratatui::text::Masked<'a>
+impl<'a> core::hash::Hash for ratatui::text::Masked<'a>
+pub fn ratatui::text::Masked<'a>::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
+pub fn ratatui::text::Masked<'a>::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
+pub fn ratatui::text::Masked<'a>::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
+impl<'a> core::marker::StructuralEq for ratatui::text::Masked<'a>
+impl<'a> core::marker::StructuralEq for ratatui::text::Masked<'a>
+impl<'a> core::marker::StructuralEq for ratatui::text::Masked<'a>
+impl<'a> core::marker::StructuralPartialEq for ratatui::text::Masked<'a>
+impl<'a> core::marker::StructuralPartialEq for ratatui::text::Masked<'a>
+impl<'a> core::marker::StructuralPartialEq for ratatui::text::Masked<'a>
+impl<'a> core::marker::Send for ratatui::text::Masked<'a>
+impl<'a> core::marker::Send for ratatui::text::Masked<'a>
+impl<'a> core::marker::Sync for ratatui::text::Masked<'a>
+impl<'a> core::marker::Sync for ratatui::text::Masked<'a>
+impl<'a> core::marker::Unpin for ratatui::text::Masked<'a>
+impl<'a> core::marker::Unpin for ratatui::text::Masked<'a>
+impl<'a> core::panic::unwind_safe::RefUnwindSafe for ratatui::text::Masked<'a>
+impl<'a> core::panic::unwind_safe::RefUnwindSafe for ratatui::text::Masked<'a>
+impl<'a> core::panic::unwind_safe::UnwindSafe for ratatui::text::Masked<'a>
+impl<'a> core::panic::unwind_safe::UnwindSafe for ratatui::text::Masked<'a>
+impl<T, U> core::convert::Into<U> for ratatui::text::Masked<'a> where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::text::Masked<'a> where U: core::convert::From<T>
+pub fn ratatui::text::Masked<'a>::into(self) -> U
+pub fn ratatui::text::Masked<'a>::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::text::Masked<'a> where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::text::Masked<'a> where U: core::convert::Into<T>
+pub type ratatui::text::Masked<'a>::Error = core::convert::Infallible
+pub type ratatui::text::Masked<'a>::Error = core::convert::Infallible
+pub fn ratatui::text::Masked<'a>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::text::Masked<'a>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::text::Masked<'a> where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::text::Masked<'a> where U: core::convert::TryFrom<T>
+pub type ratatui::text::Masked<'a>::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::text::Masked<'a>::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::text::Masked<'a>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::text::Masked<'a>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::text::Masked<'a> where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::text::Masked<'a> where T: core::clone::Clone
+pub type ratatui::text::Masked<'a>::Owned = T
+pub type ratatui::text::Masked<'a>::Owned = T
+pub fn ratatui::text::Masked<'a>::clone_into(&self, target: &mut T)
+pub fn ratatui::text::Masked<'a>::clone_into(&self, target: &mut T)
+pub fn ratatui::text::Masked<'a>::to_owned(&self) -> T
+pub fn ratatui::text::Masked<'a>::to_owned(&self) -> T
+impl<T> alloc::string::ToString for ratatui::text::Masked<'a> where T: core::fmt::Display + core::marker::Sized
+impl<T> alloc::string::ToString for ratatui::text::Masked<'a> where T: core::fmt::Display + core::marker::Sized
+pub fn ratatui::text::Masked<'a>::to_string(&self) -> alloc::string::String
+pub fn ratatui::text::Masked<'a>::to_string(&self) -> alloc::string::String
+impl<T> core::any::Any for ratatui::text::Masked<'a> where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::text::Masked<'a> where T: 'static + core::marker::Sized
+pub fn ratatui::text::Masked<'a>::type_id(&self) -> core::any::TypeId
+pub fn ratatui::text::Masked<'a>::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::text::Masked<'a> where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::text::Masked<'a> where T: core::marker::Sized
+pub fn ratatui::text::Masked<'a>::borrow(&self) -> &T
+pub fn ratatui::text::Masked<'a>::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::text::Masked<'a> where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::text::Masked<'a> where T: core::marker::Sized
+pub fn ratatui::text::Masked<'a>::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::text::Masked<'a>::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::text::Masked<'a>
+impl<T> core::convert::From<T> for ratatui::text::Masked<'a>
+pub fn ratatui::text::Masked<'a>::from(t: T) -> T
+pub fn ratatui::text::Masked<'a>::from(t: T) -> T
+pub struct ratatui::prelude::text::Span<'a>
+pub ratatui::prelude::text::Span::content: alloc::borrow::Cow<'a, str>
+pub ratatui::prelude::text::Span::style: ratatui::style::Style
+impl<'a> ratatui::text::Span<'a>
+impl<'a> ratatui::text::Span<'a>
+pub fn ratatui::text::Span<'a>::patch_style(&mut self, style: ratatui::style::Style)
+pub fn ratatui::text::Span<'a>::patch_style(&mut self, style: ratatui::style::Style)
+pub fn ratatui::text::Span<'a>::raw<T>(content: T) -> ratatui::text::Span<'a> where T: core::convert::Into<alloc::borrow::Cow<'a, str>>
+pub fn ratatui::text::Span<'a>::raw<T>(content: T) -> ratatui::text::Span<'a> where T: core::convert::Into<alloc::borrow::Cow<'a, str>>
+pub fn ratatui::text::Span<'a>::reset_style(&mut self)
+pub fn ratatui::text::Span<'a>::reset_style(&mut self)
+pub fn ratatui::text::Span<'a>::styled<T>(content: T, style: ratatui::style::Style) -> ratatui::text::Span<'a> where T: core::convert::Into<alloc::borrow::Cow<'a, str>>
+pub fn ratatui::text::Span<'a>::styled<T>(content: T, style: ratatui::style::Style) -> ratatui::text::Span<'a> where T: core::convert::Into<alloc::borrow::Cow<'a, str>>
+pub fn ratatui::text::Span<'a>::styled_graphemes(&'a self, base_style: ratatui::style::Style) -> impl core::iter::traits::iterator::Iterator<Item = ratatui::text::StyledGrapheme<'a>>
+pub fn ratatui::text::Span<'a>::styled_graphemes(&'a self, base_style: ratatui::style::Style) -> impl core::iter::traits::iterator::Iterator<Item = ratatui::text::StyledGrapheme<'a>>
+pub fn ratatui::text::Span<'a>::width(&self) -> usize
+pub fn ratatui::text::Span<'a>::width(&self) -> usize
+impl<'a> core::convert::From<&'a str> for ratatui::text::Span<'a>
+impl<'a> core::convert::From<&'a str> for ratatui::text::Span<'a>
+pub fn ratatui::text::Span<'a>::from(s: &'a str) -> ratatui::text::Span<'a>
+pub fn ratatui::text::Span<'a>::from(s: &'a str) -> ratatui::text::Span<'a>
+impl<'a> core::convert::From<alloc::string::String> for ratatui::text::Span<'a>
+impl<'a> core::convert::From<alloc::string::String> for ratatui::text::Span<'a>
+pub fn ratatui::text::Span<'a>::from(s: alloc::string::String) -> ratatui::text::Span<'a>
+pub fn ratatui::text::Span<'a>::from(s: alloc::string::String) -> ratatui::text::Span<'a>
+impl<'a> core::convert::From<ratatui::text::Span<'a>> for ratatui::text::Spans<'a>
+impl<'a> core::convert::From<ratatui::text::Span<'a>> for ratatui::text::Spans<'a>
+impl<'a> core::convert::From<ratatui::text::Span<'a>> for ratatui::text::Spans<'a>
+pub fn ratatui::text::Spans<'a>::from(span: ratatui::text::Span<'a>) -> ratatui::text::Spans<'a>
+pub fn ratatui::text::Spans<'a>::from(span: ratatui::text::Span<'a>) -> ratatui::text::Spans<'a>
+pub fn ratatui::text::Spans<'a>::from(span: ratatui::text::Span<'a>) -> ratatui::text::Spans<'a>
+impl<'a> core::convert::From<ratatui::text::Span<'a>> for ratatui::text::Text<'a>
+impl<'a> core::convert::From<ratatui::text::Span<'a>> for ratatui::text::Text<'a>
+impl<'a> core::convert::From<ratatui::text::Span<'a>> for ratatui::text::Text<'a>
+impl<'a> core::convert::From<ratatui::text::Span<'a>> for ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(span: ratatui::text::Span<'a>) -> ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(span: ratatui::text::Span<'a>) -> ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(span: ratatui::text::Span<'a>) -> ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(span: ratatui::text::Span<'a>) -> ratatui::text::Text<'a>
+impl<'a> core::clone::Clone for ratatui::text::Span<'a>
+impl<'a> core::clone::Clone for ratatui::text::Span<'a>
+pub fn ratatui::text::Span<'a>::clone(&self) -> ratatui::text::Span<'a>
+pub fn ratatui::text::Span<'a>::clone(&self) -> ratatui::text::Span<'a>
+impl<'a> core::cmp::Eq for ratatui::text::Span<'a>
+impl<'a> core::cmp::Eq for ratatui::text::Span<'a>
+impl<'a> core::cmp::PartialEq<ratatui::text::Span<'a>> for ratatui::text::Span<'a>
+impl<'a> core::cmp::PartialEq<ratatui::text::Span<'a>> for ratatui::text::Span<'a>
+pub fn ratatui::text::Span<'a>::eq(&self, other: &ratatui::text::Span<'a>) -> bool
+pub fn ratatui::text::Span<'a>::eq(&self, other: &ratatui::text::Span<'a>) -> bool
+impl<'a> core::fmt::Debug for ratatui::text::Span<'a>
+impl<'a> core::fmt::Debug for ratatui::text::Span<'a>
+pub fn ratatui::text::Span<'a>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::text::Span<'a>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl<'a> core::marker::StructuralEq for ratatui::text::Span<'a>
+impl<'a> core::marker::StructuralEq for ratatui::text::Span<'a>
+impl<'a> core::marker::StructuralPartialEq for ratatui::text::Span<'a>
+impl<'a> core::marker::StructuralPartialEq for ratatui::text::Span<'a>
+impl<'a> core::marker::Send for ratatui::text::Span<'a>
+impl<'a> core::marker::Send for ratatui::text::Span<'a>
+impl<'a> core::marker::Sync for ratatui::text::Span<'a>
+impl<'a> core::marker::Sync for ratatui::text::Span<'a>
+impl<'a> core::marker::Unpin for ratatui::text::Span<'a>
+impl<'a> core::marker::Unpin for ratatui::text::Span<'a>
+impl<'a> core::panic::unwind_safe::RefUnwindSafe for ratatui::text::Span<'a>
+impl<'a> core::panic::unwind_safe::RefUnwindSafe for ratatui::text::Span<'a>
+impl<'a> core::panic::unwind_safe::UnwindSafe for ratatui::text::Span<'a>
+impl<'a> core::panic::unwind_safe::UnwindSafe for ratatui::text::Span<'a>
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for ratatui::text::Span<'a> where U: ratatui::style::Styled<Item = T>
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for ratatui::text::Span<'a> where U: ratatui::style::Styled<Item = T>
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for ratatui::text::Span<'a> where U: ratatui::style::Styled<Item = T>
+pub fn ratatui::text::Span<'a>::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::text::Span<'a>::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::text::Span<'a>::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::text::Span<'a>::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::text::Span<'a>::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::text::Span<'a>::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::text::Span<'a>::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn ratatui::text::Span<'a>::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn ratatui::text::Span<'a>::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn ratatui::text::Span<'a>::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::text::Span<'a>::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::text::Span<'a>::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::text::Span<'a>::reset(self) -> T
+pub fn ratatui::text::Span<'a>::reset(self) -> T
+pub fn ratatui::text::Span<'a>::reset(self) -> T
+impl<T, U> core::convert::Into<U> for ratatui::text::Span<'a> where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::text::Span<'a> where U: core::convert::From<T>
+pub fn ratatui::text::Span<'a>::into(self) -> U
+pub fn ratatui::text::Span<'a>::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::text::Span<'a> where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::text::Span<'a> where U: core::convert::Into<T>
+pub type ratatui::text::Span<'a>::Error = core::convert::Infallible
+pub type ratatui::text::Span<'a>::Error = core::convert::Infallible
+pub fn ratatui::text::Span<'a>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::text::Span<'a>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::text::Span<'a> where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::text::Span<'a> where U: core::convert::TryFrom<T>
+pub type ratatui::text::Span<'a>::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::text::Span<'a>::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::text::Span<'a>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::text::Span<'a>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::text::Span<'a> where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::text::Span<'a> where T: core::clone::Clone
+pub type ratatui::text::Span<'a>::Owned = T
+pub type ratatui::text::Span<'a>::Owned = T
+pub fn ratatui::text::Span<'a>::clone_into(&self, target: &mut T)
+pub fn ratatui::text::Span<'a>::clone_into(&self, target: &mut T)
+pub fn ratatui::text::Span<'a>::to_owned(&self) -> T
+pub fn ratatui::text::Span<'a>::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::text::Span<'a> where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::text::Span<'a> where T: 'static + core::marker::Sized
+pub fn ratatui::text::Span<'a>::type_id(&self) -> core::any::TypeId
+pub fn ratatui::text::Span<'a>::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::text::Span<'a> where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::text::Span<'a> where T: core::marker::Sized
+pub fn ratatui::text::Span<'a>::borrow(&self) -> &T
+pub fn ratatui::text::Span<'a>::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::text::Span<'a> where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::text::Span<'a> where T: core::marker::Sized
+pub fn ratatui::text::Span<'a>::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::text::Span<'a>::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::text::Span<'a>
+impl<T> core::convert::From<T> for ratatui::text::Span<'a>
+pub fn ratatui::text::Span<'a>::from(t: T) -> T
+pub fn ratatui::text::Span<'a>::from(t: T) -> T
+pub struct ratatui::prelude::text::Spans<'a>(pub alloc::vec::Vec<ratatui::text::Span<'a>>)
+impl<'a> ratatui::text::Spans<'a>
+pub fn ratatui::text::Spans<'a>::alignment(self, alignment: ratatui::layout::Alignment) -> ratatui::text::Line<'a>
+pub fn ratatui::text::Spans<'a>::patch_style(&mut self, style: ratatui::style::Style)
+pub fn ratatui::text::Spans<'a>::reset_style(&mut self)
+pub fn ratatui::text::Spans<'a>::width(&self) -> usize
+impl<'a> core::convert::From<&'a str> for ratatui::text::Spans<'a>
+pub fn ratatui::text::Spans<'a>::from(s: &'a str) -> ratatui::text::Spans<'a>
+impl<'a> core::convert::From<alloc::string::String> for ratatui::text::Spans<'a>
+pub fn ratatui::text::Spans<'a>::from(s: alloc::string::String) -> ratatui::text::Spans<'a>
+impl<'a> core::convert::From<alloc::vec::Vec<ratatui::text::Span<'a>, alloc::alloc::Global>> for ratatui::text::Spans<'a>
+pub fn ratatui::text::Spans<'a>::from(spans: alloc::vec::Vec<ratatui::text::Span<'a>>) -> ratatui::text::Spans<'a>
+impl<'a> core::convert::From<ratatui::text::Spans<'a>> for alloc::string::String
+pub fn alloc::string::String::from(line: ratatui::text::Spans<'a>) -> alloc::string::String
+impl<'a> core::convert::From<ratatui::text::Spans<'a>> for ratatui::text::Text<'a>
+impl<'a> core::convert::From<ratatui::text::Spans<'a>> for ratatui::text::Text<'a>
+impl<'a> core::convert::From<ratatui::text::Spans<'a>> for ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(spans: ratatui::text::Spans<'a>) -> ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(spans: ratatui::text::Spans<'a>) -> ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(spans: ratatui::text::Spans<'a>) -> ratatui::text::Text<'a>
+impl<'a> core::clone::Clone for ratatui::text::Spans<'a>
+pub fn ratatui::text::Spans<'a>::clone(&self) -> ratatui::text::Spans<'a>
+impl<'a> core::cmp::Eq for ratatui::text::Spans<'a>
+impl<'a> core::cmp::PartialEq<ratatui::text::Spans<'a>> for ratatui::text::Spans<'a>
+pub fn ratatui::text::Spans<'a>::eq(&self, other: &ratatui::text::Spans<'a>) -> bool
+impl<'a> core::default::Default for ratatui::text::Spans<'a>
+pub fn ratatui::text::Spans<'a>::default() -> ratatui::text::Spans<'a>
+impl<'a> core::fmt::Debug for ratatui::text::Spans<'a>
+pub fn ratatui::text::Spans<'a>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl<'a> core::marker::StructuralEq for ratatui::text::Spans<'a>
+impl<'a> core::marker::StructuralPartialEq for ratatui::text::Spans<'a>
+impl<'a> core::marker::Send for ratatui::text::Spans<'a>
+impl<'a> core::marker::Sync for ratatui::text::Spans<'a>
+impl<'a> core::marker::Unpin for ratatui::text::Spans<'a>
+impl<'a> core::panic::unwind_safe::RefUnwindSafe for ratatui::text::Spans<'a>
+impl<'a> core::panic::unwind_safe::UnwindSafe for ratatui::text::Spans<'a>
+impl<T, U> core::convert::Into<U> for ratatui::text::Spans<'a> where U: core::convert::From<T>
+pub fn ratatui::text::Spans<'a>::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::text::Spans<'a> where U: core::convert::Into<T>
+pub type ratatui::text::Spans<'a>::Error = core::convert::Infallible
+pub fn ratatui::text::Spans<'a>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::text::Spans<'a> where U: core::convert::TryFrom<T>
+pub type ratatui::text::Spans<'a>::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::text::Spans<'a>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::text::Spans<'a> where T: core::clone::Clone
+pub type ratatui::text::Spans<'a>::Owned = T
+pub fn ratatui::text::Spans<'a>::clone_into(&self, target: &mut T)
+pub fn ratatui::text::Spans<'a>::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::text::Spans<'a> where T: 'static + core::marker::Sized
+pub fn ratatui::text::Spans<'a>::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::text::Spans<'a> where T: core::marker::Sized
+pub fn ratatui::text::Spans<'a>::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::text::Spans<'a> where T: core::marker::Sized
+pub fn ratatui::text::Spans<'a>::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::text::Spans<'a>
+pub fn ratatui::text::Spans<'a>::from(t: T) -> T
+pub struct ratatui::prelude::text::StyledGrapheme<'a>
+pub ratatui::prelude::text::StyledGrapheme::style: ratatui::style::Style
+pub ratatui::prelude::text::StyledGrapheme::symbol: &'a str
+impl<'a> ratatui::text::StyledGrapheme<'a>
+impl<'a> ratatui::text::StyledGrapheme<'a>
+pub fn ratatui::text::StyledGrapheme<'a>::new(symbol: &'a str, style: ratatui::style::Style) -> ratatui::text::StyledGrapheme<'a>
+pub fn ratatui::text::StyledGrapheme<'a>::new(symbol: &'a str, style: ratatui::style::Style) -> ratatui::text::StyledGrapheme<'a>
+impl<'a> core::clone::Clone for ratatui::text::StyledGrapheme<'a>
+pub fn ratatui::text::StyledGrapheme<'a>::clone(&self) -> ratatui::text::StyledGrapheme<'a>
+impl<'a> core::cmp::Eq for ratatui::text::StyledGrapheme<'a>
+impl<'a> core::cmp::PartialEq<ratatui::text::StyledGrapheme<'a>> for ratatui::text::StyledGrapheme<'a>
+pub fn ratatui::text::StyledGrapheme<'a>::eq(&self, other: &ratatui::text::StyledGrapheme<'a>) -> bool
+impl<'a> core::fmt::Debug for ratatui::text::StyledGrapheme<'a>
+pub fn ratatui::text::StyledGrapheme<'a>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl<'a> core::marker::StructuralEq for ratatui::text::StyledGrapheme<'a>
+impl<'a> core::marker::StructuralPartialEq for ratatui::text::StyledGrapheme<'a>
+impl<'a> core::marker::Send for ratatui::text::StyledGrapheme<'a>
+impl<'a> core::marker::Sync for ratatui::text::StyledGrapheme<'a>
+impl<'a> core::marker::Unpin for ratatui::text::StyledGrapheme<'a>
+impl<'a> core::panic::unwind_safe::RefUnwindSafe for ratatui::text::StyledGrapheme<'a>
+impl<'a> core::panic::unwind_safe::UnwindSafe for ratatui::text::StyledGrapheme<'a>
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for ratatui::text::StyledGrapheme<'a> where U: ratatui::style::Styled<Item = T>
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for ratatui::text::StyledGrapheme<'a> where U: ratatui::style::Styled<Item = T>
+pub fn ratatui::text::StyledGrapheme<'a>::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::text::StyledGrapheme<'a>::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::text::StyledGrapheme<'a>::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::text::StyledGrapheme<'a>::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::text::StyledGrapheme<'a>::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn ratatui::text::StyledGrapheme<'a>::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn ratatui::text::StyledGrapheme<'a>::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::text::StyledGrapheme<'a>::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::text::StyledGrapheme<'a>::reset(self) -> T
+pub fn ratatui::text::StyledGrapheme<'a>::reset(self) -> T
+impl<T, U> core::convert::Into<U> for ratatui::text::StyledGrapheme<'a> where U: core::convert::From<T>
+pub fn ratatui::text::StyledGrapheme<'a>::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::text::StyledGrapheme<'a> where U: core::convert::Into<T>
+pub type ratatui::text::StyledGrapheme<'a>::Error = core::convert::Infallible
+pub fn ratatui::text::StyledGrapheme<'a>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::text::StyledGrapheme<'a> where U: core::convert::TryFrom<T>
+pub type ratatui::text::StyledGrapheme<'a>::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::text::StyledGrapheme<'a>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::text::StyledGrapheme<'a> where T: core::clone::Clone
+pub type ratatui::text::StyledGrapheme<'a>::Owned = T
+pub fn ratatui::text::StyledGrapheme<'a>::clone_into(&self, target: &mut T)
+pub fn ratatui::text::StyledGrapheme<'a>::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::text::StyledGrapheme<'a> where T: 'static + core::marker::Sized
+pub fn ratatui::text::StyledGrapheme<'a>::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::text::StyledGrapheme<'a> where T: core::marker::Sized
+pub fn ratatui::text::StyledGrapheme<'a>::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::text::StyledGrapheme<'a> where T: core::marker::Sized
+pub fn ratatui::text::StyledGrapheme<'a>::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::text::StyledGrapheme<'a>
+pub fn ratatui::text::StyledGrapheme<'a>::from(t: T) -> T
+pub struct ratatui::prelude::text::Text<'a>
+pub ratatui::prelude::text::Text::lines: alloc::vec::Vec<ratatui::text::Line<'a>>
+impl<'a> ratatui::text::Text<'a>
+impl<'a> ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::height(&self) -> usize
+pub fn ratatui::text::Text<'a>::height(&self) -> usize
+pub fn ratatui::text::Text<'a>::patch_style(&mut self, style: ratatui::style::Style)
+pub fn ratatui::text::Text<'a>::patch_style(&mut self, style: ratatui::style::Style)
+pub fn ratatui::text::Text<'a>::raw<T>(content: T) -> ratatui::text::Text<'a> where T: core::convert::Into<alloc::borrow::Cow<'a, str>>
+pub fn ratatui::text::Text<'a>::raw<T>(content: T) -> ratatui::text::Text<'a> where T: core::convert::Into<alloc::borrow::Cow<'a, str>>
+pub fn ratatui::text::Text<'a>::reset_style(&mut self)
+pub fn ratatui::text::Text<'a>::reset_style(&mut self)
+pub fn ratatui::text::Text<'a>::styled<T>(content: T, style: ratatui::style::Style) -> ratatui::text::Text<'a> where T: core::convert::Into<alloc::borrow::Cow<'a, str>>
+pub fn ratatui::text::Text<'a>::styled<T>(content: T, style: ratatui::style::Style) -> ratatui::text::Text<'a> where T: core::convert::Into<alloc::borrow::Cow<'a, str>>
+pub fn ratatui::text::Text<'a>::width(&self) -> usize
+pub fn ratatui::text::Text<'a>::width(&self) -> usize
+impl<'a, T> core::iter::traits::collect::Extend<T> for ratatui::text::Text<'a> where T: core::convert::Into<ratatui::text::Line<'a>>
+impl<'a, T> core::iter::traits::collect::Extend<T> for ratatui::text::Text<'a> where T: core::convert::Into<ratatui::text::Line<'a>>
+pub fn ratatui::text::Text<'a>::extend<I: core::iter::traits::collect::IntoIterator<Item = T>>(&mut self, iter: I)
+pub fn ratatui::text::Text<'a>::extend<I: core::iter::traits::collect::IntoIterator<Item = T>>(&mut self, iter: I)
+impl<'a> core::convert::From<&'a str> for ratatui::text::Text<'a>
+impl<'a> core::convert::From<&'a str> for ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(s: &'a str) -> ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(s: &'a str) -> ratatui::text::Text<'a>
+impl<'a> core::convert::From<alloc::borrow::Cow<'a, str>> for ratatui::text::Text<'a>
+impl<'a> core::convert::From<alloc::borrow::Cow<'a, str>> for ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(s: alloc::borrow::Cow<'a, str>) -> ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(s: alloc::borrow::Cow<'a, str>) -> ratatui::text::Text<'a>
+impl<'a> core::convert::From<alloc::string::String> for ratatui::text::Text<'a>
+impl<'a> core::convert::From<alloc::string::String> for ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(s: alloc::string::String) -> ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(s: alloc::string::String) -> ratatui::text::Text<'a>
+impl<'a> core::convert::From<alloc::vec::Vec<ratatui::text::Line<'a>, alloc::alloc::Global>> for ratatui::text::Text<'a>
+impl<'a> core::convert::From<alloc::vec::Vec<ratatui::text::Line<'a>, alloc::alloc::Global>> for ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(lines: alloc::vec::Vec<ratatui::text::Line<'a>>) -> ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(lines: alloc::vec::Vec<ratatui::text::Line<'a>>) -> ratatui::text::Text<'a>
+impl<'a> core::convert::From<alloc::vec::Vec<ratatui::text::Spans<'a>, alloc::alloc::Global>> for ratatui::text::Text<'a>
+impl<'a> core::convert::From<alloc::vec::Vec<ratatui::text::Spans<'a>, alloc::alloc::Global>> for ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(lines: alloc::vec::Vec<ratatui::text::Spans<'a>>) -> ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(lines: alloc::vec::Vec<ratatui::text::Spans<'a>>) -> ratatui::text::Text<'a>
+impl<'a> core::iter::traits::collect::IntoIterator for ratatui::text::Text<'a>
+impl<'a> core::iter::traits::collect::IntoIterator for ratatui::text::Text<'a>
+pub type ratatui::text::Text<'a>::IntoIter = alloc::vec::into_iter::IntoIter<<ratatui::text::Text<'a> as core::iter::traits::collect::IntoIterator>::Item, alloc::alloc::Global>
+pub type ratatui::text::Text<'a>::IntoIter = alloc::vec::into_iter::IntoIter<<ratatui::text::Text<'a> as core::iter::traits::collect::IntoIterator>::Item, alloc::alloc::Global>
+pub type ratatui::text::Text<'a>::Item = ratatui::text::Line<'a>
+pub type ratatui::text::Text<'a>::Item = ratatui::text::Line<'a>
+pub fn ratatui::text::Text<'a>::into_iter(self) -> Self::IntoIter
+pub fn ratatui::text::Text<'a>::into_iter(self) -> Self::IntoIter
+impl<'a> core::clone::Clone for ratatui::text::Text<'a>
+impl<'a> core::clone::Clone for ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::clone(&self) -> ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::clone(&self) -> ratatui::text::Text<'a>
+impl<'a> core::cmp::Eq for ratatui::text::Text<'a>
+impl<'a> core::cmp::Eq for ratatui::text::Text<'a>
+impl<'a> core::cmp::PartialEq<ratatui::text::Text<'a>> for ratatui::text::Text<'a>
+impl<'a> core::cmp::PartialEq<ratatui::text::Text<'a>> for ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::eq(&self, other: &ratatui::text::Text<'a>) -> bool
+pub fn ratatui::text::Text<'a>::eq(&self, other: &ratatui::text::Text<'a>) -> bool
+impl<'a> core::default::Default for ratatui::text::Text<'a>
+impl<'a> core::default::Default for ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::default() -> ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::default() -> ratatui::text::Text<'a>
+impl<'a> core::fmt::Debug for ratatui::text::Text<'a>
+impl<'a> core::fmt::Debug for ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::text::Text<'a>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl<'a> core::marker::StructuralEq for ratatui::text::Text<'a>
+impl<'a> core::marker::StructuralEq for ratatui::text::Text<'a>
+impl<'a> core::marker::StructuralPartialEq for ratatui::text::Text<'a>
+impl<'a> core::marker::StructuralPartialEq for ratatui::text::Text<'a>
+impl<'a> core::marker::Send for ratatui::text::Text<'a>
+impl<'a> core::marker::Send for ratatui::text::Text<'a>
+impl<'a> core::marker::Sync for ratatui::text::Text<'a>
+impl<'a> core::marker::Sync for ratatui::text::Text<'a>
+impl<'a> core::marker::Unpin for ratatui::text::Text<'a>
+impl<'a> core::marker::Unpin for ratatui::text::Text<'a>
+impl<'a> core::panic::unwind_safe::RefUnwindSafe for ratatui::text::Text<'a>
+impl<'a> core::panic::unwind_safe::RefUnwindSafe for ratatui::text::Text<'a>
+impl<'a> core::panic::unwind_safe::UnwindSafe for ratatui::text::Text<'a>
+impl<'a> core::panic::unwind_safe::UnwindSafe for ratatui::text::Text<'a>
+impl<T, U> core::convert::Into<U> for ratatui::text::Text<'a> where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::text::Text<'a> where U: core::convert::From<T>
+pub fn ratatui::text::Text<'a>::into(self) -> U
+pub fn ratatui::text::Text<'a>::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::text::Text<'a> where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::text::Text<'a> where U: core::convert::Into<T>
+pub type ratatui::text::Text<'a>::Error = core::convert::Infallible
+pub type ratatui::text::Text<'a>::Error = core::convert::Infallible
+pub fn ratatui::text::Text<'a>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::text::Text<'a>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::text::Text<'a> where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::text::Text<'a> where U: core::convert::TryFrom<T>
+pub type ratatui::text::Text<'a>::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::text::Text<'a>::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::text::Text<'a>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::text::Text<'a>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::text::Text<'a> where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::text::Text<'a> where T: core::clone::Clone
+pub type ratatui::text::Text<'a>::Owned = T
+pub type ratatui::text::Text<'a>::Owned = T
+pub fn ratatui::text::Text<'a>::clone_into(&self, target: &mut T)
+pub fn ratatui::text::Text<'a>::clone_into(&self, target: &mut T)
+pub fn ratatui::text::Text<'a>::to_owned(&self) -> T
+pub fn ratatui::text::Text<'a>::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::text::Text<'a> where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::text::Text<'a> where T: 'static + core::marker::Sized
+pub fn ratatui::text::Text<'a>::type_id(&self) -> core::any::TypeId
+pub fn ratatui::text::Text<'a>::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::text::Text<'a> where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::text::Text<'a> where T: core::marker::Sized
+pub fn ratatui::text::Text<'a>::borrow(&self) -> &T
+pub fn ratatui::text::Text<'a>::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::text::Text<'a> where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::text::Text<'a> where T: core::marker::Sized
+pub fn ratatui::text::Text<'a>::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::text::Text<'a>::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::text::Text<'a>
+impl<T> core::convert::From<T> for ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(t: T) -> T
+pub fn ratatui::text::Text<'a>::from(t: T) -> T
+pub enum ratatui::prelude::Alignment
+pub ratatui::prelude::Alignment::Center
+pub ratatui::prelude::Alignment::Left
+pub ratatui::prelude::Alignment::Right
+pub enum ratatui::prelude::Color
+pub ratatui::prelude::Color::Black
+pub ratatui::prelude::Color::Blue
+pub ratatui::prelude::Color::Cyan
+pub ratatui::prelude::Color::DarkGray
+pub ratatui::prelude::Color::Gray
+pub ratatui::prelude::Color::Green
+pub ratatui::prelude::Color::Indexed(u8)
+pub ratatui::prelude::Color::LightBlue
+pub ratatui::prelude::Color::LightCyan
+pub ratatui::prelude::Color::LightGreen
+pub ratatui::prelude::Color::LightMagenta
+pub ratatui::prelude::Color::LightRed
+pub ratatui::prelude::Color::LightYellow
+pub ratatui::prelude::Color::Magenta
+pub ratatui::prelude::Color::Red
+pub ratatui::prelude::Color::Reset
+pub ratatui::prelude::Color::Rgb(u8, u8, u8)
+pub ratatui::prelude::Color::White
+pub ratatui::prelude::Color::Yellow
+pub enum ratatui::prelude::Constraint
+pub ratatui::prelude::Constraint::Length(u16)
+pub ratatui::prelude::Constraint::Max(u16)
+pub ratatui::prelude::Constraint::Min(u16)
+pub ratatui::prelude::Constraint::Percentage(u16)
+pub ratatui::prelude::Constraint::Ratio(u32, u32)
+pub enum ratatui::prelude::Corner
+pub ratatui::prelude::Corner::BottomLeft
+pub ratatui::prelude::Corner::BottomRight
+pub ratatui::prelude::Corner::TopLeft
+pub ratatui::prelude::Corner::TopRight
+pub enum ratatui::prelude::Direction
+pub ratatui::prelude::Direction::Horizontal
+pub ratatui::prelude::Direction::Vertical
+pub enum ratatui::prelude::Marker
+pub ratatui::prelude::Marker::Bar
+pub ratatui::prelude::Marker::Block
+pub ratatui::prelude::Marker::Braille
+pub ratatui::prelude::Marker::Dot
+pub enum ratatui::prelude::Viewport
+pub ratatui::prelude::Viewport::Fixed(ratatui::layout::Rect)
+pub ratatui::prelude::Viewport::Fullscreen
+pub ratatui::prelude::Viewport::Inline(u16)
+pub struct ratatui::prelude::Buffer
+pub ratatui::prelude::Buffer::area: ratatui::layout::Rect
+pub ratatui::prelude::Buffer::content: alloc::vec::Vec<ratatui::buffer::Cell>
+pub struct ratatui::prelude::CrosstermBackend<W: std::io::Write>
+pub struct ratatui::prelude::Frame<'a, B> where B: ratatui::backend::Backend + 'a
+pub struct ratatui::prelude::Layout
+pub struct ratatui::prelude::Line<'a>
+pub ratatui::prelude::Line::alignment: core::option::Option<ratatui::layout::Alignment>
+pub ratatui::prelude::Line::spans: alloc::vec::Vec<ratatui::text::Span<'a>>
+pub struct ratatui::prelude::Margin
+pub ratatui::prelude::Margin::horizontal: u16
+pub ratatui::prelude::Margin::vertical: u16
+pub struct ratatui::prelude::Masked<'a>
+pub struct ratatui::prelude::Modifier(_)
+pub struct ratatui::prelude::Rect
+pub ratatui::prelude::Rect::height: u16
+pub ratatui::prelude::Rect::width: u16
+pub ratatui::prelude::Rect::x: u16
+pub ratatui::prelude::Rect::y: u16
+pub struct ratatui::prelude::Span<'a>
+pub ratatui::prelude::Span::content: alloc::borrow::Cow<'a, str>
+pub ratatui::prelude::Span::style: ratatui::style::Style
+pub struct ratatui::prelude::Style
+pub ratatui::prelude::Style::add_modifier: ratatui::style::Modifier
+pub ratatui::prelude::Style::bg: core::option::Option<ratatui::style::Color>
+pub ratatui::prelude::Style::fg: core::option::Option<ratatui::style::Color>
+pub ratatui::prelude::Style::sub_modifier: ratatui::style::Modifier
+pub ratatui::prelude::Style::underline_color: core::option::Option<ratatui::style::Color>
+pub struct ratatui::prelude::Terminal<B> where B: ratatui::backend::Backend
+pub struct ratatui::prelude::TerminalOptions
+pub ratatui::prelude::TerminalOptions::viewport: ratatui::terminal::Viewport
+pub struct ratatui::prelude::Text<'a>
+pub ratatui::prelude::Text::lines: alloc::vec::Vec<ratatui::text::Line<'a>>
+pub trait ratatui::prelude::Backend
+pub fn ratatui::prelude::Backend::append_lines(&mut self, _n: u16) -> std::io::error::Result<()>
+pub fn ratatui::prelude::Backend::clear(&mut self) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::prelude::Backend::clear_region(&mut self, clear_type: ratatui::backend::ClearType) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::prelude::Backend::draw<'a, I>(&mut self, content: I) -> core::result::Result<(), std::io::error::Error> where I: core::iter::traits::iterator::Iterator<Item = (u16, u16, &'a ratatui::buffer::Cell)>
+pub fn ratatui::prelude::Backend::flush(&mut self) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::prelude::Backend::get_cursor(&mut self) -> core::result::Result<(u16, u16), std::io::error::Error>
+pub fn ratatui::prelude::Backend::hide_cursor(&mut self) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::prelude::Backend::set_cursor(&mut self, x: u16, y: u16) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::prelude::Backend::show_cursor(&mut self) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::prelude::Backend::size(&self) -> core::result::Result<ratatui::layout::Rect, std::io::error::Error>
+pub trait ratatui::prelude::Styled
+pub type ratatui::prelude::Styled::Item
+pub fn ratatui::prelude::Styled::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::prelude::Styled::style(&self) -> ratatui::style::Style
+pub trait ratatui::prelude::Stylize<'a, T>: core::marker::Sized
+pub fn ratatui::prelude::Stylize::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::prelude::Stylize::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::prelude::Stylize::black(self) -> T
+pub fn ratatui::prelude::Stylize::blue(self) -> T
+pub fn ratatui::prelude::Stylize::bold(self) -> T
+pub fn ratatui::prelude::Stylize::crossed_out(self) -> T
+pub fn ratatui::prelude::Stylize::cyan(self) -> T
+pub fn ratatui::prelude::Stylize::dark_gray(self) -> T
+pub fn ratatui::prelude::Stylize::dim(self) -> T
+pub fn ratatui::prelude::Stylize::fg<S: core::convert::Into<ratatui::style::Color>>(self, color: S) -> T
+pub fn ratatui::prelude::Stylize::gray(self) -> T
+pub fn ratatui::prelude::Stylize::green(self) -> T
+pub fn ratatui::prelude::Stylize::hidden(self) -> T
+pub fn ratatui::prelude::Stylize::italic(self) -> T
+pub fn ratatui::prelude::Stylize::light_blue(self) -> T
+pub fn ratatui::prelude::Stylize::light_cyan(self) -> T
+pub fn ratatui::prelude::Stylize::light_green(self) -> T
+pub fn ratatui::prelude::Stylize::light_magenta(self) -> T
+pub fn ratatui::prelude::Stylize::light_red(self) -> T
+pub fn ratatui::prelude::Stylize::light_yellow(self) -> T
+pub fn ratatui::prelude::Stylize::magenta(self) -> T
+pub fn ratatui::prelude::Stylize::not_bold(self) -> T
+pub fn ratatui::prelude::Stylize::not_crossed_out(self) -> T
+pub fn ratatui::prelude::Stylize::not_dim(self) -> T
+pub fn ratatui::prelude::Stylize::not_hidden(self) -> T
+pub fn ratatui::prelude::Stylize::not_italic(self) -> T
+pub fn ratatui::prelude::Stylize::not_rapid_blink(self) -> T
+pub fn ratatui::prelude::Stylize::not_reversed(self) -> T
+pub fn ratatui::prelude::Stylize::not_slow_blink(self) -> T
+pub fn ratatui::prelude::Stylize::not_underlined(self) -> T
+pub fn ratatui::prelude::Stylize::on_black(self) -> T
+pub fn ratatui::prelude::Stylize::on_blue(self) -> T
+pub fn ratatui::prelude::Stylize::on_cyan(self) -> T
+pub fn ratatui::prelude::Stylize::on_dark_gray(self) -> T
+pub fn ratatui::prelude::Stylize::on_gray(self) -> T
+pub fn ratatui::prelude::Stylize::on_green(self) -> T
+pub fn ratatui::prelude::Stylize::on_light_blue(self) -> T
+pub fn ratatui::prelude::Stylize::on_light_cyan(self) -> T
+pub fn ratatui::prelude::Stylize::on_light_green(self) -> T
+pub fn ratatui::prelude::Stylize::on_light_magenta(self) -> T
+pub fn ratatui::prelude::Stylize::on_light_red(self) -> T
+pub fn ratatui::prelude::Stylize::on_light_yellow(self) -> T
+pub fn ratatui::prelude::Stylize::on_magenta(self) -> T
+pub fn ratatui::prelude::Stylize::on_red(self) -> T
+pub fn ratatui::prelude::Stylize::on_white(self) -> T
+pub fn ratatui::prelude::Stylize::on_yellow(self) -> T
+pub fn ratatui::prelude::Stylize::rapid_blink(self) -> T
+pub fn ratatui::prelude::Stylize::red(self) -> T
+pub fn ratatui::prelude::Stylize::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::prelude::Stylize::reset(self) -> T
+pub fn ratatui::prelude::Stylize::reversed(self) -> T
+pub fn ratatui::prelude::Stylize::slow_blink(self) -> T
+pub fn ratatui::prelude::Stylize::underlined(self) -> T
+pub fn ratatui::prelude::Stylize::white(self) -> T
+pub fn ratatui::prelude::Stylize::yellow(self) -> T
+pub ratatui::style::Style::underline_color: core::option::Option<ratatui::style::Color>
+pub trait ratatui::style::Styled
+pub type ratatui::style::Styled::Item
+pub fn ratatui::style::Styled::set_style(self, style: ratatui::style::Style) -> Self::Item
+pub fn ratatui::style::Styled::style(&self) -> ratatui::style::Style
+pub trait ratatui::style::Stylize<'a, T>: core::marker::Sized
+pub fn ratatui::style::Stylize::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::style::Stylize::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::style::Stylize::black(self) -> T
+pub fn ratatui::style::Stylize::blue(self) -> T
+pub fn ratatui::style::Stylize::bold(self) -> T
+pub fn ratatui::style::Stylize::crossed_out(self) -> T
+pub fn ratatui::style::Stylize::cyan(self) -> T
+pub fn ratatui::style::Stylize::dark_gray(self) -> T
+pub fn ratatui::style::Stylize::dim(self) -> T
+pub fn ratatui::style::Stylize::fg<S: core::convert::Into<ratatui::style::Color>>(self, color: S) -> T
+pub fn ratatui::style::Stylize::gray(self) -> T
+pub fn ratatui::style::Stylize::green(self) -> T
+pub fn ratatui::style::Stylize::hidden(self) -> T
+pub fn ratatui::style::Stylize::italic(self) -> T
+pub fn ratatui::style::Stylize::light_blue(self) -> T
+pub fn ratatui::style::Stylize::light_cyan(self) -> T
+pub fn ratatui::style::Stylize::light_green(self) -> T
+pub fn ratatui::style::Stylize::light_magenta(self) -> T
+pub fn ratatui::style::Stylize::light_red(self) -> T
+pub fn ratatui::style::Stylize::light_yellow(self) -> T
+pub fn ratatui::style::Stylize::magenta(self) -> T
+pub fn ratatui::style::Stylize::not_bold(self) -> T
+pub fn ratatui::style::Stylize::not_crossed_out(self) -> T
+pub fn ratatui::style::Stylize::not_dim(self) -> T
+pub fn ratatui::style::Stylize::not_hidden(self) -> T
+pub fn ratatui::style::Stylize::not_italic(self) -> T
+pub fn ratatui::style::Stylize::not_rapid_blink(self) -> T
+pub fn ratatui::style::Stylize::not_reversed(self) -> T
+pub fn ratatui::style::Stylize::not_slow_blink(self) -> T
+pub fn ratatui::style::Stylize::not_underlined(self) -> T
+pub fn ratatui::style::Stylize::on_black(self) -> T
+pub fn ratatui::style::Stylize::on_blue(self) -> T
+pub fn ratatui::style::Stylize::on_cyan(self) -> T
+pub fn ratatui::style::Stylize::on_dark_gray(self) -> T
+pub fn ratatui::style::Stylize::on_gray(self) -> T
+pub fn ratatui::style::Stylize::on_green(self) -> T
+pub fn ratatui::style::Stylize::on_light_blue(self) -> T
+pub fn ratatui::style::Stylize::on_light_cyan(self) -> T
+pub fn ratatui::style::Stylize::on_light_green(self) -> T
+pub fn ratatui::style::Stylize::on_light_magenta(self) -> T
+pub fn ratatui::style::Stylize::on_light_red(self) -> T
+pub fn ratatui::style::Stylize::on_light_yellow(self) -> T
+pub fn ratatui::style::Stylize::on_magenta(self) -> T
+pub fn ratatui::style::Stylize::on_red(self) -> T
+pub fn ratatui::style::Stylize::on_white(self) -> T
+pub fn ratatui::style::Stylize::on_yellow(self) -> T
+pub fn ratatui::style::Stylize::rapid_blink(self) -> T
+pub fn ratatui::style::Stylize::red(self) -> T
+pub fn ratatui::style::Stylize::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::style::Stylize::reset(self) -> T
+pub fn ratatui::style::Stylize::reversed(self) -> T
+pub fn ratatui::style::Stylize::slow_blink(self) -> T
+pub fn ratatui::style::Stylize::underlined(self) -> T
+pub fn ratatui::style::Stylize::white(self) -> T
+pub fn ratatui::style::Stylize::yellow(self) -> T
+pub mod ratatui::widgets::block
+pub mod ratatui::widgets::block::title
+pub enum ratatui::widgets::block::title::Position
+pub ratatui::widgets::block::title::Position::Bottom
+pub ratatui::widgets::block::title::Position::Top
+impl core::clone::Clone for ratatui::widgets::block::title::Position
+impl core::clone::Clone for ratatui::widgets::block::title::Position
+pub fn ratatui::widgets::block::title::Position::clone(&self) -> ratatui::widgets::block::title::Position
+pub fn ratatui::widgets::block::title::Position::clone(&self) -> ratatui::widgets::block::title::Position
+impl core::cmp::Eq for ratatui::widgets::block::title::Position
+impl core::cmp::Eq for ratatui::widgets::block::title::Position
+impl core::cmp::PartialEq<ratatui::widgets::block::title::Position> for ratatui::widgets::block::title::Position
+impl core::cmp::PartialEq<ratatui::widgets::block::title::Position> for ratatui::widgets::block::title::Position
+pub fn ratatui::widgets::block::title::Position::eq(&self, other: &ratatui::widgets::block::title::Position) -> bool
+pub fn ratatui::widgets::block::title::Position::eq(&self, other: &ratatui::widgets::block::title::Position) -> bool
+impl core::default::Default for ratatui::widgets::block::title::Position
+impl core::default::Default for ratatui::widgets::block::title::Position
+pub fn ratatui::widgets::block::title::Position::default() -> ratatui::widgets::block::title::Position
+pub fn ratatui::widgets::block::title::Position::default() -> ratatui::widgets::block::title::Position
+impl core::fmt::Debug for ratatui::widgets::block::title::Position
+impl core::fmt::Debug for ratatui::widgets::block::title::Position
+pub fn ratatui::widgets::block::title::Position::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::widgets::block::title::Position::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::hash::Hash for ratatui::widgets::block::title::Position
+impl core::hash::Hash for ratatui::widgets::block::title::Position
+pub fn ratatui::widgets::block::title::Position::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
+pub fn ratatui::widgets::block::title::Position::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
+impl core::marker::Copy for ratatui::widgets::block::title::Position
+impl core::marker::Copy for ratatui::widgets::block::title::Position
+impl core::marker::StructuralEq for ratatui::widgets::block::title::Position
+impl core::marker::StructuralEq for ratatui::widgets::block::title::Position
+impl core::marker::StructuralPartialEq for ratatui::widgets::block::title::Position
+impl core::marker::StructuralPartialEq for ratatui::widgets::block::title::Position
+impl core::marker::Send for ratatui::widgets::block::title::Position
+impl core::marker::Send for ratatui::widgets::block::title::Position
+impl core::marker::Sync for ratatui::widgets::block::title::Position
+impl core::marker::Sync for ratatui::widgets::block::title::Position
+impl core::marker::Unpin for ratatui::widgets::block::title::Position
+impl core::marker::Unpin for ratatui::widgets::block::title::Position
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::block::title::Position
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::block::title::Position
+impl core::panic::unwind_safe::UnwindSafe for ratatui::widgets::block::title::Position
+impl core::panic::unwind_safe::UnwindSafe for ratatui::widgets::block::title::Position
+impl<T, U> core::convert::Into<U> for ratatui::widgets::block::title::Position where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::widgets::block::title::Position where U: core::convert::From<T>
+pub fn ratatui::widgets::block::title::Position::into(self) -> U
+pub fn ratatui::widgets::block::title::Position::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::block::title::Position where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::block::title::Position where U: core::convert::Into<T>
+pub type ratatui::widgets::block::title::Position::Error = core::convert::Infallible
+pub type ratatui::widgets::block::title::Position::Error = core::convert::Infallible
+pub fn ratatui::widgets::block::title::Position::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::widgets::block::title::Position::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::widgets::block::title::Position where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::widgets::block::title::Position where U: core::convert::TryFrom<T>
+pub type ratatui::widgets::block::title::Position::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::widgets::block::title::Position::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::widgets::block::title::Position::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::widgets::block::title::Position::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::widgets::block::title::Position where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::widgets::block::title::Position where T: core::clone::Clone
+pub type ratatui::widgets::block::title::Position::Owned = T
+pub type ratatui::widgets::block::title::Position::Owned = T
+pub fn ratatui::widgets::block::title::Position::clone_into(&self, target: &mut T)
+pub fn ratatui::widgets::block::title::Position::clone_into(&self, target: &mut T)
+pub fn ratatui::widgets::block::title::Position::to_owned(&self) -> T
+pub fn ratatui::widgets::block::title::Position::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::widgets::block::title::Position where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::widgets::block::title::Position where T: 'static + core::marker::Sized
+pub fn ratatui::widgets::block::title::Position::type_id(&self) -> core::any::TypeId
+pub fn ratatui::widgets::block::title::Position::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::widgets::block::title::Position where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::widgets::block::title::Position where T: core::marker::Sized
+pub fn ratatui::widgets::block::title::Position::borrow(&self) -> &T
+pub fn ratatui::widgets::block::title::Position::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::block::title::Position where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::block::title::Position where T: core::marker::Sized
+pub fn ratatui::widgets::block::title::Position::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::widgets::block::title::Position::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::widgets::block::title::Position
+impl<T> core::convert::From<T> for ratatui::widgets::block::title::Position
+pub fn ratatui::widgets::block::title::Position::from(t: T) -> T
+pub fn ratatui::widgets::block::title::Position::from(t: T) -> T
+pub struct ratatui::widgets::block::title::Title<'a>
+pub ratatui::widgets::block::title::Title::alignment: core::option::Option<ratatui::layout::Alignment>
+pub ratatui::widgets::block::title::Title::content: ratatui::text::Line<'a>
+pub ratatui::widgets::block::title::Title::position: core::option::Option<ratatui::widgets::block::title::Position>
+impl<'a> ratatui::widgets::block::title::Title<'a>
+impl<'a> ratatui::widgets::block::title::Title<'a>
+pub fn ratatui::widgets::block::title::Title<'a>::alignment(self, alignment: ratatui::layout::Alignment) -> ratatui::widgets::block::title::Title<'a>
+pub fn ratatui::widgets::block::title::Title<'a>::alignment(self, alignment: ratatui::layout::Alignment) -> ratatui::widgets::block::title::Title<'a>
+pub fn ratatui::widgets::block::title::Title<'a>::content<T>(self, content: T) -> ratatui::widgets::block::title::Title<'a> where T: core::convert::Into<ratatui::text::Line<'a>>
+pub fn ratatui::widgets::block::title::Title<'a>::content<T>(self, content: T) -> ratatui::widgets::block::title::Title<'a> where T: core::convert::Into<ratatui::text::Line<'a>>
+pub fn ratatui::widgets::block::title::Title<'a>::position(self, position: ratatui::widgets::block::title::Position) -> ratatui::widgets::block::title::Title<'a>
+pub fn ratatui::widgets::block::title::Title<'a>::position(self, position: ratatui::widgets::block::title::Position) -> ratatui::widgets::block::title::Title<'a>
+impl<'a, T> core::convert::From<T> for ratatui::widgets::block::title::Title<'a> where T: core::convert::Into<ratatui::text::Line<'a>>
+impl<'a, T> core::convert::From<T> for ratatui::widgets::block::title::Title<'a> where T: core::convert::Into<ratatui::text::Line<'a>>
+pub fn ratatui::widgets::block::title::Title<'a>::from(value: T) -> Self
+pub fn ratatui::widgets::block::title::Title<'a>::from(value: T) -> Self
+impl<'a> core::default::Default for ratatui::widgets::block::title::Title<'a>
+impl<'a> core::default::Default for ratatui::widgets::block::title::Title<'a>
+pub fn ratatui::widgets::block::title::Title<'a>::default() -> Self
+pub fn ratatui::widgets::block::title::Title<'a>::default() -> Self
+impl<'a> core::clone::Clone for ratatui::widgets::block::title::Title<'a>
+impl<'a> core::clone::Clone for ratatui::widgets::block::title::Title<'a>
+pub fn ratatui::widgets::block::title::Title<'a>::clone(&self) -> ratatui::widgets::block::title::Title<'a>
+pub fn ratatui::widgets::block::title::Title<'a>::clone(&self) -> ratatui::widgets::block::title::Title<'a>
+impl<'a> core::cmp::Eq for ratatui::widgets::block::title::Title<'a>
+impl<'a> core::cmp::Eq for ratatui::widgets::block::title::Title<'a>
+impl<'a> core::cmp::PartialEq<ratatui::widgets::block::title::Title<'a>> for ratatui::widgets::block::title::Title<'a>
+impl<'a> core::cmp::PartialEq<ratatui::widgets::block::title::Title<'a>> for ratatui::widgets::block::title::Title<'a>
+pub fn ratatui::widgets::block::title::Title<'a>::eq(&self, other: &ratatui::widgets::block::title::Title<'a>) -> bool
+pub fn ratatui::widgets::block::title::Title<'a>::eq(&self, other: &ratatui::widgets::block::title::Title<'a>) -> bool
+impl<'a> core::fmt::Debug for ratatui::widgets::block::title::Title<'a>
+impl<'a> core::fmt::Debug for ratatui::widgets::block::title::Title<'a>
+pub fn ratatui::widgets::block::title::Title<'a>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::widgets::block::title::Title<'a>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl<'a> core::marker::StructuralEq for ratatui::widgets::block::title::Title<'a>
+impl<'a> core::marker::StructuralEq for ratatui::widgets::block::title::Title<'a>
+impl<'a> core::marker::StructuralPartialEq for ratatui::widgets::block::title::Title<'a>
+impl<'a> core::marker::StructuralPartialEq for ratatui::widgets::block::title::Title<'a>
+impl<'a> core::marker::Send for ratatui::widgets::block::title::Title<'a>
+impl<'a> core::marker::Send for ratatui::widgets::block::title::Title<'a>
+impl<'a> core::marker::Sync for ratatui::widgets::block::title::Title<'a>
+impl<'a> core::marker::Sync for ratatui::widgets::block::title::Title<'a>
+impl<'a> core::marker::Unpin for ratatui::widgets::block::title::Title<'a>
+impl<'a> core::marker::Unpin for ratatui::widgets::block::title::Title<'a>
+impl<'a> core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::block::title::Title<'a>
+impl<'a> core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::block::title::Title<'a>
+impl<'a> core::panic::unwind_safe::UnwindSafe for ratatui::widgets::block::title::Title<'a>
+impl<'a> core::panic::unwind_safe::UnwindSafe for ratatui::widgets::block::title::Title<'a>
+impl<T, U> core::convert::Into<U> for ratatui::widgets::block::title::Title<'a> where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::widgets::block::title::Title<'a> where U: core::convert::From<T>
+pub fn ratatui::widgets::block::title::Title<'a>::into(self) -> U
+pub fn ratatui::widgets::block::title::Title<'a>::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::block::title::Title<'a> where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::block::title::Title<'a> where U: core::convert::Into<T>
+pub type ratatui::widgets::block::title::Title<'a>::Error = core::convert::Infallible
+pub type ratatui::widgets::block::title::Title<'a>::Error = core::convert::Infallible
+pub fn ratatui::widgets::block::title::Title<'a>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::widgets::block::title::Title<'a>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::widgets::block::title::Title<'a> where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::widgets::block::title::Title<'a> where U: core::convert::TryFrom<T>
+pub type ratatui::widgets::block::title::Title<'a>::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::widgets::block::title::Title<'a>::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::widgets::block::title::Title<'a>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::widgets::block::title::Title<'a>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::widgets::block::title::Title<'a> where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::widgets::block::title::Title<'a> where T: core::clone::Clone
+pub type ratatui::widgets::block::title::Title<'a>::Owned = T
+pub type ratatui::widgets::block::title::Title<'a>::Owned = T
+pub fn ratatui::widgets::block::title::Title<'a>::clone_into(&self, target: &mut T)
+pub fn ratatui::widgets::block::title::Title<'a>::clone_into(&self, target: &mut T)
+pub fn ratatui::widgets::block::title::Title<'a>::to_owned(&self) -> T
+pub fn ratatui::widgets::block::title::Title<'a>::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::widgets::block::title::Title<'a> where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::widgets::block::title::Title<'a> where T: 'static + core::marker::Sized
+pub fn ratatui::widgets::block::title::Title<'a>::type_id(&self) -> core::any::TypeId
+pub fn ratatui::widgets::block::title::Title<'a>::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::widgets::block::title::Title<'a> where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::widgets::block::title::Title<'a> where T: core::marker::Sized
+pub fn ratatui::widgets::block::title::Title<'a>::borrow(&self) -> &T
+pub fn ratatui::widgets::block::title::Title<'a>::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::block::title::Title<'a> where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::block::title::Title<'a> where T: core::marker::Sized
+pub fn ratatui::widgets::block::title::Title<'a>::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::widgets::block::title::Title<'a>::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::widgets::block::title::Title<'a>
+impl<T> core::convert::From<T> for ratatui::widgets::block::title::Title<'a>
+pub fn ratatui::widgets::block::title::Title<'a>::from(t: T) -> T
+pub fn ratatui::widgets::block::title::Title<'a>::from(t: T) -> T
+pub enum ratatui::widgets::block::BorderType
+pub ratatui::widgets::block::BorderType::Double
+pub ratatui::widgets::block::BorderType::Plain
+pub ratatui::widgets::block::BorderType::Rounded
+pub ratatui::widgets::block::BorderType::Thick
+impl ratatui::widgets::block::BorderType
+impl ratatui::widgets::block::BorderType
+pub const fn ratatui::widgets::block::BorderType::line_symbols(border_type: ratatui::widgets::block::BorderType) -> ratatui::symbols::line::Set
+pub const fn ratatui::widgets::block::BorderType::line_symbols(border_type: ratatui::widgets::block::BorderType) -> ratatui::symbols::line::Set
+impl core::clone::Clone for ratatui::widgets::block::BorderType
+impl core::clone::Clone for ratatui::widgets::block::BorderType
+pub fn ratatui::widgets::block::BorderType::clone(&self) -> ratatui::widgets::block::BorderType
+pub fn ratatui::widgets::block::BorderType::clone(&self) -> ratatui::widgets::block::BorderType
+impl core::cmp::Eq for ratatui::widgets::block::BorderType
+impl core::cmp::Eq for ratatui::widgets::block::BorderType
+impl core::cmp::PartialEq<ratatui::widgets::block::BorderType> for ratatui::widgets::block::BorderType
+impl core::cmp::PartialEq<ratatui::widgets::block::BorderType> for ratatui::widgets::block::BorderType
+pub fn ratatui::widgets::block::BorderType::eq(&self, other: &ratatui::widgets::block::BorderType) -> bool
+pub fn ratatui::widgets::block::BorderType::eq(&self, other: &ratatui::widgets::block::BorderType) -> bool
+impl core::fmt::Debug for ratatui::widgets::block::BorderType
+impl core::fmt::Debug for ratatui::widgets::block::BorderType
+pub fn ratatui::widgets::block::BorderType::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::widgets::block::BorderType::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::marker::Copy for ratatui::widgets::block::BorderType
+impl core::marker::Copy for ratatui::widgets::block::BorderType
+impl core::marker::StructuralEq for ratatui::widgets::block::BorderType
+impl core::marker::StructuralEq for ratatui::widgets::block::BorderType
+impl core::marker::StructuralPartialEq for ratatui::widgets::block::BorderType
+impl core::marker::StructuralPartialEq for ratatui::widgets::block::BorderType
+impl core::marker::Send for ratatui::widgets::block::BorderType
+impl core::marker::Send for ratatui::widgets::block::BorderType
+impl core::marker::Sync for ratatui::widgets::block::BorderType
+impl core::marker::Sync for ratatui::widgets::block::BorderType
+impl core::marker::Unpin for ratatui::widgets::block::BorderType
+impl core::marker::Unpin for ratatui::widgets::block::BorderType
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::block::BorderType
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::block::BorderType
+impl core::panic::unwind_safe::UnwindSafe for ratatui::widgets::block::BorderType
+impl core::panic::unwind_safe::UnwindSafe for ratatui::widgets::block::BorderType
+impl<T, U> core::convert::Into<U> for ratatui::widgets::block::BorderType where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::widgets::block::BorderType where U: core::convert::From<T>
+pub fn ratatui::widgets::block::BorderType::into(self) -> U
+pub fn ratatui::widgets::block::BorderType::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::block::BorderType where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::block::BorderType where U: core::convert::Into<T>
+pub type ratatui::widgets::block::BorderType::Error = core::convert::Infallible
+pub type ratatui::widgets::block::BorderType::Error = core::convert::Infallible
+pub fn ratatui::widgets::block::BorderType::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::widgets::block::BorderType::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::widgets::block::BorderType where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::widgets::block::BorderType where U: core::convert::TryFrom<T>
+pub type ratatui::widgets::block::BorderType::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::widgets::block::BorderType::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::widgets::block::BorderType::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::widgets::block::BorderType::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::widgets::block::BorderType where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::widgets::block::BorderType where T: core::clone::Clone
+pub type ratatui::widgets::block::BorderType::Owned = T
+pub type ratatui::widgets::block::BorderType::Owned = T
+pub fn ratatui::widgets::block::BorderType::clone_into(&self, target: &mut T)
+pub fn ratatui::widgets::block::BorderType::clone_into(&self, target: &mut T)
+pub fn ratatui::widgets::block::BorderType::to_owned(&self) -> T
+pub fn ratatui::widgets::block::BorderType::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::widgets::block::BorderType where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::widgets::block::BorderType where T: 'static + core::marker::Sized
+pub fn ratatui::widgets::block::BorderType::type_id(&self) -> core::any::TypeId
+pub fn ratatui::widgets::block::BorderType::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::widgets::block::BorderType where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::widgets::block::BorderType where T: core::marker::Sized
+pub fn ratatui::widgets::block::BorderType::borrow(&self) -> &T
+pub fn ratatui::widgets::block::BorderType::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::block::BorderType where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::block::BorderType where T: core::marker::Sized
+pub fn ratatui::widgets::block::BorderType::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::widgets::block::BorderType::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::widgets::block::BorderType
+impl<T> core::convert::From<T> for ratatui::widgets::block::BorderType
+pub fn ratatui::widgets::block::BorderType::from(t: T) -> T
+pub fn ratatui::widgets::block::BorderType::from(t: T) -> T
+pub enum ratatui::widgets::block::Position
+pub ratatui::widgets::block::Position::Bottom
+pub ratatui::widgets::block::Position::Top
+pub struct ratatui::widgets::block::Block<'a>
+impl<'a> ratatui::widgets::block::Block<'a>
+impl<'a> ratatui::widgets::block::Block<'a>
+pub const fn ratatui::widgets::block::Block<'a>::border_style(self, style: ratatui::style::Style) -> ratatui::widgets::block::Block<'a>
+pub const fn ratatui::widgets::block::Block<'a>::border_style(self, style: ratatui::style::Style) -> ratatui::widgets::block::Block<'a>
+pub const fn ratatui::widgets::block::Block<'a>::border_type(self, border_type: ratatui::widgets::block::BorderType) -> ratatui::widgets::block::Block<'a>
+pub const fn ratatui::widgets::block::Block<'a>::border_type(self, border_type: ratatui::widgets::block::BorderType) -> ratatui::widgets::block::Block<'a>
+pub const fn ratatui::widgets::block::Block<'a>::borders(self, flag: ratatui::widgets::Borders) -> ratatui::widgets::block::Block<'a>
+pub const fn ratatui::widgets::block::Block<'a>::borders(self, flag: ratatui::widgets::Borders) -> ratatui::widgets::block::Block<'a>
+pub fn ratatui::widgets::block::Block<'a>::inner(&self, area: ratatui::layout::Rect) -> ratatui::layout::Rect
+pub fn ratatui::widgets::block::Block<'a>::inner(&self, area: ratatui::layout::Rect) -> ratatui::layout::Rect
+pub const fn ratatui::widgets::block::Block<'a>::new() -> Self
+pub const fn ratatui::widgets::block::Block<'a>::new() -> Self
+pub const fn ratatui::widgets::block::Block<'a>::padding(self, padding: ratatui::widgets::block::Padding) -> ratatui::widgets::block::Block<'a>
+pub const fn ratatui::widgets::block::Block<'a>::padding(self, padding: ratatui::widgets::block::Padding) -> ratatui::widgets::block::Block<'a>
+pub const fn ratatui::widgets::block::Block<'a>::style(self, style: ratatui::style::Style) -> ratatui::widgets::block::Block<'a>
+pub const fn ratatui::widgets::block::Block<'a>::style(self, style: ratatui::style::Style) -> ratatui::widgets::block::Block<'a>
+pub fn ratatui::widgets::block::Block<'a>::title<T>(self, title: T) -> ratatui::widgets::block::Block<'a> where T: core::convert::Into<ratatui::widgets::block::title::Title<'a>>
+pub fn ratatui::widgets::block::Block<'a>::title<T>(self, title: T) -> ratatui::widgets::block::Block<'a> where T: core::convert::Into<ratatui::widgets::block::title::Title<'a>>
+pub const fn ratatui::widgets::block::Block<'a>::title_alignment(self, alignment: ratatui::layout::Alignment) -> ratatui::widgets::block::Block<'a>
+pub const fn ratatui::widgets::block::Block<'a>::title_alignment(self, alignment: ratatui::layout::Alignment) -> ratatui::widgets::block::Block<'a>
+pub fn ratatui::widgets::block::Block<'a>::title_on_bottom(self) -> ratatui::widgets::block::Block<'a>
+pub fn ratatui::widgets::block::Block<'a>::title_on_bottom(self) -> ratatui::widgets::block::Block<'a>
+pub const fn ratatui::widgets::block::Block<'a>::title_position(self, position: ratatui::widgets::block::title::Position) -> ratatui::widgets::block::Block<'a>
+pub const fn ratatui::widgets::block::Block<'a>::title_position(self, position: ratatui::widgets::block::title::Position) -> ratatui::widgets::block::Block<'a>
+pub const fn ratatui::widgets::block::Block<'a>::title_style(self, style: ratatui::style::Style) -> ratatui::widgets::block::Block<'a>
+pub const fn ratatui::widgets::block::Block<'a>::title_style(self, style: ratatui::style::Style) -> ratatui::widgets::block::Block<'a>
+impl<'a> core::default::Default for ratatui::widgets::block::Block<'a>
+impl<'a> core::default::Default for ratatui::widgets::block::Block<'a>
+pub fn ratatui::widgets::block::Block<'a>::default() -> ratatui::widgets::block::Block<'a>
+pub fn ratatui::widgets::block::Block<'a>::default() -> ratatui::widgets::block::Block<'a>
+impl<'a> ratatui::widgets::Widget for ratatui::widgets::block::Block<'a>
+impl<'a> ratatui::widgets::Widget for ratatui::widgets::block::Block<'a>
+impl<'a> ratatui::widgets::Widget for ratatui::widgets::block::Block<'a>
+pub fn ratatui::widgets::block::Block<'a>::render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer)
+pub fn ratatui::widgets::block::Block<'a>::render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer)
+pub fn ratatui::widgets::block::Block<'a>::render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer)
+impl<'a> core::clone::Clone for ratatui::widgets::block::Block<'a>
+impl<'a> core::clone::Clone for ratatui::widgets::block::Block<'a>
+pub fn ratatui::widgets::block::Block<'a>::clone(&self) -> ratatui::widgets::block::Block<'a>
+pub fn ratatui::widgets::block::Block<'a>::clone(&self) -> ratatui::widgets::block::Block<'a>
+impl<'a> core::cmp::Eq for ratatui::widgets::block::Block<'a>
+impl<'a> core::cmp::Eq for ratatui::widgets::block::Block<'a>
+impl<'a> core::cmp::PartialEq<ratatui::widgets::block::Block<'a>> for ratatui::widgets::block::Block<'a>
+impl<'a> core::cmp::PartialEq<ratatui::widgets::block::Block<'a>> for ratatui::widgets::block::Block<'a>
+pub fn ratatui::widgets::block::Block<'a>::eq(&self, other: &ratatui::widgets::block::Block<'a>) -> bool
+pub fn ratatui::widgets::block::Block<'a>::eq(&self, other: &ratatui::widgets::block::Block<'a>) -> bool
+impl<'a> core::fmt::Debug for ratatui::widgets::block::Block<'a>
+impl<'a> core::fmt::Debug for ratatui::widgets::block::Block<'a>
+pub fn ratatui::widgets::block::Block<'a>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::widgets::block::Block<'a>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl<'a> core::marker::StructuralEq for ratatui::widgets::block::Block<'a>
+impl<'a> core::marker::StructuralEq for ratatui::widgets::block::Block<'a>
+impl<'a> core::marker::StructuralPartialEq for ratatui::widgets::block::Block<'a>
+impl<'a> core::marker::StructuralPartialEq for ratatui::widgets::block::Block<'a>
+impl<'a> core::marker::Send for ratatui::widgets::block::Block<'a>
+impl<'a> core::marker::Send for ratatui::widgets::block::Block<'a>
+impl<'a> core::marker::Sync for ratatui::widgets::block::Block<'a>
+impl<'a> core::marker::Sync for ratatui::widgets::block::Block<'a>
+impl<'a> core::marker::Unpin for ratatui::widgets::block::Block<'a>
+impl<'a> core::marker::Unpin for ratatui::widgets::block::Block<'a>
+impl<'a> core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::block::Block<'a>
+impl<'a> core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::block::Block<'a>
+impl<'a> core::panic::unwind_safe::UnwindSafe for ratatui::widgets::block::Block<'a>
+impl<'a> core::panic::unwind_safe::UnwindSafe for ratatui::widgets::block::Block<'a>
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for ratatui::widgets::block::Block<'a> where U: ratatui::style::Styled<Item = T>
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for ratatui::widgets::block::Block<'a> where U: ratatui::style::Styled<Item = T>
+pub fn ratatui::widgets::block::Block<'a>::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::block::Block<'a>::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::block::Block<'a>::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::widgets::block::Block<'a>::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::widgets::block::Block<'a>::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn ratatui::widgets::block::Block<'a>::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn ratatui::widgets::block::Block<'a>::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::block::Block<'a>::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::block::Block<'a>::reset(self) -> T
+pub fn ratatui::widgets::block::Block<'a>::reset(self) -> T
+impl<T, U> core::convert::Into<U> for ratatui::widgets::block::Block<'a> where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::widgets::block::Block<'a> where U: core::convert::From<T>
+pub fn ratatui::widgets::block::Block<'a>::into(self) -> U
+pub fn ratatui::widgets::block::Block<'a>::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::block::Block<'a> where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::block::Block<'a> where U: core::convert::Into<T>
+pub type ratatui::widgets::block::Block<'a>::Error = core::convert::Infallible
+pub type ratatui::widgets::block::Block<'a>::Error = core::convert::Infallible
+pub fn ratatui::widgets::block::Block<'a>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::widgets::block::Block<'a>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::widgets::block::Block<'a> where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::widgets::block::Block<'a> where U: core::convert::TryFrom<T>
+pub type ratatui::widgets::block::Block<'a>::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::widgets::block::Block<'a>::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::widgets::block::Block<'a>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::widgets::block::Block<'a>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::widgets::block::Block<'a> where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::widgets::block::Block<'a> where T: core::clone::Clone
+pub type ratatui::widgets::block::Block<'a>::Owned = T
+pub type ratatui::widgets::block::Block<'a>::Owned = T
+pub fn ratatui::widgets::block::Block<'a>::clone_into(&self, target: &mut T)
+pub fn ratatui::widgets::block::Block<'a>::clone_into(&self, target: &mut T)
+pub fn ratatui::widgets::block::Block<'a>::to_owned(&self) -> T
+pub fn ratatui::widgets::block::Block<'a>::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::widgets::block::Block<'a> where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::widgets::block::Block<'a> where T: 'static + core::marker::Sized
+pub fn ratatui::widgets::block::Block<'a>::type_id(&self) -> core::any::TypeId
+pub fn ratatui::widgets::block::Block<'a>::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::widgets::block::Block<'a> where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::widgets::block::Block<'a> where T: core::marker::Sized
+pub fn ratatui::widgets::block::Block<'a>::borrow(&self) -> &T
+pub fn ratatui::widgets::block::Block<'a>::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::block::Block<'a> where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::block::Block<'a> where T: core::marker::Sized
+pub fn ratatui::widgets::block::Block<'a>::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::widgets::block::Block<'a>::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::widgets::block::Block<'a>
+impl<T> core::convert::From<T> for ratatui::widgets::block::Block<'a>
+pub fn ratatui::widgets::block::Block<'a>::from(t: T) -> T
+pub fn ratatui::widgets::block::Block<'a>::from(t: T) -> T
+pub struct ratatui::widgets::block::Padding
+pub ratatui::widgets::block::Padding::bottom: u16
+pub ratatui::widgets::block::Padding::left: u16
+pub ratatui::widgets::block::Padding::right: u16
+pub ratatui::widgets::block::Padding::top: u16
+impl ratatui::widgets::block::Padding
+impl ratatui::widgets::block::Padding
+pub const fn ratatui::widgets::block::Padding::horizontal(value: u16) -> Self
+pub const fn ratatui::widgets::block::Padding::horizontal(value: u16) -> Self
+pub const fn ratatui::widgets::block::Padding::new(left: u16, right: u16, top: u16, bottom: u16) -> Self
+pub const fn ratatui::widgets::block::Padding::new(left: u16, right: u16, top: u16, bottom: u16) -> Self
+pub const fn ratatui::widgets::block::Padding::uniform(value: u16) -> Self
+pub const fn ratatui::widgets::block::Padding::uniform(value: u16) -> Self
+pub const fn ratatui::widgets::block::Padding::vertical(value: u16) -> Self
+pub const fn ratatui::widgets::block::Padding::vertical(value: u16) -> Self
+pub const fn ratatui::widgets::block::Padding::zero() -> Self
+pub const fn ratatui::widgets::block::Padding::zero() -> Self
+impl core::clone::Clone for ratatui::widgets::block::Padding
+impl core::clone::Clone for ratatui::widgets::block::Padding
+pub fn ratatui::widgets::block::Padding::clone(&self) -> ratatui::widgets::block::Padding
+pub fn ratatui::widgets::block::Padding::clone(&self) -> ratatui::widgets::block::Padding
+impl core::cmp::Eq for ratatui::widgets::block::Padding
+impl core::cmp::Eq for ratatui::widgets::block::Padding
+impl core::cmp::PartialEq<ratatui::widgets::block::Padding> for ratatui::widgets::block::Padding
+impl core::cmp::PartialEq<ratatui::widgets::block::Padding> for ratatui::widgets::block::Padding
+pub fn ratatui::widgets::block::Padding::eq(&self, other: &ratatui::widgets::block::Padding) -> bool
+pub fn ratatui::widgets::block::Padding::eq(&self, other: &ratatui::widgets::block::Padding) -> bool
+impl core::fmt::Debug for ratatui::widgets::block::Padding
+impl core::fmt::Debug for ratatui::widgets::block::Padding
+pub fn ratatui::widgets::block::Padding::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::widgets::block::Padding::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::hash::Hash for ratatui::widgets::block::Padding
+impl core::hash::Hash for ratatui::widgets::block::Padding
+pub fn ratatui::widgets::block::Padding::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
+pub fn ratatui::widgets::block::Padding::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
+impl core::marker::StructuralEq for ratatui::widgets::block::Padding
+impl core::marker::StructuralEq for ratatui::widgets::block::Padding
+impl core::marker::StructuralPartialEq for ratatui::widgets::block::Padding
+impl core::marker::StructuralPartialEq for ratatui::widgets::block::Padding
+impl core::marker::Send for ratatui::widgets::block::Padding
+impl core::marker::Send for ratatui::widgets::block::Padding
+impl core::marker::Sync for ratatui::widgets::block::Padding
+impl core::marker::Sync for ratatui::widgets::block::Padding
+impl core::marker::Unpin for ratatui::widgets::block::Padding
+impl core::marker::Unpin for ratatui::widgets::block::Padding
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::block::Padding
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::block::Padding
+impl core::panic::unwind_safe::UnwindSafe for ratatui::widgets::block::Padding
+impl core::panic::unwind_safe::UnwindSafe for ratatui::widgets::block::Padding
+impl<T, U> core::convert::Into<U> for ratatui::widgets::block::Padding where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::widgets::block::Padding where U: core::convert::From<T>
+pub fn ratatui::widgets::block::Padding::into(self) -> U
+pub fn ratatui::widgets::block::Padding::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::block::Padding where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::block::Padding where U: core::convert::Into<T>
+pub type ratatui::widgets::block::Padding::Error = core::convert::Infallible
+pub type ratatui::widgets::block::Padding::Error = core::convert::Infallible
+pub fn ratatui::widgets::block::Padding::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::widgets::block::Padding::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::widgets::block::Padding where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::widgets::block::Padding where U: core::convert::TryFrom<T>
+pub type ratatui::widgets::block::Padding::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::widgets::block::Padding::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::widgets::block::Padding::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::widgets::block::Padding::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::widgets::block::Padding where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::widgets::block::Padding where T: core::clone::Clone
+pub type ratatui::widgets::block::Padding::Owned = T
+pub type ratatui::widgets::block::Padding::Owned = T
+pub fn ratatui::widgets::block::Padding::clone_into(&self, target: &mut T)
+pub fn ratatui::widgets::block::Padding::clone_into(&self, target: &mut T)
+pub fn ratatui::widgets::block::Padding::to_owned(&self) -> T
+pub fn ratatui::widgets::block::Padding::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::widgets::block::Padding where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::widgets::block::Padding where T: 'static + core::marker::Sized
+pub fn ratatui::widgets::block::Padding::type_id(&self) -> core::any::TypeId
+pub fn ratatui::widgets::block::Padding::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::widgets::block::Padding where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::widgets::block::Padding where T: core::marker::Sized
+pub fn ratatui::widgets::block::Padding::borrow(&self) -> &T
+pub fn ratatui::widgets::block::Padding::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::block::Padding where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::block::Padding where T: core::marker::Sized
+pub fn ratatui::widgets::block::Padding::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::widgets::block::Padding::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::widgets::block::Padding
+impl<T> core::convert::From<T> for ratatui::widgets::block::Padding
+pub fn ratatui::widgets::block::Padding::from(t: T) -> T
+pub fn ratatui::widgets::block::Padding::from(t: T) -> T
+pub struct ratatui::widgets::block::Title<'a>
+pub ratatui::widgets::block::Title::alignment: core::option::Option<ratatui::layout::Alignment>
+pub ratatui::widgets::block::Title::content: ratatui::text::Line<'a>
+pub ratatui::widgets::block::Title::position: core::option::Option<ratatui::widgets::block::title::Position>
+pub mod ratatui::widgets::scrollbar
+pub enum ratatui::widgets::scrollbar::ScrollDirection
+pub ratatui::widgets::scrollbar::ScrollDirection::Backward
+pub ratatui::widgets::scrollbar::ScrollDirection::Forward
+impl core::clone::Clone for ratatui::widgets::scrollbar::ScrollDirection
+impl core::clone::Clone for ratatui::widgets::scrollbar::ScrollDirection
+pub fn ratatui::widgets::scrollbar::ScrollDirection::clone(&self) -> ratatui::widgets::scrollbar::ScrollDirection
+pub fn ratatui::widgets::scrollbar::ScrollDirection::clone(&self) -> ratatui::widgets::scrollbar::ScrollDirection
+impl core::cmp::Eq for ratatui::widgets::scrollbar::ScrollDirection
+impl core::cmp::Eq for ratatui::widgets::scrollbar::ScrollDirection
+impl core::cmp::PartialEq<ratatui::widgets::scrollbar::ScrollDirection> for ratatui::widgets::scrollbar::ScrollDirection
+impl core::cmp::PartialEq<ratatui::widgets::scrollbar::ScrollDirection> for ratatui::widgets::scrollbar::ScrollDirection
+pub fn ratatui::widgets::scrollbar::ScrollDirection::eq(&self, other: &ratatui::widgets::scrollbar::ScrollDirection) -> bool
+pub fn ratatui::widgets::scrollbar::ScrollDirection::eq(&self, other: &ratatui::widgets::scrollbar::ScrollDirection) -> bool
+impl core::default::Default for ratatui::widgets::scrollbar::ScrollDirection
+impl core::default::Default for ratatui::widgets::scrollbar::ScrollDirection
+pub fn ratatui::widgets::scrollbar::ScrollDirection::default() -> ratatui::widgets::scrollbar::ScrollDirection
+pub fn ratatui::widgets::scrollbar::ScrollDirection::default() -> ratatui::widgets::scrollbar::ScrollDirection
+impl core::fmt::Debug for ratatui::widgets::scrollbar::ScrollDirection
+impl core::fmt::Debug for ratatui::widgets::scrollbar::ScrollDirection
+pub fn ratatui::widgets::scrollbar::ScrollDirection::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::widgets::scrollbar::ScrollDirection::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::marker::Copy for ratatui::widgets::scrollbar::ScrollDirection
+impl core::marker::Copy for ratatui::widgets::scrollbar::ScrollDirection
+impl core::marker::StructuralEq for ratatui::widgets::scrollbar::ScrollDirection
+impl core::marker::StructuralEq for ratatui::widgets::scrollbar::ScrollDirection
+impl core::marker::StructuralPartialEq for ratatui::widgets::scrollbar::ScrollDirection
+impl core::marker::StructuralPartialEq for ratatui::widgets::scrollbar::ScrollDirection
+impl core::marker::Send for ratatui::widgets::scrollbar::ScrollDirection
+impl core::marker::Send for ratatui::widgets::scrollbar::ScrollDirection
+impl core::marker::Sync for ratatui::widgets::scrollbar::ScrollDirection
+impl core::marker::Sync for ratatui::widgets::scrollbar::ScrollDirection
+impl core::marker::Unpin for ratatui::widgets::scrollbar::ScrollDirection
+impl core::marker::Unpin for ratatui::widgets::scrollbar::ScrollDirection
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::scrollbar::ScrollDirection
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::scrollbar::ScrollDirection
+impl core::panic::unwind_safe::UnwindSafe for ratatui::widgets::scrollbar::ScrollDirection
+impl core::panic::unwind_safe::UnwindSafe for ratatui::widgets::scrollbar::ScrollDirection
+impl<T, U> core::convert::Into<U> for ratatui::widgets::scrollbar::ScrollDirection where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::widgets::scrollbar::ScrollDirection where U: core::convert::From<T>
+pub fn ratatui::widgets::scrollbar::ScrollDirection::into(self) -> U
+pub fn ratatui::widgets::scrollbar::ScrollDirection::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::scrollbar::ScrollDirection where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::scrollbar::ScrollDirection where U: core::convert::Into<T>
+pub type ratatui::widgets::scrollbar::ScrollDirection::Error = core::convert::Infallible
+pub type ratatui::widgets::scrollbar::ScrollDirection::Error = core::convert::Infallible
+pub fn ratatui::widgets::scrollbar::ScrollDirection::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::widgets::scrollbar::ScrollDirection::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::widgets::scrollbar::ScrollDirection where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::widgets::scrollbar::ScrollDirection where U: core::convert::TryFrom<T>
+pub type ratatui::widgets::scrollbar::ScrollDirection::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::widgets::scrollbar::ScrollDirection::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::widgets::scrollbar::ScrollDirection::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::widgets::scrollbar::ScrollDirection::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::widgets::scrollbar::ScrollDirection where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::widgets::scrollbar::ScrollDirection where T: core::clone::Clone
+pub type ratatui::widgets::scrollbar::ScrollDirection::Owned = T
+pub type ratatui::widgets::scrollbar::ScrollDirection::Owned = T
+pub fn ratatui::widgets::scrollbar::ScrollDirection::clone_into(&self, target: &mut T)
+pub fn ratatui::widgets::scrollbar::ScrollDirection::clone_into(&self, target: &mut T)
+pub fn ratatui::widgets::scrollbar::ScrollDirection::to_owned(&self) -> T
+pub fn ratatui::widgets::scrollbar::ScrollDirection::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::widgets::scrollbar::ScrollDirection where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::widgets::scrollbar::ScrollDirection where T: 'static + core::marker::Sized
+pub fn ratatui::widgets::scrollbar::ScrollDirection::type_id(&self) -> core::any::TypeId
+pub fn ratatui::widgets::scrollbar::ScrollDirection::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::widgets::scrollbar::ScrollDirection where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::widgets::scrollbar::ScrollDirection where T: core::marker::Sized
+pub fn ratatui::widgets::scrollbar::ScrollDirection::borrow(&self) -> &T
+pub fn ratatui::widgets::scrollbar::ScrollDirection::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::scrollbar::ScrollDirection where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::scrollbar::ScrollDirection where T: core::marker::Sized
+pub fn ratatui::widgets::scrollbar::ScrollDirection::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::widgets::scrollbar::ScrollDirection::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::widgets::scrollbar::ScrollDirection
+impl<T> core::convert::From<T> for ratatui::widgets::scrollbar::ScrollDirection
+pub fn ratatui::widgets::scrollbar::ScrollDirection::from(t: T) -> T
+pub fn ratatui::widgets::scrollbar::ScrollDirection::from(t: T) -> T
+pub enum ratatui::widgets::scrollbar::ScrollbarOrientation
+pub ratatui::widgets::scrollbar::ScrollbarOrientation::HorizontalBottom
+pub ratatui::widgets::scrollbar::ScrollbarOrientation::HorizontalTop
+pub ratatui::widgets::scrollbar::ScrollbarOrientation::VerticalLeft
+pub ratatui::widgets::scrollbar::ScrollbarOrientation::VerticalRight
+impl core::clone::Clone for ratatui::widgets::scrollbar::ScrollbarOrientation
+impl core::clone::Clone for ratatui::widgets::scrollbar::ScrollbarOrientation
+pub fn ratatui::widgets::scrollbar::ScrollbarOrientation::clone(&self) -> ratatui::widgets::scrollbar::ScrollbarOrientation
+pub fn ratatui::widgets::scrollbar::ScrollbarOrientation::clone(&self) -> ratatui::widgets::scrollbar::ScrollbarOrientation
+impl core::default::Default for ratatui::widgets::scrollbar::ScrollbarOrientation
+impl core::default::Default for ratatui::widgets::scrollbar::ScrollbarOrientation
+pub fn ratatui::widgets::scrollbar::ScrollbarOrientation::default() -> ratatui::widgets::scrollbar::ScrollbarOrientation
+pub fn ratatui::widgets::scrollbar::ScrollbarOrientation::default() -> ratatui::widgets::scrollbar::ScrollbarOrientation
+impl core::fmt::Debug for ratatui::widgets::scrollbar::ScrollbarOrientation
+impl core::fmt::Debug for ratatui::widgets::scrollbar::ScrollbarOrientation
+pub fn ratatui::widgets::scrollbar::ScrollbarOrientation::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::widgets::scrollbar::ScrollbarOrientation::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::marker::Send for ratatui::widgets::scrollbar::ScrollbarOrientation
+impl core::marker::Send for ratatui::widgets::scrollbar::ScrollbarOrientation
+impl core::marker::Sync for ratatui::widgets::scrollbar::ScrollbarOrientation
+impl core::marker::Sync for ratatui::widgets::scrollbar::ScrollbarOrientation
+impl core::marker::Unpin for ratatui::widgets::scrollbar::ScrollbarOrientation
+impl core::marker::Unpin for ratatui::widgets::scrollbar::ScrollbarOrientation
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::scrollbar::ScrollbarOrientation
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::scrollbar::ScrollbarOrientation
+impl core::panic::unwind_safe::UnwindSafe for ratatui::widgets::scrollbar::ScrollbarOrientation
+impl core::panic::unwind_safe::UnwindSafe for ratatui::widgets::scrollbar::ScrollbarOrientation
+impl<T, U> core::convert::Into<U> for ratatui::widgets::scrollbar::ScrollbarOrientation where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::widgets::scrollbar::ScrollbarOrientation where U: core::convert::From<T>
+pub fn ratatui::widgets::scrollbar::ScrollbarOrientation::into(self) -> U
+pub fn ratatui::widgets::scrollbar::ScrollbarOrientation::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::scrollbar::ScrollbarOrientation where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::scrollbar::ScrollbarOrientation where U: core::convert::Into<T>
+pub type ratatui::widgets::scrollbar::ScrollbarOrientation::Error = core::convert::Infallible
+pub type ratatui::widgets::scrollbar::ScrollbarOrientation::Error = core::convert::Infallible
+pub fn ratatui::widgets::scrollbar::ScrollbarOrientation::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::widgets::scrollbar::ScrollbarOrientation::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::widgets::scrollbar::ScrollbarOrientation where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::widgets::scrollbar::ScrollbarOrientation where U: core::convert::TryFrom<T>
+pub type ratatui::widgets::scrollbar::ScrollbarOrientation::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::widgets::scrollbar::ScrollbarOrientation::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::widgets::scrollbar::ScrollbarOrientation::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::widgets::scrollbar::ScrollbarOrientation::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::widgets::scrollbar::ScrollbarOrientation where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::widgets::scrollbar::ScrollbarOrientation where T: core::clone::Clone
+pub type ratatui::widgets::scrollbar::ScrollbarOrientation::Owned = T
+pub type ratatui::widgets::scrollbar::ScrollbarOrientation::Owned = T
+pub fn ratatui::widgets::scrollbar::ScrollbarOrientation::clone_into(&self, target: &mut T)
+pub fn ratatui::widgets::scrollbar::ScrollbarOrientation::clone_into(&self, target: &mut T)
+pub fn ratatui::widgets::scrollbar::ScrollbarOrientation::to_owned(&self) -> T
+pub fn ratatui::widgets::scrollbar::ScrollbarOrientation::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::widgets::scrollbar::ScrollbarOrientation where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::widgets::scrollbar::ScrollbarOrientation where T: 'static + core::marker::Sized
+pub fn ratatui::widgets::scrollbar::ScrollbarOrientation::type_id(&self) -> core::any::TypeId
+pub fn ratatui::widgets::scrollbar::ScrollbarOrientation::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::widgets::scrollbar::ScrollbarOrientation where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::widgets::scrollbar::ScrollbarOrientation where T: core::marker::Sized
+pub fn ratatui::widgets::scrollbar::ScrollbarOrientation::borrow(&self) -> &T
+pub fn ratatui::widgets::scrollbar::ScrollbarOrientation::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::scrollbar::ScrollbarOrientation where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::scrollbar::ScrollbarOrientation where T: core::marker::Sized
+pub fn ratatui::widgets::scrollbar::ScrollbarOrientation::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::widgets::scrollbar::ScrollbarOrientation::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::widgets::scrollbar::ScrollbarOrientation
+impl<T> core::convert::From<T> for ratatui::widgets::scrollbar::ScrollbarOrientation
+pub fn ratatui::widgets::scrollbar::ScrollbarOrientation::from(t: T) -> T
+pub fn ratatui::widgets::scrollbar::ScrollbarOrientation::from(t: T) -> T
+pub struct ratatui::widgets::scrollbar::Scrollbar<'a>
+impl<'a> ratatui::widgets::scrollbar::Scrollbar<'a>
+impl<'a> ratatui::widgets::scrollbar::Scrollbar<'a>
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::begin_style(self, begin_style: ratatui::style::Style) -> Self
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::begin_style(self, begin_style: ratatui::style::Style) -> Self
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::begin_symbol(self, begin_symbol: core::option::Option<&'a str>) -> Self
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::begin_symbol(self, begin_symbol: core::option::Option<&'a str>) -> Self
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::end_style(self, end_style: ratatui::style::Style) -> Self
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::end_style(self, end_style: ratatui::style::Style) -> Self
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::end_symbol(self, end_symbol: core::option::Option<&'a str>) -> Self
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::end_symbol(self, end_symbol: core::option::Option<&'a str>) -> Self
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::new(orientation: ratatui::widgets::scrollbar::ScrollbarOrientation) -> Self
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::new(orientation: ratatui::widgets::scrollbar::ScrollbarOrientation) -> Self
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::orientation(self, orientation: ratatui::widgets::scrollbar::ScrollbarOrientation) -> Self
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::orientation(self, orientation: ratatui::widgets::scrollbar::ScrollbarOrientation) -> Self
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::orientation_and_symbol(self, orientation: ratatui::widgets::scrollbar::ScrollbarOrientation, set: ratatui::widgets::scrollbar::Set) -> Self
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::orientation_and_symbol(self, orientation: ratatui::widgets::scrollbar::ScrollbarOrientation, set: ratatui::widgets::scrollbar::Set) -> Self
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::style(self, style: ratatui::style::Style) -> Self
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::style(self, style: ratatui::style::Style) -> Self
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::symbols(self, symbol: ratatui::widgets::scrollbar::Set) -> Self
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::symbols(self, symbol: ratatui::widgets::scrollbar::Set) -> Self
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::thumb_style(self, thumb_style: ratatui::style::Style) -> Self
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::thumb_style(self, thumb_style: ratatui::style::Style) -> Self
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::thumb_symbol(self, thumb_symbol: &'a str) -> Self
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::thumb_symbol(self, thumb_symbol: &'a str) -> Self
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::track_style(self, track_style: ratatui::style::Style) -> Self
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::track_style(self, track_style: ratatui::style::Style) -> Self
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::track_symbol(self, track_symbol: &'a str) -> Self
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::track_symbol(self, track_symbol: &'a str) -> Self
+impl<'a> core::default::Default for ratatui::widgets::scrollbar::Scrollbar<'a>
+impl<'a> core::default::Default for ratatui::widgets::scrollbar::Scrollbar<'a>
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::default() -> Self
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::default() -> Self
+impl<'a> ratatui::widgets::StatefulWidget for ratatui::widgets::scrollbar::Scrollbar<'a>
+impl<'a> ratatui::widgets::StatefulWidget for ratatui::widgets::scrollbar::Scrollbar<'a>
+impl<'a> ratatui::widgets::StatefulWidget for ratatui::widgets::scrollbar::Scrollbar<'a>
+pub type ratatui::widgets::scrollbar::Scrollbar<'a>::State = ratatui::widgets::scrollbar::ScrollbarState
+pub type ratatui::widgets::scrollbar::Scrollbar<'a>::State = ratatui::widgets::scrollbar::ScrollbarState
+pub type ratatui::widgets::scrollbar::Scrollbar<'a>::State = ratatui::widgets::scrollbar::ScrollbarState
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer, state: &mut Self::State)
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer, state: &mut Self::State)
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer, state: &mut Self::State)
+impl<'a> core::clone::Clone for ratatui::widgets::scrollbar::Scrollbar<'a>
+impl<'a> core::clone::Clone for ratatui::widgets::scrollbar::Scrollbar<'a>
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::clone(&self) -> ratatui::widgets::scrollbar::Scrollbar<'a>
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::clone(&self) -> ratatui::widgets::scrollbar::Scrollbar<'a>
+impl<'a> core::fmt::Debug for ratatui::widgets::scrollbar::Scrollbar<'a>
+impl<'a> core::fmt::Debug for ratatui::widgets::scrollbar::Scrollbar<'a>
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl<'a> core::marker::Send for ratatui::widgets::scrollbar::Scrollbar<'a>
+impl<'a> core::marker::Send for ratatui::widgets::scrollbar::Scrollbar<'a>
+impl<'a> core::marker::Sync for ratatui::widgets::scrollbar::Scrollbar<'a>
+impl<'a> core::marker::Sync for ratatui::widgets::scrollbar::Scrollbar<'a>
+impl<'a> core::marker::Unpin for ratatui::widgets::scrollbar::Scrollbar<'a>
+impl<'a> core::marker::Unpin for ratatui::widgets::scrollbar::Scrollbar<'a>
+impl<'a> core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::scrollbar::Scrollbar<'a>
+impl<'a> core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::scrollbar::Scrollbar<'a>
+impl<'a> core::panic::unwind_safe::UnwindSafe for ratatui::widgets::scrollbar::Scrollbar<'a>
+impl<'a> core::panic::unwind_safe::UnwindSafe for ratatui::widgets::scrollbar::Scrollbar<'a>
+impl<T, U> core::convert::Into<U> for ratatui::widgets::scrollbar::Scrollbar<'a> where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::widgets::scrollbar::Scrollbar<'a> where U: core::convert::From<T>
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::into(self) -> U
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::scrollbar::Scrollbar<'a> where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::scrollbar::Scrollbar<'a> where U: core::convert::Into<T>
+pub type ratatui::widgets::scrollbar::Scrollbar<'a>::Error = core::convert::Infallible
+pub type ratatui::widgets::scrollbar::Scrollbar<'a>::Error = core::convert::Infallible
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::widgets::scrollbar::Scrollbar<'a> where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::widgets::scrollbar::Scrollbar<'a> where U: core::convert::TryFrom<T>
+pub type ratatui::widgets::scrollbar::Scrollbar<'a>::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::widgets::scrollbar::Scrollbar<'a>::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::widgets::scrollbar::Scrollbar<'a> where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::widgets::scrollbar::Scrollbar<'a> where T: core::clone::Clone
+pub type ratatui::widgets::scrollbar::Scrollbar<'a>::Owned = T
+pub type ratatui::widgets::scrollbar::Scrollbar<'a>::Owned = T
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::clone_into(&self, target: &mut T)
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::clone_into(&self, target: &mut T)
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::to_owned(&self) -> T
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::widgets::scrollbar::Scrollbar<'a> where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::widgets::scrollbar::Scrollbar<'a> where T: 'static + core::marker::Sized
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::type_id(&self) -> core::any::TypeId
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::widgets::scrollbar::Scrollbar<'a> where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::widgets::scrollbar::Scrollbar<'a> where T: core::marker::Sized
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::borrow(&self) -> &T
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::scrollbar::Scrollbar<'a> where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::scrollbar::Scrollbar<'a> where T: core::marker::Sized
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::widgets::scrollbar::Scrollbar<'a>
+impl<T> core::convert::From<T> for ratatui::widgets::scrollbar::Scrollbar<'a>
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::from(t: T) -> T
+pub fn ratatui::widgets::scrollbar::Scrollbar<'a>::from(t: T) -> T
+pub struct ratatui::widgets::scrollbar::ScrollbarState
+impl ratatui::widgets::scrollbar::ScrollbarState
+impl ratatui::widgets::scrollbar::ScrollbarState
+pub fn ratatui::widgets::scrollbar::ScrollbarState::content_length(self, content_length: u16) -> Self
+pub fn ratatui::widgets::scrollbar::ScrollbarState::content_length(self, content_length: u16) -> Self
+pub fn ratatui::widgets::scrollbar::ScrollbarState::first(&mut self)
+pub fn ratatui::widgets::scrollbar::ScrollbarState::first(&mut self)
+pub fn ratatui::widgets::scrollbar::ScrollbarState::last(&mut self)
+pub fn ratatui::widgets::scrollbar::ScrollbarState::last(&mut self)
+pub fn ratatui::widgets::scrollbar::ScrollbarState::next(&mut self)
+pub fn ratatui::widgets::scrollbar::ScrollbarState::next(&mut self)
+pub fn ratatui::widgets::scrollbar::ScrollbarState::position(self, position: u16) -> Self
+pub fn ratatui::widgets::scrollbar::ScrollbarState::position(self, position: u16) -> Self
+pub fn ratatui::widgets::scrollbar::ScrollbarState::prev(&mut self)
+pub fn ratatui::widgets::scrollbar::ScrollbarState::prev(&mut self)
+pub fn ratatui::widgets::scrollbar::ScrollbarState::scroll(&mut self, direction: ratatui::widgets::scrollbar::ScrollDirection)
+pub fn ratatui::widgets::scrollbar::ScrollbarState::scroll(&mut self, direction: ratatui::widgets::scrollbar::ScrollDirection)
+pub fn ratatui::widgets::scrollbar::ScrollbarState::viewport_content_length(self, viewport_content_length: u16) -> Self
+pub fn ratatui::widgets::scrollbar::ScrollbarState::viewport_content_length(self, viewport_content_length: u16) -> Self
+impl core::clone::Clone for ratatui::widgets::scrollbar::ScrollbarState
+impl core::clone::Clone for ratatui::widgets::scrollbar::ScrollbarState
+pub fn ratatui::widgets::scrollbar::ScrollbarState::clone(&self) -> ratatui::widgets::scrollbar::ScrollbarState
+pub fn ratatui::widgets::scrollbar::ScrollbarState::clone(&self) -> ratatui::widgets::scrollbar::ScrollbarState
+impl core::default::Default for ratatui::widgets::scrollbar::ScrollbarState
+impl core::default::Default for ratatui::widgets::scrollbar::ScrollbarState
+pub fn ratatui::widgets::scrollbar::ScrollbarState::default() -> ratatui::widgets::scrollbar::ScrollbarState
+pub fn ratatui::widgets::scrollbar::ScrollbarState::default() -> ratatui::widgets::scrollbar::ScrollbarState
+impl core::fmt::Debug for ratatui::widgets::scrollbar::ScrollbarState
+impl core::fmt::Debug for ratatui::widgets::scrollbar::ScrollbarState
+pub fn ratatui::widgets::scrollbar::ScrollbarState::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::widgets::scrollbar::ScrollbarState::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::marker::Copy for ratatui::widgets::scrollbar::ScrollbarState
+impl core::marker::Copy for ratatui::widgets::scrollbar::ScrollbarState
+impl core::marker::Send for ratatui::widgets::scrollbar::ScrollbarState
+impl core::marker::Send for ratatui::widgets::scrollbar::ScrollbarState
+impl core::marker::Sync for ratatui::widgets::scrollbar::ScrollbarState
+impl core::marker::Sync for ratatui::widgets::scrollbar::ScrollbarState
+impl core::marker::Unpin for ratatui::widgets::scrollbar::ScrollbarState
+impl core::marker::Unpin for ratatui::widgets::scrollbar::ScrollbarState
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::scrollbar::ScrollbarState
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::scrollbar::ScrollbarState
+impl core::panic::unwind_safe::UnwindSafe for ratatui::widgets::scrollbar::ScrollbarState
+impl core::panic::unwind_safe::UnwindSafe for ratatui::widgets::scrollbar::ScrollbarState
+impl<T, U> core::convert::Into<U> for ratatui::widgets::scrollbar::ScrollbarState where U: core::convert::From<T>
+impl<T, U> core::convert::Into<U> for ratatui::widgets::scrollbar::ScrollbarState where U: core::convert::From<T>
+pub fn ratatui::widgets::scrollbar::ScrollbarState::into(self) -> U
+pub fn ratatui::widgets::scrollbar::ScrollbarState::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::scrollbar::ScrollbarState where U: core::convert::Into<T>
+impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::scrollbar::ScrollbarState where U: core::convert::Into<T>
+pub type ratatui::widgets::scrollbar::ScrollbarState::Error = core::convert::Infallible
+pub type ratatui::widgets::scrollbar::ScrollbarState::Error = core::convert::Infallible
+pub fn ratatui::widgets::scrollbar::ScrollbarState::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+pub fn ratatui::widgets::scrollbar::ScrollbarState::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::widgets::scrollbar::ScrollbarState where U: core::convert::TryFrom<T>
+impl<T, U> core::convert::TryInto<U> for ratatui::widgets::scrollbar::ScrollbarState where U: core::convert::TryFrom<T>
+pub type ratatui::widgets::scrollbar::ScrollbarState::Error = <U as core::convert::TryFrom<T>>::Error
+pub type ratatui::widgets::scrollbar::ScrollbarState::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::widgets::scrollbar::ScrollbarState::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+pub fn ratatui::widgets::scrollbar::ScrollbarState::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::widgets::scrollbar::ScrollbarState where T: core::clone::Clone
+impl<T> alloc::borrow::ToOwned for ratatui::widgets::scrollbar::ScrollbarState where T: core::clone::Clone
+pub type ratatui::widgets::scrollbar::ScrollbarState::Owned = T
+pub type ratatui::widgets::scrollbar::ScrollbarState::Owned = T
+pub fn ratatui::widgets::scrollbar::ScrollbarState::clone_into(&self, target: &mut T)
+pub fn ratatui::widgets::scrollbar::ScrollbarState::clone_into(&self, target: &mut T)
+pub fn ratatui::widgets::scrollbar::ScrollbarState::to_owned(&self) -> T
+pub fn ratatui::widgets::scrollbar::ScrollbarState::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::widgets::scrollbar::ScrollbarState where T: 'static + core::marker::Sized
+impl<T> core::any::Any for ratatui::widgets::scrollbar::ScrollbarState where T: 'static + core::marker::Sized
+pub fn ratatui::widgets::scrollbar::ScrollbarState::type_id(&self) -> core::any::TypeId
+pub fn ratatui::widgets::scrollbar::ScrollbarState::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::widgets::scrollbar::ScrollbarState where T: core::marker::Sized
+impl<T> core::borrow::Borrow<T> for ratatui::widgets::scrollbar::ScrollbarState where T: core::marker::Sized
+pub fn ratatui::widgets::scrollbar::ScrollbarState::borrow(&self) -> &T
+pub fn ratatui::widgets::scrollbar::ScrollbarState::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::scrollbar::ScrollbarState where T: core::marker::Sized
+impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::scrollbar::ScrollbarState where T: core::marker::Sized
+pub fn ratatui::widgets::scrollbar::ScrollbarState::borrow_mut(&mut self) -> &mut T
+pub fn ratatui::widgets::scrollbar::ScrollbarState::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::widgets::scrollbar::ScrollbarState
+impl<T> core::convert::From<T> for ratatui::widgets::scrollbar::ScrollbarState
+pub fn ratatui::widgets::scrollbar::ScrollbarState::from(t: T) -> T
+pub fn ratatui::widgets::scrollbar::ScrollbarState::from(t: T) -> T
+pub struct ratatui::widgets::scrollbar::Set
+pub ratatui::widgets::scrollbar::Set::begin: &'static str
+pub ratatui::widgets::scrollbar::Set::end: &'static str
+pub ratatui::widgets::scrollbar::Set::thumb: &'static str
+pub ratatui::widgets::scrollbar::Set::track: &'static str
+impl core::clone::Clone for ratatui::widgets::scrollbar::Set
+pub fn ratatui::widgets::scrollbar::Set::clone(&self) -> ratatui::widgets::scrollbar::Set
+impl core::fmt::Debug for ratatui::widgets::scrollbar::Set
+pub fn ratatui::widgets::scrollbar::Set::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::marker::Send for ratatui::widgets::scrollbar::Set
+impl core::marker::Sync for ratatui::widgets::scrollbar::Set
+impl core::marker::Unpin for ratatui::widgets::scrollbar::Set
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::scrollbar::Set
+impl core::panic::unwind_safe::UnwindSafe for ratatui::widgets::scrollbar::Set
+impl<T, U> core::convert::Into<U> for ratatui::widgets::scrollbar::Set where U: core::convert::From<T>
+pub fn ratatui::widgets::scrollbar::Set::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::scrollbar::Set where U: core::convert::Into<T>
+pub type ratatui::widgets::scrollbar::Set::Error = core::convert::Infallible
+pub fn ratatui::widgets::scrollbar::Set::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::widgets::scrollbar::Set where U: core::convert::TryFrom<T>
+pub type ratatui::widgets::scrollbar::Set::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::widgets::scrollbar::Set::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::widgets::scrollbar::Set where T: core::clone::Clone
+pub type ratatui::widgets::scrollbar::Set::Owned = T
+pub fn ratatui::widgets::scrollbar::Set::clone_into(&self, target: &mut T)
+pub fn ratatui::widgets::scrollbar::Set::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::widgets::scrollbar::Set where T: 'static + core::marker::Sized
+pub fn ratatui::widgets::scrollbar::Set::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::widgets::scrollbar::Set where T: core::marker::Sized
+pub fn ratatui::widgets::scrollbar::Set::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::scrollbar::Set where T: core::marker::Sized
+pub fn ratatui::widgets::scrollbar::Set::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::widgets::scrollbar::Set
+pub fn ratatui::widgets::scrollbar::Set::from(t: T) -> T
+pub const ratatui::widgets::scrollbar::DOUBLE_HORIZONTAL: _
+pub const ratatui::widgets::scrollbar::DOUBLE_VERTICAL: _
+pub const ratatui::widgets::scrollbar::HORIZONTAL: _
+pub const ratatui::widgets::scrollbar::VERTICAL: _
+pub enum ratatui::widgets::ScrollDirection
+pub ratatui::widgets::ScrollDirection::Backward
+pub ratatui::widgets::ScrollDirection::Forward
+pub enum ratatui::widgets::ScrollbarOrientation
+pub ratatui::widgets::ScrollbarOrientation::HorizontalBottom
+pub ratatui::widgets::ScrollbarOrientation::HorizontalTop
+pub ratatui::widgets::ScrollbarOrientation::VerticalLeft
+pub ratatui::widgets::ScrollbarOrientation::VerticalRight
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for ratatui::widgets::Axis<'a> where U: ratatui::style::Styled<Item = T>
+pub fn ratatui::widgets::Axis<'a>::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::Axis<'a>::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::widgets::Axis<'a>::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn ratatui::widgets::Axis<'a>::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::Axis<'a>::reset(self) -> T
+pub struct ratatui::widgets::Bar<'a>
+impl<'a> ratatui::widgets::Bar<'a>
+pub fn ratatui::widgets::Bar<'a>::label(self, label: ratatui::text::Line<'a>) -> ratatui::widgets::Bar<'a>
+pub fn ratatui::widgets::Bar<'a>::style(self, style: ratatui::style::Style) -> ratatui::widgets::Bar<'a>
+pub fn ratatui::widgets::Bar<'a>::text_value(self, text_value: alloc::string::String) -> ratatui::widgets::Bar<'a>
+pub fn ratatui::widgets::Bar<'a>::value(self, value: u64) -> ratatui::widgets::Bar<'a>
+pub fn ratatui::widgets::Bar<'a>::value_style(self, style: ratatui::style::Style) -> ratatui::widgets::Bar<'a>
+impl<'a> core::clone::Clone for ratatui::widgets::Bar<'a>
+pub fn ratatui::widgets::Bar<'a>::clone(&self) -> ratatui::widgets::Bar<'a>
+impl<'a> core::default::Default for ratatui::widgets::Bar<'a>
+pub fn ratatui::widgets::Bar<'a>::default() -> ratatui::widgets::Bar<'a>
+impl<'a> core::fmt::Debug for ratatui::widgets::Bar<'a>
+pub fn ratatui::widgets::Bar<'a>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl<'a> core::marker::Send for ratatui::widgets::Bar<'a>
+impl<'a> core::marker::Sync for ratatui::widgets::Bar<'a>
+impl<'a> core::marker::Unpin for ratatui::widgets::Bar<'a>
+impl<'a> core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::Bar<'a>
+impl<'a> core::panic::unwind_safe::UnwindSafe for ratatui::widgets::Bar<'a>
+impl<T, U> core::convert::Into<U> for ratatui::widgets::Bar<'a> where U: core::convert::From<T>
+pub fn ratatui::widgets::Bar<'a>::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::Bar<'a> where U: core::convert::Into<T>
+pub type ratatui::widgets::Bar<'a>::Error = core::convert::Infallible
+pub fn ratatui::widgets::Bar<'a>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::widgets::Bar<'a> where U: core::convert::TryFrom<T>
+pub type ratatui::widgets::Bar<'a>::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::widgets::Bar<'a>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::widgets::Bar<'a> where T: core::clone::Clone
+pub type ratatui::widgets::Bar<'a>::Owned = T
+pub fn ratatui::widgets::Bar<'a>::clone_into(&self, target: &mut T)
+pub fn ratatui::widgets::Bar<'a>::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::widgets::Bar<'a> where T: 'static + core::marker::Sized
+pub fn ratatui::widgets::Bar<'a>::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::widgets::Bar<'a> where T: core::marker::Sized
+pub fn ratatui::widgets::Bar<'a>::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::Bar<'a> where T: core::marker::Sized
+pub fn ratatui::widgets::Bar<'a>::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::widgets::Bar<'a>
+pub fn ratatui::widgets::Bar<'a>::from(t: T) -> T
+pub fn ratatui::widgets::BarChart<'a>::block(self, block: ratatui::widgets::block::Block<'a>) -> ratatui::widgets::BarChart<'a>
+pub fn ratatui::widgets::BarChart<'a>::data(self, data: impl core::convert::Into<ratatui::widgets::BarGroup<'a>>) -> ratatui::widgets::BarChart<'a>
+pub fn ratatui::widgets::BarChart<'a>::group_gap(self, gap: u16) -> ratatui::widgets::BarChart<'a>
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for ratatui::widgets::BarChart<'a> where U: ratatui::style::Styled<Item = T>
+pub fn ratatui::widgets::BarChart<'a>::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::BarChart<'a>::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::widgets::BarChart<'a>::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn ratatui::widgets::BarChart<'a>::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::BarChart<'a>::reset(self) -> T
+pub struct ratatui::widgets::BarGroup<'a>
+impl<'a> ratatui::widgets::BarGroup<'a>
+pub fn ratatui::widgets::BarGroup<'a>::bars(self, bars: &[ratatui::widgets::Bar<'a>]) -> ratatui::widgets::BarGroup<'a>
+pub fn ratatui::widgets::BarGroup<'a>::label(self, label: ratatui::text::Line<'a>) -> ratatui::widgets::BarGroup<'a>
+impl<'a, const N: usize> core::convert::From<&[(&'a str, u64); N]> for ratatui::widgets::BarGroup<'a>
+pub fn ratatui::widgets::BarGroup<'a>::from(value: &[(&'a str, u64); N]) -> ratatui::widgets::BarGroup<'a>
+impl<'a> core::convert::From<&[(&'a str, u64)]> for ratatui::widgets::BarGroup<'a>
+pub fn ratatui::widgets::BarGroup<'a>::from(value: &[(&'a str, u64)]) -> ratatui::widgets::BarGroup<'a>
+impl<'a> core::convert::From<&alloc::vec::Vec<(&'a str, u64), alloc::alloc::Global>> for ratatui::widgets::BarGroup<'a>
+pub fn ratatui::widgets::BarGroup<'a>::from(value: &alloc::vec::Vec<(&'a str, u64)>) -> ratatui::widgets::BarGroup<'a>
+impl<'a> core::clone::Clone for ratatui::widgets::BarGroup<'a>
+pub fn ratatui::widgets::BarGroup<'a>::clone(&self) -> ratatui::widgets::BarGroup<'a>
+impl<'a> core::default::Default for ratatui::widgets::BarGroup<'a>
+pub fn ratatui::widgets::BarGroup<'a>::default() -> ratatui::widgets::BarGroup<'a>
+impl<'a> core::fmt::Debug for ratatui::widgets::BarGroup<'a>
+pub fn ratatui::widgets::BarGroup<'a>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl<'a> core::marker::Send for ratatui::widgets::BarGroup<'a>
+impl<'a> core::marker::Sync for ratatui::widgets::BarGroup<'a>
+impl<'a> core::marker::Unpin for ratatui::widgets::BarGroup<'a>
+impl<'a> core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::BarGroup<'a>
+impl<'a> core::panic::unwind_safe::UnwindSafe for ratatui::widgets::BarGroup<'a>
+impl<T, U> core::convert::Into<U> for ratatui::widgets::BarGroup<'a> where U: core::convert::From<T>
+pub fn ratatui::widgets::BarGroup<'a>::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::BarGroup<'a> where U: core::convert::Into<T>
+pub type ratatui::widgets::BarGroup<'a>::Error = core::convert::Infallible
+pub fn ratatui::widgets::BarGroup<'a>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::widgets::BarGroup<'a> where U: core::convert::TryFrom<T>
+pub type ratatui::widgets::BarGroup<'a>::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::widgets::BarGroup<'a>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::widgets::BarGroup<'a> where T: core::clone::Clone
+pub type ratatui::widgets::BarGroup<'a>::Owned = T
+pub fn ratatui::widgets::BarGroup<'a>::clone_into(&self, target: &mut T)
+pub fn ratatui::widgets::BarGroup<'a>::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::widgets::BarGroup<'a> where T: 'static + core::marker::Sized
+pub fn ratatui::widgets::BarGroup<'a>::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::widgets::BarGroup<'a> where T: core::marker::Sized
+pub fn ratatui::widgets::BarGroup<'a>::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::BarGroup<'a> where T: core::marker::Sized
+pub fn ratatui::widgets::BarGroup<'a>::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::widgets::BarGroup<'a>
+pub fn ratatui::widgets::BarGroup<'a>::from(t: T) -> T
+impl ratatui::widgets::Borders
+impl ratatui::widgets::Borders
+pub const fn ratatui::widgets::Borders::from_bits_retain(bits: u8) -> Self
+pub fn ratatui::widgets::Borders::from_name(name: &str) -> core::option::Option<Self>
+pub const fn ratatui::widgets::Borders::iter(&self) -> bitflags::iter::Iter<ratatui::widgets::Borders>
+pub const fn ratatui::widgets::Borders::iter_names(&self) -> bitflags::iter::IterNames<ratatui::widgets::Borders>
+impl bitflags::traits::Flags for ratatui::widgets::Borders
+pub type ratatui::widgets::Borders::Bits = u8
+pub const ratatui::widgets::Borders::FLAGS: &'static [bitflags::traits::Flag<ratatui::widgets::Borders>]
+pub fn ratatui::widgets::Borders::bits(&self) -> u8
+pub fn ratatui::widgets::Borders::from_bits_retain(bits: u8) -> ratatui::widgets::Borders
+impl bitflags::traits::PublicFlags for ratatui::widgets::Borders
+pub type ratatui::widgets::Borders::Internal = InternalBitFlags
+pub type ratatui::widgets::Borders::Primitive = u8
+impl core::iter::traits::collect::IntoIterator for ratatui::widgets::Borders
+pub type ratatui::widgets::Borders::IntoIter = bitflags::iter::Iter<ratatui::widgets::Borders>
+pub type ratatui::widgets::Borders::Item = ratatui::widgets::Borders
+pub fn ratatui::widgets::Borders::into_iter(self) -> Self::IntoIter
+impl core::default::Default for ratatui::widgets::Borders
+pub fn ratatui::widgets::Borders::default() -> ratatui::widgets::Borders
+impl<B> bitflags::traits::BitFlags for ratatui::widgets::Borders where B: bitflags::traits::Flags
+pub type ratatui::widgets::Borders::Iter = bitflags::iter::Iter<B>
+pub type ratatui::widgets::Borders::IterNames = bitflags::iter::IterNames<B>
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for ratatui::widgets::Cell<'a> where U: ratatui::style::Styled<Item = T>
+pub fn ratatui::widgets::Cell<'a>::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::Cell<'a>::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::widgets::Cell<'a>::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn ratatui::widgets::Cell<'a>::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::Cell<'a>::reset(self) -> T
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for ratatui::widgets::Chart<'a> where U: ratatui::style::Styled<Item = T>
+pub fn ratatui::widgets::Chart<'a>::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::Chart<'a>::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::widgets::Chart<'a>::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn ratatui::widgets::Chart<'a>::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::Chart<'a>::reset(self) -> T
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for ratatui::widgets::Dataset<'a> where U: ratatui::style::Styled<Item = T>
+pub fn ratatui::widgets::Dataset<'a>::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::Dataset<'a>::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::widgets::Dataset<'a>::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn ratatui::widgets::Dataset<'a>::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::Dataset<'a>::reset(self) -> T
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for ratatui::widgets::Gauge<'a> where U: ratatui::style::Styled<Item = T>
+pub fn ratatui::widgets::Gauge<'a>::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::Gauge<'a>::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::widgets::Gauge<'a>::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn ratatui::widgets::Gauge<'a>::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::Gauge<'a>::reset(self) -> T
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for ratatui::widgets::LineGauge<'a> where U: ratatui::style::Styled<Item = T>
+pub fn ratatui::widgets::LineGauge<'a>::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::LineGauge<'a>::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::widgets::LineGauge<'a>::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn ratatui::widgets::LineGauge<'a>::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::LineGauge<'a>::reset(self) -> T
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for ratatui::widgets::List<'a> where U: ratatui::style::Styled<Item = T>
+pub fn ratatui::widgets::List<'a>::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::List<'a>::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::widgets::List<'a>::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn ratatui::widgets::List<'a>::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::List<'a>::reset(self) -> T
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for ratatui::widgets::ListItem<'a> where U: ratatui::style::Styled<Item = T>
+pub fn ratatui::widgets::ListItem<'a>::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::ListItem<'a>::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::widgets::ListItem<'a>::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn ratatui::widgets::ListItem<'a>::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::ListItem<'a>::reset(self) -> T
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for ratatui::widgets::Paragraph<'a> where U: ratatui::style::Styled<Item = T>
+pub fn ratatui::widgets::Paragraph<'a>::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::Paragraph<'a>::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::widgets::Paragraph<'a>::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn ratatui::widgets::Paragraph<'a>::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::Paragraph<'a>::reset(self) -> T
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for ratatui::widgets::Row<'a> where U: ratatui::style::Styled<Item = T>
+pub fn ratatui::widgets::Row<'a>::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::Row<'a>::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::widgets::Row<'a>::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn ratatui::widgets::Row<'a>::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::Row<'a>::reset(self) -> T
+pub struct ratatui::widgets::Scrollbar<'a>
+pub struct ratatui::widgets::ScrollbarState
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for ratatui::widgets::Sparkline<'a> where U: ratatui::style::Styled<Item = T>
+pub fn ratatui::widgets::Sparkline<'a>::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::Sparkline<'a>::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::widgets::Sparkline<'a>::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn ratatui::widgets::Sparkline<'a>::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::Sparkline<'a>::reset(self) -> T
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for ratatui::widgets::Table<'a> where U: ratatui::style::Styled<Item = T>
+pub fn ratatui::widgets::Table<'a>::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::Table<'a>::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::widgets::Table<'a>::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn ratatui::widgets::Table<'a>::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::Table<'a>::reset(self) -> T
+impl<'a, T, U> ratatui::style::Stylize<'a, T> for ratatui::widgets::Tabs<'a> where U: ratatui::style::Styled<Item = T>
+pub fn ratatui::widgets::Tabs<'a>::add_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::Tabs<'a>::bg(self, color: ratatui::style::Color) -> T
+pub fn ratatui::widgets::Tabs<'a>::fg<S>(self, color: S) -> T where S: core::convert::Into<ratatui::style::Color>
+pub fn ratatui::widgets::Tabs<'a>::remove_modifier(self, modifier: ratatui::style::Modifier) -> T
+pub fn ratatui::widgets::Tabs<'a>::reset(self) -> T
```

</details>

<details>
<summary>
Public API differences between ratatui 0.20.1 and ratatui 0.21.0
</summary>

```diff
❯ cargo public-api diff v0.20.1..v0.21.0

Removed items from the public API
=================================
-pub fn ratatui::style::Style::bg(self, color: ratatui::style::Color) -> ratatui::style::Style
-pub fn ratatui::style::Style::fg(self, color: ratatui::style::Color) -> ratatui::style::Style
-pub fn ratatui::style::Style::reset() -> ratatui::style::Style
-pub fn ratatui::terminal::Terminal<B>::resize(&mut self, area: ratatui::layout::Rect) -> std::io::error::Result<()>
-pub fn ratatui::terminal::Terminal<B>::resize(&mut self, area: ratatui::layout::Rect) -> std::io::error::Result<()>
-pub struct ratatui::terminal::Viewport
-impl ratatui::terminal::Viewport
-impl ratatui::terminal::Viewport
-pub fn ratatui::terminal::Viewport::fixed(area: ratatui::layout::Rect) -> ratatui::terminal::Viewport
-pub fn ratatui::terminal::Viewport::fixed(area: ratatui::layout::Rect) -> ratatui::terminal::Viewport
-impl<'a> core::iter::traits::collect::Extend<ratatui::text::Spans<'a>> for ratatui::text::Text<'a>
-impl<'a> core::iter::traits::collect::Extend<ratatui::text::Spans<'a>> for ratatui::text::Text<'a>
-pub fn ratatui::text::Text<'a>::extend<T: core::iter::traits::collect::IntoIterator<Item = ratatui::text::Spans<'a>>>(&mut self, iter: T)
-pub fn ratatui::text::Text<'a>::extend<T: core::iter::traits::collect::IntoIterator<Item = ratatui::text::Spans<'a>>>(&mut self, iter: T)
-pub fn ratatui::widgets::Block<'a>::title<T>(self, title: T) -> ratatui::widgets::Block<'a> where T: core::convert::Into<ratatui::text::Spans<'a>>
-pub struct ratatui::Viewport

Changed items in the public API
===============================
-pub ratatui::text::Text::lines: alloc::vec::Vec<ratatui::text::Spans<'a>>
+pub ratatui::text::Text::lines: alloc::vec::Vec<ratatui::text::Line<'a>>
-pub type ratatui::text::Text<'a>::Item = ratatui::text::Spans<'a>
+pub type ratatui::text::Text<'a>::Item = ratatui::text::Line<'a>
-pub fn ratatui::widgets::canvas::Context<'a>::print<T>(&mut self, x: f64, y: f64, spans: T) where T: core::convert::Into<ratatui::text::Spans<'a>>
+pub fn ratatui::widgets::canvas::Context<'a>::print<T>(&mut self, x: f64, y: f64, line: T) where T: core::convert::Into<ratatui::text::Line<'a>>
-pub fn ratatui::widgets::Axis<'a>::title<T>(self, title: T) -> ratatui::widgets::Axis<'a> where T: core::convert::Into<ratatui::text::Spans<'a>>
+pub fn ratatui::widgets::Axis<'a>::title<T>(self, title: T) -> ratatui::widgets::Axis<'a> where T: core::convert::Into<ratatui::text::Line<'a>>
-pub fn ratatui::widgets::LineGauge<'a>::label<T>(self, label: T) -> Self where T: core::convert::Into<ratatui::text::Spans<'a>>
+pub fn ratatui::widgets::LineGauge<'a>::label<T>(self, label: T) -> Self where T: core::convert::Into<ratatui::text::Line<'a>>
-pub fn ratatui::widgets::Tabs<'a>::new(titles: alloc::vec::Vec<ratatui::text::Spans<'a>>) -> ratatui::widgets::Tabs<'a>
+pub fn ratatui::widgets::Tabs<'a>::new<T>(titles: alloc::vec::Vec<T>) -> ratatui::widgets::Tabs<'a> where T: core::convert::Into<ratatui::text::Line<'a>>

Added items to the public API
=============================
+pub enum ratatui::backend::ClearType
+pub ratatui::backend::ClearType::AfterCursor
+pub ratatui::backend::ClearType::All
+pub ratatui::backend::ClearType::BeforeCursor
+pub ratatui::backend::ClearType::CurrentLine
+pub ratatui::backend::ClearType::UntilNewLine
+impl core::clone::Clone for ratatui::backend::ClearType
+pub fn ratatui::backend::ClearType::clone(&self) -> ratatui::backend::ClearType
+impl core::cmp::Eq for ratatui::backend::ClearType
+impl core::cmp::PartialEq<ratatui::backend::ClearType> for ratatui::backend::ClearType
+pub fn ratatui::backend::ClearType::eq(&self, other: &ratatui::backend::ClearType) -> bool
+impl core::fmt::Debug for ratatui::backend::ClearType
+pub fn ratatui::backend::ClearType::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::marker::Copy for ratatui::backend::ClearType
+impl core::marker::StructuralEq for ratatui::backend::ClearType
+impl core::marker::StructuralPartialEq for ratatui::backend::ClearType
+impl core::marker::Send for ratatui::backend::ClearType
+impl core::marker::Sync for ratatui::backend::ClearType
+impl core::marker::Unpin for ratatui::backend::ClearType
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::backend::ClearType
+impl core::panic::unwind_safe::UnwindSafe for ratatui::backend::ClearType
+impl<T, U> core::convert::Into<U> for ratatui::backend::ClearType where U: core::convert::From<T>
+pub fn ratatui::backend::ClearType::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::backend::ClearType where U: core::convert::Into<T>
+pub type ratatui::backend::ClearType::Error = core::convert::Infallible
+pub fn ratatui::backend::ClearType::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::backend::ClearType where U: core::convert::TryFrom<T>
+pub type ratatui::backend::ClearType::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::backend::ClearType::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::backend::ClearType where T: core::clone::Clone
+pub type ratatui::backend::ClearType::Owned = T
+pub fn ratatui::backend::ClearType::clone_into(&self, target: &mut T)
+pub fn ratatui::backend::ClearType::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::backend::ClearType where T: 'static + core::marker::Sized
+pub fn ratatui::backend::ClearType::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::backend::ClearType where T: core::marker::Sized
+pub fn ratatui::backend::ClearType::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::backend::ClearType where T: core::marker::Sized
+pub fn ratatui::backend::ClearType::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::backend::ClearType
+pub fn ratatui::backend::ClearType::from(t: T) -> T
+pub fn ratatui::backend::CrosstermBackend<W>::append_lines(&mut self, n: u16) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::append_lines(&mut self, n: u16) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::clear_region(&mut self, clear_type: ratatui::backend::ClearType) -> std::io::error::Result<()>
+pub fn ratatui::backend::CrosstermBackend<W>::clear_region(&mut self, clear_type: ratatui::backend::ClearType) -> std::io::error::Result<()>
+impl core::fmt::Display for ratatui::backend::TestBackend
+pub fn ratatui::backend::TestBackend::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl<T> alloc::string::ToString for ratatui::backend::TestBackend where T: core::fmt::Display + core::marker::Sized
+pub fn ratatui::backend::TestBackend::to_string(&self) -> alloc::string::String
+pub fn ratatui::backend::Backend::append_lines(&mut self, n: u16) -> std::io::error::Result<()>
+pub fn ratatui::backend::Backend::clear_region(&mut self, clear_type: ratatui::backend::ClearType) -> core::result::Result<(), std::io::error::Error>
+pub fn ratatui::buffer::Buffer::set_line(&mut self, x: u16, y: u16, line: &ratatui::text::Line<'_>, width: u16) -> (u16, u16)
+impl core::str::traits::FromStr for ratatui::style::Color
+pub type ratatui::style::Color::Err = ratatui::style::ParseColorError
+pub fn ratatui::style::Color::from_str(s: &str) -> core::result::Result<Self, Self::Err>
+pub struct ratatui::style::ParseColorError
+impl core::error::Error for ratatui::style::ParseColorError
+impl core::fmt::Display for ratatui::style::ParseColorError
+pub fn ratatui::style::ParseColorError::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::style::ParseColorError::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::fmt::Debug for ratatui::style::ParseColorError
+impl core::marker::Send for ratatui::style::ParseColorError
+impl core::marker::Sync for ratatui::style::ParseColorError
+impl core::marker::Unpin for ratatui::style::ParseColorError
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::style::ParseColorError
+impl core::panic::unwind_safe::UnwindSafe for ratatui::style::ParseColorError
+impl<E> core::any::Provider for ratatui::style::ParseColorError where E: core::error::Error + core::marker::Sized
+pub fn ratatui::style::ParseColorError::provide<'a>(&'a self, demand: &mut core::any::Demand<'a>)
+impl<T, U> core::convert::Into<U> for ratatui::style::ParseColorError where U: core::convert::From<T>
+pub fn ratatui::style::ParseColorError::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::style::ParseColorError where U: core::convert::Into<T>
+pub type ratatui::style::ParseColorError::Error = core::convert::Infallible
+pub fn ratatui::style::ParseColorError::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::style::ParseColorError where U: core::convert::TryFrom<T>
+pub type ratatui::style::ParseColorError::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::style::ParseColorError::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::string::ToString for ratatui::style::ParseColorError where T: core::fmt::Display + core::marker::Sized
+pub fn ratatui::style::ParseColorError::to_string(&self) -> alloc::string::String
+impl<T> core::any::Any for ratatui::style::ParseColorError where T: 'static + core::marker::Sized
+pub fn ratatui::style::ParseColorError::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::style::ParseColorError where T: core::marker::Sized
+pub fn ratatui::style::ParseColorError::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::style::ParseColorError where T: core::marker::Sized
+pub fn ratatui::style::ParseColorError::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::style::ParseColorError
+pub fn ratatui::style::ParseColorError::from(t: T) -> T
+pub const fn ratatui::style::Style::bg(self, color: ratatui::style::Color) -> ratatui::style::Style
+pub const fn ratatui::style::Style::fg(self, color: ratatui::style::Color) -> ratatui::style::Style
+pub const fn ratatui::style::Style::new() -> ratatui::style::Style
+pub const fn ratatui::style::Style::reset() -> ratatui::style::Style
+pub ratatui::symbols::Marker::Bar
+pub enum ratatui::terminal::Viewport
+pub ratatui::terminal::Viewport::Fixed(ratatui::layout::Rect)
+pub ratatui::terminal::Viewport::Fullscreen
+pub ratatui::terminal::Viewport::Inline(u16)
+impl core::cmp::Eq for ratatui::terminal::Viewport
+impl core::cmp::Eq for ratatui::terminal::Viewport
+impl core::marker::StructuralEq for ratatui::terminal::Viewport
+impl core::marker::StructuralEq for ratatui::terminal::Viewport
+pub fn ratatui::terminal::Terminal<B>::insert_before<F>(&mut self, height: u16, draw_fn: F) -> std::io::error::Result<()> where F: core::ops::function::FnOnce(&mut ratatui::buffer::Buffer)
+pub fn ratatui::terminal::Terminal<B>::insert_before<F>(&mut self, height: u16, draw_fn: F) -> std::io::error::Result<()> where F: core::ops::function::FnOnce(&mut ratatui::buffer::Buffer)
+pub fn ratatui::terminal::Terminal<B>::resize(&mut self, size: ratatui::layout::Rect) -> std::io::error::Result<()>
+pub fn ratatui::terminal::Terminal<B>::resize(&mut self, size: ratatui::layout::Rect) -> std::io::error::Result<()>
+impl core::cmp::Eq for ratatui::terminal::TerminalOptions
+impl core::cmp::Eq for ratatui::terminal::TerminalOptions
+impl core::marker::StructuralEq for ratatui::terminal::TerminalOptions
+impl core::marker::StructuralEq for ratatui::terminal::TerminalOptions
+pub struct ratatui::text::Line<'a>
+pub ratatui::text::Line::alignment: core::option::Option<ratatui::layout::Alignment>
+pub ratatui::text::Line::spans: alloc::vec::Vec<ratatui::text::Span<'a>>
+impl<'a> ratatui::text::Line<'a>
+pub fn ratatui::text::Line<'a>::alignment(self, alignment: ratatui::layout::Alignment) -> Self
+pub fn ratatui::text::Line<'a>::patch_style(&mut self, style: ratatui::style::Style)
+pub fn ratatui::text::Line<'a>::reset_style(&mut self)
+pub fn ratatui::text::Line<'a>::width(&self) -> usize
+impl<'a> core::convert::From<&'a str> for ratatui::text::Line<'a>
+pub fn ratatui::text::Line<'a>::from(s: &'a str) -> Self
+impl<'a> core::convert::From<alloc::string::String> for ratatui::text::Line<'a>
+pub fn ratatui::text::Line<'a>::from(s: alloc::string::String) -> Self
+impl<'a> core::convert::From<alloc::vec::Vec<ratatui::text::Span<'a>, alloc::alloc::Global>> for ratatui::text::Line<'a>
+pub fn ratatui::text::Line<'a>::from(spans: alloc::vec::Vec<ratatui::text::Span<'a>>) -> Self
+impl<'a> core::convert::From<ratatui::text::Line<'a>> for alloc::string::String
+pub fn alloc::string::String::from(line: ratatui::text::Line<'a>) -> alloc::string::String
+impl<'a> core::convert::From<ratatui::text::Line<'a>> for ratatui::text::Text<'a>
+impl<'a> core::convert::From<ratatui::text::Line<'a>> for ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(line: ratatui::text::Line<'a>) -> ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(line: ratatui::text::Line<'a>) -> ratatui::text::Text<'a>
+impl<'a> core::convert::From<ratatui::text::Span<'a>> for ratatui::text::Line<'a>
+impl<'a> core::convert::From<ratatui::text::Span<'a>> for ratatui::text::Line<'a>
+pub fn ratatui::text::Line<'a>::from(span: ratatui::text::Span<'a>) -> Self
+pub fn ratatui::text::Line<'a>::from(span: ratatui::text::Span<'a>) -> Self
+impl<'a> core::convert::From<ratatui::text::Spans<'a>> for ratatui::text::Line<'a>
+impl<'a> core::convert::From<ratatui::text::Spans<'a>> for ratatui::text::Line<'a>
+pub fn ratatui::text::Line<'a>::from(value: ratatui::text::Spans<'a>) -> Self
+pub fn ratatui::text::Line<'a>::from(value: ratatui::text::Spans<'a>) -> Self
+impl<'a> core::clone::Clone for ratatui::text::Line<'a>
+pub fn ratatui::text::Line<'a>::clone(&self) -> ratatui::text::Line<'a>
+impl<'a> core::cmp::Eq for ratatui::text::Line<'a>
+impl<'a> core::cmp::PartialEq<ratatui::text::Line<'a>> for ratatui::text::Line<'a>
+pub fn ratatui::text::Line<'a>::eq(&self, other: &ratatui::text::Line<'a>) -> bool
+impl<'a> core::default::Default for ratatui::text::Line<'a>
+pub fn ratatui::text::Line<'a>::default() -> ratatui::text::Line<'a>
+impl<'a> core::fmt::Debug for ratatui::text::Line<'a>
+pub fn ratatui::text::Line<'a>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl<'a> core::marker::StructuralEq for ratatui::text::Line<'a>
+impl<'a> core::marker::StructuralPartialEq for ratatui::text::Line<'a>
+impl<'a> core::marker::Send for ratatui::text::Line<'a>
+impl<'a> core::marker::Sync for ratatui::text::Line<'a>
+impl<'a> core::marker::Unpin for ratatui::text::Line<'a>
+impl<'a> core::panic::unwind_safe::RefUnwindSafe for ratatui::text::Line<'a>
+impl<'a> core::panic::unwind_safe::UnwindSafe for ratatui::text::Line<'a>
+impl<T, U> core::convert::Into<U> for ratatui::text::Line<'a> where U: core::convert::From<T>
+pub fn ratatui::text::Line<'a>::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::text::Line<'a> where U: core::convert::Into<T>
+pub type ratatui::text::Line<'a>::Error = core::convert::Infallible
+pub fn ratatui::text::Line<'a>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::text::Line<'a> where U: core::convert::TryFrom<T>
+pub type ratatui::text::Line<'a>::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::text::Line<'a>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::text::Line<'a> where T: core::clone::Clone
+pub type ratatui::text::Line<'a>::Owned = T
+pub fn ratatui::text::Line<'a>::clone_into(&self, target: &mut T)
+pub fn ratatui::text::Line<'a>::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::text::Line<'a> where T: 'static + core::marker::Sized
+pub fn ratatui::text::Line<'a>::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::text::Line<'a> where T: core::marker::Sized
+pub fn ratatui::text::Line<'a>::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::text::Line<'a> where T: core::marker::Sized
+pub fn ratatui::text::Line<'a>::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::text::Line<'a>
+pub fn ratatui::text::Line<'a>::from(t: T) -> T
+pub struct ratatui::text::Masked<'a>
+impl<'a> ratatui::text::Masked<'a>
+pub fn ratatui::text::Masked<'a>::mask_char(&self) -> char
+pub fn ratatui::text::Masked<'a>::new(s: impl core::convert::Into<alloc::borrow::Cow<'a, str>>, mask_char: char) -> Self
+pub fn ratatui::text::Masked<'a>::value(&self) -> alloc::borrow::Cow<'a, str>
+impl core::fmt::Debug for ratatui::text::Masked<'_>
+pub fn ratatui::text::Masked<'_>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+pub fn ratatui::text::Masked<'_>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::fmt::Display for ratatui::text::Masked<'_>
+impl<'a> core::convert::From<&'a ratatui::text::Masked<'_>> for ratatui::text::Text<'a>
+impl<'a> core::convert::From<&'a ratatui::text::Masked<'_>> for ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(masked: &'a ratatui::text::Masked<'_>) -> ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(masked: &'a ratatui::text::Masked<'_>) -> ratatui::text::Text<'a>
+impl<'a> core::convert::From<&'a ratatui::text::Masked<'a>> for alloc::borrow::Cow<'a, str>
+pub fn alloc::borrow::Cow<'a, str>::from(masked: &'a ratatui::text::Masked<'_>) -> alloc::borrow::Cow<'a, str>
+impl<'a> core::convert::From<ratatui::text::Masked<'a>> for alloc::borrow::Cow<'a, str>
+pub fn alloc::borrow::Cow<'a, str>::from(masked: ratatui::text::Masked<'a>) -> alloc::borrow::Cow<'a, str>
+impl<'a> core::convert::From<ratatui::text::Masked<'a>> for ratatui::text::Text<'a>
+impl<'a> core::convert::From<ratatui::text::Masked<'a>> for ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(masked: ratatui::text::Masked<'a>) -> ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(masked: ratatui::text::Masked<'a>) -> ratatui::text::Text<'a>
+impl<'a> core::clone::Clone for ratatui::text::Masked<'a>
+pub fn ratatui::text::Masked<'a>::clone(&self) -> ratatui::text::Masked<'a>
+impl<'a> core::marker::Send for ratatui::text::Masked<'a>
+impl<'a> core::marker::Sync for ratatui::text::Masked<'a>
+impl<'a> core::marker::Unpin for ratatui::text::Masked<'a>
+impl<'a> core::panic::unwind_safe::RefUnwindSafe for ratatui::text::Masked<'a>
+impl<'a> core::panic::unwind_safe::UnwindSafe for ratatui::text::Masked<'a>
+impl<T, U> core::convert::Into<U> for ratatui::text::Masked<'a> where U: core::convert::From<T>
+pub fn ratatui::text::Masked<'a>::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::text::Masked<'a> where U: core::convert::Into<T>
+pub type ratatui::text::Masked<'a>::Error = core::convert::Infallible
+pub fn ratatui::text::Masked<'a>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::text::Masked<'a> where U: core::convert::TryFrom<T>
+pub type ratatui::text::Masked<'a>::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::text::Masked<'a>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::text::Masked<'a> where T: core::clone::Clone
+pub type ratatui::text::Masked<'a>::Owned = T
+pub fn ratatui::text::Masked<'a>::clone_into(&self, target: &mut T)
+pub fn ratatui::text::Masked<'a>::to_owned(&self) -> T
+impl<T> alloc::string::ToString for ratatui::text::Masked<'a> where T: core::fmt::Display + core::marker::Sized
+pub fn ratatui::text::Masked<'a>::to_string(&self) -> alloc::string::String
+impl<T> core::any::Any for ratatui::text::Masked<'a> where T: 'static + core::marker::Sized
+pub fn ratatui::text::Masked<'a>::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::text::Masked<'a> where T: core::marker::Sized
+pub fn ratatui::text::Masked<'a>::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::text::Masked<'a> where T: core::marker::Sized
+pub fn ratatui::text::Masked<'a>::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::text::Masked<'a>
+pub fn ratatui::text::Masked<'a>::from(t: T) -> T
+pub fn ratatui::text::Span<'a>::patch_style(&mut self, style: ratatui::style::Style)
+pub fn ratatui::text::Span<'a>::reset_style(&mut self)
+pub fn ratatui::text::Spans<'a>::alignment(self, alignment: ratatui::layout::Alignment) -> ratatui::text::Line<'a>
+pub fn ratatui::text::Spans<'a>::patch_style(&mut self, style: ratatui::style::Style)
+pub fn ratatui::text::Spans<'a>::reset_style(&mut self)
+pub fn ratatui::text::Text<'a>::reset_style(&mut self)
+impl<'a, T> core::iter::traits::collect::Extend<T> for ratatui::text::Text<'a> where T: core::convert::Into<ratatui::text::Line<'a>>
+pub fn ratatui::text::Text<'a>::extend<I: core::iter::traits::collect::IntoIterator<Item = T>>(&mut self, iter: I)
+impl<'a> core::convert::From<alloc::vec::Vec<ratatui::text::Line<'a>, alloc::alloc::Global>> for ratatui::text::Text<'a>
+pub fn ratatui::text::Text<'a>::from(lines: alloc::vec::Vec<ratatui::text::Line<'a>>) -> ratatui::text::Text<'a>
+pub struct ratatui::widgets::canvas::Circle
+pub ratatui::widgets::canvas::Circle::color: ratatui::style::Color
+pub ratatui::widgets::canvas::Circle::radius: f64
+pub ratatui::widgets::canvas::Circle::x: f64
+pub ratatui::widgets::canvas::Circle::y: f64
+impl ratatui::widgets::canvas::Shape for ratatui::widgets::canvas::Circle
+impl ratatui::widgets::canvas::Shape for ratatui::widgets::canvas::Circle
+pub fn ratatui::widgets::canvas::Circle::draw(&self, painter: &mut ratatui::widgets::canvas::Painter<'_, '_>)
+pub fn ratatui::widgets::canvas::Circle::draw(&self, painter: &mut ratatui::widgets::canvas::Painter<'_, '_>)
+impl core::clone::Clone for ratatui::widgets::canvas::Circle
+pub fn ratatui::widgets::canvas::Circle::clone(&self) -> ratatui::widgets::canvas::Circle
+impl core::fmt::Debug for ratatui::widgets::canvas::Circle
+pub fn ratatui::widgets::canvas::Circle::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::marker::Send for ratatui::widgets::canvas::Circle
+impl core::marker::Sync for ratatui::widgets::canvas::Circle
+impl core::marker::Unpin for ratatui::widgets::canvas::Circle
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::canvas::Circle
+impl core::panic::unwind_safe::UnwindSafe for ratatui::widgets::canvas::Circle
+impl<T, U> core::convert::Into<U> for ratatui::widgets::canvas::Circle where U: core::convert::From<T>
+pub fn ratatui::widgets::canvas::Circle::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::canvas::Circle where U: core::convert::Into<T>
+pub type ratatui::widgets::canvas::Circle::Error = core::convert::Infallible
+pub fn ratatui::widgets::canvas::Circle::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::widgets::canvas::Circle where U: core::convert::TryFrom<T>
+pub type ratatui::widgets::canvas::Circle::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::widgets::canvas::Circle::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::widgets::canvas::Circle where T: core::clone::Clone
+pub type ratatui::widgets::canvas::Circle::Owned = T
+pub fn ratatui::widgets::canvas::Circle::clone_into(&self, target: &mut T)
+pub fn ratatui::widgets::canvas::Circle::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::widgets::canvas::Circle where T: 'static + core::marker::Sized
+pub fn ratatui::widgets::canvas::Circle::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::widgets::canvas::Circle where T: core::marker::Sized
+pub fn ratatui::widgets::canvas::Circle::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::canvas::Circle where T: core::marker::Sized
+pub fn ratatui::widgets::canvas::Circle::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::widgets::canvas::Circle
+pub fn ratatui::widgets::canvas::Circle::from(t: T) -> T
+pub enum ratatui::widgets::RenderDirection
+pub ratatui::widgets::RenderDirection::LeftToRight
+pub ratatui::widgets::RenderDirection::RightToLeft
+impl core::clone::Clone for ratatui::widgets::RenderDirection
+pub fn ratatui::widgets::RenderDirection::clone(&self) -> ratatui::widgets::RenderDirection
+impl core::fmt::Debug for ratatui::widgets::RenderDirection
+pub fn ratatui::widgets::RenderDirection::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::marker::Copy for ratatui::widgets::RenderDirection
+impl core::marker::Send for ratatui::widgets::RenderDirection
+impl core::marker::Sync for ratatui::widgets::RenderDirection
+impl core::marker::Unpin for ratatui::widgets::RenderDirection
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::RenderDirection
+impl core::panic::unwind_safe::UnwindSafe for ratatui::widgets::RenderDirection
+impl<T, U> core::convert::Into<U> for ratatui::widgets::RenderDirection where U: core::convert::From<T>
+pub fn ratatui::widgets::RenderDirection::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::RenderDirection where U: core::convert::Into<T>
+pub type ratatui::widgets::RenderDirection::Error = core::convert::Infallible
+pub fn ratatui::widgets::RenderDirection::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::widgets::RenderDirection where U: core::convert::TryFrom<T>
+pub type ratatui::widgets::RenderDirection::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::widgets::RenderDirection::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::widgets::RenderDirection where T: core::clone::Clone
+pub type ratatui::widgets::RenderDirection::Owned = T
+pub fn ratatui::widgets::RenderDirection::clone_into(&self, target: &mut T)
+pub fn ratatui::widgets::RenderDirection::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::widgets::RenderDirection where T: 'static + core::marker::Sized
+pub fn ratatui::widgets::RenderDirection::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::widgets::RenderDirection where T: core::marker::Sized
+pub fn ratatui::widgets::RenderDirection::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::RenderDirection where T: core::marker::Sized
+pub fn ratatui::widgets::RenderDirection::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::widgets::RenderDirection
+pub fn ratatui::widgets::RenderDirection::from(t: T) -> T
+pub fn ratatui::widgets::Block<'a>::padding(self, padding: ratatui::widgets::Padding) -> ratatui::widgets::Block<'a>
+pub fn ratatui::widgets::Block<'a>::title<T>(self, title: T) -> ratatui::widgets::Block<'a> where T: core::convert::Into<ratatui::text::Line<'a>>
+pub fn ratatui::widgets::Block<'a>::title_on_bottom(self) -> ratatui::widgets::Block<'a>
+pub fn ratatui::widgets::List<'a>::is_empty(&self) -> bool
+pub fn ratatui::widgets::List<'a>::len(&self) -> usize
+pub fn ratatui::widgets::ListState::offset(&self) -> usize
+pub fn ratatui::widgets::ListState::offset_mut(&mut self) -> &mut usize
+pub fn ratatui::widgets::ListState::with_offset(self, offset: usize) -> Self
+pub fn ratatui::widgets::ListState::with_selected(self, selected: core::option::Option<usize>) -> Self
+pub struct ratatui::widgets::Padding
+pub ratatui::widgets::Padding::bottom: u16
+pub ratatui::widgets::Padding::left: u16
+pub ratatui::widgets::Padding::right: u16
+pub ratatui::widgets::Padding::top: u16
+impl ratatui::widgets::Padding
+pub fn ratatui::widgets::Padding::horizontal(value: u16) -> Self
+pub fn ratatui::widgets::Padding::new(left: u16, right: u16, top: u16, bottom: u16) -> Self
+pub fn ratatui::widgets::Padding::uniform(value: u16) -> Self
+pub fn ratatui::widgets::Padding::vertical(value: u16) -> Self
+pub fn ratatui::widgets::Padding::zero() -> Self
+impl core::clone::Clone for ratatui::widgets::Padding
+pub fn ratatui::widgets::Padding::clone(&self) -> ratatui::widgets::Padding
+impl core::cmp::Eq for ratatui::widgets::Padding
+impl core::cmp::PartialEq<ratatui::widgets::Padding> for ratatui::widgets::Padding
+pub fn ratatui::widgets::Padding::eq(&self, other: &ratatui::widgets::Padding) -> bool
+impl core::fmt::Debug for ratatui::widgets::Padding
+pub fn ratatui::widgets::Padding::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
+impl core::hash::Hash for ratatui::widgets::Padding
+pub fn ratatui::widgets::Padding::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
+impl core::marker::StructuralEq for ratatui::widgets::Padding
+impl core::marker::StructuralPartialEq for ratatui::widgets::Padding
+impl core::marker::Send for ratatui::widgets::Padding
+impl core::marker::Sync for ratatui::widgets::Padding
+impl core::marker::Unpin for ratatui::widgets::Padding
+impl core::panic::unwind_safe::RefUnwindSafe for ratatui::widgets::Padding
+impl core::panic::unwind_safe::UnwindSafe for ratatui::widgets::Padding
+impl<T, U> core::convert::Into<U> for ratatui::widgets::Padding where U: core::convert::From<T>
+pub fn ratatui::widgets::Padding::into(self) -> U
+impl<T, U> core::convert::TryFrom<U> for ratatui::widgets::Padding where U: core::convert::Into<T>
+pub type ratatui::widgets::Padding::Error = core::convert::Infallible
+pub fn ratatui::widgets::Padding::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
+impl<T, U> core::convert::TryInto<U> for ratatui::widgets::Padding where U: core::convert::TryFrom<T>
+pub type ratatui::widgets::Padding::Error = <U as core::convert::TryFrom<T>>::Error
+pub fn ratatui::widgets::Padding::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
+impl<T> alloc::borrow::ToOwned for ratatui::widgets::Padding where T: core::clone::Clone
+pub type ratatui::widgets::Padding::Owned = T
+pub fn ratatui::widgets::Padding::clone_into(&self, target: &mut T)
+pub fn ratatui::widgets::Padding::to_owned(&self) -> T
+impl<T> core::any::Any for ratatui::widgets::Padding where T: 'static + core::marker::Sized
+pub fn ratatui::widgets::Padding::type_id(&self) -> core::any::TypeId
+impl<T> core::borrow::Borrow<T> for ratatui::widgets::Padding where T: core::marker::Sized
+pub fn ratatui::widgets::Padding::borrow(&self) -> &T
+impl<T> core::borrow::BorrowMut<T> for ratatui::widgets::Padding where T: core::marker::Sized
+pub fn ratatui::widgets::Padding::borrow_mut(&mut self) -> &mut T
+impl<T> core::convert::From<T> for ratatui::widgets::Padding
+pub fn ratatui::widgets::Padding::from(t: T) -> T
+pub fn ratatui::widgets::Sparkline<'a>::direction(self, direction: ratatui::widgets::RenderDirection) -> ratatui::widgets::Sparkline<'a>
+pub fn ratatui::widgets::TableState::offset_mut(&mut self) -> &mut usize
+pub fn ratatui::widgets::TableState::with_offset(self, offset: usize) -> Self
+pub fn ratatui::widgets::TableState::with_selected(self, selected: core::option::Option<usize>) -> Self
+pub macro ratatui::assert_buffer_eq!
+pub enum ratatui::Viewport
+pub ratatui::Viewport::Fixed(ratatui::layout::Rect)
+pub ratatui::Viewport::Fullscreen
+pub ratatui::Viewport::Inline(u16)
```
</details>

<details>
<summary>
Public API differences between tui 0.19 and ratatui 0.20.1:
</summary>

```diff
❯ cargo public-api diff ratatui-0.19.0-api..v0.20.1

Removed items from the public API
=================================
(none)

Changed items in the public API
===============================
-pub fn ratatui::buffer::Buffer::set_span<'a>(&mut self, x: u16, y: u16, span: &ratatui::text::Span<'a>, width: u16) -> (u16, u16)
+pub fn ratatui::buffer::Buffer::set_span(&mut self, x: u16, y: u16, span: &ratatui::text::Span<'_>, width: u16) -> (u16, u16)
-pub fn ratatui::buffer::Buffer::set_spans<'a>(&mut self, x: u16, y: u16, spans: &ratatui::text::Spans<'a>, width: u16) -> (u16, u16)
+pub fn ratatui::buffer::Buffer::set_spans(&mut self, x: u16, y: u16, spans: &ratatui::text::Spans<'_>, width: u16) -> (u16, u16)
-pub fn ratatui::layout::Layout::split(&self, area: ratatui::layout::Rect) -> alloc::vec::Vec<ratatui::layout::Rect>
+pub fn ratatui::layout::Layout::split(&self, area: ratatui::layout::Rect) -> alloc::rc::Rc<[ratatui::layout::Rect]>
-pub const fn ratatui::widgets::Borders::bits(&self) -> u32
+pub const fn ratatui::widgets::Borders::bits(&self) -> u8
-pub const fn ratatui::widgets::Borders::from_bits(bits: u32) -> core::option::Option<Self>
+pub const fn ratatui::widgets::Borders::from_bits(bits: u8) -> core::option::Option<Self>
-pub const fn ratatui::widgets::Borders::from_bits_truncate(bits: u32) -> Self
+pub const fn ratatui::widgets::Borders::from_bits_truncate(bits: u8) -> Self
-pub unsafe const fn ratatui::widgets::Borders::from_bits_unchecked(bits: u32) -> Self
+pub unsafe const fn ratatui::widgets::Borders::from_bits_unchecked(bits: u8) -> Self

Added items to the public API
=============================
+impl<W> crossterm::command::SynchronizedUpdate for ratatui::backend::CrosstermBackend<W> where W: std::io::Write + core::marker::Sized
+pub fn ratatui::backend::CrosstermBackend::sync_update<T>(&mut self, operations: impl core::ops::function::FnOnce(&mut W) -> T) -> core::result::Result<T, std::io::error::Error>
+pub fn ratatui::widgets::ListItem::width(&self) -> usize
+pub fn ratatui::widgets::TableState::offset(&self) -> usize
```

</details>

## TODO - Add backwards compatibility notes here

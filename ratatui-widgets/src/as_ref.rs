/// Implement `AsRef<Self>` for widget types to enable `as_ref()` in generic contexts.
///
/// This keeps widget rendering ergonomic when APIs accept `AsRef<WidgetType>` bounds, avoiding
/// the need for `(&widget).render(...)` just to satisfy a trait bound.
///
/// # Example
///
/// ```rust
/// use ratatui_widgets::block::Block;
///
/// let block = Block::default();
/// let block_ref: &Block<'_> = block.as_ref();
/// ```
///
/// # Generated impls
///
/// ```rust,ignore
/// // Non-generic widgets (e.g. Clear, RatatuiLogo).
/// impl AsRef<Clear> for Clear {
///     fn as_ref(&self) -> &Clear {
///         self
///     }
/// }
///
/// // Generic widgets (e.g. Block with a lifetime, Canvas with a lifetime + type parameter).
/// impl<'a> AsRef<Block<'a>> for Block<'a> {
///     fn as_ref(&self) -> &Block<'a> {
///         self
///     }
/// }
///
/// impl<'a, F> AsRef<Canvas<'a, F>> for Canvas<'a, F>
/// where
///     F: Fn(&mut Context),
/// {
///     fn as_ref(&self) -> &Canvas<'a, F> {
///         self
///     }
/// }
/// ```
macro_rules! impl_as_ref {
    ($type:ty, <$($gen:tt),+> $(where $($bounds:tt)+)?) => {
        impl<$($gen),+> AsRef<$type> for $type $(where $($bounds)+)? {
            fn as_ref(&self) -> &$type {
                self
            }
        }
    };
    ($type:ty) => {
        impl AsRef<$type> for $type {
            fn as_ref(&self) -> &$type {
                self
            }
        }
    };
}

impl_as_ref!(crate::barchart::BarChart<'a>, <'a>);
impl_as_ref!(crate::block::Block<'a>, <'a>);
impl_as_ref!(crate::canvas::Canvas<'a, F>, <'a, F> where F: Fn(&mut crate::canvas::Context));
impl_as_ref!(crate::chart::Chart<'a>, <'a>);
impl_as_ref!(crate::clear::Clear);
impl_as_ref!(crate::gauge::Gauge<'a>, <'a>);
impl_as_ref!(crate::gauge::LineGauge<'a>, <'a>);
impl_as_ref!(crate::list::List<'a>, <'a>);
impl_as_ref!(crate::logo::RatatuiLogo);
impl_as_ref!(crate::mascot::RatatuiMascot);
impl_as_ref!(crate::paragraph::Paragraph<'a>, <'a>);
impl_as_ref!(crate::scrollbar::Scrollbar<'a>, <'a>);
impl_as_ref!(crate::sparkline::Sparkline<'a>, <'a>);
impl_as_ref!(crate::table::Table<'a>, <'a>);
impl_as_ref!(crate::tabs::Tabs<'a>, <'a>);
#[cfg(feature = "calendar")]
impl_as_ref!(
    crate::calendar::Monthly<'a, DS>,
    <'a, DS> where DS: crate::calendar::DateStyler
);

#[cfg(test)]
mod tests {
    use alloc::vec;

    #[test]
    fn widgets_implement_as_ref() {
        let _ = crate::barchart::BarChart::default().as_ref();
        let _ = crate::block::Block::new().as_ref();
        let _ = crate::canvas::Canvas::default().paint(|_| {}).as_ref();
        let _ = crate::chart::Chart::new(vec![]).as_ref();
        let _ = crate::clear::Clear.as_ref();
        let _ = crate::gauge::Gauge::default().as_ref();
        let _ = crate::gauge::LineGauge::default().as_ref();
        let _ = crate::list::List::new(["foo"]).as_ref();
        let _ = crate::logo::RatatuiLogo::default().as_ref();
        let _ = crate::mascot::RatatuiMascot::default().as_ref();
        let _ = crate::paragraph::Paragraph::new("").as_ref();
        let _ = crate::scrollbar::Scrollbar::default().as_ref();
        let _ = crate::sparkline::Sparkline::default().as_ref();
        let _ = crate::table::Table::default().as_ref();
        let _ = crate::tabs::Tabs::default().as_ref();
    }

    #[cfg(feature = "calendar")]
    #[test]
    fn calendar_widget_implements_as_ref() {
        use time::{Date, Month};

        let date = Date::from_calendar_date(2024, Month::January, 1).unwrap();
        let _ = crate::calendar::Monthly::new(date, crate::calendar::CalendarEventStore::default())
            .as_ref();
    }
}

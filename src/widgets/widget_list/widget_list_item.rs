use crate::{
    buffer::Buffer,
    layout::{Rect, Size},
    widgets::{SizeHint, Widget},
};

pub enum WidgetListItem<T, U>
where
    T: Widget + SizeHint,
    U: Widget + SizeHint,
{
    One(T),
    Two(U),
}

pub enum WidgetListItem3<T, U, W>
where
    T: Widget + SizeHint,
    U: Widget + SizeHint,
    W: Widget + SizeHint,
{
    One(T),
    Two(U),
    Three(W),
}

pub enum WidgetListItem4<T, U, W, X>
where
    T: Widget + SizeHint,
    U: Widget + SizeHint,
    W: Widget + SizeHint,
    X: Widget + SizeHint,
{
    One(T),
    Two(U),
    Three(W),
    Four(X),
}

pub enum WidgetListItem5<T, U, W, X, Z>
where
    T: Widget + SizeHint,
    U: Widget + SizeHint,
    W: Widget + SizeHint,
    X: Widget + SizeHint,
    Z: Widget + SizeHint,
{
    One(T),
    Two(U),
    Three(W),
    Four(X),
    Five(Z),
}

impl<T, U> SizeHint for WidgetListItem<T, U>
where
    T: Widget + SizeHint,
    U: Widget + SizeHint,
{
    fn size_hint(&self, area: &Rect) -> Size {
        match self {
            WidgetListItem::One(e) => e.size_hint(area),
            WidgetListItem::Two(e) => e.size_hint(area),
        }
    }
}

impl<T, U> Widget for WidgetListItem<T, U>
where
    T: Widget + SizeHint,
    U: Widget + SizeHint,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            WidgetListItem::One(e) => e.render(area, buf),
            WidgetListItem::Two(e) => e.render(area, buf),
        }
    }
}

impl<T, U, W> SizeHint for WidgetListItem3<T, U, W>
where
    T: Widget + SizeHint,
    U: Widget + SizeHint,
    W: Widget + SizeHint,
{
    fn size_hint(&self, area: &Rect) -> Size {
        match self {
            WidgetListItem3::One(e) => e.size_hint(area),
            WidgetListItem3::Two(e) => e.size_hint(area),
            WidgetListItem3::Three(e) => e.size_hint(area),
        }
    }
}

impl<T, U, W> Widget for WidgetListItem3<T, U, W>
where
    T: Widget + SizeHint,
    U: Widget + SizeHint,
    W: Widget + SizeHint,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            WidgetListItem3::One(e) => e.render(area, buf),
            WidgetListItem3::Two(e) => e.render(area, buf),
            WidgetListItem3::Three(e) => e.render(area, buf),
        }
    }
}

impl<T, U, W, X> SizeHint for WidgetListItem4<T, U, W, X>
where
    T: Widget + SizeHint,
    U: Widget + SizeHint,
    W: Widget + SizeHint,
    X: Widget + SizeHint,
{
    fn size_hint(&self, area: &Rect) -> Size {
        match self {
            WidgetListItem4::One(e) => e.size_hint(area),
            WidgetListItem4::Two(e) => e.size_hint(area),
            WidgetListItem4::Three(e) => e.size_hint(area),
            WidgetListItem4::Four(e) => e.size_hint(area),
        }
    }
}

impl<T, U, W, X> Widget for WidgetListItem4<T, U, W, X>
where
    T: Widget + SizeHint,
    U: Widget + SizeHint,
    W: Widget + SizeHint,
    X: Widget + SizeHint,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            WidgetListItem4::One(e) => e.render(area, buf),
            WidgetListItem4::Two(e) => e.render(area, buf),
            WidgetListItem4::Three(e) => e.render(area, buf),
            WidgetListItem4::Four(e) => e.render(area, buf),
        }
    }
}

impl<T, U, W, X, Z> SizeHint for WidgetListItem5<T, U, W, X, Z>
where
    T: Widget + SizeHint,
    U: Widget + SizeHint,
    W: Widget + SizeHint,
    X: Widget + SizeHint,
    Z: Widget + SizeHint,
{
    fn size_hint(&self, area: &Rect) -> Size {
        match self {
            WidgetListItem5::One(e) => e.size_hint(area),
            WidgetListItem5::Two(e) => e.size_hint(area),
            WidgetListItem5::Three(e) => e.size_hint(area),
            WidgetListItem5::Four(e) => e.size_hint(area),
            WidgetListItem5::Five(e) => e.size_hint(area),
        }
    }
}

impl<T, U, W, X, Z> Widget for WidgetListItem5<T, U, W, X, Z>
where
    T: Widget + SizeHint,
    U: Widget + SizeHint,
    W: Widget + SizeHint,
    X: Widget + SizeHint,
    Z: Widget + SizeHint,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            WidgetListItem5::One(e) => e.render(area, buf),
            WidgetListItem5::Two(e) => e.render(area, buf),
            WidgetListItem5::Three(e) => e.render(area, buf),
            WidgetListItem5::Four(e) => e.render(area, buf),
            WidgetListItem5::Five(e) => e.render(area, buf),
        }
    }
}

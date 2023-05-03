use crate::{
    buffer::Buffer,
    layout::{Rect, Size},
    widgets::{SizeHint, StatefulWidget},
};

pub enum WidgetListItem<'a, S, T, U>
where
    T: StatefulWidget + SizeHint,
    U: StatefulWidget + SizeHint,
{
    One(T, &'a S),
    Two(U, &'a S),
}


impl<S, T, U> SizeHint for WidgetListItem<S, T, U>
where
    T: StatefulWidget + SizeHint,
    U: StatefulWidget + SizeHint,
{
    fn size_hint(&self, area: &Rect) -> Size {
        match self {
            WidgetListItem::One(e) => e.size_hint(area),
            WidgetListItem::Two(e) => e.size_hint(area),
        }
    }
}

impl<S, T, U> StatefulWidget for WidgetListItem<S, T, U>
where
    T: StatefulWidget + SizeHint,
    U: StatefulWidget + SizeHint,
{
    type State = S;

    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            WidgetListItem::One(e, s) => e.render(area, buf, &mut s),
            WidgetListItem::Two(e, s) => e.render(area, buf, &mut s),
        }
    }
}

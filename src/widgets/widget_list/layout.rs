use crate::{
    layout::{Constraint, Rect},
    widgets::SizeHint,
};

pub struct Layout {
    pub offset: usize,
    pub item_areas: Vec<Rect>,
}

impl Layout {
    pub fn new(
        area: &Rect,
        spacing: u16,
        items: &[impl SizeHint],
        selected: &Option<usize>,
        offset: usize,
        item_lengths: &[Option<Constraint>],
    ) -> Self {
        let mut start = offset.min(items.len().saturating_sub(1));
        if let Some(selected) = selected {
            start = start.min(*selected);
        }

        let iter = items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                (
                    item,
                    item_lengths.get(index).map_or(&None::<Constraint>, |e| e),
                )
            })
            .skip(start);

        let mut layout = create_layout(iter, *area, spacing);
        let end = start + layout.item_areas.len();
        layout.offset = start;

        if let Some(selected) = selected {
            if *selected > 0 && *selected >= end.saturating_sub(1) {
                layout = create_layout(
                    items
                        .iter()
                        .enumerate()
                        .map(|(index, item)| {
                            (
                                item,
                                item_lengths.get(index).map_or(&None::<Constraint>, |e| e),
                            )
                        })
                        .take(selected + 1)
                        .rev(),
                    *area,
                    spacing,
                );
                layout.offset = selected + 1 - layout.item_areas.len();
                layout.item_areas.reverse();

                let first_bottom = layout.item_areas.first().map(|e| e.bottom());
                for item in layout.item_areas.iter_mut() {
                    item.y = first_bottom.unwrap() - item.bottom() + area.y;
                }
            }
        }

        layout
    }
}

fn get_item_height(item: &impl SizeHint, constraint: &Option<Constraint>, area: &Rect) -> u16 {
    match constraint {
        Some(constraint) => {
            let area = Rect {
                height: constraint.apply(area.height).min(area.height),
                ..*area
            };
            let height = item.size_hint(&area).height;
            constraint.apply(height).max(height)
        }

        None => item.size_hint(area).height,
    }
}

fn create_layout<'b>(
    iter: impl Iterator<Item = (&'b (impl SizeHint + 'b), &'b Option<Constraint>)>,
    mut area: Rect,
    spacing: u16,
) -> Layout {
    let mut layout = Layout {
        offset: 0,
        item_areas: vec![],
    };

    for (item, constraint) in iter {
        let item_height = get_item_height(item, constraint, &area);
        if item_height > area.height {
            break;
        }
        layout.item_areas.push(Rect {
            height: item_height,
            ..area
        });

        area.y += item_height + spacing;
        area.height = area.height.saturating_sub(item_height + spacing);
        if area.height == 0 {
            break;
        }
    }

    layout
}

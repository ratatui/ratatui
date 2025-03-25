use crate::symbols::line::*;

pub fn merge_border(prev: BorderSymbol, next: BorderSymbol) -> BorderSymbol {
    let mut res = BorderSymbol::new(None, None, None, None);

    res.right = merge_line_style(prev.right, next.right);
    res.up = merge_line_style(prev.up, next.up);
    res.left = merge_line_style(prev.left, next.left);
    res.down = merge_line_style(prev.down, next.down);

    res
}

pub fn merge_line_style(prev: Option<LineStyle>, next: Option<LineStyle>) -> Option<LineStyle> {
    match (prev, next) {
        (None, None) => None,
        (Some(s), None) | (None, Some(s)) => Some(s),
        (Some(LineStyle::Thick), Some(LineStyle::Normal))
        | (Some(LineStyle::Normal), Some(LineStyle::Thick)) => Some(LineStyle::Thick),
        (Some(LineStyle::Double), Some(LineStyle::Normal))
        | (Some(LineStyle::Normal), Some(LineStyle::Double)) => Some(LineStyle::Double),
        (Some(LineStyle::Normal), Some(LineStyle::Normal)) => Some(LineStyle::Normal),
        (Some(LineStyle::Thick), Some(LineStyle::Thick)) => Some(LineStyle::Thick),
        (Some(LineStyle::Double), Some(LineStyle::Double)) => Some(LineStyle::Double),
        (Some(LineStyle::DoubleDash), Some(LineStyle::DoubleDash)) => Some(LineStyle::DoubleDash),
        (Some(LineStyle::TripleDash), Some(LineStyle::TripleDash)) => Some(LineStyle::TripleDash),
        (Some(LineStyle::QuadrupleDash), Some(LineStyle::QuadrupleDash)) => {
            Some(LineStyle::QuadrupleDash)
        }
        (_, Some(next)) => Some(next),
    }
}

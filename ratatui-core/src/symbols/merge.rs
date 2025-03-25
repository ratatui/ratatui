use crate::symbols::line::{BorderSymbol, LineStyle};

pub const fn merge_border(prev: BorderSymbol, next: BorderSymbol) -> BorderSymbol {
    use LineStyle::*;
    let mut res = BorderSymbol::new(Nothing, Nothing, Nothing, Nothing);

    res.right = merge_line_style(prev.right, next.right);
    res.up = merge_line_style(prev.up, next.up);
    res.left = merge_line_style(prev.left, next.left);
    res.down = merge_line_style(prev.down, next.down);

    res
}

pub const fn merge_line_style(prev: LineStyle, next: LineStyle) -> LineStyle {
    use LineStyle::*;
    match (prev, next) {
        (Nothing, Nothing) => Nothing,
        (s, Nothing) | (Nothing, s) => s,
        (LineStyle::Thick, LineStyle::Plain | LineStyle::Thick)
        | (LineStyle::Plain, LineStyle::Thick) => LineStyle::Thick,
        (LineStyle::Double, LineStyle::Plain | LineStyle::Double)
        | (LineStyle::Plain, LineStyle::Double) => LineStyle::Double,
        (LineStyle::Plain, LineStyle::Plain) => LineStyle::Plain,
        (LineStyle::DoubleDash, LineStyle::DoubleDash) => LineStyle::DoubleDash,
        (LineStyle::TripleDash, LineStyle::TripleDash) => LineStyle::TripleDash,
        (LineStyle::QuadrupleDash, LineStyle::QuadrupleDash) => LineStyle::QuadrupleDash,
        (_, next) => next,
    }
}

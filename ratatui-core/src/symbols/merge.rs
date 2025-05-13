use crate::symbols::line::{BorderSymbol, LineStyle};

/// Defines the merge strategy of overlapping characters.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum MergeStyle {
    /// Merges symbols only if an exact composite unicode character exists.
    /// Example: `┐` and `┗` will be merged into `╄`
    #[default]
    Exact,
    /// Merges symbols even if an exact composite unicode character doesn't exist,
    /// using the closest match.
    /// Example: `╮` and `└` will be merged into `┼`
    BestFit,
}

pub fn merge_border(prev: &BorderSymbol, next: &BorderSymbol, style: &MergeStyle) -> BorderSymbol {
    let exact_result = BorderSymbol::new(
        merge_line_style(&prev.right, &next.right),
        merge_line_style(&prev.up, &next.up),
        merge_line_style(&prev.left, &next.left),
        merge_line_style(&prev.down, &next.down),
    );
    match style {
        MergeStyle::BestFit => exact_result.best_fit(),
        MergeStyle::Exact => exact_result,
    }
}

pub fn merge_line_style(prev: &LineStyle, next: &LineStyle) -> LineStyle {
    use LineStyle::{
        DoubleDash, Nothing, Plain, QuadrupleDash, QuadrupleDashThick, TripleDash, TripleDashThick,
    };
    match (prev, next) {
        (Nothing, Nothing) => Nothing,
        (s, Nothing) | (Nothing, s) => s.clone(),
        // (Thick, Plain | Thick) | (Plain, Thick) => Thick,
        // (Double, Plain | Double) | (Plain, Double) => Double,
        (Plain, Plain) => Plain,
        (DoubleDash, DoubleDash) => DoubleDash,
        (TripleDash, TripleDash) => TripleDash,
        (TripleDashThick, TripleDashThick) => TripleDashThick,
        (QuadrupleDash, QuadrupleDash) => QuadrupleDash,
        (QuadrupleDashThick, QuadrupleDashThick) => QuadrupleDashThick,
        (_, next) => next.clone(),
    }
}

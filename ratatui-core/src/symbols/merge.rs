use crate::symbols::line::BorderSymbol;

/// Defines the merge strategy of overlapping characters.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum MergeStyle {
    /// Merges symbols only if an exact composite unicode character exists.
    ///
    /// Example: `┐` and `┗` will be merged into `╄`
    #[default]
    Exact,
    /// Merges symbols even if an exact composite unicode character doesn't exist,
    /// using the closest match.
    ///
    /// Example: `╮` and `└` will be merged into `┼`
    BestFit,
}

/// Merges two border symbols into one.
pub fn merge_border(prev: &BorderSymbol, next: &BorderSymbol, style: &MergeStyle) -> BorderSymbol {
    let exact_result = BorderSymbol::new(
        prev.right.merge(next.right),
        prev.up.merge(next.up),
        prev.left.merge(next.left),
        prev.down.merge(next.down),
    );
    match style {
        MergeStyle::BestFit => exact_result.best_fit(),
        MergeStyle::Exact => exact_result,
    }
}

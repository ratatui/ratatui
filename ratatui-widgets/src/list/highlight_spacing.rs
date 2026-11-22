use strum::{Display, EnumString};

/// This option allows the user to configure the "highlight symbol" column width spacing
#[derive(Debug, Display, EnumString, PartialEq, Eq, Clone, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ListHighlightSpacing {
    /// Always add spacing for the selection symbol column
    ///
    /// With this variant, the column for the selection symbol will always be allocated, and so the
    /// list will never change size, regardless of whether an item is selected or not
    Always,

    /// Only add spacing for the selection symbol column if an item is selected
    ///
    /// With this variant, the column for the selection symbol will only be allocated if there is a
    /// selection, causing the list to shift if selected / unselected
    #[default]
    WhenSelected,

    /// Never add spacing to the selection symbol column, regardless of whether something is
    /// selected or not
    ///
    /// This means that the highlight symbol will never be drawn
    Never,
}

impl ListHighlightSpacing {
    /// Determine if a selection column should be displayed
    ///
    /// `has_selection`: true if an item is selected in the list
    ///
    /// Returns true if a selection column should be displayed
    pub(crate) const fn should_add(&self, has_selection: bool) -> bool {
        match self {
            Self::Always => true,
            Self::WhenSelected => has_selection,
            Self::Never => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn to_string() {
        assert_eq!(
            ListHighlightSpacing::Always.to_string(),
            "Always".to_string()
        );
        assert_eq!(
            ListHighlightSpacing::WhenSelected.to_string(),
            "WhenSelected".to_string()
        );
        assert_eq!(ListHighlightSpacing::Never.to_string(), "Never".to_string());
    }

    #[test]
    fn from_str() {
        assert_eq!(
            "Always".parse::<ListHighlightSpacing>(),
            Ok(ListHighlightSpacing::Always)
        );
        assert_eq!(
            "WhenSelected".parse::<ListHighlightSpacing>(),
            Ok(ListHighlightSpacing::WhenSelected)
        );
        assert_eq!(
            "Never".parse::<ListHighlightSpacing>(),
            Ok(ListHighlightSpacing::Never)
        );
        assert_eq!(
            "".parse::<ListHighlightSpacing>(),
            Err(strum::ParseError::VariantNotFound)
        );
    }
}

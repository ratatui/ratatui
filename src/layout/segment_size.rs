use strum::{Display, EnumString};

/// Option for segment size preferences
///
/// This controls how the space is distributed when the constraints are satisfied. By default, the
/// last chunk is expanded to fill the remaining space, but this can be changed to prefer equal
/// chunks or to not distribute extra space at all (which is the default used for laying out the
/// columns for [`Table`] widgets).
///
/// Note: If you're using this feature please help us come up with a good name. See [Issue
/// #536](https://github.com/ratatui-org/ratatui/issues/536) for more information.
///
/// [`Table`]: crate::widgets::Table
#[stability::unstable(
    feature = "segment-size",
    reason = "The name for this feature is not final and may change in the future",
    issue = "https://github.com/ratatui-org/ratatui/issues/536"
)]
#[derive(Copy, Debug, Default, Display, EnumString, Clone, Eq, PartialEq, Hash)]
pub enum SegmentSize {
    /// prefer equal chunks if other constraints are all satisfied
    EvenDistribution,

    /// the last chunk is expanded to fill the remaining space
    #[default]
    LastTakesRemainder,

    /// extra space is not distributed
    None,
}
#[cfg(test)]
mod tests {
    use strum::ParseError;

    use super::{SegmentSize::*, *};
    use crate::prelude::{Constraint::*, *};
    #[test]
    fn segment_size_to_string() {
        assert_eq!(EvenDistribution.to_string(), "EvenDistribution");
        assert_eq!(LastTakesRemainder.to_string(), "LastTakesRemainder");
        assert_eq!(None.to_string(), "None");
    }

    #[test]
    fn segment_size_from_string() {
        assert_eq!(
            "EvenDistribution".parse::<SegmentSize>(),
            Ok(EvenDistribution)
        );
        assert_eq!(
            "LastTakesRemainder".parse::<SegmentSize>(),
            Ok(LastTakesRemainder)
        );
        assert_eq!("None".parse::<SegmentSize>(), Ok(None));
        assert_eq!("".parse::<SegmentSize>(), Err(ParseError::VariantNotFound));
    }

    fn get_x_width_with_segment_size(
        segment_size: SegmentSize,
        constraints: Vec<Constraint>,
        target: Rect,
    ) -> Vec<(u16, u16)> {
        #[allow(deprecated)]
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .segment_size(segment_size);
        let chunks = layout.split(target);
        chunks.iter().map(|r| (r.x, r.width)).collect()
    }

    #[test]
    fn test_split_equally_in_underspecified_case() {
        let target = Rect::new(100, 200, 10, 10);
        assert_eq!(
            get_x_width_with_segment_size(LastTakesRemainder, vec![Min(2), Min(2), Min(0)], target),
            [(100, 2), (102, 2), (104, 6)]
        );
        // assert_eq!(
        //     get_x_width_with_segment_size(EvenDistribution, vec![Min(2), Min(2), Min(0)],
        // target),     [(100, 3), (103, 4), (107, 3)]
        // );
    }

    #[test]
    fn test_split_equally_in_overconstrained_case_for_min() {
        let target = Rect::new(100, 200, 100, 10);
        assert_eq!(
            get_x_width_with_segment_size(
                LastTakesRemainder,
                vec![Percentage(50), Min(10), Percentage(50)],
                target
            ),
            [(100, 50), (150, 10), (160, 40)]
        );
        // assert_eq!(
        //     get_x_width_with_segment_size(
        //         EvenDistribution,
        //         vec![Percentage(50), Min(10), Percentage(50)],
        //         target
        //     ),
        //     [(100, 45), (145, 10), (155, 45)]
        // );
    }

    #[test]
    fn test_split_equally_in_overconstrained_case_for_max() {
        let target = Rect::new(100, 200, 100, 10);
        assert_eq!(
            get_x_width_with_segment_size(
                LastTakesRemainder,
                vec![Percentage(30), Max(10), Percentage(30)],
                target
            ),
            [(100, 30), (130, 10), (140, 60)]
        );
        // assert_eq!(
        //     get_x_width_with_segment_size(
        //         EvenDistribution,
        //         vec![Percentage(30), Max(10), Percentage(30)],
        //         target
        //     ),
        //     [(100, 45), (145, 10), (155, 45)]
        // );
    }

    #[test]
    fn test_split_equally_in_overconstrained_case_for_length() {
        let target = Rect::new(100, 200, 100, 10);
        assert_eq!(
            get_x_width_with_segment_size(
                LastTakesRemainder,
                vec![Percentage(50), Length(10), Percentage(50)],
                target
            ),
            [(100, 50), (150, 10), (160, 40)]
        );
        // assert_eq!(
        //     get_x_width_with_segment_size(
        //         EvenDistribution,
        //         vec![Percentage(50), Length(10), Percentage(50)],
        //         target
        //     ),
        //     [(100, 45), (145, 10), (155, 45)]
        // );
    }
}

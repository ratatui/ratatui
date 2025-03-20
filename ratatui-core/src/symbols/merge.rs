use super::line;

/// Merge the two given symbols.
/// Returns `None` if there is nothing to merge.
/// Nothing to merge means you have to choose which one to display.
///
/// TODO : so much "if then else" are
/// - bad pratice
/// - bad performance
/// - easy to achieve with macro (but not a good idea)
/// - compiler optimized ?
/// I didn't achieved it simply with a match case
pub fn merge_symbol(first: &str, second: &str) -> Option<&'static str> {
    if first == line::NORMAL.vertical {
        if second == line::NORMAL.vertical {
            None
        } else if second == line::NORMAL.horizontal {
            Some(line::NORMAL.cross)
        } else if second == line::NORMAL.top_right {
            Some(line::NORMAL.vertical_left)
        } else if second == line::NORMAL.top_left {
            Some(line::NORMAL.vertical_right)
        } else if second == line::NORMAL.bottom_right {
            Some(line::NORMAL.vertical_left)
        } else if second == line::NORMAL.bottom_left {
            Some(line::NORMAL.vertical_right)
        } else if second == line::NORMAL.vertical_left {
            None
        } else if second == line::NORMAL.vertical_right {
            None
        } else if second == line::NORMAL.horizontal_down {
            Some(line::NORMAL.cross)
        } else if second == line::NORMAL.horizontal_up {
            Some(line::NORMAL.cross)
        } else {
            None
        }
    } else if first == line::NORMAL.horizontal {
        if second == line::NORMAL.vertical {
            Some(line::NORMAL.cross)
        } else if second == line::NORMAL.horizontal {
            None
        } else if second == line::NORMAL.top_right {
            Some(line::NORMAL.horizontal_down)
        } else if second == line::NORMAL.top_left {
            Some(line::NORMAL.horizontal_down)
        } else if second == line::NORMAL.bottom_right {
            Some(line::NORMAL.horizontal_up)
        } else if second == line::NORMAL.bottom_left {
            Some(line::NORMAL.horizontal_up)
        } else if second == line::NORMAL.vertical_left {
            Some(line::NORMAL.cross)
        } else if second == line::NORMAL.vertical_right {
            Some(line::NORMAL.cross)
        } else if second == line::NORMAL.horizontal_down {
            None
        } else if second == line::NORMAL.horizontal_up {
            None
        } else {
            None
        }
    } else {
        None
    }
    // ... other cases
}

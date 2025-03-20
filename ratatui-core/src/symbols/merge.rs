use crate::symbols::line::{DOUBLE, NORMAL, ROUNDED, THICK};
use std::collections::HashMap;
use std::sync::OnceLock;

/// Map to know how to merge the two given symbols.
/// If a couple of symbols are not is the map there is nothing to merge.
/// This map should be symetric.
static MERGE_MAP: OnceLock<HashMap<(&'static str, &'static str), &'static str>> = OnceLock::new();

macro_rules! insert_merge_rules {
    ($map:expr, $s:ident) => {
        // $map.insert(($s.vertical, $s.vertical), $s.vertical);
        $map.insert(($s.vertical, $s.horizontal), $s.cross);
        $map.insert(($s.vertical, $s.top_right), $s.vertical_left);
        $map.insert(($s.vertical, $s.top_left), $s.vertical_right);
        $map.insert(($s.vertical, $s.bottom_right), $s.vertical_left);
        $map.insert(($s.vertical, $s.bottom_left), $s.vertical_right);
        $map.insert(($s.vertical, $s.vertical_left), $s.vertical_left);
        $map.insert(($s.vertical, $s.vertical_right), $s.cross);
        $map.insert(($s.vertical, $s.horizontal_down), $s.cross);
        $map.insert(($s.vertical, $s.horizontal_up), $s.cross);
        $map.insert(($s.vertical, $s.cross), $s.cross);

        $map.insert(($s.horizontal, $s.vertical), $s.cross);
        // $map.insert(($s.horizontal, $s.horizontal), $s.horizontal);
        $map.insert(($s.horizontal, $s.top_right), $s.horizontal_down);
        $map.insert(($s.horizontal, $s.top_left), $s.horizontal_down);
        $map.insert(($s.horizontal, $s.bottom_right), $s.horizontal_up);
        $map.insert(($s.horizontal, $s.bottom_left), $s.horizontal_up);
        $map.insert(($s.horizontal, $s.vertical_left), $s.cross);
        $map.insert(($s.horizontal, $s.vertical_right), $s.cross);
        $map.insert(($s.horizontal, $s.horizontal_down), $s.horizontal_down);
        $map.insert(($s.horizontal, $s.horizontal_up), $s.horizontal_up);
        $map.insert(($s.horizontal, $s.cross), $s.cross);

        $map.insert(($s.top_right, $s.vertical), $s.vertical_left);
        $map.insert(($s.top_right, $s.horizontal), $s.horizontal_down);
        // $map.insert(($s.top_right, $s.top_right), $s.top_right);
        $map.insert(($s.top_right, $s.top_left), $s.horizontal_down);
        $map.insert(($s.top_right, $s.bottom_right), $s.vertical_left);
        $map.insert(($s.top_right, $s.bottom_left), $s.cross);
        $map.insert(($s.top_right, $s.vertical_left), $s.vertical_left);
        $map.insert(($s.top_right, $s.vertical_right), $s.cross);
        $map.insert(($s.top_right, $s.horizontal_down), $s.horizontal_down);
        $map.insert(($s.top_right, $s.horizontal_up), $s.cross);
        $map.insert(($s.top_right, $s.cross), $s.cross);

        $map.insert(($s.top_left, $s.vertical), $s.vertical_right);
        $map.insert(($s.top_left, $s.horizontal), $s.horizontal_down);
        $map.insert(($s.top_left, $s.top_right), $s.horizontal_down);
        // $map.insert(($s.top_left, $s.top_left), $s.top_left);
        $map.insert(($s.top_left, $s.bottom_right), $s.cross);
        $map.insert(($s.top_left, $s.bottom_left), $s.vertical_right);
        $map.insert(($s.top_left, $s.vertical_left), $s.cross);
        $map.insert(($s.top_left, $s.vertical_right), $s.vertical_right);
        $map.insert(($s.top_left, $s.horizontal_down), $s.horizontal_down);
        $map.insert(($s.top_left, $s.horizontal_up), $s.cross);
        $map.insert(($s.top_left, $s.cross), $s.cross);

        $map.insert(($s.bottom_right, $s.vertical), $s.vertical_left);
        $map.insert(($s.bottom_right, $s.horizontal), $s.horizontal_up);
        $map.insert(($s.bottom_right, $s.top_right), $s.vertical_left);
        $map.insert(($s.bottom_right, $s.top_left), $s.cross);
        // $map.insert(($s.bottom_right, $s.bottom_right), $s.bottom_right);
        $map.insert(($s.bottom_right, $s.bottom_left), $s.horizontal_up);
        $map.insert(($s.bottom_right, $s.vertical_left), $s.vertical_left);
        $map.insert(($s.bottom_right, $s.vertical_right), $s.cross);
        $map.insert(($s.bottom_right, $s.horizontal_down), $s.cross);
        $map.insert(($s.bottom_right, $s.horizontal_up), $s.horizontal_up);
        $map.insert(($s.bottom_right, $s.cross), $s.cross);

        $map.insert(($s.bottom_left, $s.vertical), $s.vertical_right);
        $map.insert(($s.bottom_left, $s.horizontal), $s.horizontal_up);
        $map.insert(($s.bottom_left, $s.top_right), $s.cross);
        $map.insert(($s.bottom_left, $s.top_left), $s.vertical_right);
        $map.insert(($s.bottom_left, $s.bottom_right), $s.horizontal_up);
        // $map.insert(($s.bottom_left, $s.bottom_left), $s.bottom_left);
        $map.insert(($s.bottom_left, $s.vertical_left), $s.cross);
        $map.insert(($s.bottom_left, $s.vertical_right), $s.vertical_right);
        $map.insert(($s.bottom_left, $s.horizontal_down), $s.cross);
        $map.insert(($s.bottom_left, $s.horizontal_up), $s.horizontal_up);
        $map.insert(($s.bottom_left, $s.cross), $s.cross);

        $map.insert(($s.vertical_left, $s.vertical), $s.vertical_left);
        $map.insert(($s.vertical_left, $s.horizontal), $s.cross);
        $map.insert(($s.vertical_left, $s.top_right), $s.vertical_left);
        $map.insert(($s.vertical_left, $s.top_left), $s.cross);
        $map.insert(($s.vertical_left, $s.bottom_right), $s.vertical_left);
        $map.insert(($s.vertical_left, $s.bottom_left), $s.cross);
        // $map.insert(($s.vertical_left, $s.vertical_left), $s.vertical_left);
        $map.insert(($s.vertical_left, $s.vertical_right), $s.cross);
        $map.insert(($s.vertical_left, $s.horizontal_down), $s.cross);
        $map.insert(($s.vertical_left, $s.horizontal_up), $s.cross);
        $map.insert(($s.vertical_left, $s.cross), $s.cross);

        $map.insert(($s.vertical_right, $s.vertical), $s.vertical_right);
        $map.insert(($s.vertical_right, $s.horizontal), $s.cross);
        $map.insert(($s.vertical_right, $s.top_right), $s.cross);
        $map.insert(($s.vertical_right, $s.top_left), $s.vertical_right);
        $map.insert(($s.vertical_right, $s.bottom_right), $s.cross);
        $map.insert(($s.vertical_right, $s.bottom_left), $s.vertical_right);
        $map.insert(($s.vertical_right, $s.vertical_left), $s.cross);
        // $map.insert(($s.vertical_right, $s.vertical_right), $s.vertical_right);
        $map.insert(($s.vertical_right, $s.horizontal_down), $s.cross);
        $map.insert(($s.vertical_right, $s.horizontal_up), $s.cross);
        $map.insert(($s.vertical_right, $s.cross), $s.cross);

        $map.insert(($s.horizontal_down, $s.vertical), $s.cross);
        $map.insert(($s.horizontal_down, $s.horizontal), $s.horizontal_down);
        $map.insert(($s.horizontal_down, $s.top_right), $s.horizontal_down);
        $map.insert(($s.horizontal_down, $s.top_left), $s.horizontal_down);
        $map.insert(($s.horizontal_down, $s.bottom_right), $s.cross);
        $map.insert(($s.horizontal_down, $s.bottom_left), $s.cross);
        $map.insert(($s.horizontal_down, $s.vertical_left), $s.cross);
        $map.insert(($s.horizontal_down, $s.vertical_right), $s.cross);
        // $map.insert(($s.horizontal_down, $s.horizontal_down), $s.horizontal_down);
        $map.insert(($s.horizontal_down, $s.horizontal_up), $s.cross);
        $map.insert(($s.horizontal_down, $s.cross), $s.cross);

        $map.insert(($s.horizontal_up, $s.vertical), $s.cross);
        $map.insert(($s.horizontal_up, $s.horizontal), $s.horizontal_up);
        $map.insert(($s.horizontal_up, $s.top_right), $s.cross);
        $map.insert(($s.horizontal_up, $s.top_left), $s.cross);
        $map.insert(($s.horizontal_up, $s.bottom_right), $s.horizontal_up);
        $map.insert(($s.horizontal_up, $s.bottom_left), $s.horizontal_up);
        $map.insert(($s.horizontal_up, $s.vertical_left), $s.cross);
        $map.insert(($s.horizontal_up, $s.vertical_right), $s.cross);
        $map.insert(($s.horizontal_up, $s.horizontal_down), $s.cross);
        // $map.insert(($s.horizontal_up, $s.horizontal_up), $s.horizontal_up);
        $map.insert(($s.horizontal_up, $s.cross), $s.cross);

        $map.insert(($s.cross, $s.vertical), $s.cross);
        $map.insert(($s.cross, $s.horizontal), $s.cross);
        $map.insert(($s.cross, $s.top_right), $s.cross);
        $map.insert(($s.cross, $s.top_left), $s.cross);
        $map.insert(($s.cross, $s.bottom_right), $s.cross);
        $map.insert(($s.cross, $s.bottom_left), $s.cross);
        $map.insert(($s.cross, $s.vertical_left), $s.cross);
        $map.insert(($s.cross, $s.vertical_right), $s.cross);
        $map.insert(($s.cross, $s.horizontal_down), $s.cross);
        $map.insert(($s.cross, $s.horizontal_up), $s.cross);
        // $map.insert(($s.cross, $s.cross), $s.cross);
    };
}

/// Lazy initialization of `MERGE_MAP`
pub fn get_merge_map() -> &'static HashMap<(&'static str, &'static str), &'static str> {
    MERGE_MAP.get_or_init(|| {
        let mut map = HashMap::new();
        insert_merge_rules!(map, NORMAL);
        insert_merge_rules!(map, ROUNDED);
        insert_merge_rules!(map, THICK);
        insert_merge_rules!(map, DOUBLE);
        map
    })
}

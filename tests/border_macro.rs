#![cfg(feature = "macros")]
use ratatui::{border, widgets::Borders};

#[test]
fn border_empty_test() {
    let empty = Borders::NONE;
    assert_eq!(empty, border!());
}
#[test]
fn border_all_test() {
    let all = Borders::ALL;
    assert_eq!(all, border!(ALL));
    assert_eq!(all, border!(TOP, BOTTOM, LEFT, RIGHT));
}
#[test]
fn border_left_right_test() {
    let left_right = Borders::from_bits(Borders::LEFT.bits() | Borders::RIGHT.bits());
    assert_eq!(left_right, Some(border!(RIGHT, LEFT)));
}

use ratatui::prelude::*;
use ratatui_macros::{constraints, layout};

#[test]
fn layout_constraints_macro() {
  let rect = Rect { x: 0, y: 0, width: 10, height: 10 };

  let [rect1, rect2] = layout!([==7, <=3]).split(rect).to_vec().try_into().unwrap();
  assert_eq!(rect1, Rect::new(0, 0, 10, 7));
  assert_eq!(rect2, Rect::new(0, 7, 10, 3));

  let one = 1;
  let two = 2;
  let ten = 10;
  let zero = 0;
  let [a, b, c, d, e, f] = layout!([==one, >=one, <=one, == 1 / two, == ten %, >=zero], direction = h)
    .split(rect)
    .to_vec()
    .try_into()
    .unwrap();

  assert_eq!(a, Rect::new(0, 0, 1, 10));
  assert_eq!(b, Rect::new(1, 0, 1, 10));
  assert_eq!(c, Rect::new(2, 0, 1, 10));
  assert_eq!(d, Rect::new(3, 0, 5, 10));
  assert_eq!(e, Rect::new(8, 0, 1, 10));
  assert_eq!(f, Rect::new(9, 0, 1, 10));

  let one = 1;
  let two = 2;
  let ten = 10;
  let zero = 0;
  let [a, b, c, d, e, f] = layout!(
      [
          == one*one, // expr allowed here
          >= one+zero, // expr allowed here
          <= one-zero, // expr allowed here
          == 1/two, // only single token allowed in numerator and denominator
          == ten%, // only single token allowed before %
          >= zero // no trailing comma
      ],
      direction = h
  )
  .split(rect)
  .to_vec()
  .try_into()
  .unwrap();

  assert_eq!(a, Rect::new(0, 0, 1, 10));
  assert_eq!(b, Rect::new(1, 0, 1, 10));
  assert_eq!(c, Rect::new(2, 0, 1, 10));
  assert_eq!(d, Rect::new(3, 0, 5, 10));
  assert_eq!(e, Rect::new(8, 0, 1, 10));
  assert_eq!(f, Rect::new(9, 0, 1, 10));

  let [a, b, c, d, e] = constraints!([>=0, ==1, <=5, ==10%, ==1/2]).collect::<Vec<Constraint>>().try_into().unwrap();
  assert_eq!(a, Constraint::Min(0));
  assert_eq!(b, Constraint::Length(1));
  assert_eq!(c, Constraint::Max(5));
  assert_eq!(d, Constraint::Percentage(10));
  assert_eq!(e, Constraint::Ratio(1, 2));
}

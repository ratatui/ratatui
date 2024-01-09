use ratatui_macros::constraints;

fn test() {
  constraints![,];
}

fn test2() {
  let [a, b] = constraints![ == 1/2, == 2, ];
  assert_eq!(a, Constraint::Ratio(1, 2));
  assert_eq!(b, Constraint::Length(2));
}

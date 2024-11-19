#![cfg(feature = "unstable-widget-ref")]

use ratatui::widgets::StatefulWidgetRef;
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::text::Line;
use ratatui_core::widgets::Widget;
use std::any::{type_name_of_val, Any};
use std::cell::RefCell;

trait AnyWindow: StatefulWidgetRef<State = dyn Any> {
    fn type_name(&self) -> &str;
}

struct Window1;

struct Window1State {
    pub value: u32,
}

impl AnyWindow for Window1 {
    fn type_name(&self) -> &str {
        type_name_of_val(&self)
    }
}

impl StatefulWidgetRef for Window1 {
    type State = dyn Any;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let state = state.downcast_mut::<Window1State>().expect("window1 state");
        Line::from(format!("u32: {}", state.value)).render(area, buf);
    }
}

struct Window2;

struct Window2State {
    pub value: String,
}

impl AnyWindow for Window2 {
    fn type_name(&self) -> &str {
        type_name_of_val(&self)
    }
}

impl StatefulWidgetRef for Window2 {
    type State = dyn Any;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let state = state.downcast_mut::<Window2State>().expect("window2 state");
        Line::from(format!("String: {}", state.value)).render(area, buf);
    }
}

#[test]
fn render_dyn_widgets() {
    let mut windows: Vec<(Box<dyn AnyWindow>, Box<RefCell<dyn Any>>)> = Vec::new();

    windows.push((
        Box::new(Window1),
        Box::new(RefCell::new(Window1State { value: 32 })),
    ));
    windows.push((
        Box::new(Window2),
        Box::new(RefCell::new(Window2State {
            value: "Some".to_string(),
        })),
    ));
    windows.push((
        Box::new(Window1),
        Box::new(RefCell::new(Window1State { value: 42 })),
    ));

    let mut buf = Buffer::empty(Rect::new(0, 0, 20, 5));

    let mut area = Rect::new(0, 0, 20, 1);
    for (w, s) in windows.iter() {
        eprintln!("render {}", w.type_name());
        let mut s = s.borrow_mut();
        w.render_ref(area, &mut buf, &mut *s);
        area.y += 1;
    }

    let buf_string = buf
        .content
        .iter()
        .map(|v| v.symbol())
        .fold(String::new(), |mut v, w| {
            v.push_str(w);
            v
        });
    assert_eq!(
        buf_string,
        "u32: 32             String: Some        u32: 42                                                     "
    );
}

use core::panic::{RefUnwindSafe, UnwindSafe};

use ratatui::widgets::calendar::{CalendarEventStore, Monthly};
use ratatui::widgets::canvas::{Canvas, Context};
use ratatui::widgets::{
    BarChart, Block, Chart, Gauge, LineGauge, List, Paragraph, Sparkline, Table, Tabs,
};

const fn assert_auto_traits<T: Send + Sync + UnwindSafe + RefUnwindSafe>() {}

#[test]
fn block_backed_widgets_keep_auto_traits() {
    // This covers the widgets that store `Block`, and therefore `Shadow`. These auto traits are
    // part of the public API surface: downstream code can require `Paragraph<'static>: Send`, for
    // example. Keep this test near the public `ratatui::widgets` re-exports so changes to private
    // `Block` fields do not silently break those bounds.
    assert_auto_traits::<Block<'static>>();
    assert_auto_traits::<Paragraph<'static>>();
    assert_auto_traits::<Table<'static>>();
    assert_auto_traits::<Sparkline<'static>>();
    assert_auto_traits::<LineGauge<'static>>();
    assert_auto_traits::<Chart<'static>>();
    assert_auto_traits::<BarChart<'static>>();
    assert_auto_traits::<Canvas<'static, fn(&mut Context)>>();
    assert_auto_traits::<List<'static>>();
    assert_auto_traits::<Monthly<'static, CalendarEventStore>>();
    assert_auto_traits::<Gauge<'static>>();
    assert_auto_traits::<Tabs<'static>>();
}

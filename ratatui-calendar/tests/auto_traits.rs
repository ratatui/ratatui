use core::panic::{RefUnwindSafe, UnwindSafe};

use ratatui_calendar::{CalendarEventStore, Monthly};

/// A compile-time assertion for checking that the given type implements the auto traits.
const fn assert_auto_traits<T: Send + Sync + UnwindSafe + RefUnwindSafe>() {}

#[test]
fn monthly_keeps_auto_traits() {
    assert_auto_traits::<Monthly<'static, CalendarEventStore>>();
}

use metrics::Histogram;

/// A helper macro that registers and describes a counter
#[macro_export]
macro_rules! counter {
    ($name:expr, $description:expr $(,)?) => {{
        ::metrics::describe_counter!($name, ::metrics::Unit::Count, $description);
        ::metrics::counter!($name)
    }};
}

/// A helper macro that registers and describes a histogram that tracks durations.
#[macro_export]
macro_rules! duration_histogram {
    ($name:expr, $description:expr $(,)?) => {{
        ::metrics::describe_histogram!($name, ::metrics::Unit::Seconds, $description);
        ::metrics::histogram!($name)
    }};
}

/// A helper macro that registers and describes a histogram that tracks bytes.
#[macro_export]
macro_rules! bytes_histogram {
    ($name:expr, $description:expr $(,)?) => {{
        ::metrics::describe_histogram!($name, ::metrics::Unit::Bytes, $description);
        ::metrics::histogram!($name)
    }};
}

pub(crate) trait HistogramExt {
    fn measure_duration<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R;

    fn start_timing(&self) -> DurationMeasurementGuard;
}

impl HistogramExt for Histogram {
    fn measure_duration<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let start = quanta::Instant::now();
        let result = f();
        self.record(start.elapsed().as_secs_f64());
        result
    }

    fn start_timing(&self) -> DurationMeasurementGuard {
        DurationMeasurementGuard {
            start: quanta::Instant::now(),
            histogram: self.clone(), // this is safe because `Histogram` stores an `Arc`
        }
    }
}

pub struct DurationMeasurementGuard {
    start: quanta::Instant,
    histogram: Histogram,
}

impl Drop for DurationMeasurementGuard {
    fn drop(&mut self) {
        self.histogram.record(self.start.elapsed().as_secs_f64());
    }
}

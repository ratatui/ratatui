use std::{
    sync::{atomic::Ordering, Arc},
    time::{Duration, Instant},
};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use metrics::{Counter, Gauge, Histogram, Key, KeyName, Metadata, Recorder, SharedString, Unit};
use metrics_util::{
    registry::{AtomicStorage, Registry},
    Summary,
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Row, Table, Widget},
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let recorder = MetricsRecorder::new();
    let recorder_widget = recorder.widget();
    recorder.install();
    let terminal = ratatui::init();
    let app = App::new(recorder_widget);
    let result = app.run(terminal);
    ratatui::restore();
    result
}

#[derive(Debug)]
struct App {
    should_quit: bool,
    recorder_widget: RecorderWidget,
}

impl App {
    fn new(recorder_widget: RecorderWidget) -> Self {
        Self {
            should_quit: false,
            recorder_widget,
        }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let mut last_frame = Instant::now();
        let frame_duration = Duration::from_secs_f64(1.0 / 60.0);
        while !self.should_quit {
            if last_frame.elapsed() >= frame_duration {
                last_frame = Instant::now();
                terminal.draw(|frame| self.draw(frame))?;
            }
            self.handle_events(frame_duration.saturating_sub(last_frame.elapsed()))?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let [top, main] =
            Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(frame.area());
        let title = if cfg!(debug_assertions) {
            "Metrics Example (debug)"
        } else {
            "Metrics Example (release)"
        };
        frame.render_widget(title.blue().into_centered_line(), top);
        frame.render_widget(&self.recorder_widget, main);
    }

    fn handle_events(&mut self, timeout: Duration) -> Result<()> {
        if !event::poll(timeout)? {
            return Ok(());
        }
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_press(key),
            _ => {}
        }
        Ok(())
    }

    fn on_key_press(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            _ => {}
        }
    }
}

#[derive(Debug, Default)]
struct MetricsRecorder {
    metrics: Arc<Metrics>,
}

impl MetricsRecorder {
    fn new() -> Self {
        Self::default()
    }

    fn widget(&self) -> RecorderWidget {
        RecorderWidget {
            metrics: Arc::clone(&self.metrics),
        }
    }

    fn install(self) {
        metrics::set_global_recorder(self).unwrap();
    }
}

#[derive(Debug)]
struct Metrics {
    registry: Registry<Key, AtomicStorage>,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            registry: Registry::atomic(),
        }
    }
}

impl Metrics {
    fn counter(&self, key: &Key) -> Counter {
        self.registry
            .get_or_create_counter(key, |c| Counter::from_arc(c.clone()))
    }

    fn gauge(&self, key: &Key) -> Gauge {
        self.registry
            .get_or_create_gauge(key, |g| Gauge::from_arc(g.clone()))
    }

    fn histogram(&self, key: &Key) -> Histogram {
        self.registry
            .get_or_create_histogram(key, |h| Histogram::from_arc(h.clone()))
    }
}

#[derive(Debug)]
struct RecorderWidget {
    metrics: Arc<Metrics>,
}

impl Widget for &RecorderWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let mut lines = Vec::<(Key, String)>::new();
        self.metrics.registry.visit_counters(|key, counter| {
            let value = counter.load(Ordering::SeqCst);
            lines.push((key.clone(), value.to_string()));
        });
        self.metrics.registry.visit_gauges(|key, gauge| {
            let value = gauge.load(Ordering::SeqCst);
            lines.push((key.clone(), value.to_string()));
        });
        self.metrics.registry.visit_histograms(|key, histogram| {
            let mut summary = Summary::with_defaults();
            for data in histogram.data() {
                summary.add(data);
            }
            if summary.is_empty() {
                lines.push((key.clone(), "empty".to_string()));
            } else {
                let min = Duration::from_secs_f64(summary.min());
                let max = Duration::from_secs_f64(summary.max());
                let p50 = Duration::from_secs_f64(summary.quantile(0.5).unwrap());
                let p90 = Duration::from_secs_f64(summary.quantile(0.9).unwrap());
                let p99 = Duration::from_secs_f64(summary.quantile(0.99).unwrap());
                let line = format!(
                    "min={min:.2?} max={max:.2?} p50={p50:.2?} p90={p90:.2?} p99={p99:.2?}"
                );
                lines.push((key.clone(), line));
            }
        });
        lines.sort();
        let rows = lines
            .iter()
            .map(|(key, line)| Row::new([key.name(), line]))
            .enumerate()
            .map(|(i, row)| {
                if (i % 2) == 0 {
                    row.on_dark_gray()
                } else {
                    row.on_black()
                }
            })
            .collect::<Vec<_>>();
        Table::new(rows, [Constraint::Length(40), Constraint::Fill(1)]).render(area, buf);
    }
}

#[allow(unused_variables)]
impl Recorder for MetricsRecorder {
    fn describe_counter(&self, key: KeyName, unit: Option<Unit>, description: SharedString) {
        // todo!()
    }

    fn describe_gauge(&self, key: KeyName, unit: Option<Unit>, description: SharedString) {
        // todo!()
    }

    fn describe_histogram(&self, key: KeyName, unit: Option<Unit>, description: SharedString) {
        // todo!()
    }

    fn register_counter(&self, key: &Key, metadata: &Metadata<'_>) -> Counter {
        self.metrics.counter(key)
    }

    fn register_gauge(&self, key: &Key, metadata: &Metadata<'_>) -> Gauge {
        self.metrics.gauge(key)
    }

    fn register_histogram(&self, key: &Key, metadata: &Metadata<'_>) -> Histogram {
        self.metrics.histogram(key)
    }
}

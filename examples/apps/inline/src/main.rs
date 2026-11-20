/// A Ratatui example that demonstrates how to use an inline viewport.
///
/// The example simulates downloading a queue of files using a small pool of worker threads.
/// There are more downloads ([`NUM_DOWNLOADS`]) than workers ([`NUM_WORKERS`]), so each worker
/// runs one download at a time and takes the next download from the queue when it finishes.
///
/// The inline viewport shows the total progress and one row per worker, with that worker's
/// current download and a progress bar. When a download finishes, a line naming the worker and
/// the download is printed above the viewport with [`Terminal::insert_before`], so it stays in
/// the terminal's scrollback after the example exits.
///
/// Press `q` or `Ctrl+C` to quit early.
///
/// This example runs with the Ratatui library code in the branch that you are currently
/// reading. See the [`latest`] branch for the code which works with the most recent Ratatui
/// release.
///
/// [`Terminal::insert_before`]: ratatui::Terminal::insert_before
/// [`latest`]: https://github.com/ratatui/ratatui/tree/latest
use std::collections::VecDeque;
use std::sync::mpsc::{self, SendError};
use std::time::{Duration, Instant};
use std::{io, thread};

use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyModifiers};
use rand::distr::{Distribution, Uniform};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::macros::{line, span};
use ratatui::style::{Color, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Gauge, LineGauge, Widget};
use ratatui::{DefaultTerminal, TerminalOptions, Viewport};

/// How many downloads are queued at the start.
const NUM_DOWNLOADS: u32 = 10;

/// How many worker threads run the downloads.
///
/// This is fewer than [`NUM_DOWNLOADS`], so at most this many downloads run at the same time and
/// the rest wait in the queue. Each worker has its own row in the viewport.
const NUM_WORKERS: usize = 4;

/// How long to wait for a key press before redrawing and applying worker updates.
const TICK_RATE: Duration = Duration::from_millis(200);

/// Runs the example in an inline viewport, restoring the terminal afterwards.
///
/// The viewport is 6 lines high: the total progress, a blank line, and one row for each of the
/// [`NUM_WORKERS`] workers.
fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init_with_options(TerminalOptions {
        viewport: Viewport::Inline(6),
    });
    let result = App::new().run(&mut terminal);
    ratatui::restore();
    result
}

/// A queue of downloads and the pool of workers that take downloads from it.
struct App {
    /// Downloads waiting for a free worker, in the order they will start.
    queue: VecDeque<Download>,
    /// The workers, indexed by worker id, each with the download it is running.
    workers: Vec<Worker>,
    /// How many downloads have finished.
    finished: u32,
    /// Receives progress updates from all of the worker threads.
    update_rx: mpsc::Receiver<Update>,
}

impl App {
    /// Spawns the worker threads, with an empty queue of downloads.
    fn new() -> Self {
        let (update_tx, update_rx) = mpsc::channel();
        let workers = (0..NUM_WORKERS)
            .map(|id| Worker::spawn(id, update_tx.clone()))
            .collect();

        Self {
            queue: VecDeque::new(),
            workers,
            finished: 0,
            update_rx,
        }
    }

    /// Runs until every download has finished or the user presses `q` or `Ctrl+C`.
    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.queue_downloads();
        for worker_id in 0..NUM_WORKERS {
            self.start_next_download(worker_id)?;
        }

        while !self.is_finished() {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            if Self::quit_requested()? {
                return Self::print_line(terminal, self.cancelled_line());
            }
            self.apply_updates(terminal)?;
        }
        Self::print_line(terminal, line!["Done!"])
    }

    /// Adds downloads with random sizes to the queue.
    fn queue_downloads(&mut self) {
        let sizes = Uniform::new(0, 1000).expect("valid range");
        let mut rng = rand::rng();
        let downloads = (0..NUM_DOWNLOADS).map(|id| Download {
            id,
            size: sizes.sample(&mut rng),
        });
        self.queue.extend(downloads);
    }

    /// Gives the worker the next download from the queue, if there is one.
    fn start_next_download(&mut self, worker_id: usize) -> Result<()> {
        if let Some(download) = self.queue.pop_front() {
            self.workers[worker_id].start(download)?;
        }
        Ok(())
    }

    /// Whether the queue is empty and every worker is idle.
    fn is_finished(&self) -> bool {
        self.queue.is_empty() && self.workers.iter().all(Worker::is_idle)
    }

    /// Waits up to one tick for the user to press `q` or `Ctrl+C`.
    fn quit_requested() -> io::Result<bool> {
        if !event::poll(TICK_RATE)? {
            return Ok(false);
        }
        let Some(key) = event::read()?.as_key_press_event() else {
            return Ok(false);
        };
        let quit = match key.code {
            KeyCode::Char('q') => true,
            KeyCode::Char('c') => key.modifiers.contains(KeyModifiers::CONTROL),
            _ => false,
        };
        Ok(quit)
    }

    /// Prints a line above the inline viewport, where it stays after the example exits.
    fn print_line(terminal: &mut DefaultTerminal, line: Line) -> Result<()> {
        terminal.insert_before(1, |buf| line.render(buf.area, buf))?;
        Ok(())
    }

    /// The line printed above the viewport when the user quits early.
    fn cancelled_line(&self) -> Line<'static> {
        let finished = self.finished;
        let busy_workers = self.workers.iter().filter(|worker| !worker.is_idle());
        let running = busy_workers.count();
        let summary = span!(" with {finished}/{NUM_DOWNLOADS} finished and {running} running");
        line!["Cancelled".bold(), summary]
    }

    /// Applies every update the worker threads have sent since the last call.
    fn apply_updates(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while let Ok(update) = self.update_rx.try_recv() {
            match update {
                Update::Progress {
                    worker_id,
                    progress,
                } => self.workers[worker_id].set_progress(progress),
                Update::Done { worker_id } => self.finish_download(terminal, worker_id)?,
            }
        }
        Ok(())
    }

    /// Prints the worker's finished download above the viewport, then gives the now free worker
    /// the next download from the queue.
    fn finish_download(&mut self, terminal: &mut DefaultTerminal, worker_id: usize) -> Result<()> {
        let worker = &mut self.workers[worker_id];
        if let Some(download) = worker.finish() {
            self.finished += 1;
            Self::print_line(terminal, worker.finished_line(&download))?;
        }
        self.start_next_download(worker_id)
    }
}

/// Renders the total progress, then one row for each worker.
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
        let [total_area, workers_area] = area.layout(&vertical);

        self.render_total_progress(total_area, buf);
        for (worker, row) in self.workers.iter().zip(workers_area.rows()) {
            worker.render(row, buf);
        }
    }
}

impl App {
    /// Renders a gauge showing how many of the downloads have finished.
    fn render_total_progress(&self, area: Rect, buf: &mut Buffer) {
        LineGauge::default()
            .filled_style(Color::Blue)
            .label(format!("{}/{NUM_DOWNLOADS}", self.finished))
            .ratio(f64::from(self.finished) / f64::from(NUM_DOWNLOADS))
            .render(area, buf);
    }
}

/// The main thread's view of a worker: a handle to its thread and the download it is running.
///
/// This follows the actor pattern: the [`WorkerThread`] owns its own state and runs on its own
/// thread, and the main thread only talks to it through this type. The thread stops once its
/// `Worker` is dropped.
struct Worker {
    /// Identifies the worker in the output. This is also its row in the viewport and its index
    /// in [`App::workers`].
    id: usize,
    /// Sends downloads to the worker thread.
    download_tx: mpsc::Sender<Download>,
    /// The download the worker is running, or `None` if it is idle.
    current: Option<ActiveDownload>,
}

impl Worker {
    /// Spawns a worker thread with the given id, which sends its progress to `update_tx`.
    fn spawn(id: usize, update_tx: mpsc::Sender<Update>) -> Self {
        let (download_tx, download_rx) = mpsc::channel();
        let worker_thread = WorkerThread {
            id,
            download_rx,
            update_tx,
        };
        thread::spawn(move || worker_thread.run());
        Self {
            id,
            download_tx,
            current: None,
        }
    }

    /// Starts the download on this worker's thread.
    ///
    /// This only fails if the worker thread has stopped.
    fn start(&mut self, download: Download) -> Result<()> {
        self.current = Some(ActiveDownload {
            id: download.id,
            started_at: Instant::now(),
            progress: 0.0,
        });
        self.download_tx.send(download)?;
        Ok(())
    }

    /// Whether the worker has no download to run.
    const fn is_idle(&self) -> bool {
        self.current.is_none()
    }

    /// Records the progress of the worker's current download.
    const fn set_progress(&mut self, progress: f64) {
        if let Some(download) = &mut self.current {
            download.progress = progress;
        }
    }

    /// Marks the worker as idle, returning the download it finished.
    const fn finish(&mut self) -> Option<ActiveDownload> {
        self.current.take()
    }

    /// The line printed above the viewport when this worker finishes a download.
    fn finished_line(&self, download: &ActiveDownload) -> Line<'static> {
        let name = span!("download {}", download.id).bold();
        let elapsed = span!(" in {}ms", download.started_at.elapsed().as_millis());
        line![self.label(), " finished ", name, elapsed]
    }

    /// The label that identifies this worker in the viewport and in the lines printed above it.
    fn label(&self) -> Span<'static> {
        span!("worker {}", self.id).cyan()
    }
}

/// Renders the worker's label, then its current download or that it is idle.
impl Widget for &Worker {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let horizontal = Layout::horizontal([Constraint::Length(10), Constraint::Fill(1)]);
        let [label_area, download_area] = area.layout(&horizontal);

        self.label().render(label_area, buf);
        match &self.current {
            Some(download) => download.render(download_area, buf),
            None => "idle".dark_gray().render(download_area, buf),
        }
    }
}

/// The actor that runs on a worker's thread, simulating each download it is given.
struct WorkerThread {
    /// Identifies the worker in the updates it sends.
    id: usize,
    /// Receives the downloads to run, one at a time.
    download_rx: mpsc::Receiver<Download>,
    /// Sends progress updates to the main thread.
    update_tx: mpsc::Sender<Update>,
}

impl WorkerThread {
    /// Runs until the [`Worker`] is dropped or the main thread stops receiving updates.
    fn run(&self) -> Result<(), SendError<Update>> {
        while let Ok(download) = self.download_rx.recv() {
            self.simulate(&download)?;
        }
        Ok(())
    }

    /// Simulates one download, sending progress updates and then a final `Done`.
    ///
    /// Each step downloads 10 units of the download's size and takes 100ms.
    fn simulate(&self, download: &Download) -> Result<(), SendError<Update>> {
        let mut downloaded = 0;
        while downloaded < download.size {
            thread::sleep(Duration::from_millis(100));
            downloaded = (downloaded + 10).min(download.size);
            let progress = f64::from(downloaded) / f64::from(download.size);
            self.update_tx.send(Update::Progress {
                worker_id: self.id,
                progress,
            })?;
        }
        self.update_tx.send(Update::Done { worker_id: self.id })
    }
}

/// Updates sent from the worker threads to the main thread.
enum Update {
    /// The worker's current download has made progress, from 0.0 to 1.0.
    Progress { worker_id: usize, progress: f64 },
    /// The worker has finished its current download.
    Done { worker_id: usize },
}

/// A download to simulate, which waits in the queue until a worker takes it.
struct Download {
    /// Identifies the download in the output.
    id: u32,
    /// The simulated size, which sets how long the download takes (see
    /// [`WorkerThread::simulate`]).
    size: u32,
}

/// A download that a worker is running.
struct ActiveDownload {
    /// Identifies the download in the output.
    id: u32,
    /// When the download was given to the worker.
    started_at: Instant,
    /// Progress from 0.0 to 1.0.
    progress: f64,
}

/// Renders the download's name and elapsed time, followed by a progress bar.
impl Widget for &ActiveDownload {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let horizontal = Layout::horizontal([Constraint::Length(22), Constraint::Fill(1)]);
        let [status_area, gauge_area] = area.layout(&horizontal);

        let name = span!("download {:>2}", self.id).light_green().bold();
        let elapsed = span!(" ({}ms)", self.started_at.elapsed().as_millis());
        line![name, elapsed].render(status_area, buf);

        Gauge::default()
            .gauge_style(Color::Yellow)
            .ratio(self.progress)
            .render(gauge_area, buf);
    }
}

/// A Ratatui example that demonstrates different types of constraints.
///
/// This example runs with the Ratatui library code in the branch that you are currently
/// reading. See the [`latest`] branch for the code which works with the most recent Ratatui
/// release.
///
/// [`latest`]: https://github.com/ratatui/ratatui/tree/latest
use std::sync::LazyLock;

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::buffer::Buffer;
use ratatui::layout::Constraint::{self, Fill, Length, Max, Min, Percentage, Ratio};
use ratatui::layout::{Layout, Rect};
use ratatui::style::palette::tailwind;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{
    Block, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget,
    Tabs, Widget,
};
use ratatui::{DefaultTerminal, symbols};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

const SPACER_HEIGHT: u16 = 0;
const ILLUSTRATION_HEIGHT: u16 = 4;
const EXAMPLE_HEIGHT: u16 = ILLUSTRATION_HEIGHT + SPACER_HEIGHT;

// Colors are resolved once at startup. On terminals that don't support 24-bit color
// (e.g. Apple Terminal.app before build 465) the truecolor tailwind palette looks bad,
// so we fall back to indexed colors. See issue #1972.
static THEME: LazyLock<Theme> = LazyLock::new(Theme::new);

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::default().run(terminal))
}

#[derive(Default, Clone, Copy)]
struct App {
    selected_tab: SelectedTab,
    scroll_offset: u16,
    max_scroll_offset: u16,
    state: AppState,
}

/// Tabs for the different examples
///
/// The order of the variants is the order in which they are displayed.
#[derive(Default, Debug, Copy, Clone, Display, FromRepr, EnumIter, PartialEq, Eq)]
enum SelectedTab {
    #[default]
    Min,
    Max,
    Length,
    Percentage,
    Ratio,
    Fill,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quit,
}

impl App {
    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.update_max_scroll_offset();
        while self.is_running() {
            terminal.draw(|frame| frame.render_widget(self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    const fn update_max_scroll_offset(&mut self) {
        self.max_scroll_offset = (self.selected_tab.get_example_count() - 1) * EXAMPLE_HEIGHT;
    }

    fn is_running(self) -> bool {
        self.state == AppState::Running
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                KeyCode::Char('l') | KeyCode::Right => self.next(),
                KeyCode::Char('h') | KeyCode::Left => self.previous(),
                KeyCode::Char('j') | KeyCode::Down => self.down(),
                KeyCode::Char('k') | KeyCode::Up => self.up(),
                KeyCode::Char('g') | KeyCode::Home => self.top(),
                KeyCode::Char('G') | KeyCode::End => self.bottom(),
                _ => (),
            }
        }
        Ok(())
    }

    const fn quit(&mut self) {
        self.state = AppState::Quit;
    }

    fn next(&mut self) {
        self.selected_tab = self.selected_tab.next();
        self.update_max_scroll_offset();
        self.scroll_offset = 0;
    }

    fn previous(&mut self) {
        self.selected_tab = self.selected_tab.previous();
        self.update_max_scroll_offset();
        self.scroll_offset = 0;
    }

    const fn up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    fn down(&mut self) {
        self.scroll_offset = self
            .scroll_offset
            .saturating_add(1)
            .min(self.max_scroll_offset);
    }

    const fn top(&mut self) {
        self.scroll_offset = 0;
    }

    const fn bottom(&mut self) {
        self.scroll_offset = self.max_scroll_offset;
    }
}

impl Widget for App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [tabs, axis, demo] = area.layout(&Layout::vertical([Length(3), Length(3), Fill(0)]));

        self.render_tabs(tabs, buf);
        Self::render_axis(axis, buf);
        self.render_demo(demo, buf);
    }
}

impl App {
    fn render_tabs(self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedTab::iter().map(SelectedTab::to_tab_title);
        let block = Block::new()
            .title("Constraints ".bold())
            .title(" Use h l or ◄ ► to change tab and j k or ▲ ▼  to scroll");
        Tabs::new(titles)
            .block(block)
            .highlight_style(Modifier::REVERSED)
            .select(self.selected_tab as usize)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }

    fn render_axis(area: Rect, buf: &mut Buffer) {
        let width = area.width as usize;
        // a bar like `<----- 80 px ----->`
        let width_label = format!("{width} px");
        let width_bar = format!(
            "<{width_label:-^width$}>",
            width = width - width_label.len() / 2
        );
        Paragraph::new(width_bar.dark_gray())
            .centered()
            .block(Block::new().padding(Padding {
                left: 0,
                right: 0,
                top: 1,
                bottom: 0,
            }))
            .render(area, buf);
    }

    /// Render the demo content
    ///
    /// This function renders the demo content into a separate buffer and then splices the buffer
    /// into the main buffer. This is done to make it possible to handle scrolling easily.
    #[expect(clippy::cast_possible_truncation)]
    fn render_demo(self, area: Rect, buf: &mut Buffer) {
        // render demo content into a separate buffer so all examples fit we add an extra
        // area.height to make sure the last example is fully visible even when the scroll offset is
        // at the max
        let height = self.selected_tab.get_example_count() * EXAMPLE_HEIGHT;
        let demo_area = Rect::new(0, 0, area.width, height + area.height);
        let mut demo_buf = Buffer::empty(demo_area);

        let scrollbar_needed = self.scroll_offset != 0 || height > area.height;
        let content_area = if scrollbar_needed {
            Rect {
                width: demo_area.width - 1,
                ..demo_area
            }
        } else {
            demo_area
        };
        self.selected_tab.render(content_area, &mut demo_buf);

        let visible_content = demo_buf
            .content
            .into_iter()
            .skip((demo_area.width * self.scroll_offset) as usize)
            .take(area.area() as usize);
        for (i, cell) in visible_content.enumerate() {
            let x = i as u16 % area.width;
            let y = i as u16 / area.width;
            buf[(area.x + x, area.y + y)] = cell;
        }

        if scrollbar_needed {
            let mut state = ScrollbarState::new(self.max_scroll_offset as usize)
                .position(self.scroll_offset as usize);
            Scrollbar::new(ScrollbarOrientation::VerticalRight).render(area, buf, &mut state);
        }
    }
}

impl SelectedTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }

    const fn get_example_count(self) -> u16 {
        #[expect(clippy::match_same_arms)]
        match self {
            Self::Length => 4,
            Self::Percentage => 5,
            Self::Ratio => 4,
            Self::Fill => 2,
            Self::Min => 5,
            Self::Max => 5,
        }
    }

    fn to_tab_title(value: Self) -> Line<'static> {
        let text = format!("  {value}  ");
        let color = match value {
            Self::Length => THEME.length,
            Self::Percentage => THEME.percentage,
            Self::Ratio => THEME.ratio,
            Self::Fill => THEME.fill,
            Self::Min => THEME.min,
            Self::Max => THEME.max,
        };
        text.fg(THEME.tab_fg).bg(color).into()
    }
}

impl Widget for SelectedTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            Self::Length => Self::render_length_example(area, buf),
            Self::Percentage => Self::render_percentage_example(area, buf),
            Self::Ratio => Self::render_ratio_example(area, buf),
            Self::Fill => Self::render_fill_example(area, buf),
            Self::Min => Self::render_min_example(area, buf),
            Self::Max => Self::render_max_example(area, buf),
        }
    }
}

impl SelectedTab {
    fn render_length_example(area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([Length(EXAMPLE_HEIGHT); 4]);
        let [example1, example2, example3, _] = area.layout(&layout);

        Example::new(&[Length(20), Length(20)]).render(example1, buf);
        Example::new(&[Length(20), Min(20)]).render(example2, buf);
        Example::new(&[Length(20), Max(20)]).render(example3, buf);
    }

    fn render_percentage_example(area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([Length(EXAMPLE_HEIGHT); 6]);
        let [example1, example2, example3, example4, example5, _] = area.layout(&layout);

        Example::new(&[Percentage(75), Fill(0)]).render(example1, buf);
        Example::new(&[Percentage(25), Fill(0)]).render(example2, buf);
        Example::new(&[Percentage(50), Min(20)]).render(example3, buf);
        Example::new(&[Percentage(0), Max(0)]).render(example4, buf);
        Example::new(&[Percentage(0), Fill(0)]).render(example5, buf);
    }

    fn render_ratio_example(area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([Length(EXAMPLE_HEIGHT); 5]);
        let [example1, example2, example3, example4, _] = area.layout(&layout);

        Example::new(&[Ratio(1, 2); 2]).render(example1, buf);
        Example::new(&[Ratio(1, 4); 4]).render(example2, buf);
        Example::new(&[Ratio(1, 2), Ratio(1, 3), Ratio(1, 4)]).render(example3, buf);
        Example::new(&[Ratio(1, 2), Percentage(25), Length(10)]).render(example4, buf);
    }

    fn render_fill_example(area: Rect, buf: &mut Buffer) {
        let [example1, example2, _] = area.layout(&Layout::vertical([Length(EXAMPLE_HEIGHT); 3]));

        Example::new(&[Fill(1), Fill(2), Fill(3)]).render(example1, buf);
        Example::new(&[Fill(1), Percentage(50), Fill(1)]).render(example2, buf);
    }

    fn render_min_example(area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([Length(EXAMPLE_HEIGHT); 6]);
        let [example1, example2, example3, example4, example5, _] = area.layout(&layout);

        Example::new(&[Percentage(100), Min(0)]).render(example1, buf);
        Example::new(&[Percentage(100), Min(20)]).render(example2, buf);
        Example::new(&[Percentage(100), Min(40)]).render(example3, buf);
        Example::new(&[Percentage(100), Min(60)]).render(example4, buf);
        Example::new(&[Percentage(100), Min(80)]).render(example5, buf);
    }

    fn render_max_example(area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([Length(EXAMPLE_HEIGHT); 6]);
        let [example1, example2, example3, example4, example5, _] = area.layout(&layout);

        Example::new(&[Percentage(0), Max(0)]).render(example1, buf);
        Example::new(&[Percentage(0), Max(20)]).render(example2, buf);
        Example::new(&[Percentage(0), Max(40)]).render(example3, buf);
        Example::new(&[Percentage(0), Max(60)]).render(example4, buf);
        Example::new(&[Percentage(0), Max(80)]).render(example5, buf);
    }
}

struct Example {
    constraints: Vec<Constraint>,
}

impl Example {
    fn new(constraints: &[Constraint]) -> Self {
        Self {
            constraints: constraints.into(),
        }
    }
}

impl Widget for Example {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Length(ILLUSTRATION_HEIGHT), Length(SPACER_HEIGHT)]);
        let horizontal = Layout::horizontal(&self.constraints);
        let [area, _] = area.layout(&vertical);
        let blocks = area.layout_vec(&horizontal);

        for (block, constraint) in blocks.iter().zip(&self.constraints) {
            Self::illustration(*constraint, block.width).render(*block, buf);
        }
    }
}

impl Example {
    fn illustration(constraint: Constraint, width: u16) -> impl Widget {
        let color = match constraint {
            Constraint::Length(_) => THEME.length,
            Constraint::Percentage(_) => THEME.percentage,
            Constraint::Ratio(_, _) => THEME.ratio,
            Constraint::Fill(_) => THEME.fill,
            Constraint::Min(_) => THEME.min,
            Constraint::Max(_) => THEME.max,
        };
        let fg = Color::White;
        let title = format!("{constraint}");
        let content = format!("{width} px");
        let text = format!("{title}\n{content}");
        let block = Block::bordered()
            .border_set(symbols::border::QUADRANT_OUTSIDE)
            .border_style(Style::reset().fg(color).reversed())
            .style(Style::default().fg(fg).bg(color));
        Paragraph::new(text).centered().block(block)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Theme {
    min: Color,
    max: Color,
    length: Color,
    percentage: Color,
    ratio: Color,
    fill: Color,
    tab_fg: Color,
}

impl Theme {
    fn new() -> Self {
        let term_program = std::env::var("TERM_PROGRAM").ok();
        let term_program_version = std::env::var("TERM_PROGRAM_VERSION").ok();
        let is_true_color =
            Self::supports_true_color(term_program.as_deref(), term_program_version.as_deref());
        Self::from_true_color(is_true_color)
    }

    // Builds the theme for a terminal that either does or does not support 24-bit color.
    fn from_true_color(is_true_color: bool) -> Self {
        use tailwind::{BLUE, SLATE};

        let color = |true_color, indexed_color| {
            if is_true_color {
                true_color
            } else {
                indexed_color
            }
        };

        Self {
            min: color(BLUE.c900, Color::Indexed(24)),
            max: color(BLUE.c800, Color::Indexed(25)),
            length: color(SLATE.c700, Color::Indexed(67)),
            percentage: color(SLATE.c800, Color::Indexed(18)),
            ratio: color(SLATE.c900, Color::Indexed(17)),
            fill: color(SLATE.c950, Color::Indexed(16)),
            tab_fg: color(SLATE.c200, Color::Indexed(252)),
        }
    }

    // Checks whether truecolor (24-bit color) is supported by the current terminal.
    //
    // Terminals known *not* to support truecolor:
    // - Apple Terminal.app: all versions before build 465
    //
    // Environment variables used:
    // - `TERM_PROGRAM`: identifies the terminal application in use
    // - `TERM_PROGRAM_VERSION`: build number of that terminal application
    fn supports_true_color(term_program: Option<&str>, term_program_version: Option<&str>) -> bool {
        if term_program != Some("Apple_Terminal") {
            return true;
        }
        let build_number = term_program_version
            .and_then(|version| version.split('.').next())
            .and_then(|version| version.parse::<u32>().ok())
            .unwrap_or(0);
        build_number >= 465
    }
}

#[cfg(test)]
mod tests {
    use ratatui::style::Color;

    use super::Theme;

    #[test]
    fn apple_terminal_before_build_465_does_not_support_true_color() {
        assert!(!Theme::supports_true_color(
            Some("Apple_Terminal"),
            Some("464")
        ));
        assert!(!Theme::supports_true_color(
            Some("Apple_Terminal"),
            Some("455.1")
        ));
        assert!(!Theme::supports_true_color(
            Some("Apple_Terminal"),
            Some("unknown")
        ));
        assert!(!Theme::supports_true_color(Some("Apple_Terminal"), None));
    }

    #[test]
    fn apple_terminal_build_465_and_later_supports_true_color() {
        assert!(Theme::supports_true_color(
            Some("Apple_Terminal"),
            Some("465")
        ));
        assert!(Theme::supports_true_color(
            Some("Apple_Terminal"),
            Some("465.1")
        ));
        assert!(Theme::supports_true_color(
            Some("Apple_Terminal"),
            Some("500")
        ));
    }

    #[test]
    fn other_terminals_support_true_color() {
        assert!(Theme::supports_true_color(Some("iTerm.app"), Some("3.5.0")));
        assert!(Theme::supports_true_color(Some("WezTerm"), None));
        assert!(Theme::supports_true_color(None, None));
    }

    #[test]
    fn theme_falls_back_to_indexed_colors_without_true_color() {
        let theme = Theme::from_true_color(false);
        assert_eq!(theme.min, Color::Indexed(24));
        assert_eq!(theme.max, Color::Indexed(25));
        assert_eq!(theme.length, Color::Indexed(67));
        assert_eq!(theme.percentage, Color::Indexed(18));
        assert_eq!(theme.ratio, Color::Indexed(17));
        assert_eq!(theme.fill, Color::Indexed(16));
        assert_eq!(theme.tab_fg, Color::Indexed(252));
    }

    #[test]
    fn theme_uses_truecolor_when_supported() {
        let theme = Theme::from_true_color(true);
        assert!(matches!(theme.min, Color::Rgb(..)));
        assert!(matches!(theme.tab_fg, Color::Rgb(..)));
    }
}

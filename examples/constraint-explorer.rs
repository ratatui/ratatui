//! # [Ratatui] Flex example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui-org/ratatui
//! [examples]: https://github.com/ratatui-org/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui-org/ratatui/blob/main/examples/README.md

use std::io::{self, stdout};

use color_eyre::{config::HookBuilder, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    layout::{Constraint::*, Flex},
    prelude::*,
    style::palette::tailwind,
    symbols::line,
    widgets::*,
};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

// priority 2
const MIN_COLOR: Color = tailwind::BLUE.c900;
const MAX_COLOR: Color = tailwind::BLUE.c800;
// priority 3
const LENGTH_COLOR: Color = tailwind::SLATE.c700;
const PERCENTAGE_COLOR: Color = tailwind::SLATE.c800;
const RATIO_COLOR: Color = tailwind::SLATE.c900;
// priority 4
const FILL_COLOR: Color = tailwind::SLATE.c950;

fn color_for_constraint(constraint: Constraint) -> Color {
    match constraint {
        Constraint::Min(_) => MIN_COLOR,
        Constraint::Max(_) => MAX_COLOR,
        Constraint::Length(_) => LENGTH_COLOR,
        Constraint::Percentage(_) => PERCENTAGE_COLOR,
        Constraint::Ratio(_, _) => RATIO_COLOR,
        Constraint::Fill(_) => FILL_COLOR,
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumIter, FromRepr, Display)]
enum SelectedConstraint {
    #[default]
    Length,
    Percentage,
    Ratio,
    Min,
    Max,
    Fill,
}

impl SelectedConstraint {
    /// Get the previous tab, if there is no previous tab return the current tab.
    fn prev(&self) -> Self {
        let current_index: usize = *self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(*self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    fn next(&self) -> Self {
        let current_index = *self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(*self)
    }

    fn to_tab_title(value: Self) -> Line<'static> {
        use SelectedConstraint::*;
        let text = format!("  {value}  ");
        let color = match value {
            Length => LENGTH_COLOR,
            Percentage => PERCENTAGE_COLOR,
            Ratio => RATIO_COLOR,
            Fill => FILL_COLOR,
            Min => MIN_COLOR,
            Max => MAX_COLOR,
        };
        text.fg(tailwind::SLATE.c200).bg(color).into()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quit,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum PromptState {
    #[default]
    SelectConstraint,
    PromptConstraintDetails(SelectedConstraint),
}

#[derive(Default, Clone)]
struct App {
    spacing: u16,
    state: AppState,
    selected_constraint: SelectedConstraint,
    current: Constraint,
    flex: Flex,
    constraints: Vec<Constraint>,
    prompt_state: PromptState,
}

impl App {
    fn run(&mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        self.draw(&mut terminal)?;
        while self.is_running() {
            self.handle_events()?;
            self.draw(&mut terminal)?;
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.state == AppState::Running
    }

    fn draw(&self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.size()))?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        use KeyCode::*;
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                Char('q') | Esc => self.quit(),
                Char('+') => self.increment_spacing(),
                Char('-') => self.decrement_spacing(),
                Char('l') => self.next(),
                Char('h') => self.prev(),
                Enter => match self.prompt_state {
                    PromptState::SelectConstraint => self.prompt(),
                    PromptState::PromptConstraintDetails(selection) => self.confirm(selection),
                },
                _ => (),
            },
            _ => {}
        }
        Ok(())
    }

    fn increment_spacing(&mut self) {
        self.spacing = self.spacing.saturating_add(1);
    }

    fn decrement_spacing(&mut self) {
        self.spacing = self.spacing.saturating_sub(1);
    }

    fn next(&mut self) {
        self.selected_constraint = self.selected_constraint.next();
    }

    fn prev(&mut self) {
        self.selected_constraint = self.selected_constraint.prev();
    }

    fn prompt(&mut self) {
        self.prompt_state = PromptState::PromptConstraintDetails(self.selected_constraint);
    }

    fn confirm(&mut self, selection: SelectedConstraint) {
        use SelectedConstraint::*;
        let c = match selection {
            Length => Constraint::Length(10),
            Percentage => Constraint::Percentage(10),
            Ratio => Constraint::Ratio(1, 10),
            Min => Constraint::Min(10),
            Max => Constraint::Max(10),
            Fill => Constraint::Fill(1),
        };
        self.constraints.push(c);
        self.prompt_state = PromptState::SelectConstraint;
    }

    fn quit(&mut self) {
        self.state = AppState::Quit;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([Length(3), Length(1), Fill(0)]).spacing(2);
        let [tabs, axis, demo] = area.split(&layout);
        self.render_tabs(tabs, buf);
        let scroll_needed = self.render_demo(demo, buf);
        let axis_width = if scroll_needed {
            axis.width.saturating_sub(1)
        } else {
            axis.width
        };
        let spacing = self.spacing;
        self.axis(axis_width, spacing).render(axis, buf);
        match self.prompt_state {
            PromptState::PromptConstraintDetails(selected) => {
                let [area] = area.split(&Layout::vertical([Percentage(50)]).flex(Flex::Center));
                let [area] = area.split(&Layout::horizontal([Percentage(50)]).flex(Flex::Center));
                self.render_prompt(area, buf);
            }
            _ => {}
        }
    }
}

impl App {
    /// a bar like `<----- 80 px (gap: 2 px)? ----->`
    fn axis(&self, width: u16, spacing: u16) -> impl Widget {
        let width = width as usize;
        // only show gap when spacing is not zero
        let label = if spacing != 0 {
            format!("{} px (gap: {} px)", width, spacing)
        } else {
            format!("{} px", width)
        };
        let bar_width = width.saturating_sub(2); // we want to `<` and `>` at the ends
        let width_bar = format!("<{label:-^bar_width$}>");
        Paragraph::new(width_bar.dark_gray()).centered()
    }

    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedConstraint::iter().map(SelectedConstraint::to_tab_title);
        let block = Block::new()
            .title("Constraints ".bold())
            .title(" Use h l or ◄ ► to change tab and j k or ▲ ▼  to scroll");
        Tabs::new(titles)
            .block(block)
            .highlight_style(Modifier::REVERSED)
            .select(self.selected_constraint as usize)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }

    fn render_prompt(&self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        Paragraph::new("Input")
            .style(Style::reset().dark_gray())
            .alignment(Alignment::Center)
            .block(Block::bordered())
            .render(area, buf);
    }

    fn render_demo(&self, area: Rect, buf: &mut Buffer) -> bool {
        let height = 4;
        let area = Rect::new(area.x, area.y, area.width, height);

        let (blocks, spacers) = Layout::horizontal(&self.constraints)
            .flex(self.flex)
            .spacing(self.spacing)
            .split_with_spacers(area);

        for (block, constraint) in blocks.iter().zip(&self.constraints) {
            self.illustration(*constraint, block.width)
                .render(*block, buf);
        }

        for spacer in spacers.iter() {
            self.render_spacer(*spacer, buf);
        }

        false
    }

    fn render_spacer(&self, spacer: Rect, buf: &mut Buffer) {
        if spacer.width > 1 {
            let corners_only = symbols::border::Set {
                top_left: line::NORMAL.top_left,
                top_right: line::NORMAL.top_right,
                bottom_left: line::NORMAL.bottom_left,
                bottom_right: line::NORMAL.bottom_right,
                vertical_left: " ",
                vertical_right: " ",
                horizontal_top: " ",
                horizontal_bottom: " ",
            };
            Block::bordered()
                .border_set(corners_only)
                .border_style(Style::reset().dark_gray())
                .render(spacer, buf);
        } else {
            Paragraph::new(Text::from(vec![
                Line::from(""),
                Line::from("│"),
                Line::from("│"),
                Line::from(""),
            ]))
            .style(Style::reset().dark_gray())
            .render(spacer, buf);
        }
        let width = spacer.width;
        let label = if width > 4 {
            format!("{width} px")
        } else if width > 2 {
            format!("{width}")
        } else {
            "".to_string()
        };
        let text = Text::from(vec![
            Line::raw(""),
            Line::raw(""),
            Line::styled(label, Style::reset().dark_gray()),
        ]);
        Paragraph::new(text)
            .style(Style::reset().dark_gray())
            .alignment(Alignment::Center)
            .render(spacer, buf);
    }

    fn illustration(&self, constraint: Constraint, width: u16) -> Paragraph {
        let main_color = color_for_constraint(constraint);
        let fg_color = Color::White;
        let title = format!("{constraint}");
        let content = format!("{width} px");
        let text = format!("{title}\n{content}");
        let block = Block::bordered()
            .border_set(symbols::border::QUADRANT_OUTSIDE)
            .border_style(Style::reset().fg(main_color).reversed())
            .style(Style::default().fg(fg_color).bg(main_color));
        Paragraph::new(text).centered().block(block)
    }
}

fn init_error_hooks() -> Result<()> {
    let (panic, error) = HookBuilder::default().into_hooks();
    let panic = panic.into_panic_hook();
    let error = error.into_eyre_hook();
    color_eyre::eyre::set_hook(Box::new(move |e| {
        let _ = restore_terminal();
        error(e)
    }))?;
    std::panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal();
        panic(info)
    }));
    Ok(())
}

fn init_terminal() -> Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn main() -> Result<()> {
    // assuming the user changes spacing about a 100 times or so
    init_error_hooks()?;
    let terminal = init_terminal()?;
    App::default().run(terminal)?;

    restore_terminal()?;
    Ok(())
}

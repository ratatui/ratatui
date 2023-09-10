use std::time::Duration;

use anyhow::{Context, Result};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::prelude::Rect;

use crate::{Root, Term};

#[derive(Debug)]
pub struct App {
    term: Term,
    should_quit: bool,
    context: AppContext,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct AppContext {
    pub tab_index: usize,
    pub row_index: usize,
}

impl App {
    fn new() -> Result<Self> {
        Ok(Self {
            term: Term::start()?,
            should_quit: false,
            context: AppContext::default(),
        })
    }

    pub fn run() -> Result<()> {
        install_panic_hook();
        let mut app = Self::new()?;
        while !app.should_quit {
            app.draw()?;
            app.handle_events()?;
        }
        Term::stop()?;
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        self.term
            .draw(|frame| frame.render_widget(Root::new(&self.context), frame.size()))
            .context("terminal.draw")?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        match Term::next_event(Duration::from_millis(16))? {
            Some(Event::Key(key)) => self.handle_key_event(key),
            Some(Event::Resize(width, height)) => {
                Ok(self.term.resize(Rect::new(0, 0, width, height))?)
            }
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        if key.kind != KeyEventKind::Press {
            return Ok(());
        }

        let context = &mut self.context;
        const TAB_COUNT: usize = 5;
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Tab | KeyCode::BackTab if key.modifiers.contains(KeyModifiers::SHIFT) => {
                let tab_index = context.tab_index + TAB_COUNT; // to wrap around properly
                context.tab_index = tab_index.saturating_sub(1) % TAB_COUNT;
                context.row_index = 0;
            }
            KeyCode::Tab | KeyCode::BackTab => {
                context.tab_index = context.tab_index.saturating_add(1) % TAB_COUNT;
                context.row_index = 0;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                context.row_index = context.row_index.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                context.row_index = context.row_index.saturating_add(1);
            }
            _ => {}
        };
        Ok(())
    }
}

pub fn install_panic_hook() {
    better_panic::install();
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = Term::stop();
        hook(info);
        std::process::exit(1);
    }));
}

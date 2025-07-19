//! # [Ratatui] Enhanced Table Borders Example
//!
//! This example demonstrates the enhanced table border capabilities introduced in ratatui,
//! including fine-grained border control, custom border symbols, and header-specific styling.
//!
//! The latest version of this example is available in the [widget examples] folder in the
//! repository.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [widget examples]: https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Row, Table};
use ratatui_widgets::table::{TableBorders, TableBorderSet};

#[derive(Debug, Clone, Copy)]
enum BorderExample {
    Legacy,
    Individual,
    Outer,
    HeaderSeparator,
    CustomSymbols,
    Mixed,
}

impl BorderExample {
    fn next(self) -> Self {
        match self {
            Self::Legacy => Self::Individual,
            Self::Individual => Self::Outer,
            Self::Outer => Self::HeaderSeparator,
            Self::HeaderSeparator => Self::CustomSymbols,
            Self::CustomSymbols => Self::Mixed,
            Self::Mixed => Self::Legacy,
        }
    }

    fn title(self) -> &'static str {
        match self {
            Self::Legacy => "Legacy Borders (ALL)",
            Self::Individual => "Individual Border Control",
            Self::Outer => "Outer Borders Only",
            Self::HeaderSeparator => "Header Separator + Vertical",
            Self::CustomSymbols => "Custom Border Symbols (Thick)",
            Self::Mixed => "Mixed Configuration",
        }
    }

    fn description(self) -> &'static str {
        match self {
            Self::Legacy => "Traditional internal_borders(TableBorders::ALL)",
            Self::Individual => "table_borders(TOP | BOTTOM | INNER_HORIZONTAL)",
            Self::Outer => "table_borders(TableBorders::OUTER)",
            Self::HeaderSeparator => "table_borders(HEADER_TOP | INNER_VERTICAL)",
            Self::CustomSymbols => "border_set(TableBorderSet::thick())",
            Self::Mixed => "Complex combination with header styling",
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut current_example = BorderExample::Legacy;
    
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| render(frame, current_example))?;
            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Char(' ') | KeyCode::Right | KeyCode::Tab => {
                        current_example = current_example.next();
                    }
                    KeyCode::Left => {
                        // Go to previous example (reverse direction)
                        for _ in 0..5 {
                            current_example = current_example.next();
                        }
                    }
                    _ => {}
                }
            }
        }
    })
}

/// Render the UI with different border examples.
fn render(frame: &mut Frame, example: BorderExample) {
    let layout = Layout::vertical([
        Constraint::Length(3),  // Title and instructions
        Constraint::Fill(1),    // Main table
        Constraint::Length(2),  // Description
    ]).spacing(1);
    let [top, main, bottom] = frame.area().layout(&layout);

    // Title and instructions
    let title_lines = vec![
        Line::from_iter([
            Span::from("Enhanced Table Borders Demo").bold().cyan(),
            Span::from(" - "),
            Span::from(example.title()).yellow(),
        ]),
        Line::from("Press SPACE/TAB/→ for next example, ← for previous, 'q' to quit").dim(),
    ];
    for (i, line) in title_lines.into_iter().enumerate() {
        frame.render_widget(line.centered(), Rect::new(top.x, top.y + i as u16, top.width, 1));
    }

    // Main table with the current border example
    render_example_table(frame, main, example);

    // Description
    let description = Line::from_iter([
        Span::from("Configuration: ").bold(),
        Span::from(example.description()).italic(),
    ]);
    frame.render_widget(description.centered(), bottom);
}

/// Render a table demonstrating the specified border example.
fn render_example_table(frame: &mut Frame, area: Rect, example: BorderExample) {
    let header = Row::new(["Product", "Price", "Category", "Stock"])
        .style(Style::new().bold().white())
        .bottom_margin(1);

    let rows = [
        Row::new(["Laptop", "$999", "Electronics", "15"]),
        Row::new(["Coffee Mug", "$12", "Kitchen", "50"]),
        Row::new(["Notebook", "$5", "Office", "100"]),
        Row::new(["Headphones", "$79", "Electronics", "25"]),
        Row::new(["Desk Lamp", "$45", "Furniture", "8"]),
    ];

    let footer = Row::new(["Total Items", "", "", "198"])
        .style(Style::new().bold().green());

    let widths = [
        Constraint::Percentage(25),
        Constraint::Percentage(20),
        Constraint::Percentage(25),
        Constraint::Percentage(30),
    ];

    let table = match example {
        BorderExample::Legacy => {
            // Traditional approach using internal_borders
            Table::new(rows, widths)
                .header(header)
                .footer(footer)
                .column_spacing(1)
                .style(Color::White)
                .internal_borders(TableBorders::ALL)
                .border_style(Style::new().blue())
        }
        BorderExample::Individual => {
            // Individual border control
            Table::new(rows, widths)
                .header(header)
                .footer(footer)
                .column_spacing(1)
                .style(Color::White)
                .table_borders(TableBorders::TOP | TableBorders::BOTTOM | TableBorders::INNER_HORIZONTAL)
                .border_style(Style::new().green())
        }
        BorderExample::Outer => {
            // Only outer borders
            Table::new(rows, widths)
                .header(header)
                .footer(footer)
                .column_spacing(1)
                .style(Color::White)
                .table_borders(TableBorders::OUTER)
                .border_style(Style::new().red())
        }
        BorderExample::HeaderSeparator => {
            // Header separator with vertical borders
            Table::new(rows, widths)
                .header(header)
                .footer(footer)
                .column_spacing(1)
                .style(Color::White)
                .table_borders(TableBorders::HEADER_TOP | TableBorders::INNER_VERTICAL)
                .border_set(TableBorderSet::with_header_style())
                .border_style(Style::new().magenta())
        }
        BorderExample::CustomSymbols => {
            // Custom thick border symbols
            Table::new(rows, widths)
                .header(header)
                .footer(footer)
                .column_spacing(1)
                .style(Color::White)
                .table_borders(TableBorders::ALL_BORDERS)
                .border_set(TableBorderSet::thick())
                .border_style(Style::new().cyan())
        }
        BorderExample::Mixed => {
            // Complex mixed configuration
            Table::new(rows, widths)
                .header(header)
                .footer(footer)
                .column_spacing(1)
                .style(Color::White)
                .table_borders(
                    TableBorders::OUTER | 
                    TableBorders::INNER_VERTICAL | 
                    TableBorders::HEADER_TOP
                )
                .border_set(TableBorderSet::double())
                .border_style(Style::new().yellow())
        }
    };

    // Wrap in a block to show the difference between internal and external borders
    let block = Block::bordered()
        .title("Product Inventory")
        .title_style(Style::new().bold())
        .border_style(Style::new().dim());

    frame.render_widget(table.block(block), area);
}
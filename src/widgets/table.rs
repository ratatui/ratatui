use std::collections::HashMap;
use std::fmt::Display;
use std::iter::Iterator;

use cassowary::strength::{MEDIUM, REQUIRED, WEAK};
use cassowary::WeightedRelation::*;
use cassowary::{Expression, Solver};

use crate::buffer::Buffer;
use crate::layout::{Constraint, Rect};
use crate::style::Style;
use crate::widgets::{Block, Widget};

/// Holds data to be displayed in a Table widget
pub enum Row<D>
where
    D: Iterator,
    D::Item: Display,
{
    Data(D),
    StyledData(D, Style),
}

/// A widget to display data in formatted columns
///
/// # Examples
///
/// ```
/// # use tui::widgets::{Block, Borders, Table, Row};
/// # use tui::layout::Constraint;
/// # use tui::style::{Style, Color};
/// let row_style = Style::default().fg(Color::White);
/// Table::new(
///         ["Col1", "Col2", "Col3"].into_iter(),
///         vec![
///             Row::StyledData(["Row11", "Row12", "Row13"].into_iter(), row_style),
///             Row::StyledData(["Row21", "Row22", "Row23"].into_iter(), row_style),
///             Row::StyledData(["Row31", "Row32", "Row33"].into_iter(), row_style),
///             Row::Data(["Row41", "Row42", "Row43"].into_iter())
///         ].into_iter()
///     )
///     .block(Block::default().title("Table"))
///     .header_style(Style::default().fg(Color::Yellow))
///     .widths(&[Constraint::Length(5), Constraint::Length(5), Constraint::Length(10)])
///     .style(Style::default().fg(Color::White))
///     .column_spacing(1);
/// ```
pub struct Table<'a, H, R> {
    /// A block to wrap the widget in
    block: Option<Block<'a>>,
    /// Base style for the widget
    style: Style,
    /// Header row for all columns
    header: H,
    /// Style for the header
    header_style: Style,
    /// Width constraints for each column
    widths: &'a [Constraint],
    /// Space between each column
    column_spacing: u16,
    /// Space between the header and the rows
    header_gap: u16,
    /// Data to display in each row
    rows: R,
}

impl<'a, H, R> Default for Table<'a, H, R>
where
    H: Iterator + Default,
    R: Iterator + Default,
{
    fn default() -> Table<'a, H, R> {
        Table {
            block: None,
            style: Style::default(),
            header: H::default(),
            header_style: Style::default(),
            widths: &[],
            rows: R::default(),
            column_spacing: 1,
            header_gap: 1,
        }
    }
}
impl<'a, H, D, R> Table<'a, H, R>
where
    H: Iterator,
    D: Iterator,
    D::Item: Display,
    R: Iterator<Item = Row<D>>,
{
    pub fn new(header: H, rows: R) -> Table<'a, H, R> {
        Table {
            block: None,
            style: Style::default(),
            header: header.into_iter(),
            header_style: Style::default(),
            widths: &[],
            rows,
            column_spacing: 1,
            header_gap: 1,
        }
    }
    pub fn block(mut self, block: Block<'a>) -> Table<'a, H, R> {
        self.block = Some(block);
        self
    }

    pub fn header<II>(mut self, header: II) -> Table<'a, H, R>
    where
        II: IntoIterator<Item = H::Item, IntoIter = H>,
    {
        self.header = header.into_iter();
        self
    }

    pub fn header_style(mut self, style: Style) -> Table<'a, H, R> {
        self.header_style = style;
        self
    }

    pub fn widths(mut self, widths: &'a [Constraint]) -> Table<'a, H, R> {
        let between_0_and_100 = |&w| match w {
            Constraint::Percentage(p) => p <= 100,
            _ => true,
        };
        assert!(
            widths.iter().all(between_0_and_100),
            "Percentages should be between 0 and 100 inclusively."
        );
        self.widths = widths;
        self
    }

    pub fn rows<II>(mut self, rows: II) -> Table<'a, H, R>
    where
        II: IntoIterator<Item = Row<D>, IntoIter = R>,
    {
        self.rows = rows.into_iter();
        self
    }

    pub fn style(mut self, style: Style) -> Table<'a, H, R> {
        self.style = style;
        self
    }

    pub fn column_spacing(mut self, spacing: u16) -> Table<'a, H, R> {
        self.column_spacing = spacing;
        self
    }

    pub fn header_gap(mut self, gap: u16) -> Table<'a, H, R> {
        self.header_gap = gap;
        self
    }
}

impl<'a, H, D, R> Widget for Table<'a, H, R>
where
    H: Iterator,
    H::Item: Display,
    D: Iterator,
    D::Item: Display,
    R: Iterator<Item = Row<D>>,
{
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        // Render block if necessary and get the drawing area
        let table_area = match self.block {
            Some(ref mut b) => {
                b.render(area, buf);
                b.inner(area)
            }
            None => area,
        };

        buf.set_background(table_area, self.style.bg);

        let mut solver = Solver::new();
        let mut var_indices = HashMap::new();
        let mut ccs = Vec::new();
        let mut variables = Vec::new();
        for i in 0..self.widths.len() {
            let var = cassowary::Variable::new();
            variables.push(var);
            var_indices.insert(var, i);
        }
        for (i, constraint) in self.widths.iter().enumerate() {
            ccs.push(variables[i] | GE(WEAK) | 0.);
            ccs.push(match *constraint {
                Constraint::Length(v) => variables[i] | EQ(MEDIUM) | f64::from(v),
                Constraint::Percentage(v) => {
                    variables[i] | EQ(WEAK) | (f64::from(v * area.width) / 100.0)
                }
                Constraint::Ratio(n, d) => {
                    variables[i] | EQ(WEAK) | (f64::from(area.width) * f64::from(n) / f64::from(d))
                }
                Constraint::Min(v) => variables[i] | GE(WEAK) | f64::from(v),
                Constraint::Max(v) => variables[i] | LE(WEAK) | f64::from(v),
            })
        }
        solver
            .add_constraint(
                variables
                    .iter()
                    .fold(Expression::from_constant(0.), |acc, v| acc + *v)
                    | LE(REQUIRED)
                    | f64::from(
                        area.width - 2 - (self.column_spacing * (variables.len() as u16 - 1)),
                    ),
            )
            .unwrap();
        solver.add_constraints(&ccs).unwrap();
        let mut solved_widths = vec![0; variables.len()];
        for &(var, value) in solver.fetch_changes() {
            let index = var_indices[&var];
            let value = if value.is_sign_negative() {
                0
            } else {
                value as u16
            };
            solved_widths[index] = value
        }

        let mut y = table_area.top();
        let mut x = table_area.left();

        // Draw header
        if y < table_area.bottom() {
            for (w, t) in solved_widths.iter().zip(self.header.by_ref()) {
                buf.set_stringn(x, y, format!("{}", t), *w as usize, self.header_style);
                x += *w + self.column_spacing;
            }
        }
        y += 1 + self.header_gap;

        // Draw rows
        let default_style = Style::default();
        if y < table_area.bottom() {
            let remaining = (table_area.bottom() - y) as usize;
            for (i, row) in self.rows.by_ref().take(remaining).enumerate() {
                let (data, style) = match row {
                    Row::Data(d) => (d, default_style),
                    Row::StyledData(d, s) => (d, s),
                };
                x = table_area.left();
                for (w, elt) in solved_widths.iter().zip(data) {
                    buf.set_stringn(x, y + i as u16, format!("{}", elt), *w as usize, style);
                    x += *w + self.column_spacing;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn table_invalid_percentages() {
        Table::new([""].iter(), vec![Row::Data([""].iter())].into_iter())
            .widths(&[Constraint::Percentage(110)]);
    }
}

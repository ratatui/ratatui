use crate::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::Style,
    symbols,
    widgets::{
        canvas::{Canvas, Line, Points},
        Block, Borders, Widget,
    },
};
use std::{borrow::Cow, cmp::max};
use unicode_width::UnicodeWidthStr;

/// An X or Y axis for the chart widget
pub struct Axis<'a, L>
where
    L: AsRef<str> + 'a,
{
    /// Title displayed next to axis end
    title: Option<&'a str>,
    /// Style of the title
    title_style: Style,
    /// Bounds for the axis (all data points outside these limits will not be represented)
    bounds: [f64; 2],
    /// A list of labels to put to the left or below the axis
    labels: Option<&'a [L]>,
    /// The labels' style
    labels_style: Style,
    /// The style used to draw the axis itself
    style: Style,
}

impl<'a, L> Default for Axis<'a, L>
where
    L: AsRef<str>,
{
    fn default() -> Axis<'a, L> {
        Axis {
            title: None,
            title_style: Default::default(),
            bounds: [0.0, 0.0],
            labels: None,
            labels_style: Default::default(),
            style: Default::default(),
        }
    }
}

impl<'a, L> Axis<'a, L>
where
    L: AsRef<str>,
{
    pub fn title(mut self, title: &'a str) -> Axis<'a, L> {
        self.title = Some(title);
        self
    }

    pub fn title_style(mut self, style: Style) -> Axis<'a, L> {
        self.title_style = style;
        self
    }

    pub fn bounds(mut self, bounds: [f64; 2]) -> Axis<'a, L> {
        self.bounds = bounds;
        self
    }

    pub fn labels(mut self, labels: &'a [L]) -> Axis<'a, L> {
        self.labels = Some(labels);
        self
    }

    pub fn labels_style(mut self, style: Style) -> Axis<'a, L> {
        self.labels_style = style;
        self
    }

    pub fn style(mut self, style: Style) -> Axis<'a, L> {
        self.style = style;
        self
    }
}

/// Marker to use when plotting data points
pub enum Marker {
    /// One point per cell
    Dot,
    /// Up to 8 points per cell
    Braille,
}

/// Used to determine which style of graphing to use
pub enum GraphType {
    /// Draw each point
    Scatter,
    /// Draw each point and lines between each point using the same marker
    Line,
}

/// A group of data points
pub struct Dataset<'a> {
    /// Name of the dataset (used in the legend if shown)
    name: Cow<'a, str>,
    /// A reference to the actual data
    data: &'a [(f64, f64)],
    /// Symbol used for each points of this dataset
    marker: Marker,
    /// Determines graph type used for drawing points
    graph_type: GraphType,
    /// Style used to plot this dataset
    style: Style,
}

impl<'a> Default for Dataset<'a> {
    fn default() -> Dataset<'a> {
        Dataset {
            name: Cow::from(""),
            data: &[],
            marker: Marker::Dot,
            graph_type: GraphType::Scatter,
            style: Style::default(),
        }
    }
}

impl<'a> Dataset<'a> {
    pub fn name<S>(mut self, name: S) -> Dataset<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        self.name = name.into();
        self
    }

    pub fn data(mut self, data: &'a [(f64, f64)]) -> Dataset<'a> {
        self.data = data;
        self
    }

    pub fn marker(mut self, marker: Marker) -> Dataset<'a> {
        self.marker = marker;
        self
    }

    pub fn graph_type(mut self, graph_type: GraphType) -> Dataset<'a> {
        self.graph_type = graph_type;
        self
    }

    pub fn style(mut self, style: Style) -> Dataset<'a> {
        self.style = style;
        self
    }
}

/// A container that holds all the infos about where to display each elements of the chart (axis,
/// labels, legend, ...).
#[derive(Debug, Clone, PartialEq)]
struct ChartLayout {
    /// Location of the title of the x axis
    title_x: Option<(u16, u16)>,
    /// Location of the title of the y axis
    title_y: Option<(u16, u16)>,
    /// Location of the first label of the x axis
    label_x: Option<u16>,
    /// Location of the first label of the y axis
    label_y: Option<u16>,
    /// Y coordinate of the horizontal axis
    axis_x: Option<u16>,
    /// X coordinate of the vertical axis
    axis_y: Option<u16>,
    /// Area of the legend
    legend_area: Option<Rect>,
    /// Area of the graph
    graph_area: Rect,
}

impl Default for ChartLayout {
    fn default() -> ChartLayout {
        ChartLayout {
            title_x: None,
            title_y: None,
            label_x: None,
            label_y: None,
            axis_x: None,
            axis_y: None,
            legend_area: None,
            graph_area: Rect::default(),
        }
    }
}

/// A widget to plot one or more dataset in a cartesian coordinate system
///
/// # Examples
///
/// ```
/// # use tui::widgets::{Block, Borders, Chart, Axis, Dataset, Marker, GraphType};
/// # use tui::style::{Style, Color};
/// Chart::default()
///     .block(Block::default().title("Chart"))
///     .x_axis(Axis::default()
///         .title("X Axis")
///         .title_style(Style::default().fg(Color::Red))
///         .style(Style::default().fg(Color::White))
///         .bounds([0.0, 10.0])
///         .labels(&["0.0", "5.0", "10.0"]))
///     .y_axis(Axis::default()
///         .title("Y Axis")
///         .title_style(Style::default().fg(Color::Red))
///         .style(Style::default().fg(Color::White))
///         .bounds([0.0, 10.0])
///         .labels(&["0.0", "5.0", "10.0"]))
///     .datasets(&[Dataset::default()
///                     .name("data1")
///                     .marker(Marker::Dot)
///                     .graph_type(GraphType::Scatter)
///                     .style(Style::default().fg(Color::Cyan))
///                     .data(&[(0.0, 5.0), (1.0, 6.0), (1.5, 6.434)]),
///                 Dataset::default()
///                     .name("data2")
///                     .marker(Marker::Braille)
///                     .graph_type(GraphType::Line)
///                     .style(Style::default().fg(Color::Magenta))
///                     .data(&[(4.0, 5.0), (5.0, 8.0), (7.66, 13.5)])]);
/// ```
pub struct Chart<'a, LX, LY>
where
    LX: AsRef<str> + 'a,
    LY: AsRef<str> + 'a,
{
    /// A block to display around the widget eventually
    block: Option<Block<'a>>,
    /// The horizontal axis
    x_axis: Axis<'a, LX>,
    /// The vertical axis
    y_axis: Axis<'a, LY>,
    /// A reference to the datasets
    datasets: &'a [Dataset<'a>],
    /// The widget base style
    style: Style,
    /// Constraints used to determine whether the legend should be shown or
    /// not
    hidden_legend_constraints: (Constraint, Constraint),
}

impl<'a, LX, LY> Default for Chart<'a, LX, LY>
where
    LX: AsRef<str>,
    LY: AsRef<str>,
{
    fn default() -> Chart<'a, LX, LY> {
        Chart {
            block: None,
            x_axis: Axis::default(),
            y_axis: Axis::default(),
            style: Default::default(),
            datasets: &[],
            hidden_legend_constraints: (Constraint::Ratio(1, 4), Constraint::Ratio(1, 4)),
        }
    }
}

impl<'a, LX, LY> Chart<'a, LX, LY>
where
    LX: AsRef<str>,
    LY: AsRef<str>,
{
    pub fn block(mut self, block: Block<'a>) -> Chart<'a, LX, LY> {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> Chart<'a, LX, LY> {
        self.style = style;
        self
    }

    pub fn x_axis(mut self, axis: Axis<'a, LX>) -> Chart<'a, LX, LY> {
        self.x_axis = axis;
        self
    }

    pub fn y_axis(mut self, axis: Axis<'a, LY>) -> Chart<'a, LX, LY> {
        self.y_axis = axis;
        self
    }

    pub fn datasets(mut self, datasets: &'a [Dataset<'a>]) -> Chart<'a, LX, LY> {
        self.datasets = datasets;
        self
    }

    /// Set the constraints used to determine whether the legend should be shown or
    /// not.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tui::widgets::Chart;
    /// # use tui::layout::Constraint;
    /// let constraints = (
    ///     Constraint::Ratio(1, 3),
    ///     Constraint::Ratio(1, 4)
    /// );
    /// // Hide the legend when either its width is greater than 33% of the total widget width
    /// // or if its height is greater than 25% of the total widget height.
    /// let _chart: Chart<String, String> = Chart::default()
    ///     .hidden_legend_constraints(constraints);
    pub fn hidden_legend_constraints(
        mut self,
        constraints: (Constraint, Constraint),
    ) -> Chart<'a, LX, LY> {
        self.hidden_legend_constraints = constraints;
        self
    }

    /// Compute the internal layout of the chart given the area. If the area is too small some
    /// elements may be automatically hidden
    fn layout(&self, area: Rect) -> ChartLayout {
        let mut layout = ChartLayout::default();
        if area.height == 0 || area.width == 0 {
            return layout;
        }
        let mut x = area.left();
        let mut y = area.bottom() - 1;

        if self.x_axis.labels.is_some() && y > area.top() {
            layout.label_x = Some(y);
            y -= 1;
        }

        if let Some(y_labels) = self.y_axis.labels {
            let mut max_width = y_labels
                .iter()
                .fold(0, |acc, l| max(l.as_ref().width(), acc))
                as u16;
            if let Some(x_labels) = self.x_axis.labels {
                if !x_labels.is_empty() {
                    max_width = max(max_width, x_labels[0].as_ref().width() as u16);
                }
            }
            if x + max_width < area.right() {
                layout.label_y = Some(x);
                x += max_width;
            }
        }

        if self.x_axis.labels.is_some() && y > area.top() {
            layout.axis_x = Some(y);
            y -= 1;
        }

        if self.y_axis.labels.is_some() && x + 1 < area.right() {
            layout.axis_y = Some(x);
            x += 1;
        }

        if x < area.right() && y > 1 {
            layout.graph_area = Rect::new(x, area.top(), area.right() - x, y - area.top() + 1);
        }

        if let Some(title) = self.x_axis.title {
            let w = title.width() as u16;
            if w < layout.graph_area.width && layout.graph_area.height > 2 {
                layout.title_x = Some((x + layout.graph_area.width - w, y));
            }
        }

        if let Some(title) = self.y_axis.title {
            let w = title.width() as u16;
            if w + 1 < layout.graph_area.width && layout.graph_area.height > 2 {
                layout.title_y = Some((x + 1, area.top()));
            }
        }

        if let Some(inner_width) = self.datasets.iter().map(|d| d.name.width() as u16).max() {
            let legend_width = inner_width + 2;
            let legend_height = self.datasets.len() as u16 + 2;
            let max_legend_width = self
                .hidden_legend_constraints
                .0
                .apply(layout.graph_area.width);
            let max_legend_height = self
                .hidden_legend_constraints
                .1
                .apply(layout.graph_area.height);
            if inner_width > 0
                && legend_width < max_legend_width
                && legend_height < max_legend_height
            {
                layout.legend_area = Some(Rect::new(
                    layout.graph_area.right() - legend_width,
                    layout.graph_area.top(),
                    legend_width,
                    legend_height,
                ));
            }
        }
        layout
    }
}

impl<'a, LX, LY> Widget for Chart<'a, LX, LY>
where
    LX: AsRef<str>,
    LY: AsRef<str>,
{
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let chart_area = match self.block {
            Some(ref mut b) => {
                b.render(area, buf);
                b.inner(area)
            }
            None => area,
        };

        let layout = self.layout(chart_area);
        let graph_area = layout.graph_area;
        if graph_area.width < 1 || graph_area.height < 1 {
            return;
        }

        buf.set_background(chart_area, self.style.bg);

        if let Some((x, y)) = layout.title_x {
            let title = self.x_axis.title.unwrap();
            buf.set_string(x, y, title, self.x_axis.title_style);
        }

        if let Some((x, y)) = layout.title_y {
            let title = self.y_axis.title.unwrap();
            buf.set_string(x, y, title, self.y_axis.title_style);
        }

        if let Some(y) = layout.label_x {
            let labels = self.x_axis.labels.unwrap();
            let total_width = labels.iter().fold(0, |acc, l| l.as_ref().width() + acc) as u16;
            let labels_len = labels.len() as u16;
            if total_width < graph_area.width && labels_len > 1 {
                for (i, label) in labels.iter().enumerate() {
                    buf.set_string(
                        graph_area.left() + i as u16 * (graph_area.width - 1) / (labels_len - 1)
                            - label.as_ref().width() as u16,
                        y,
                        label.as_ref(),
                        self.x_axis.labels_style,
                    );
                }
            }
        }

        if let Some(x) = layout.label_y {
            let labels = self.y_axis.labels.unwrap();
            let labels_len = labels.len() as u16;
            for (i, label) in labels.iter().enumerate() {
                let dy = i as u16 * (graph_area.height - 1) / (labels_len - 1);
                if dy < graph_area.bottom() {
                    buf.set_string(
                        x,
                        graph_area.bottom() - 1 - dy,
                        label.as_ref(),
                        self.y_axis.labels_style,
                    );
                }
            }
        }

        if let Some(y) = layout.axis_x {
            for x in graph_area.left()..graph_area.right() {
                buf.get_mut(x, y)
                    .set_symbol(symbols::line::HORIZONTAL)
                    .set_style(self.x_axis.style);
            }
        }

        if let Some(x) = layout.axis_y {
            for y in graph_area.top()..graph_area.bottom() {
                buf.get_mut(x, y)
                    .set_symbol(symbols::line::VERTICAL)
                    .set_style(self.y_axis.style);
            }
        }

        if let Some(y) = layout.axis_x {
            if let Some(x) = layout.axis_y {
                buf.get_mut(x, y)
                    .set_symbol(symbols::line::BOTTOM_LEFT)
                    .set_style(self.x_axis.style);
            }
        }

        for dataset in self.datasets {
            match dataset.marker {
                Marker::Dot => {
                    for &(x, y) in dataset.data.iter().filter(|&&(x, y)| {
                        !(x < self.x_axis.bounds[0]
                            || x > self.x_axis.bounds[1]
                            || y < self.y_axis.bounds[0]
                            || y > self.y_axis.bounds[1])
                    }) {
                        let dy = ((self.y_axis.bounds[1] - y) * f64::from(graph_area.height - 1)
                            / (self.y_axis.bounds[1] - self.y_axis.bounds[0]))
                            as u16;
                        let dx = ((x - self.x_axis.bounds[0]) * f64::from(graph_area.width - 1)
                            / (self.x_axis.bounds[1] - self.x_axis.bounds[0]))
                            as u16;

                        buf.get_mut(graph_area.left() + dx, graph_area.top() + dy)
                            .set_symbol(symbols::DOT)
                            .set_fg(dataset.style.fg)
                            .set_bg(dataset.style.bg);
                    }
                }
                Marker::Braille => {
                    Canvas::default()
                        .background_color(self.style.bg)
                        .x_bounds(self.x_axis.bounds)
                        .y_bounds(self.y_axis.bounds)
                        .paint(|ctx| {
                            ctx.draw(&Points {
                                coords: dataset.data,
                                color: dataset.style.fg,
                            });
                            if let GraphType::Line = dataset.graph_type {
                                for i in 0..dataset.data.len() - 1 {
                                    ctx.draw(&Line {
                                        x1: dataset.data[i].0,
                                        y1: dataset.data[i].1,
                                        x2: dataset.data[i + 1].0,
                                        y2: dataset.data[i + 1].1,
                                        color: dataset.style.fg,
                                    })
                                }
                            }
                        })
                        .render(graph_area, buf);
                }
            }
        }

        if let Some(legend_area) = layout.legend_area {
            Block::default()
                .borders(Borders::ALL)
                .render(legend_area, buf);
            for (i, dataset) in self.datasets.iter().enumerate() {
                buf.set_string(
                    legend_area.x + 1,
                    legend_area.y + 1 + i as u16,
                    &dataset.name,
                    dataset.style,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct LegendTestCase {
        chart_area: Rect,
        hidden_legend_constraints: (Constraint, Constraint),
        legend_area: Option<Rect>,
    }

    #[test]
    fn it_should_hide_the_legend() {
        let data = [(0.0, 5.0), (1.0, 6.0), (3.0, 7.0)];
        let datasets = (0..10)
            .map(|i| {
                let name = format!("Dataset #{}", i);
                Dataset::default().name(name).data(&data)
            })
            .collect::<Vec<_>>();
        let cases = [
            LegendTestCase {
                chart_area: Rect::new(0, 0, 100, 100),
                hidden_legend_constraints: (Constraint::Ratio(1, 4), Constraint::Ratio(1, 4)),
                legend_area: Some(Rect::new(88, 0, 12, 12)),
            },
            LegendTestCase {
                chart_area: Rect::new(0, 0, 100, 100),
                hidden_legend_constraints: (Constraint::Ratio(1, 10), Constraint::Ratio(1, 4)),
                legend_area: None,
            },
        ];
        for case in &cases {
            let chart: Chart<String, String> = Chart::default()
                .x_axis(Axis::default().title("X axis"))
                .y_axis(Axis::default().title("Y axis"))
                .hidden_legend_constraints(case.hidden_legend_constraints)
                .datasets(datasets.as_slice());
            let layout = chart.layout(case.chart_area);
            assert_eq!(layout.legend_area, case.legend_area);
        }
    }
}

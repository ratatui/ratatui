pub mod main {
    pub mod barchart;
    pub mod block;
    pub mod buffer;
    pub mod chart;
    pub mod canvas;
    pub mod constraints;
    pub mod from_string;
    pub mod gauge;
    pub mod line;
    pub mod list;
    pub mod paragraph;
    pub mod rect;
    pub mod sparkline;
    pub mod stored_text;
    pub mod table;
    pub mod text;
}
pub use main::*;

criterion::criterion_main!(
    barchart::benches,
    block::benches,
    canvas::benches,
    from_string::benches,
    buffer::benches,
    chart::benches,
    line::benches,
    list::benches,
    paragraph::benches,
    rect::benches,
    sparkline::benches,
    stored_text::benches,
    table::benches,
    text::benches,
    constraints::benches,
    gauge::benches,
);

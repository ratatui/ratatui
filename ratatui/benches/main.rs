pub mod main {
    pub mod barchart;
    pub mod block;
    pub mod buffer;
    pub mod line;
    pub mod list;
    pub mod paragraph;
    pub mod rect;
    pub mod sparkline;
    pub mod table;
}
pub use main::*;

criterion::criterion_main!(
    barchart::benches,
    block::benches,
    buffer::benches,
    line::benches,
    list::benches,
    paragraph::benches,
    rect::benches,
    sparkline::benches,
    table::benches,
);

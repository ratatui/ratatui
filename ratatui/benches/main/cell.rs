use std::cmp;
use std::hint::black_box;

use compact_str::CompactString;
use criterion::{BatchSize, Criterion};
use ratatui::style::{Color, Modifier};
use unicode_width::UnicodeWidthStr;

criterion::criterion_group!(
    benches,
    bench_buffer_create,
    bench_buffer_fill_ascii,
    bench_buffer_fill_cjk,
    bench_buffer_fill_box_drawing,
    bench_buffer_read_symbols,
    bench_buffer_clone,
    bench_buffer_diff,
);

// 200 cols x 50 rows = 10,000 cells (typical terminal size)
const BUFFER_SIZE: usize = 200 * 50;

// ---------------------------------------------------------------------------
// EmbeddedStr: 4-byte inline string (the current Cell approach)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
struct EmbeddedStr {
    bytes: [u8; 3],
    len: u8,
}

impl EmbeddedStr {
    fn as_str(&self) -> &str {
        #[allow(unsafe_code)]
        unsafe {
            core::str::from_utf8_unchecked(&self.bytes[..self.len as usize])
        }
    }
}

impl Default for EmbeddedStr {
    fn default() -> Self {
        Self {
            bytes: [b' ', 0, 0],
            len: 1,
        }
    }
}

impl From<char> for EmbeddedStr {
    fn from(c: char) -> Self {
        let c = c as u32;
        if c < 0x7F {
            return Self {
                bytes: [c as u8, 0, 0],
                len: 1,
            };
        }
        let mut bytes = [0u8; 3];
        let len = if c < 0x800 {
            bytes[0] = 0xC0 | ((c >> 6) as u8);
            bytes[1] = 0x80 | ((c & 0x3F) as u8);
            2
        } else if c < 0x10000 {
            bytes[0] = 0xE0 | ((c >> 12) as u8);
            bytes[1] = 0x80 | (((c >> 6) & 0x3F) as u8);
            bytes[2] = 0x80 | ((c & 0x3F) as u8);
            3
        } else {
            bytes[0] = b' ';
            1
        };
        Self { bytes, len }
    }
}

impl From<&str> for EmbeddedStr {
    fn from(s: &str) -> Self {
        let bytes = s.as_bytes();
        if bytes.len() <= 3 {
            let mut result_bytes = [0u8; 3];
            result_bytes[..bytes.len()].copy_from_slice(bytes);
            Self {
                bytes: result_bytes,
                len: bytes.len() as u8,
            }
        } else {
            Self {
                bytes: [b' ', 0, 0],
                len: 1,
            }
        }
    }
}

// ---------------------------------------------------------------------------
// EmbeddedStr2: 4-byte inline string (derives length from UTF-8 leading byte)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
struct EmbeddedStr2 {
    bytes: [u8; 4],
}

impl EmbeddedStr2 {
    fn len(&self) -> usize {
        const LEN: [u8; 16] = [1,1,1,1,1,1,1,1, 1,1,1,1, 2,2, 3, 4];
        LEN[(self.bytes[0] >> 4) as usize] as usize
    }

    fn as_str(&self) -> &str {
        #[allow(unsafe_code)]
        unsafe {
            core::str::from_utf8_unchecked(&self.bytes[..self.len()])
        }
    }
}

impl Default for EmbeddedStr2 {
    fn default() -> Self {
        Self {
            bytes: [b' ', 0, 0, 0],
        }
    }
}

impl From<char> for EmbeddedStr2 {
    fn from(c: char) -> Self {
        let c = c as u32;
        if c < 0x7F {
            return Self {
                bytes: [c as u8, 0, 0, 0],
            };
        }
        let mut bytes = [0u8; 4];
        if c < 0x800 {
            bytes[0] = 0xC0 | ((c >> 6) as u8);
            bytes[1] = 0x80 | ((c & 0x3F) as u8);
        } else if c < 0x10000 {
            bytes[0] = 0xE0 | ((c >> 12) as u8);
            bytes[1] = 0x80 | (((c >> 6) & 0x3F) as u8);
            bytes[2] = 0x80 | ((c & 0x3F) as u8);
        } else {
            bytes[0] = 0xF0 | ((c >> 18) as u8);
            bytes[1] = 0x80 | (((c >> 12) & 0x3F) as u8);
            bytes[2] = 0x80 | (((c >> 6) & 0x3F) as u8);
            bytes[3] = 0x80 | ((c & 0x3F) as u8);
        }
        Self { bytes }
    }
}

impl From<&str> for EmbeddedStr2 {
    fn from(s: &str) -> Self {
        let bytes = s.as_bytes();
        if bytes.len() <= 4 {
            let mut result_bytes = [0u8; 4];
            result_bytes[..bytes.len()].copy_from_slice(bytes);
            Self {
                bytes: result_bytes,
            }
        } else {
            Self {
                bytes: [b' ', 0, 0, 0],
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Cell variants
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
struct EmbeddedCell {
    symbol: EmbeddedStr,
    fg: Color,
    bg: Color,
    modifier: Modifier,
    skip: bool,
}

impl Default for EmbeddedCell {
    fn default() -> Self {
        Self {
            symbol: EmbeddedStr::default(),
            fg: Color::Reset,
            bg: Color::Reset,
            modifier: Modifier::empty(),
            skip: false,
        }
    }
}

impl PartialEq for EmbeddedCell {
    fn eq(&self, other: &Self) -> bool {
        self.symbol.as_str() == other.symbol.as_str()
            && self.fg == other.fg
            && self.bg == other.bg
            && self.modifier == other.modifier
            && self.skip == other.skip
    }
}

impl EmbeddedCell {
    fn symbol(&self) -> &str {
        self.symbol.as_str()
    }

    fn set_symbol(&mut self, s: &str) {
        self.symbol = s.into();
    }

    fn set_char(&mut self, ch: char) {
        self.symbol = ch.into();
    }
}

#[derive(Debug, Clone, Copy)]
struct Embedded2Cell {
    symbol: EmbeddedStr2,
    fg: Color,
    bg: Color,
    modifier: Modifier,
    skip: bool,
}

impl Default for Embedded2Cell {
    fn default() -> Self {
        Self {
            symbol: EmbeddedStr2::default(),
            fg: Color::Reset,
            bg: Color::Reset,
            modifier: Modifier::empty(),
            skip: false,
        }
    }
}

impl PartialEq for Embedded2Cell {
    fn eq(&self, other: &Self) -> bool {
        // compare raw bytes — zero-padding guarantees correctness without len()
        self.symbol.bytes == other.symbol.bytes
            && self.fg == other.fg
            && self.bg == other.bg
            && self.modifier == other.modifier
            && self.skip == other.skip
    }
}

impl Embedded2Cell {
    fn symbol(&self) -> &str {
        self.symbol.as_str()
    }

    fn set_symbol(&mut self, s: &str) {
        self.symbol = s.into();
    }

    fn set_char(&mut self, ch: char) {
        self.symbol = ch.into();
    }
}

#[derive(Debug, Clone)]
struct CompactCell {
    symbol: CompactString,
    fg: Color,
    bg: Color,
    modifier: Modifier,
    skip: bool,
}

impl Default for CompactCell {
    fn default() -> Self {
        Self {
            symbol: CompactString::const_new(" "),
            fg: Color::Reset,
            bg: Color::Reset,
            modifier: Modifier::empty(),
            skip: false,
        }
    }
}

impl PartialEq for CompactCell {
    fn eq(&self, other: &Self) -> bool {
        self.symbol == other.symbol
            && self.fg == other.fg
            && self.bg == other.bg
            && self.modifier == other.modifier
            && self.skip == other.skip
    }
}

impl CompactCell {
    fn symbol(&self) -> &str {
        &self.symbol
    }

    fn set_symbol(&mut self, s: &str) {
        self.symbol = CompactString::from(s);
    }

    fn set_char(&mut self, ch: char) {
        let mut buf = [0u8; 4];
        let s: &str = ch.encode_utf8(&mut buf);
        self.symbol = CompactString::from(s);
    }
}

// ---------------------------------------------------------------------------
// Test data
// ---------------------------------------------------------------------------

const ASCII_CHARS: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', ' ', '.', ',', ':',
    ';', '-', '=', '+', '!', '?', '#', '@', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];

const CJK_CHARS: &[char] = &[
    '你', '好', '世', '界', '日', '本', '語', '中', '文', '字', '東', '京', '大', '学', '人',
    '山', '川', '田', '上', '下',
];

const BOX_DRAWING: &[&str] = &[
    "─", "│", "┌", "┐", "└", "┘", "├", "┤", "┬", "┴", "┼", "═", "║", "╔", "╗", "╚", "╝", "╠",
    "╣", "╬",
];

// ---------------------------------------------------------------------------
// Benchmarks
// ---------------------------------------------------------------------------

fn bench_buffer_create(c: &mut Criterion) {
    let mut group = c.benchmark_group("cell/buffer_create");

    group.bench_function("embedded", |b| {
        b.iter(|| {
            let buf: Vec<EmbeddedCell> = vec![EmbeddedCell::default(); black_box(BUFFER_SIZE)];
            black_box(buf);
        });
    });

    group.bench_function("embedded2", |b| {
        b.iter(|| {
            let buf: Vec<Embedded2Cell> = vec![Embedded2Cell::default(); black_box(BUFFER_SIZE)];
            black_box(buf);
        });
    });

    group.bench_function("compact_str", |b| {
        b.iter(|| {
            let buf: Vec<CompactCell> = vec![CompactCell::default(); black_box(BUFFER_SIZE)];
            black_box(buf);
        });
    });

    group.finish();
}

fn bench_buffer_fill_ascii(c: &mut Criterion) {
    let mut group = c.benchmark_group("cell/fill_ascii");

    group.bench_function("embedded", |b| {
        b.iter_batched(
            || vec![EmbeddedCell::default(); BUFFER_SIZE],
            |mut buf| {
                for (i, cell) in buf.iter_mut().enumerate() {
                    cell.set_char(ASCII_CHARS[i % ASCII_CHARS.len()]);
                }
                black_box(buf);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("embedded2", |b| {
        b.iter_batched(
            || vec![Embedded2Cell::default(); BUFFER_SIZE],
            |mut buf| {
                for (i, cell) in buf.iter_mut().enumerate() {
                    cell.set_char(ASCII_CHARS[i % ASCII_CHARS.len()]);
                }
                black_box(buf);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("compact_str", |b| {
        b.iter_batched(
            || vec![CompactCell::default(); BUFFER_SIZE],
            |mut buf| {
                for (i, cell) in buf.iter_mut().enumerate() {
                    cell.set_char(ASCII_CHARS[i % ASCII_CHARS.len()]);
                }
                black_box(buf);
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

fn bench_buffer_fill_cjk(c: &mut Criterion) {
    let mut group = c.benchmark_group("cell/fill_cjk");

    group.bench_function("embedded", |b| {
        b.iter_batched(
            || vec![EmbeddedCell::default(); BUFFER_SIZE],
            |mut buf| {
                for (i, cell) in buf.iter_mut().enumerate() {
                    cell.set_char(CJK_CHARS[i % CJK_CHARS.len()]);
                }
                black_box(buf);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("embedded2", |b| {
        b.iter_batched(
            || vec![Embedded2Cell::default(); BUFFER_SIZE],
            |mut buf| {
                for (i, cell) in buf.iter_mut().enumerate() {
                    cell.set_char(CJK_CHARS[i % CJK_CHARS.len()]);
                }
                black_box(buf);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("compact_str", |b| {
        b.iter_batched(
            || vec![CompactCell::default(); BUFFER_SIZE],
            |mut buf| {
                for (i, cell) in buf.iter_mut().enumerate() {
                    cell.set_char(CJK_CHARS[i % CJK_CHARS.len()]);
                }
                black_box(buf);
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

fn bench_buffer_fill_box_drawing(c: &mut Criterion) {
    let mut group = c.benchmark_group("cell/fill_box_drawing");

    group.bench_function("embedded", |b| {
        b.iter_batched(
            || vec![EmbeddedCell::default(); BUFFER_SIZE],
            |mut buf| {
                for (i, cell) in buf.iter_mut().enumerate() {
                    cell.set_symbol(BOX_DRAWING[i % BOX_DRAWING.len()]);
                }
                black_box(buf);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("embedded2", |b| {
        b.iter_batched(
            || vec![Embedded2Cell::default(); BUFFER_SIZE],
            |mut buf| {
                for (i, cell) in buf.iter_mut().enumerate() {
                    cell.set_symbol(BOX_DRAWING[i % BOX_DRAWING.len()]);
                }
                black_box(buf);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("compact_str", |b| {
        b.iter_batched(
            || vec![CompactCell::default(); BUFFER_SIZE],
            |mut buf| {
                for (i, cell) in buf.iter_mut().enumerate() {
                    cell.set_symbol(BOX_DRAWING[i % BOX_DRAWING.len()]);
                }
                black_box(buf);
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

fn bench_buffer_read_symbols(c: &mut Criterion) {
    let mut group = c.benchmark_group("cell/read_symbols");

    // pre-fill with mixed content: ~60% ASCII, ~20% CJK, ~20% box drawing
    let make_embedded_buf = || {
        let mut buf = vec![EmbeddedCell::default(); BUFFER_SIZE];
        for (i, cell) in buf.iter_mut().enumerate() {
            match i % 5 {
                0..3 => cell.set_char(ASCII_CHARS[i % ASCII_CHARS.len()]),
                3 => cell.set_char(CJK_CHARS[i % CJK_CHARS.len()]),
                _ => cell.set_symbol(BOX_DRAWING[i % BOX_DRAWING.len()]),
            }
        }
        buf
    };

    let make_embedded2_buf = || {
        let mut buf = vec![Embedded2Cell::default(); BUFFER_SIZE];
        for (i, cell) in buf.iter_mut().enumerate() {
            match i % 5 {
                0..3 => cell.set_char(ASCII_CHARS[i % ASCII_CHARS.len()]),
                3 => cell.set_char(CJK_CHARS[i % CJK_CHARS.len()]),
                _ => cell.set_symbol(BOX_DRAWING[i % BOX_DRAWING.len()]),
            }
        }
        buf
    };

    let make_compact_buf = || {
        let mut buf = vec![CompactCell::default(); BUFFER_SIZE];
        for (i, cell) in buf.iter_mut().enumerate() {
            match i % 5 {
                0..3 => cell.set_char(ASCII_CHARS[i % ASCII_CHARS.len()]),
                3 => cell.set_char(CJK_CHARS[i % CJK_CHARS.len()]),
                _ => cell.set_symbol(BOX_DRAWING[i % BOX_DRAWING.len()]),
            }
        }
        buf
    };

    group.bench_function("embedded", |b| {
        let buf = make_embedded_buf();
        b.iter(|| {
            let mut total = 0usize;
            for cell in &buf {
                total += cell.symbol().len();
            }
            black_box(total);
        });
    });

    group.bench_function("embedded2", |b| {
        let buf = make_embedded2_buf();
        b.iter(|| {
            let mut total = 0usize;
            for cell in &buf {
                total += cell.symbol().len();
            }
            black_box(total);
        });
    });

    group.bench_function("compact_str", |b| {
        let buf = make_compact_buf();
        b.iter(|| {
            let mut total = 0usize;
            for cell in &buf {
                total += cell.symbol().len();
            }
            black_box(total);
        });
    });

    group.finish();
}

fn bench_buffer_clone(c: &mut Criterion) {
    let mut group = c.benchmark_group("cell/buffer_clone");

    let make_embedded_buf = || {
        let mut buf = vec![EmbeddedCell::default(); BUFFER_SIZE];
        for (i, cell) in buf.iter_mut().enumerate() {
            match i % 5 {
                0..3 => cell.set_char(ASCII_CHARS[i % ASCII_CHARS.len()]),
                3 => cell.set_char(CJK_CHARS[i % CJK_CHARS.len()]),
                _ => cell.set_symbol(BOX_DRAWING[i % BOX_DRAWING.len()]),
            }
        }
        buf
    };

    let make_embedded2_buf = || {
        let mut buf = vec![Embedded2Cell::default(); BUFFER_SIZE];
        for (i, cell) in buf.iter_mut().enumerate() {
            match i % 5 {
                0..3 => cell.set_char(ASCII_CHARS[i % ASCII_CHARS.len()]),
                3 => cell.set_char(CJK_CHARS[i % CJK_CHARS.len()]),
                _ => cell.set_symbol(BOX_DRAWING[i % BOX_DRAWING.len()]),
            }
        }
        buf
    };

    let make_compact_buf = || {
        let mut buf = vec![CompactCell::default(); BUFFER_SIZE];
        for (i, cell) in buf.iter_mut().enumerate() {
            match i % 5 {
                0..3 => cell.set_char(ASCII_CHARS[i % ASCII_CHARS.len()]),
                3 => cell.set_char(CJK_CHARS[i % CJK_CHARS.len()]),
                _ => cell.set_symbol(BOX_DRAWING[i % BOX_DRAWING.len()]),
            }
        }
        buf
    };

    group.bench_function("embedded", |b| {
        let buf = make_embedded_buf();
        b.iter(|| {
            black_box(buf.clone());
        });
    });

    group.bench_function("embedded2", |b| {
        let buf = make_embedded2_buf();
        b.iter(|| {
            black_box(buf.clone());
        });
    });

    group.bench_function("compact_str", |b| {
        let buf = make_compact_buf();
        b.iter(|| {
            black_box(buf.clone());
        });
    });

    group.finish();
}

/// Mirrors `Buffer::diff` — full cell equality, skip handling,
/// and collecting `(u16, u16, cell_index)` update tuples.
/// No width tracking needed for embedded variants (always width 1).
fn diff_embedded(prev: &[EmbeddedCell], curr: &[EmbeddedCell], width: u16) -> Vec<(u16, u16, usize)> {
    let mut updates = Vec::new();

    for (i, (current, previous)) in curr.iter().zip(prev.iter()).enumerate() {
        if !current.skip && current != previous {
            let x = (i % width as usize) as u16;
            let y = (i / width as usize) as u16;
            updates.push((x, y, i));
        }
    }
    updates
}

fn diff_embedded2(prev: &[Embedded2Cell], curr: &[Embedded2Cell], width: u16) -> Vec<(u16, u16, usize)> {
    let mut updates = Vec::new();

    for (i, (current, previous)) in curr.iter().zip(prev.iter()).enumerate() {
        if !current.skip && current != previous {
            let x = (i % width as usize) as u16;
            let y = (i / width as usize) as u16;
            updates.push((x, y, i));
        }
    }
    updates
}

fn diff_compact(prev: &[CompactCell], curr: &[CompactCell], width: u16) -> Vec<(u16, u16, usize)> {
    let mut updates = Vec::new();
    let mut invalidated: usize = 0;
    let mut to_skip: usize = 0;

    for (i, (current, previous)) in curr.iter().zip(prev.iter()).enumerate() {
        if !current.skip && (current != previous || invalidated > 0) && to_skip == 0 {
            let x = (i % width as usize) as u16;
            let y = (i / width as usize) as u16;
            updates.push((x, y, i));
        }

        let current_width = current.symbol().width();
        to_skip = current_width.saturating_sub(1);

        let affected_width = cmp::max(current_width, previous.symbol().width());
        invalidated = cmp::max(affected_width, invalidated).saturating_sub(1);
    }
    updates
}

fn bench_buffer_diff(c: &mut Criterion) {
    let mut group = c.benchmark_group("cell/buffer_diff");

    const COLS: u16 = 200;

    // Simulate the frame-to-frame diff: compare previous buffer against current.
    // Mixed content with ~10% changed cells — typical of real TUI rendering.
    let make_embedded_bufs = || {
        let mut prev = vec![EmbeddedCell::default(); BUFFER_SIZE];
        for (i, cell) in prev.iter_mut().enumerate() {
            match i % 5 {
                0..3 => cell.set_char(ASCII_CHARS[i % ASCII_CHARS.len()]),
                3 => cell.set_char(CJK_CHARS[i % CJK_CHARS.len()]),
                _ => cell.set_symbol(BOX_DRAWING[i % BOX_DRAWING.len()]),
            }
        }
        let mut curr = prev.clone();
        for i in (0..BUFFER_SIZE).step_by(10) {
            curr[i].set_char(CJK_CHARS[i % CJK_CHARS.len()]);
        }
        (prev, curr)
    };

    let make_embedded2_bufs = || {
        let mut prev = vec![Embedded2Cell::default(); BUFFER_SIZE];
        for (i, cell) in prev.iter_mut().enumerate() {
            match i % 5 {
                0..3 => cell.set_char(ASCII_CHARS[i % ASCII_CHARS.len()]),
                3 => cell.set_char(CJK_CHARS[i % CJK_CHARS.len()]),
                _ => cell.set_symbol(BOX_DRAWING[i % BOX_DRAWING.len()]),
            }
        }
        let mut curr = prev.clone();
        for i in (0..BUFFER_SIZE).step_by(10) {
            curr[i].set_char(CJK_CHARS[i % CJK_CHARS.len()]);
        }
        (prev, curr)
    };

    let make_compact_bufs = || {
        let mut prev = vec![CompactCell::default(); BUFFER_SIZE];
        for (i, cell) in prev.iter_mut().enumerate() {
            match i % 5 {
                0..3 => cell.set_char(ASCII_CHARS[i % ASCII_CHARS.len()]),
                3 => cell.set_char(CJK_CHARS[i % CJK_CHARS.len()]),
                _ => cell.set_symbol(BOX_DRAWING[i % BOX_DRAWING.len()]),
            }
        }
        let mut curr = prev.clone();
        for i in (0..BUFFER_SIZE).step_by(10) {
            curr[i].set_char(CJK_CHARS[i % CJK_CHARS.len()]);
        }
        (prev, curr)
    };

    group.bench_function("embedded", |b| {
        let (prev, curr) = make_embedded_bufs();
        b.iter(|| black_box(diff_embedded(&prev, &curr, COLS)));
    });

    group.bench_function("embedded2", |b| {
        let (prev, curr) = make_embedded2_bufs();
        b.iter(|| black_box(diff_embedded2(&prev, &curr, COLS)));
    });

    group.bench_function("compact_str", |b| {
        let (prev, curr) = make_compact_bufs();
        b.iter(|| black_box(diff_compact(&prev, &curr, COLS)));
    });

    group.finish();
}
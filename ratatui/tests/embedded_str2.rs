use compact_str::CompactString;

/// EmbeddedStr2: 4-byte inline string that derives length from the UTF-8 leading byte.
/// Handles all single Unicode codepoints (1-4 byte UTF-8).
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
struct EmbeddedStr2 {
    bytes: [u8; 4],
}

impl EmbeddedStr2 {
    fn len(&self) -> usize {
        const LEN: [u8; 16] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 3, 4];
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
// Helpers
// ---------------------------------------------------------------------------

fn compact_from_char(ch: char) -> CompactString {
    let mut buf = [0u8; 4];
    let s: &str = ch.encode_utf8(&mut buf);
    CompactString::from(s)
}

fn assert_char_eq(ch: char) {
    let embedded = EmbeddedStr2::from(ch);
    let compact = compact_from_char(ch);
    assert_eq!(
        embedded.as_str(),
        compact.as_str(),
        "mismatch for char U+{:04X} '{}'",
        ch as u32,
        ch,
    );
}

fn assert_str_eq(s: &str) {
    let embedded = EmbeddedStr2::from(s);
    if s.len() <= 4 {
        let compact = CompactString::from(s);
        assert_eq!(
            embedded.as_str(),
            compact.as_str(),
            "mismatch for str {:?}",
            s
        );
    } else {
        assert_eq!(
            embedded.as_str(),
            " ",
            "expected fallback to space for {:?}",
            s
        );
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

const ASCII_CHARS: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'a', 'b', 'c',
    'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', ' ', '.', ',', ':', ';', '-',
    '=', '+', '!', '?', '#', '@', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];

const CJK_CHARS: &[char] = &[
    '你', '好', '世', '界', '日', '本', '語', '中', '文', '字', '東', '京', '大', '学', '人', '山',
    '川', '田', '上', '下',
];

const BOX_DRAWING: &[&str] = &[
    "─", "│", "┌", "┐", "└", "┘", "├", "┤", "┬", "┴", "┼", "═", "║", "╔", "╗", "╚", "╝", "╠", "╣",
    "╬",
];

#[test]
fn ascii_chars() {
    for &ch in ASCII_CHARS {
        assert_char_eq(ch);
    }
    for ch in ['\0', '\x01', '\x7E'] {
        assert_char_eq(ch);
    }
}

#[test]
fn cjk_chars() {
    for &ch in CJK_CHARS {
        assert_char_eq(ch);
    }
}

#[test]
fn box_drawing_strs() {
    for &s in BOX_DRAWING {
        assert_str_eq(s);
    }
}

#[test]
fn four_byte_chars() {
    let chars_4byte = ['😀', '🎉', '🦀', '𐍈', '🏠', '💯'];
    for ch in chars_4byte {
        assert_char_eq(ch);
    }
}

#[test]
fn two_byte_chars() {
    let chars_2byte = ['é', 'ñ', 'ü', 'ß', 'ø', 'λ', 'π'];
    for ch in chars_2byte {
        assert_char_eq(ch);
    }
}

#[test]
fn from_str_roundtrip() {
    // 1-byte
    assert_str_eq("A");
    assert_str_eq(" ");
    // 2-byte
    assert_str_eq("é");
    assert_str_eq("λ");
    // 3-byte
    assert_str_eq("你");
    assert_str_eq("┼");
    // 4-byte
    assert_str_eq("😀");
    assert_str_eq("🦀");
    // >4 byte strings fall back to space
    assert_str_eq("hello");
    assert_str_eq("👨‍👩‍👧‍👦");
}

#[test]
fn default_is_space() {
    let e = EmbeddedStr2::default();
    assert_eq!(e.as_str(), " ");
}

#[test]
fn len_matches_str_len() {
    let test_chars = ['A', 'é', '你', '😀'];
    for ch in test_chars {
        let e = EmbeddedStr2::from(ch);
        assert_eq!(
            e.len(),
            e.as_str().len(),
            "len() mismatch for U+{:04X}",
            ch as u32,
        );
    }
}

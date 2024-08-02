use std::fmt;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Margin {
    pub horizontal: u16,
    pub vertical: u16,
}

impl Margin {
    pub const fn new(horizontal: u16, vertical: u16) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }
}

impl fmt::Display for Margin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.horizontal, self.vertical)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn margin_to_string() {
        assert_eq!(Margin::new(1, 2).to_string(), "1x2");
    }

    #[test]
    fn margin_new() {
        assert_eq!(
            Margin::new(1, 2),
            Margin {
                horizontal: 1,
                vertical: 2
            }
        );
    }
}

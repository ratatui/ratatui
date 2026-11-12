//! Polyfills providing pure rust fallback implementations of various `f64` methods for `no_std`
//! compatibility. These implementations may not be as accurate as their built-in counterparts.
//! This must be taken into account when using floating point math in this crate.
//!
//! Implementations based on [`micromath`](https://github.com/tarcieri/micromath) crate.
//!
//! Related Rust tracking issues:
//!
//! - <https://github.com/rust-lang/rust/issues/50145>
//! - <https://github.com/rust-lang/rust/issues/137578>

#[inline]
fn round(val: f64) -> f64 {
    (val + 0.5f64.copysign(val)) as i64 as f64
}

#[inline]
fn floor(val: f64) -> f64 {
    let mut res = (val as i64) as f64;
    if val < res {
        res -= 1.0;
    }
    res
}

pub(crate) trait F64Polyfills {
    /// Returns the nearest integer to `self`. If a value is half-way between two integers, round
    /// away from `0.0`.
    fn round(self) -> f64;

    /// Returns the largest integer less than or equal to `self`.
    fn floor(self) -> f64;
}

impl F64Polyfills for f64 {
    #[inline]
    fn round(self) -> f64 {
        round(self)
    }
    #[inline]
    fn floor(self) -> f64 {
        floor(self)
    }
}

#[cfg(test)]
mod tests {
    use core::f64::consts::{FRAC_PI_2, PI, TAU};

    use super::*;
    // explicitly use std to prevent testing against itself
    extern crate std;

    const TEST_VALUES: [f64; 24] = [
        0.0, 0.5, -0.5, 1.0, -1.0, PI, -PI, FRAC_PI_2, -FRAC_PI_2, TAU, -TAU, 9.4248, -9.4248,
        0.2528, -7.7047, -1.1596, -2.6095, 6.8435, 3.5392, 5.9725, 0.9172, 9.3539, 2.8843, -1.8483,
    ];

    const MAX_ERROR: f64 = 0.000_000_000_000_01;

    fn assert_with_error(computed: f64, expected: f64, max_error: f64) {
        let delta = (computed - expected).abs();
        assert!(
            delta <= max_error,
            "error exceeded max value of {max_error}: {computed} vs {expected}"
        );
    }

    #[test]
    fn f64_round() {
        for value in TEST_VALUES {
            let expected = value.round();
            let computed = round(value);
            assert_with_error(computed, expected, MAX_ERROR);
        }
    }

    #[test]
    fn f64_floor() {
        for value in TEST_VALUES {
            let expected = value.floor();
            let computed = floor(value);
            assert_with_error(computed, expected, MAX_ERROR);
        }
    }
}

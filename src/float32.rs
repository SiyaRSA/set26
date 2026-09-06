// set26/src/float32.rs

//! # F32
//!
//! A fixed-point decimal representation using an underlying integer value and a scale factor.
//!
//! `F32` represents a decimal number as a scaled integer: $\text{Value} \times 10^{-\text{Scale}}$.
//! It is useful for precise decimal arithmetic or scenarios where standard floating-point
//! rounding errors should be avoided.
//!
//! ## Overview
//!
//! * **Value (`i32`)**: The raw, unscaled integer component.
//! * **Scale (`usize`)**: The number of decimal places (powers of 10) to shift the value.
//!
//! ## Example
//!
//! ```ignore
//! use set26::f32;
//!
//! let num = f32!(3.125);
//! // Internally represented as F32(3125, 3)
//! ```

type Value = i32;
type Scale = usize;

/// A fixed-point number represented by a scaled 32-bit integer.
///
/// `F32` encapsulates a raw integer `Value` and a decimal `Scale`, providing
/// accurate parsing and representation for fixed-point math.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct F32(Value, Scale);

/// Parses a string slice into a fixed-point [`F32`] representation.
///
/// This function scans the input string byte-by-byte, handles optional negative
/// signs, detects decimal points, and calculates the appropriate integer value
/// and scale factor.
///
/// # Examples
///
/// ```ignore
/// let f = parse_fixed("3.125");
/// assert_eq!(f, F32(3125, 3));
/// ```
#[allow(unused)]
pub(crate) const fn parse_fixedf32(s: &str) -> F32 {
    let bytes = s.as_bytes();

    let mut i = 0;
    let mut value: i32 = 0;
    let mut scale: usize = 0;
    let mut negative = false;

    if bytes.len() > 0 && bytes[0] == b'-' {
        negative = true;
        i += 1;
    }

    let mut has_decimal = false;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'.' {
            has_decimal = true;
        } else if b >= b'0' && b <= b'9' {
            value = value * 10 + (b - b'0') as i32;
            if has_decimal {
                scale += 1;
            }
        }
        i += 1;
    }

    if negative {
        value = -value;
    }

    F32(value, scale)
}
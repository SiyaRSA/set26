// set26/src/float64.rs

//! # F64
//!
//! A fixed-point decimal representation using a 64-bit integer value and a scale factor.
//!
//! `F64` represents a decimal number as a scaled integer: $\text{Value} \times 10^{-\text{Scale}}$.
//! It is useful for high-precision decimal arithmetic or scenarios where standard floating-point
//! rounding errors should be avoided over a larger numeric range.
//!
//! ## Overview
//!
//! * **Value (`i64`)**: The raw, unscaled integer component.
//! * **Scale (`usize`)**: The number of decimal places (powers of 10) to shift the value.
//!
//! ## Example
//!
//! ```ignore
//! use set26::f64;
//!
//! let num = f64!(3.125);
//! // Internally represented as F64(3125, 3)
//! ```

type Value = i64;
type Scale = usize;

/// A fixed-point number represented by a scaled 64-bit integer.
///
/// `F64` encapsulates a raw integer `Value` and a decimal `Scale`, providing
/// accurate parsing and representation for high-precision fixed-point math.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct F64(Value, Scale);

/// Parses a string slice into a fixed-point [`F64`] representation.
///
/// This function scans the input string byte-by-byte, handles optional negative
/// signs, detects decimal points, and calculates the appropriate integer value
/// and scale factor.
///
/// # Examples
///
/// ```ignore
/// let f = parse_fixed("3.125");
/// assert_eq!(f, F64(3125, 3));
/// ```
#[allow(unused)]
pub(crate) const fn parse_fixedf64(s: &str) -> F64 {
    let bytes = s.as_bytes();

    let mut i = 0;
    let mut value: i64 = 0;
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
            value = value * 10 + (b - b'0') as i64;
            if has_decimal {
                scale += 1;
            }
        }
        i += 1;
    }

    if negative {
        value = -value;
    }

    F64(value, scale)
}
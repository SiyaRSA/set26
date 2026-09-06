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

impl F64 {
    pub const fn new(value: Value, scale: Scale) -> Self {
        Self(value, scale)
    }
}
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

impl F32 {
    pub const fn new(value: Value, scale: Scale) -> Self {
        Self(value, scale)
    }
}
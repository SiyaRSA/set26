// src/macros.rs

/// Creates a [`Seq`](crate::sequence::Seq) containing the given elements.
///
/// This macro provides a convenient way to initialize a `Seq` using syntax
/// similar to standard array expressions or the `vec!` macro. The size (`N`)
/// is inferred automatically from the number of provided elements.
///
/// # Examples
///
/// ```
/// use set26::seq;
///
/// let s = seq![1, 2, 3, 4];
/// assert_eq!(s.len(), 4);
/// assert_eq!(s[0], 1);
/// ```
#[macro_export]
macro_rules! seq {
    ($($element:expr),* $(,)?) => {
        $crate::Seq::new([$($element), *])
    };
}


/// A convenience macro to construct an [`F32`] from numeric literals or expressions.
///
/// This macro converts the expression to a string and parses it into the fixed-point structure.
///
/// # Examples
///
/// ```ignore
/// let val = f32!(3.125);
/// ```
#[macro_export]
macro_rules! f32 {
    ($val: expr) => {
        super::float32::parse_fixedf32(&$val.to_string())
    };
}


/// A convenience macro to construct an [`F64`] from numeric literals or expressions.
///
/// This macro converts the expression to a string and parses it into the fixed-point structure.
///
/// # Examples
///
/// ```ignore
/// let val = f64!(3.125);
/// ```
#[macro_export]
macro_rules! f64 {
    ($val: expr) => {
        $crate::float64::parse_fixedf64(&$val.to_string())
    };
}
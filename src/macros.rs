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
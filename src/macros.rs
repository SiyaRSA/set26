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
    ($val: expr) => {{
        let s = &$val.to_string();
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

        $crate::F32::new(value, scale)
    }};
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
    ($val: expr) => {{
        let s = &$val.to_string();
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

        $crate::F64::new(value, scale)
    }};
}
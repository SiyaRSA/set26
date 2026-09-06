// set26/src/lib.rs
//! Lightweight, strongly typed utilities for Rust.
//!
//! The primary type provided by this crate is [`Seq`], a fixed-size sequence
//! backed by an inline array.

mod sequence;
mod float64;
mod float32;
mod macros;


pub use sequence::Seq;
pub use float32::F32;
pub use float64::F64;


//==== Tests ====//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_sequence() {
        let seq = Seq::new([1, 2, 3]);

        assert_eq!(seq.len(), 3);
        assert_eq!(seq[0], 1);
        assert_eq!(seq[2], 3);
    }

    #[test]
    fn macro_creates_sequence() {
        let seq = seq![1, 2, 3, 4];

        assert_eq!(seq.len(), 4);
        assert_eq!(seq[1], 2);
    }

    #[test]
    fn sequence_is_copy() {
        let a = Seq::new([1u32, 2, 3]);
        let b = a;

        assert_eq!(a, b);
    }

    #[test]
    fn empty_sequence() {
        let seq: Seq<i32, 0> = Seq::new([]);

        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_parse_fixedf32() {
        println!("{:?}", f32!(3.125));
        println!("{:?}", f32!(3125.0));
    }

    #[test]
    fn test_parse_fixedf64() {
        println!("{:?}", f64!(6.125));
        println!("{:?}", f64!(6125.0));
    }
}
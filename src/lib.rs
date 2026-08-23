// src/lib.rs
//! Lightweight, strongly typed utilities for Rust.
//!
//! The primary type provided by this crate is [`Seq`], a fixed-size sequence
//! backed by an inline array.

mod sequence;
mod macros;

pub use sequence::Seq;


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
}
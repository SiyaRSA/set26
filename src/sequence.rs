// set26/src/sequence.rs

//! # Seq<T, N>
//!
//! A lightweight wrapper around a fixed-size, inline array.
//!
//! Unlike [`Vec<T>`], `Seq<T, N>` does not perform heap allocation and has a
//! length known at compile time.
//!
//! ## Comparison with `Vec<T>`
//!
//! | Feature | `Vec<T>` | `Seq<T, N>` |
//! | :--- | :--- | :--- |
//! | **Sizing** | Dynamically sized | Statically sized |
//! | **Storage** | Heap-backed | Inline |
//! | **Allocation** | Owns a heap allocation | Owns its elements directly |
//! | **Length** | Runtime | Compile time |
//! | **Copy Semantics** | Not `Copy` | `Copy` when `T: Copy` |
//!
//! `Seq<T, N>` is intended for small, fixed-size sequences where value
//! semantics and predictable storage are desirable.

/// A lightweight, fixed-size sequence backed by an inline array.
///
/// `Seq<T, N>` provides value semantics, stack or inline allocation, and a
/// compile-time known length `N`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Seq<T, const N: usize> {
    data: [T; N],
}

// API
impl<T, const N: usize> Seq<T, N> {
    /// Creates a new `Seq` from a fixed-size array.
    ///
    /// # Examples
    ///
    /// ```
    /// use set26::sequence::Seq;
    ///
    /// let seq = Seq::new([1, 2, 3, 4]);
    ///
    /// assert_eq!(seq.len(), 4);
    /// ```
    pub const fn new(data: [T; N]) -> Self {
        Self { data }
    }

    /// Returns the number of elements in the sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// use set26::sequence::Seq;
    ///
    /// let seq = Seq::new([10, 20, 30]);
    /// assert_eq!(seq.len(), 3);
    /// ```
    pub const fn len(&self) -> usize {
        N
    }

    /// Returns `true` if the sequence contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use set26::sequence::Seq;
    ///
    /// let seq: Seq<i32, 0> = Seq::new([]);
    /// assert!(seq.is_empty());
    /// ```
    pub const fn is_empty(&self) -> bool {
        N == 0
    }

    /// Returns a reference to the underlying fixed-size array.
    pub const fn as_array(&self) -> &[T; N] {
        &self.data
    }

    /// Returns a slice view of the sequence's contents.
    pub const fn as_slice(&self) -> &[T] {
        &self.data
    }

    /// Returns a mutable slice view of the sequence's contents.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }

    /// Returns a reference to the element at `index`.
    ///
    /// Returns `None` if the index is out of bounds.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    /// Returns a mutable reference to the element at `index`.
    ///
    /// Returns `None` if the index is out of bounds.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
    }

    /// Returns a reference to the first element.
    ///
    /// Returns `None` if the sequence is empty.
    pub fn first(&self) -> Option<&T> {
        self.data.first()
    }

    /// Returns a reference to the last element.
    ///
    /// Returns `None` if the sequence is empty.
    pub fn last(&self) -> Option<&T> {
        self.data.last()
    }

    /// Returns an iterator over the sequence.
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }

    /// Returns a mutable iterator over the sequence.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.data.iter_mut()
    }
}

// From<[T; N]>
impl<T, const N: usize> From<[T; N]> for Seq<T, N> {
    fn from(data: [T; N]) -> Self {
        Self::new(data)
    }
}

// AsRef<[T]>
impl<T, const N: usize> AsRef<[T]> for Seq<T, N> {
    fn as_ref(&self) -> &[T] {
        &self.data
    }
}

// AsMut<[T]>
impl<T, const N: usize> AsMut<[T]> for Seq<T, N> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}

// Index
impl<T, const N: usize> std::ops::Index<usize> for Seq<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

// IndexMut
impl<T, const N: usize> std::ops::IndexMut<usize> for Seq<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// IntoIterator
impl<T, const N: usize> IntoIterator for Seq<T, N> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

// &Seq -> iterator
impl<'a, T, const N: usize> IntoIterator for &'a Seq<T, N> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

// &mut Seq -> iterator
impl<'a, T, const N: usize> IntoIterator for &'a mut Seq<T, N> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}
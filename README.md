# set26

A small collection of lightweight, strongly typed utilities for Rust.

## Seq<T, N>

`Seq<T, N>` is a fixed-size sequence backed directly by an array.

It is designed for situations where the number of elements is known at compile time, but a dedicated sequence type with simple collection-style ergonomics is useful.

Unlike `Vec<T>`, `Seq` does not require a heap allocation, and its length is part of its type.

## Why Seq?

Rust already provides arrays and `Vec<T>`, and both are excellent choices. `Seq` focuses on the space between them.

An array provides fixed-size storage:

    let values = [1, 2, 3, 4];

A `Vec` provides dynamically sized, heap-backed storage:

    let values = vec![1, 2, 3, 4];

`Seq` provides a named sequence type with fixed-size, inline storage:

    let values = seq![1, 2, 3, 4];

This makes `Seq` useful for small pieces of data such as coordinates, color components, mathematical vectors, fixed lookup data, and other compile-time-sized sequences.

## Copy Semantics

One of the main motivations for `Seq` is value semantics.

When `T` is `Copy`, a `Seq<T, N>` can also be copied:

    let first = seq![1u32, 2, 3];
    let second = first;

    assert_eq!(first, second);

The elements are stored directly inside the `Seq`. There is no owned heap buffer that needs to be transferred or shared.

This makes `Seq` particularly suitable for small, fixed-size data where copying the entire value is inexpensive and desirable.

## Features

`Seq<T, N>` provides:

- Compile-time-known length
- Inline array storage
- `Copy` when `T: Copy`
- `Clone`, `Debug`, `PartialEq`, `Eq`, and `Hash`
- Indexing and mutable indexing
- Immutable and mutable slice access
- Iteration by value, reference, and mutable reference
- Construction from `[T; N]`
- The `seq![]` construction macro
- Support for zero-length sequences

## Seq vs Vec

| | `Seq<T, N>` | `Vec<T>` |
|---|---|---|
| Length | Compile time | Runtime |
| Storage | Inline | Heap-backed |
| Allocation | None | Dynamic |
| Growth | Fixed | Dynamic |
| Copy | When `T: Copy` | Not `Copy` |
| Best for | Small fixed data | Dynamic collections |

`Seq` is not intended to replace `Vec`. If a collection needs to grow or shrink at runtime, `Vec<T>` is the appropriate choice.

## Example

    use set26::seq;

    fn main() {
        let rgb = seq![255u8, 128, 64];

        println!("red: {}", rgb[0]);
        println!("green: {}", rgb[1]);
        println!("blue: {}", rgb[2]);
    }

## Design Philosophy

`set26` aims to provide small abstractions for situations where the standard library's general-purpose types do not quite express the intended semantics.

`Seq` follows a simple principle:

> If the size is known, the data can be represented directly.

It keeps the representation simple while providing a dedicated type for fixed-size sequences.

## Status

`set26` currently provides:
- **`Seq<T, N>`**: A fixed-size, inline array-backed sequence.
- **`F32`**: A fixed-point decimal type with `Eq`, `Ord`, and `Hash` support.

The project is still small, and its API may evolve as additional use cases are identified.

## License

Licensed under either:

- MIT License
# Mutable vs. Immutable References in Rust

This example demonstrates a common error in Rust related to mutable and immutable references.  The core issue is trying to modify a value through an immutable reference, which results in a compile-time error.

The `bug.rs` file contains code that highlights this error. The solution (`bugSolution.rs`) shows how to correct the error by ensuring all references are appropriately mutable when modification is intended.

This is a foundational concept for understanding Rust's ownership and borrowing system, a key feature for memory safety.
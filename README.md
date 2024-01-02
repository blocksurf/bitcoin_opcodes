## bitcoin_opcodes

`bitcoin_opcodes` provides OpCodes that can be used interchangably with numeric primitives

# Usage

Add this to your `Cargo.toml`:

```
[dependencies]
bitcoin_opcodes = "0.1.0"
```

# Features

This crate supports two features: `std` and `alloc`.

The `std` feature is enabled by default. If you specify `default-features = false` to disable `std`, be aware this removes Serde support for any standard library data structures that involve heap memory allocation.

The `alloc` feature enables `serde/alloc` for cases when an allocator is needed.
# 🥰 mycrush

mycrush is a pure Rust implementation of the CRUSH algorithm, converted from C with c2rust and then refactored for safety and WASM compatibility.

## Features

- ✅ **Pure Rust API** - No C ABI compatibility layer
- ✅ **WASM Compatible** - Builds for `wasm32-unknown-unknown` target
- ✅ **Safe Rust patterns** - Uses Result types and safe abstractions where possible
- ✅ **No symbol conflicts** - Independent from the C libcrush library
- ✅ **Modern error handling** - CrushError enum instead of integer error codes

## Building

```bash
# Regular build
cargo build --package mycrush

# WASM build
cargo build --package mycrush --target wasm32-unknown-unknown --release

# WASM-pack build (for npm packages)
cd mycrush && ./build-wasm.sh

# Run tests
cargo test --package mycrush
```

For detailed WASM usage and npm publishing instructions, see [WASM.md](WASM.md).

## Status

The library has been successfully converted to a pure Rust implementation:
- All `#[unsafe(no_mangle)]` and `extern "C"` declarations removed
- Safe Rust implementations for helper functions
- WASM compatibility verified
- All tests passing

For detailed conversion notes, see [PURE_RUST_CONVERSION.md](../PURE_RUST_CONVERSION.md).

## Note

This library still uses some internal libc functions (malloc/free/memcpy/memset) but is WASM-compatible through the libc crate's WASM support. Future work will replace these with pure Rust alternatives.

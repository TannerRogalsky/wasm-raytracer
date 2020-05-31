# Rust WASM Threading

Workers code can be dynamically defined from source using `Blob` and `createObjectURL`. Easily done from wasm.

The biggest issue is still file paths though.

## Build Flags
$env:RUSTFLAGS='-C target-feature=+atomics,+bulk-memory'
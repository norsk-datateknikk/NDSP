# NDSP
Norsk Datateknikk DSP library for Rust

The focus of the module is to build up a library of DSP code.
Not compute speed focused at this time.

The goal is to write a library that targets embedded platforms.

## Rust Installation:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Testing the library (for development purposes):
```
cargo test
```

## Building the library for release:
```
cargo build --release
```
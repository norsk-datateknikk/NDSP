# NDSP

Norsk Datateknikk No-STD DSP library for Rust.

The focus of the module is to build up a library of DSP code.
Not compute speed focused at this time. The first goal is to get basic features up and running to simplify writing.

The goal is to write a library that targets embedded platforms.

It utilizes the [mixed-num](https://crates.io/crates/mixed-num) library to allow flexibility in fixed and floating point sizes and precisions.

The [package](https://crates.io/crates/ndsp).

The [documentation](https://docs.rs/ndsp/).

Release notes are found under RELEASES.md.

## Testing the library (for development purposes)

```shell
cargo test --features "std"
```

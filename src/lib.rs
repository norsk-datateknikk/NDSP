//----------------------//
// Norsk Datateknikk AS //
//----------------------//

//! This is an experimental library for no-std DSP in Rust.

#![crate_name = "ndsp"]
#![no_std]

// Use std for test.
#[cfg(all(not(feature = "std"), test))]
extern crate std;

#[cfg(feature = "std")]
extern crate std;

extern crate mixed_num;
extern crate fixed_trigonometry;

pub mod traits;

#[macro_use]
pub mod vec;
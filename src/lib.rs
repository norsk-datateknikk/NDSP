//----------------------//
// Norsk Datateknikk AS //
//----------------------//

//! This is an experimental library for DSP in Rust, including for no-std environments.
//! 
//! It uses trait implementation from the `mixed_num` crate to enable both fixed point and floating point support. 
//! 
//! ### Example
//! 
//! ```
//! use ndsp::*;
//! 
//! let signalf32 = Vec::lin_range(0f32, 9f32, 10);
//! let signalf64 = Vec::lin_range(2f64, 11f64, 10);
//!
//! let result = signalf32.clone()*signalf64;
//! assert_eq!(result.to_string(), "[ 0, 3, 8, 15, 24, 35, 48, 63, 80, 99 ]" );
//! 
//! let signal1f32 = Vec::lin_range(2f64, 11f64, 10);
//!
//! let result = signalf32*signal1f32;
//! assert_eq!(result.to_string(), "[ 0, 3, 8, 15, 24, 35, 48, 63, 80, 99 ]" );
//! ```
//! 
//! This implementation simulataneously support complex numbers.
//! 
//! ### Example
//! ```
//! use ndsp::*;
//! use mixed_num::*;
//! 
//! let omega = <f32>::mixed_pi()/f32::mixed_from_num(8i32);
//! let theta = 0f32; 
//! 
//! let mut signal = Vec::osc(omega, theta, 4);
//! signal = &signal*&signal;
//! assert_eq!(signal.to_string(), "[ 1+0i, 0.7071067+0.7071068i, 0+0.99999994i, -0.7071067+0.7071068i ]" )
//! ```
//! 
//! ## Plotting
//! 
//! The package contains utility funcitons that simplify plotting of data.
//! 
//! ### Plot Example
//! 
//! ```
//! use ndsp::*;
//! let test_vec = Vec::lin_range(0f32, 1f32, 64);
//! test_vec.plot("./figures/plot_test.png", "Test Plot","x [idx]", "y" );
//! ``` 

#![crate_name = "ndsp"]
#![no_std]

// Use std for test.
#[cfg(all(not(feature = "std"), test))]
extern crate std;

#[cfg(feature = "std")]
extern crate std;

extern crate mixed_num;

pub mod traits;
pub use traits::*;

#[macro_use]
pub mod vec;
pub use vec::*;
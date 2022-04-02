//----------------------//
// Norsk Datateknikk AS //
//----------------------//

//! This is an experimental library for DSP in Rust, including for no-std environments.
//! 
//! ## `core::ops`
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
//! use plotters::prelude::*;
//! use ndsp::*;
//! 
//! fn plot( vec: &ndsp::Vec<f32> ) -> Result<(), Box<dyn std::error::Error>>
//! {
//!     let root = BitMapBackend::new("./figures/test_vector_plot.png", (1000, 500)).into_drawing_area();
//!     root.fill(&WHITE)?;
//!     let mut chart = ChartBuilder::on(&root)
//!         .caption("Test Plot", ("sans-serif", 25).into_font())
//!         .margin(10i32)
//!         .x_label_area_size(40i32)
//!         .y_label_area_size(50i32)
//!         .build_cartesian_2d( vec.indices().to_range(), vec.to_range() )?;
//! 
//!     chart
//!         .configure_mesh()
//!         .disable_x_mesh()
//!         .bold_line_style(&WHITE.mix(0.3))
//!         .y_desc("y")
//!         .x_desc("x [idx]")
//!         .axis_desc_style(("sans-serif", 15))
//!         .draw()?;
//! 
//!     chart.configure_mesh().draw()?;
//! 
//!     // Draws a single line
//!     chart
//!         .draw_series( LineSeries::new(
//!             vec.to_touples(),
//!             &BLUE) )?;
//! 
//!     Ok(())
//! }
//! 
//! let test_vec = Vec::lin_range(0f32, 1f32, 64);
//! plot(&test_vec).unwrap();
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
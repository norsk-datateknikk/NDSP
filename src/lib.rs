//----------------------//
// Norsk Datateknikk AS //
//----------------------//

//! This is an experimental library for DSP in Rust, including for no-std environments.
//! 
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
//!     let tuple_series = vec.to_touples();
//! 
//!     // Draws a sinle line
//!     chart
//!         .draw_series( LineSeries::new(
//!             tuple_series,
//!             &BLUE) )?;
//! 
//!     Ok(())
//! }
//! 
//! let test_vec = Vec::lin_range(0f32, 1f32, 50);
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
extern crate fixed_trigonometry;

pub mod traits;
pub use traits::*;

#[macro_use]
pub mod vec;
pub use vec::*;
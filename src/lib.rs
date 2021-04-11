//--------------------------------------------------------------//
// Copyright 2021 Norsk Datateknikk AS                          //
//--------------------------------------------------------------//
// This file is subject to the terms and conditions defined in  //
// file 'LICENSE', which is part of this source code package.   //
//--------------------------------------------------------------//

// use plotters::prelude::*;
use num_traits::Num;
use num_traits::Float;
use num_traits::real::Real;
//use num::complex::Complex;
use num_complex::Complex;

#[macro_export]
pub mod vector;
#[macro_export]
pub mod complex;


#[cfg(test)]
mod tests {
    #[test]
    fn linspace() {
        let test_vec = vector::linspace::<f32>(0_f32, 3_f32, 3);
        assert_eq!(test_vec, vec![ 0_f32, 1_f32, 2_f32 ]);

        /*
        println!( "{:?}", c_value!( 12_f32, 18_f32, f32) );

        let c_vec = vec![ C32F!(1,2), C32F!(3,6) ];
        println!( "{:?}", vec![ C32F!(1,2), C32F!(3,6), C32F!(4,2) ] );

        let c_vec = vec![ C32F!(1,2), C32F!(3,6), C32F!(4,2) ];
        println!( "{:?}", ndsp::c_abs( c_vec ) );
       */
    }
}
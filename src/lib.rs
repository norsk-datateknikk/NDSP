//--------------------------------------------------------------//
// Copyright 2021 Norsk Datateknikk AS                          //
//--------------------------------------------------------------//
// This file is subject to the terms and conditions defined in  //
// file 'LICENSE', which is part of this source code package.   //
//--------------------------------------------------------------//

#![crate_name = "ndsp"]

#[macro_use]
pub mod nd_complex;

#[macro_use]
pub mod nd_vector;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linspace() {
        let test_vec = nd_vector::linspace::<f32>(0_f32, 4_f32, 4);
        assert_eq!(test_vec, vec![ 0_f32, 1_f32, 2_f32, 3_f32 ]);
        /*
        println!( "{:?}", c_value!( 12_f32, 18_f32, f32) );

        let c_vec = vec![ C32F!(1,2), C32F!(3,6) ];
        println!( "{:?}", vec![ C32F!(1,2), C32F!(3,6), C32F!(4,2) ] );

        let c_vec = vec![ C32F!(1,2), C32F!(3,6), C32F!(4,2) ];
        println!( "{:?}", ndsp::c_abs( c_vec ) );
       */
    }

    #[test]
    fn test_c_abs() {
        let c_vec = vec![ C32F!(2,0), C32F!(0,4), C32F!(-2,0) ];
        assert_eq!( vec![ 2_f32, 4_f32, 2_f32 ], nd_vector::c_abs( c_vec ) );
    }
}
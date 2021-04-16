//--------------------------------------------------------------//
// Copyright 2021 Norsk Datateknikk AS                          //
//--------------------------------------------------------------//
// This file is subject to the terms and conditions defined in  //
// file 'LICENSE', which is part of this source code package.   //
//--------------------------------------------------------------//

// This file containns basic functions that can operate on scalars of numerical types.

use num_traits::Num;
use num_traits::Float;
use num_traits::real::Real;

/// Convert btween linear magnitude scale and decibel. 
#[macro_export]
macro_rules! mag2db {
    ( $item:expr ) => {
        <T>::from(20).unwrap()*$item.log10()
    };
}

/// Convert btween linear magnitude scale and decibel. 
pub fn mag2db<T>( scalar: T )-> T
    where T: Real
{
    return mag2db!(scalar);
}

// TODO db2mag
/*
#[macro_export]
macro_rules! db2mag {
    ( item:expr ) => {
        .powf()
    };
}
*/

/// Convert btween linear power scale and decibel. 
#[macro_export]
macro_rules! pow2db {
    ( $item:expr ) => {
        <T>::from(10).unwrap()*$item.log10()
    };
}

/// Convert btween linear power scale and decibel. 
pub fn pow2db<T>( scalar: T )-> T
    where T: Real
{
    return pow2db!(scalar);
}

// TODO db2mag
/*
#[macro_export]
macro_rules! db2pow {
    ( item:expr ) => {
        .powf()
    };
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sfunc_mag2db() {
        let scalar = 2_f32;
        assert_eq!( 6.0206003_f32, mag2db(scalar) );
    }

    #[test]
    fn sfunc_pow2db() {
        let scalar = 2_f32;
        assert_eq!( 3.0103002_f32, pow2db(scalar) );
    }
}
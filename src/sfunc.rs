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
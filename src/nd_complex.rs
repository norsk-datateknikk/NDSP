//--------------------------------------------------------------//
// Copyright 2021 Norsk Datateknikk AS                          //
//--------------------------------------------------------------//
// This file is subject to the terms and conditions defined in  //
// file 'LICENSE', which is part of this source code package.   //
//--------------------------------------------------------------//

use num_traits::Num;
use num_complex::Complex;

/// Complex 32-bit float
pub type C32F = Complex::<f32>;
/// Complex 64-bit float
pub type C64F = Complex::<f64>;
/// Complex 64-bit float
pub type C32I = Complex::<i32>;
/// Complex 64-bit float
pub type C64I = Complex::<i64>;

/// Create complex number of specified size and type
#[macro_export]
macro_rules! c_value {
    ( $re:expr, $im:expr, $N:ty ) => { 
        num_complex::Complex::new( $re as $N, $im as $N );
    };
}

/// Create 32-bit float complex number.
#[macro_export]
macro_rules! C32F {
    ( $re:expr, $im:expr ) => {
        c_value!( $re, $im, f32 );
    };
}

/// Create 64-bit float complex number.
#[macro_export]
macro_rules! C64F {
    ( $re:expr, $im:expr ) => {
        c_value!( $re, $im, f64 );
    };
}

/// Create 32-bit float complex number.
#[macro_export]
macro_rules! C32I {
    ( $re:expr, $im:expr ) => {
        c_value!( $re, $im, i32 );
    };
}

/// Create 64-bit float complex number.
#[macro_export]
macro_rules! C64I {
    ( $re:expr, $im:expr ) => {
        c_value!( $re, $im, i64 );
    };
}

/// Returns the complex valu re+i*im.
pub fn c_value<N>( re:N, im:N)-> Complex<N>
    where N: Num
{
    return Complex::new( re as N, im as N );
}

/// Trait for float
macro_rules! numb {
    ( $numb:expr => $F:ty  ) => { F::from($numb).unwrap(); };
}

/// Absolute of a complex scalar.
#[macro_export]
macro_rules! abs_c_scalar {
    ( $c_val:expr => $F:ty ) => { 
        F::sqrt( $c_val.re.powf( numb!(2 => F) )+$c_val.im.powf( numb!(2 => F) ) ) ;
     };
}
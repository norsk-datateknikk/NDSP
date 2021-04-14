//--------------------------------------------------------------//
// Copyright 2021 Norsk Datateknikk AS                          //
//--------------------------------------------------------------//
// This file is subject to the terms and conditions defined in  //
// file 'LICENSE', which is part of this source code package.   //
//--------------------------------------------------------------//

// This file containns basic functions that can operate on vectors of numerical types.

use rustfft::FftPlanner;

use crate::sfunc::*;

use num_traits::Num;
use num_traits::Float;
use num_traits::real::Real;
//use num::complex::Complex;
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
    ( $numb:expr ) => { T::from($numb).unwrap(); };
}

/// Absolute of a complex scalar.
#[macro_export]
macro_rules! abs_c_scalar {
    ( $c_val:expr ) => { 
        T::sqrt( $c_val.re.powf( numb!(2) )+$c_val.im.powf( numb!(2) ) ) ;
     };
}

/// Returns the complex element-wise absolute value.
pub fn abs<T>( vector: Vec<Complex<T>> )-> Vec<T>
    where T: Float
{
    let mut r_vector: Vec<T> = Vec::with_capacity( vector.len() );
    for index in 0..vector.len() {
        r_vector.push( vector[index].norm() );
    }
    return r_vector;
}

/// Multiply vector with scalar.
pub fn scale<T>( vector: Vec<Complex<T>>, scalar: T )-> Vec<Complex<T>>
    where T: Real
{
    let mut r_vector:Vec<Complex<T>> = Vec::with_capacity( vector.len() );
    for i in 0..vector.len() {
        r_vector.push(vector[i].scale(scalar));
    }
    return r_vector;
}

/// Absolute of a complex scalar.
macro_rules! c_power_of_2{
    ( $c_val:expr) => { 
        $c_val.re.powf( numb!(2) )+$c_val.im.powf( numb!(2) );
     };
}

/// Power of a vector.
pub fn power<T>( vector: Vec<Complex<T>> )-> Vec<T>
    where T: Real
{
    let mut r_vector:Vec<T> = Vec::with_capacity( vector.len() );
        for i in 0..vector.len()  {
            r_vector.push(c_power_of_2!(vector[i]));
        }
        return r_vector;
}

/// Power of a vector in dBW (dB relative to one W).
pub fn power_dBW<T>( vector: Vec<Complex<T>> )-> Vec<T>
    where T: Real
{
    let mut r_vector:Vec<T> = Vec::with_capacity( vector.len() );
        for i in 0..vector.len()  {
            r_vector.push( super::sfunc::pow2db(c_power_of_2!(vector[i])) );
        }
        return r_vector;
}

/// Power of a vector  in dBW (dB relative to one mW).
pub fn power_dBm<T>( vector: Vec<Complex<T>> )-> Vec<T>
    where T: Real
{
    let mut r_vector:Vec<T> = Vec::with_capacity( vector.len() );
        for i in 0..vector.len()  {
            r_vector.push( super::sfunc::pow2db(c_power_of_2!(vector[i])/numb!(0.001)) );
        }
        return r_vector;
}

macro_rules! power_spectrum_calculation {
    ( $vector:expr, $T:ty ) => {
        let mut planner = FftPlanner::<$T>::new();
        let fft = planner.plan_fft_forward( $vector.len() );

        fft.process(&mut $vector);
        //let magnitude_spectrum = vector/vector.len();   //TODO element-wise division.

        return $vector;
    };
}

/// Calculate power spectrum for 32-bit complex floating point, linear scale.
pub fn power_spectrum( mut vector: Vec<Complex<f32>> )->  Vec<Complex<f32>>
{
    power_spectrum_calculation!( vector, f32 );
}

/// Calculate power spectrum for 64-bit complex floating point, linear scale.
pub fn power_spectrum64( mut vector: Vec<Complex<f64>> )->  Vec<Complex<f64>>
{
    power_spectrum_calculation!( vector, f64 );
}

/// This macro generates element-wise operations on vectors of complex numbers.
/// The operations must be a trait of the vector item class.
/// vector<T> can thus have all traits of T.
macro_rules! element_wise_operand {
    (   
        $(#[$comment:meta])*
        $operand:ident
        $trait:ident
    ) => {
        $(#[$comment])*
        /// Element-wise operation on vector of real type R.
        pub fn $operand<T>( vector: Vec<Complex<T>> )-> Vec<Complex<T>>
            where T: $trait
        {
            let mut r_vector: Vec<Complex<T>> = Vec::with_capacity( vector.len() );
            for index in 0..vector.len() {
                r_vector.push( vector[index].$operand() );
            }
            return r_vector;
        }
    };
}

element_wise_operand!{
    /// Complex sin.
    sin
    Float
}
element_wise_operand!{
    /// complex cos.
    cos
    Float
}
element_wise_operand!{
    /// Complex tan.
    tan
    Float
}
element_wise_operand!{
    /// Complex exponential, e^x.
    exp
    Float
}
element_wise_operand!{
    /// Invert, 1/x.
    inv
    Float
}
element_wise_operand!{
    /// Complex jonjugate.
    conj
    Float
}
/*
element_wise_operand!{
    /// THe norm or abolute value.
    norm
    Float
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cfunc_abs() {
        let vec = vec![ C32F!(2,0), C32F!(0,4), C32F!(-2,0) ];
        assert_eq!( vec![ 2_f32, 4_f32, 2_f32 ], abs( vec ) );
    }

    #[test]
    fn cfunc_power() {
        let vec = vec![ C32F!(2,0), C32F!(0,4), C32F!(-2,0) ];
        assert_eq!( vec![ 4_f32, 16_f32, 4_f32 ], power( vec ) );
    }

    #[test]
    fn cfunc_power_dBW() {
        let vec = vec![ C32F!(2,0), C32F!(0,4), C32F!(-2,0) ];
        assert_eq!( vec![ 6.0206003_f32, 12.041201_f32, 6.0206003_f32 ], power_dBW( vec ) );
    }

    #[test]
    fn cfunc_power_dBm() {
        let vec = vec![ C32F!(2,0), C32F!(0,4), C32F!(-2,0) ];
        assert_eq!( vec![ 36.0206_f32, 42.041201_f32, 36.0206003_f32 ], power_dBm( vec ) );
    }

}
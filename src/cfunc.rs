//--------------------------------------------------------------//
// Copyright 2021 Norsk Datateknikk AS                          //
//--------------------------------------------------------------//
// This file is subject to the terms and conditions defined in  //
// file 'LICENSE', which is part of this source code package.   //
//--------------------------------------------------------------//

// This file containns basic functions that can operate on vectors of numerical types.

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
    ( $numb:expr => $F:ty  ) => { F::from($numb).unwrap(); };
}

/// Absolute of a complex scalar.
#[macro_export]
macro_rules! abs_c_scalar {
    ( $c_val:expr => $F:ty ) => { 
        F::sqrt( $c_val.re.powf( numb!(2 => F) )+$c_val.im.powf( numb!(2 => F) ) ) ;
     };
}

/// Returns the complex element-wise absolute value.
pub fn abs<F>( vector: Vec<Complex<F>> )-> Vec<F>
    where F: Float
{
    println!( "{:?}", vector.len() );
    let mut r_vector: Vec<F> = Vec::with_capacity( vector.len() );
    for index in 0..vector.len() {
        r_vector.push( abs_c_scalar!( vector[index] => F ) );
    }
    return r_vector;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cfunc_abs() {
        let vec = vec![ C32F!(2,0), C32F!(0,4), C32F!(-2,0) ];
        assert_eq!( vec![ 2_f32, 4_f32, 2_f32 ], abs( vec ) );
    }

}
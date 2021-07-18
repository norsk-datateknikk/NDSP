//--------------------------------------------------------------//
// Copyright 2021 Norsk Datateknikk AS                          //
//--------------------------------------------------------------//
// This file is subject to the terms and conditions defined in  //
// file 'LICENSE', which is part of this source code package.   //
//--------------------------------------------------------------//

// This file containns basic functions that can operate on vectors of numerical types.

use rustfft::FftPlanner;

use crate::sfunc::*;
use crate::vfunc::*;

use num::traits::Num;
use num::traits::Float;
use num::traits::PrimInt;
use num::traits::real::Real;
use num::Complex;

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
        num::Complex::new( $re as $N, $im as $N );
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

/// Returns a 1D vector of zeros of size numb_samples.
/// [0+0j,0+0j,...,0+0j]
pub fn zeros<T>( numb_samples: usize ) -> Vec<Complex<T>>
    where T: Float
{
    let mut vector: Vec<Complex<T>> = Vec::with_capacity(numb_samples);
    for _i in 0..numb_samples {
        vector.push(c_value!(T::zero(), T::zero(), T));
    }
    return vector;
}

/// Returns a 1D vector of ones of size numb_samples.
/// [1+0i,1+0i,...,1+0i]
pub fn ones<T>( numb_samples: usize )-> Vec<Complex<T>>
where T: Float
{
    let mut vector: Vec<Complex<T>> = Vec::with_capacity(numb_samples);
    for _i in 0..numb_samples {
        vector.push( c_value!( T::one(), T::zero(), T) );
    }
    return vector;
}

/// Returns a 1D vector multiplied by -1.
pub fn neg<T>( vector: Vec<Complex<T>> )-> Vec<Complex<T>>
    where T: Float
{
    let mut r_vector: Vec<Complex<T>> = Vec::with_capacity( vector.len() );
    for item in vector  {
        r_vector.push( -item );
    }
    return r_vector;
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

/// Element-wise addition of two vectors of equal or unequal size.
/// Result has the length of the longes vector.
pub fn add<T>( vector1: Vec<Complex<T>>, vector2: Vec<Complex<T>> ) -> Vec<Complex<T>>
    where T: Float
{
    // Determine length of output
    if vector1.len() < vector2.len() {
        let min_len = vector1.len();
        let mut r_vector = vector2.clone();
        
        for i in 0..min_len {
            r_vector[i] = vector1[i]+vector2[i];
        }
        return r_vector;

    }
    else    {
        let min_len = vector2.len();
        let mut r_vector =  vector1.clone();
        
        for i in 0..min_len {
            r_vector[i] = vector1[i]+vector2[i];
        }
        return r_vector;
    }
}

/// Element-wise addition of two vectors of equal or unequal size.
/// Result has the length of the longes vector.
pub fn mul<T>( vector1: Vec<Complex<T>>, vector2: Vec<Complex<T>> ) -> Vec<Complex<T>>
    where T: Float
{
    // Determine length of output
    if vector1.len() < vector2.len() {
        let min_len = vector1.len();
        let mut r_vector = vector2.clone();
        
        for i in 0..min_len {
            r_vector[i] = vector1[i]*vector2[i];
        }
        return r_vector;

    }
    else    {
        let min_len = vector2.len();
        let mut r_vector =  vector1.clone();
        
        for i in 0..min_len {
            r_vector[i] = vector1[i]*vector2[i];
        }
        return r_vector;
    }
}

/// Element-wise addition of two vectors of equal or unequal size.
/// Result has the length of the longes vector.
pub fn mulInt<T>( vector1: Vec<Complex<T>>, vector2: Vec<Complex<T>> ) -> Vec<Complex<T>>
    where T: PrimInt
{
    // Determine length of output
    if vector1.len() < vector2.len() {
        let min_len = vector1.len();
        let mut r_vector = vector2.clone();
        
        for i in 0..min_len {
            r_vector[i] = vector1[i]+vector2[i];
        }
        return r_vector;

    }
    else    {
        let min_len = vector2.len();
        let mut r_vector =  vector1.clone();
        
        for i in 0..min_len {
            r_vector[i] = vector1[i]*vector2[i];
        }
        return r_vector;
    }
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

/// Power of a vector in dBW (dB relative to one Watt).
pub fn power_dbw<T>( vector: Vec<Complex<T>> )-> Vec<T>
    where T: Real
{
    let mut r_vector:Vec<T> = Vec::with_capacity( vector.len() );
        for i in 0..vector.len()  {
            r_vector.push( super::sfunc::pow2db(c_power_of_2!(vector[i])) );
        }
        return r_vector;
}

/// Power of a vector  in dBW (dB relative to one mW).
pub fn power_dbm<T>( vector: Vec<Complex<T>> )-> Vec<T>
    where T: Real
{
    let mut r_vector:Vec<T> = Vec::with_capacity( vector.len() );
        for i in 0..vector.len()  {
            r_vector.push( super::sfunc::pow2db(c_power_of_2!(vector[i])/numb!(0.001)) );
        }
        return r_vector;
}


macro_rules! magnitude_spectrum_calculation {
    ( $vector:expr, $T:ty ) => {
        let mut temp_vector = $vector;
        let mut planner = FftPlanner::<$T>::new();
        let size = temp_vector.len();

        let fft = planner.plan_fft_forward( size );

        fft.process( temp_vector.as_mut_slice() );
        return crate::vfunc::scale( abs(temp_vector), (1 as $T) / (size as $T) );
    };
}

/// Calculate magnitue spectrum for 32-bit complex floating point vectors, linear scale.
/// Corresponding angular frequency [-pi,..., 0,...,pi-(2pi/N)].
pub fn magnitude_spectrum( vector: Vec<Complex<f32>> ) -> Vec<f32>
{
    magnitude_spectrum_calculation!( vector, f32 );
}


/// Calculate magnitue spectrum for 64-bit complex floating point vectors, linear scale.
/// Corresponding angular frequency [-pi,..., 0,...,pi-(2pi/N)].
pub fn magnitude_spectrum64( vector: Vec<Complex<f64>> )->  Vec<f64>
{
    magnitude_spectrum_calculation!( vector, f64 );
    
}

/// Calculate power spectrum for 32-bit complex floating point vectors, linear scale.
/// Corresponding angular frequency [-pi,..., 0,...,pi-(2pi/N)].
pub fn power_spectrum( vector: Vec<Complex<f32>> ) -> Vec<f32>
{
    let magnitude_vector = magnitude_spectrum(vector);
    return crate::vfunc::powf(magnitude_vector, 2 as f32);
}


/// Calculate power spectrum for 64-bit complex floating point vectors, linear scale.
/// Corresponding angular frequency [-pi,..., 0,...,pi-(2pi/N)].
pub fn power_spectrum64( vector: Vec<Complex<f64>> )->  Vec<f64>
{
    let magnitude_vector = magnitude_spectrum64(vector);
    return crate::vfunc::powf(magnitude_vector, 2 as f64);
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
    /// The norm or abolute value.
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
    fn cfunc_neg() {
        let vec = vec![ C32F!(2,0), C32F!(1,4)];
        assert_eq!( vec![ C32F!(-2,0), C32F!(-1,-4)], neg( vec ) );
    }

    #[test]
    fn cfunc_scale() {
        let vec = vec![ C32F!(2,0), C32F!(0,4), C32F!(-2,0) ];
        assert_eq!( vec![ C32F!(4,0), C32F!(0,8), C32F!(-4,0) ], scale( vec, 2_f32 ) );
    }

    #[test]
    fn cfunc_power() {
        let vec = vec![ C32F!(2,0), C32F!(0,4), C32F!(-2,0) ];
        assert_eq!( vec![ 4_f32, 16_f32, 4_f32 ], power( vec ) );
    }

    #[test]
    fn cfunc_power_dbw() {
        let vec = vec![ C32F!(2,0), C32F!(0,4), C32F!(-2,0) ];
        assert_eq!( vec![ 6.0206003_f32, 12.041201_f32, 6.0206003_f32 ], power_dbw( vec ) );
    }

    #[test]
    fn cfunc_power_dbm() {
        let vec = vec![ C32F!(2,0), C32F!(0,4), C32F!(-2,0) ];
        assert_eq!( vec![ 36.0206_f32, 42.041201_f32, 36.0206003_f32 ], power_dbm( vec ) );
    }

    #[test]
    fn cfunc_magnitude_spectrum() {
        let vec = vec![ C32F!(2,0), C32F!(1,4), C32F!(2,0), C32F!(1,4), C32F!(2,0), C32F!(1,4), C32F!(2,0), C32F!(1,4) ];
        assert_eq!( vec![ 2.5_f32, 0_f32, 0_f32, 0_f32, 2.0615528_f32, 0_f32, 0_f32, 0_f32 ], magnitude_spectrum( vec ) );
    }

    #[test]
    fn cfunc_power_spectrum() {
        let vec = vec![ C32F!(2,0), C32F!(1,4), C32F!(2,0), C32F!(1,4), C32F!(2,0), C32F!(1,4), C32F!(2,0), C32F!(1,4) ];
        assert_eq!( vec![ 6.25_f32, 0_f32, 0_f32, 0_f32, 4.25_f32, 0_f32, 0_f32, 0_f32 ], power_spectrum( vec ) );
    }
    
    #[test]
    fn cfunc_inv() {
        let vec = vec![ C32F!(2,0), C32F!(0,4)];
        assert_eq!( vec![ C32F!(0.5,-0), C32F!(0,-0.25)], inv( vec ) );
    }

    #[test]
    fn cfunc_conj() {
        let vec = vec![ C32F!(2,0), C32F!(1,4)];
        assert_eq!( vec![ C32F!(2,0), C32F!(1,-4)], conj( vec ) );
    }
}
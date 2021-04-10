//--------------------------------------------------------------//
// Norsk Datateknikk AS 2021                                    //
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

/// Complex 32-bit float
pub type C32F = Complex::<f32>;
/// Complex 64-bit float
pub type C64F = Complex::<f64>;
/// Complex 64-bit float
pub type C32I = Complex::<i32>;
/// Complex 64-bit float
pub type C64I = Complex::<i64>;

#[macro_export]
/// Create 32-bit float complex number.
macro_rules! C32F {
    ( $re:expr, $im:expr ) => {
        num_complex::Complex::new( $re as f32, $im as f32 );
    };
}

#[macro_export]
/// Create 64-bit float complex number.
macro_rules! C64F {
    ( $re:expr, $im:expr ) => {
        num_complex::Complex::new( $re as f64, $im as f64 );
    };
}

#[macro_export]
/// Create complex number of specified size and type
macro_rules! c_value {
    ( $re:expr, $im:expr, $N:ty ) => { 
        num_complex::Complex::new( $re as $N, $im as $N );
    };
}

/// Returns the complex valu re+i*im.
pub fn c_value<N>( re:N, im:N)-> Complex<N>
    where N: Num
{
    return Complex::new( re as N, im as N );
}

/// Returns the index of the highest valued item.
pub fn arg_max<R>( vector: Vec<R> )-> usize
    where R: Real
{
    let mut max_val:R = R::min_value();
    let mut arg_max:usize = 0;
    for index in 0..vector.len() {
        if max_val < vector[index]
        {
            max_val = vector[index];
            arg_max = index;
        }
    } 
    return arg_max;
}

/// Returns the index of the lowest valued item.
pub fn arg_min<R>( vector: Vec<R> )-> usize
    where R: Real
{
    let mut min_val:R = R::max_value();
    let mut arg_min:usize = 0;
    for index in 0..vector.len() {
        if min_val < vector[index]
        {
            min_val = vector[index];
            arg_min = index;
        }
    } 
    return arg_min;
}

/// Returns the highest valued item.
pub fn max<R>( vector: Vec<R> )-> R
    where R: Real
{
    let mut max_val:R = R::min_value();
    for index in 0..vector.len() {
        if max_val < vector[index]
        {
            max_val = vector[index];
        }
    } 
    return max_val;
}

/// Returns the index of the lowest valued item.
pub fn min<R>( vector: Vec<R> )-> R
    where R: Real
{
    let mut min_val:R = R::max_value();
    for index in 0..vector.len() {
        if min_val < vector[index]
        {
            min_val = vector[index];
        }
    } 
    return min_val;
}

/// Returns the element-wise sine of a vector.
pub fn exp<F>( vector: Vec<F> )-> Vec<F>
    where F: Float
{
    let mut r_vector: Vec<F> = Vec::with_capacity( vector.len() );
    for index in 0..vector.len() {
        r_vector[index] = vector[index].exp();
    }        
            
    return r_vector;
}

/// Returns the complex element-wise sine of a vector.
pub fn c_exp<F>( vector: Vec<Complex<F>> )-> Vec<Complex<F>>
    where F: Float
{
    let mut r_vector: Vec<Complex<F>> = Vec::with_capacity( vector.len() );
    for index in 0..vector.len() {
        r_vector[index] = vector[index].exp();
    }
            
    return r_vector;
}

macro_rules! F2 {
    () => { F::from(2).unwrap(); };
}

/// Absolute of a complex scalar.
macro_rules! abs_c_scalar {
    ( $c_val:expr) => { 
        F::sqrt( $c_val.re.powf( F2!() )+$c_val.im.powf( F2!() ) ) ;
     };
}

/// Returns the complex element-wise absolute value.
pub fn c_abs<F>( vector: Vec<Complex<F>> )-> Vec<F>
    where F: Float
{
    println!( "{:?}", vector.len() );
    let mut r_vector: Vec<F> = Vec::with_capacity( vector.len() );
    for index in 0..vector.len() {
        r_vector.push( abs_c_scalar!( vector[index] ) );
    }
    return r_vector;
}

/// Returns the complex element-wise absolute value.
pub fn abs<R>( vector: Vec<R> )-> Vec<R>
    where R: Real
{
    let mut r_vector: Vec<R> = Vec::with_capacity( vector.len() );
    for index in 0..vector.len() {
        r_vector.push( vector[index].abs() );
    }
    return r_vector;
}

/// Returns the element-wise sine of a vector.
pub fn sin<F>( vector: Vec<F> )-> Vec<F>
    where F: Float
{
    let mut r_vector: Vec<F> = Vec::with_capacity(vector.len());
    for index in 0..vector.len() {
        r_vector.push( F::sin(vector[index]) );
    }
    return r_vector;
}

/// Returns the element-wise cosine of a vector.
pub fn cos<F>( vector: Vec<F> )-> Vec<F>
    where F: Float
{
    let mut r_vector: Vec<F> = Vec::with_capacity(vector.len());
    for index in 0..vector.len() {
        r_vector.push( F::cos(vector[index]) );
    }
    return r_vector;
}

/// Returns a 1D vector of zeros of size numb_samples.
/// [0,0,...,0]
pub fn zeros<N>( numb_samples: usize ) -> Vec<N>
    where N: Num
{
    let mut vector: Vec<N> = Vec::with_capacity(numb_samples);
    for i in 0..numb_samples {
        vector.push(N::zero());
    }
    return vector;
}

/// Returns a 1D vector of ones of size numb_samples.
/// [1,1,...,1]
pub fn ones<N>( numb_samples: usize ) -> Vec<N>
    where N: Num
{
    let mut vector: Vec<N> = Vec::with_capacity(numb_samples);
    for i in 0..numb_samples {
        vector.push(N::one());
    }
    return vector;
}


/// Returns a 1D vector of evenly spaced numbers of type F.
/// [0,1,...,N-1]
pub fn linspace<F>( start: F, stop: F, numb_samples: usize ) -> Vec<F>
    where F: Float
{
    let delta = if numb_samples > 1
    {
        let nf: F = F::from(numb_samples).unwrap();
        (stop - start) / (nf - F::one())
    } else {
        F::zero()
    };

    let mut vector = Vec::<F>::with_capacity(numb_samples);

    for index in 0..vector.capacity()
    {
        let idx = F::from(index).unwrap();
        vector.push(start+(idx)*delta);
    }
    return vector;
}
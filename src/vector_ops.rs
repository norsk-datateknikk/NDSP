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

/// Returns the complex element-wise sine of a vector.
pub fn c_exp<F>( vector: Vec<Complex<F>> )-> Vec<Complex<F>>
    where F: Float
{
    let mut r_vector: Vec<Complex<F>> = Vec::with_capacity( vector.len() );
    for index in 0..vector.len() {
        r_vector.push( Complex::<F>::exp( vector[index] ) );
    }
    return r_vector;
}

/// Returns the complex element-wise absolute value.
pub fn c_abs<F>( vector: Vec<Complex<F>> )-> Vec<F>
    where F: Float
{
    println!( "{:?}", vector.len() );
    let mut r_vector: Vec<F> = Vec::with_capacity( vector.len() );
    for index in 0..vector.len() {
        r_vector.push( abs_c_scalar!( vector[index] => F ) );
    }
    return r_vector;
}


/// This macro generates element-wise operations on vectors.
/// The operations must be a trait of the vector item class.
/// vector<T> can thus have all traits of T.
macro_rules! element_wise_operand {
    (   
        $(#[$meta:meta])*
        $operand:ident 
        $trait:ident
    ) => {
        $(#[$meta])*
        /// Element-wise operation on vector of real type R.
        pub fn $operand<T>( vector: Vec<T> )-> Vec<T>
        where T: $trait
        {
        let mut r_vector: Vec<T> = Vec::with_capacity( vector.len() );
        for index in 0..vector.len() {
            r_vector.push( T::$operand(vector[index]) );
        }
        return r_vector;
        }
    };
}

/*
/// This macro generates element-wise operations on vectors of complex numbers.
/// The operations must be a trait of the vector item class.
/// vector<T> can thus have all traits of T.
macro_rules! complex_element_wise_operand {
    (   
        $(#[$meta:meta])*
        $operand:ident
        $trait:ident
    ) => {
        $(#[$meta])*
        /// Element-wise operation on vector of real type R.
        pub fn $operand<T>( vector: Vec<Complex<T>> )-> Vec<Complex<T>>
            where T: $trait
        {
            let mut r_vector: Vec<T> = Vec::with_capacity( vector.len() );
            for index in 0..vector.len() {
                r_vector.push( T::$operand(vector[index]) );
            }
            return r_vector;
        }
    };
}

complex_element_wise_operand!{
    /// Absolute value of complex number.
    abs
    Float
}
*/

element_wise_operand!{
    /// Absolute value.
    abs
    Real
}
element_wise_operand!{
    /// Sin.
    sin
    Float
}
element_wise_operand!{
    /// Cos.
    cos
    Float
}
element_wise_operand!{
    /// Tan.
    tan
    Float
}
element_wise_operand!{
    /// Exponential, e^x.
    exp
    Float
}

/// Returns a 1D vector of zeros of size numb_samples.
/// [0,0,...,0]
pub fn zeros<N>( numb_samples: usize ) -> Vec<N>
    where N: Num
{
    let mut vector: Vec<N> = Vec::with_capacity(numb_samples);
    for _i in 0..numb_samples {
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
    for _i in 0..numb_samples {
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
        let nf: F = F::from(numb_samples+1).unwrap();
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
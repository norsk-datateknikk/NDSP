//--------------------------------------------------------------//
// Copyright 2021 Norsk Datateknikk AS                          //
//--------------------------------------------------------------//
// This file is subject to the terms and conditions defined in  //
// file 'LICENSE', which is part of this source code package.   //
//--------------------------------------------------------------//

// This file containns basic functions that can operate on vectors of numerical types.

use num::traits::Num;
use num::traits::Float;
use num::traits::real::Real;

use rustfft::FftPlanner;

// Import scalar operations
use crate::sfunc::*;

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

/// Sum of vector.
pub fn sum<T>( vector: Vec<T> )-> T
    where T: Real
{
    let mut r_val:T = <T>::from(0).unwrap();
    for item in vector {
        r_val = r_val+item;
    }
    return r_val;
}

/// Mean of vector.
pub fn mean<T>( vector: Vec<T> )-> T
    where T: Real
{
    let mut r_val:T = <T>::from(0).unwrap();
    for item in &vector {
        r_val = r_val+*item;
    } 
    return r_val/( num::cast( vector.len() ).unwrap() );
}

//TODO variance, both methods.

/// Multiply vector with scalar.
pub fn scale<T>( vector: &Vec<T>, scalar: &T )-> Vec<T>
    where T: Real
{
    let mut r_vector:Vec<T> = Vec::with_capacity( vector.len() );
    for i in 0..vector.len() {
        r_vector.push(vector[i]*scalar.clone());
    }
    return r_vector;
}

/// Power of a vector.
pub fn power<T>( vector: Vec<T> )-> Vec<T>
    where T: Real
{
    let mut r_vector:Vec<T> = Vec::with_capacity( vector.len() );
        for i in 0..vector.len()  {
            r_vector.push(vector[i].powf(<T>::from(2).unwrap()));
        }
        return r_vector;
}

/// Energy of a vector.
pub fn energy<T>( vector: Vec<T> )-> T
    where T: Real
{
    return sum(power(vector));
}

pub fn neg<T>( vector: &Vec<T> )-> Vec<T>
    where T: Float
{
    let mut r_vector: Vec<T> = Vec::with_capacity( vector.len() );
    for item in vector  {
        r_vector.push( -item.clone() );
    }
    return r_vector;
}

/// This macro generates element-wise operations on vectors.
/// The operations must be a trait of the vector item class.
/// vector<T> can thus have all traits of T.
macro_rules! element_wise_operand {
    (   
        $(#[$comment:meta])*
        $operand:ident 
        $trait:ident
    ) => {
        $(#[$comment])*
        /// Element-wise operation on vector of real type T.
        pub fn $operand<T>( vector: &Vec<T> )-> Vec<T>
        where T: $trait
        {
        let mut r_vector: Vec<T> = Vec::with_capacity( vector.len() );
        for item in vector  {
            r_vector.push( item.$operand() );
        }
        return r_vector;
        }
    };
}

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
element_wise_operand!{
    /// Square root.
    sqrt
    Float
}
element_wise_operand!{
    /// Natural logarithm.
    ln
    Float
}
element_wise_operand!{
    /// Natural logarithm.
    log10
    Float
}
element_wise_operand!{
    /// Base-2 logarithm.
    log2
    Float
}
element_wise_operand!{
    /// Angle conversion to degree.
    to_degrees
    Float
}
element_wise_operand!{
    /// Angle conversion to radians.
    to_radians
    Float
}
element_wise_operand!{
    /// Round to nearest integer not superseeding.
    floor
    Float
}
element_wise_operand!{
    /// Round to nearest integer superseeding.
    ceil
    Float
}
element_wise_operand!{
    /// The fractional part.
    fract
    Float
}

/// This macro generates element-wise operations on vectors.
/// The operations must be a trait of the vector item class.
/// vector<T> can thus have all traits of T.
macro_rules! element_wise_arg_operand {
    (   
        $(#[$comment:meta])*
        $operand:ident
        $trait:ident
    ) => {
        $(#[$comment])*
        /// Element-wise operation on vector of real type T.
        pub fn $operand<T>( vector: Vec<T>, argument : T )-> Vec<T>
        where T: $trait
        {
        let mut r_vector: Vec<T> = Vec::with_capacity( vector.len() );
        for item in vector  {
            r_vector.push( item.$operand( argument ) );
        }
        return r_vector;
        }
    };

    (   
        $(#[$comment:meta])*
        $operand:ident
        $trait:ident
        $arg_type:ty
    ) => {
        $(#[$comment])*
        /// Element-wise operation on vector of real type T.
        pub fn $operand<T>( vector: Vec<T>, argument : $arg_type )-> Vec<T>
        where T: $trait
        {
        let mut r_vector: Vec<T> = Vec::with_capacity( vector.len() );
        for item in vector  {
            r_vector.push( item.$operand( argument ) );
        }
        return r_vector;
        }
    };
}

element_wise_arg_operand!{
    /// Raise to a floating point valued power.
    powf
    Float
}
element_wise_arg_operand!{
    /// Raise to a floating point valued power.
    powi
    Float
    i32
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

/// Rotate the items in the vector to the left.
pub fn rotate_left<T>( vector: Vec<T>, steps: usize ) -> Vec<T>
    where T: Float
{
    let size = vector.len();
    
    let r_vector = [ &vector[ steps..size ], &vector[ 0..steps ] ].concat();

    //r_vector.append( &mut vector[ 1..steps ] );
    return r_vector;
}

/// Rotate the items in the vector to the right.
pub fn rotate_right<T>( vector: Vec<T>, steps: usize ) -> Vec<T>
    where T: Float
{
    let size = vector.len();
    
    let r_vector = [ &vector[ size-steps..size ], &vector[ 0..size-steps ] ].concat();

    //r_vector.append( &mut vector[ 1..steps ] );
    return r_vector;
}

//TODO This functions appears multiple times with add mul and div. Make macro.
/// Element-wise addition of two vectors of equal or unequal size.
pub fn add<T>( vector1: &Vec<T>, vector2: &Vec<T> ) -> Vec<T>
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

//TODO This functions appears multiple times with add mul and div. Make macro.
/// Element-wise addition of two vectors of equal or unequal size.
/// Result has the length of the longes vector.
pub fn mul<T>( vector1: &Vec<T>, vector2: &Vec<T> ) -> Vec<T>
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

//TODO This functions appears multiple times with add mul and div. Make macro.
/// Element-wise addition of two vectors of equal or unequal size.
/// Result has the length of the longes vector.
pub fn div<T>( vector1: &Vec<T>, vector2: &Vec<T> ) -> Vec<T>
    where T: Float
{
    // Determine length of output
    if vector1.len() < vector2.len() {
        let min_len = vector1.len();
        let mut r_vector = vector2.clone();
        
        for i in 0..min_len {
            r_vector[i] = vector1[i]/vector2[i];
        }
        return r_vector;

    }
    else    {
        let min_len = vector2.len();
        let mut r_vector =  vector1.clone();
        
        for i in 0..min_len {
            r_vector[i] = vector1[i]/vector2[i];
        }
        return r_vector;
    }
}

/*
macro_rules! magnitude_spectrum_calculation {
    ( $vector:expr, $T:ty ) => {
        let mut temp_vector:Vec<Complex<$T>> = Vec::with_capacity( $vector.len() );
        let mut planner = FftPlanner::<$T>::new();
        let size = temp_vector.len();
        
        let fft = planner.plan_fft_forward( size );

        fft.process(&mut temp_vector);
        let magnitude = scale( crate::cvfunc::abs(temp_vector), (1 as $T) / (size as $T) ); 
        return rotate_right( magnitude, size/2);
    };
}

/// Calculate magnitue spectrum for 32-bit floating point vectors, linear scale.
/// Corresponding angular frequency [-pi,..., 0,...,pi-(2pi/N)].
pub fn magnitude_spectrum( vector: Vec<f32> ) -> Vec<f32>
{
    magnitude_spectrum_calculation!( vector, f32 );
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn func_linspace() {
        let vec = linspace::<f32>(0_f32, 4_f32, 4);
        assert_eq!(vec, vec![ 0_f32, 1_f32, 2_f32, 3_f32 ]);
    }

    #[test]
    fn func_abs() {
        let vec = vec![ 2_f32, -2_f32 ];
        assert_eq!( vec![ 2_f32, 2_f32 ], abs( &vec ) );
    }
    
    #[test]
    fn func_sum() {
        let vec = vec![ 2_f32, -2_f32, -2_f32 ];
        assert_eq!( -2_f32, sum( vec ) );
    }  

    #[test]
    fn func_mean() {
        let vec = vec![ 2_f32, 4_f32, 6_f32 ];
        assert_eq!( 4_f32, mean( vec ) );
    }

    #[test]
    fn func_power() {
        let vec = vec![ 1_f32, -2_f32 ];
        assert_eq!( vec![ 1_f32, 4_f32 ] , power( vec ) );
    }

    #[test]
    fn func_neg() {
        let vec = vec![ 1_f32, -2_f32 ];
        assert_eq!( vec![ -1_f32, 2_f32 ] , neg( &vec ) );
    }

    #[test]
    fn func_energy() {
        let vec = vec![ 1_f32, -2_f32 ];
        assert_eq!( 5_f32 , energy( vec ) );
    }

    #[test]
    fn func_rotate_right() {
        let vec = vec![ 1_f32, 2_f32, 3_f32, 4_f32, 5_f32, 6_f32 ];
        assert_eq!( vec![ 5_f32, 6_f32, 1_f32, 2_f32, 3_f32, 4_f32 ] , rotate_right( vec, 2) );
    }

    #[test]
    fn func_rotate_left() {
        let vec = vec![ 1_f32, 2_f32, 3_f32, 4_f32, 5_f32, 6_f32 ];
        assert_eq!( vec![ 3_f32, 4_f32, 5_f32, 6_f32, 1_f32, 2_f32 ] , rotate_left( vec, 2) );
    }

    #[test]
    fn func_add() {
        let vec1 = vec![ 1_f32, 2_f32, 3_f32, 4_f32, 5_f32, 6_f32 ];
        let vec2 = vec![ 1_f32, 2_f32, 3_f32, 4_f32, 5_f32 ];
        assert_eq!( vec![ 2_f32, 4_f32, 6_f32, 8_f32, 10_f32, 6_f32 ] , add( vec1, vec2) );
    }
}
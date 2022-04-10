//----------------------//
// Norsk Datateknikk AS //
//----------------------//

extern crate alloc;

use mixed_num::{Cartesian, Polar};

use crate::vec::Vec;

// Generic vector operations.
pub trait Len {
    /// The number of items in the vector.
    fn len( &self ) -> usize;
}

pub trait NewFromVec<T> {
    /// Create a new vector from an std or alloc vector.
    fn new_from_vec( vec: alloc::vec::Vec<T> ) -> Vec<T>;
}

pub trait VecNumConversion<T1, T2> {
    /// Create a `Vec<T2>` form `Vec<T1>`
    /// 
    /// ### Example
    ///
    /// ```
    /// use ndsp::*;
    /// let test_vec_f32 = Vec::lin_range(0f32, 3f32, 4);
    /// let test_vec_f64 = Vec::<f64>::vec_from_num(&test_vec_f32);
    /// 
    /// let result_vec = Vec::lin_range(0f64, 3f64, 4);
    /// assert_eq!( test_vec_f64, result_vec );
    /// ```
    fn vec_from_num( in_vec: &Vec<T2>) -> Self;
    /// Create a `Vec<T2>` form `Vec<T1>`
    /// 
    /// ### Example
    ///
    /// ```
    /// use ndsp::*;
    /// let test_vec_f32 = Vec::lin_range(0f32, 3f32, 4);
    /// let test_vec_f64: Vec<f64> = test_vec_f32.vec_to_num();
    /// 
    /// let result_vec = Vec::lin_range(0f64, 3f64, 4);
    /// assert_eq!( test_vec_f64, result_vec );
    /// ```
    fn vec_to_num(&self) -> Vec<T2>;
}

pub trait Cap {
    /// The capacity of the vector. The amount of allocated positions.
    fn capacity( &self ) -> usize;
}

// Generic vector operations.
pub trait PushBack<T> {
    /// Add item to the back of the vector.
    fn push_back( &mut self, value: T );
}

pub trait Ones<T> {
    /// Create a vector of ones.
    fn ones(len: usize) -> Vec<T>;
}

pub trait Zeros<T> {
    /// Create a vector of zeros.
    fn zeros(len: usize) -> Vec<T>;
}

pub trait LinRange<T>{
    /// Generate a 1D vector of evenly spaced numbers of type T.
    fn lin_range( start:T, stop:T, num:usize ) -> Self;
} 

// Compute-In-place operations.
pub trait Powi {
    /// Rase the vector to an integer power. Computed-in-place.
    fn powi( &mut self, power:i32 );
}

pub trait Pow<T> {
    /// Rase the vector to an integer power. Computed-in-place.
    fn pow( &mut self, power:T );
}

pub trait Abs {
    /// Element-wise absolute value of `self`. Computed-in-place.
    fn abs( &mut self );
}

pub trait Sqrt {
    /// Element-wise square of `self`. The specific implimentation varies with type. Computed-in-place.
    fn sqrt( &mut self );
}

pub trait Sin {
    /// Element-wise sin of `self`. Computed-in-place.
    fn sin( &mut self );
}

pub trait Cos {
    /// Element-wise cos of `self`. Computed-in-place.
    fn cos( &mut self );
}

pub trait Tan {
    fn tan( &mut self );
}

pub trait Atan {
    /// Element-wise atan of `self`. The specific implimentation varies with type. Computed-in-place.
    fn atan( &mut self );
}

pub trait Ang<T> {
    /// Element-wise angle of complex numbers.
    fn ang( &mut self );
}

pub trait Mag<T> {
    /// Element-wise magnitude of complex numbers.
    fn mag( &mut self );
}

pub trait WrapPhase {
    /// Wrapps `self` to the -π=<x<π range. Computed-in-place.
    fn wrap_phase( &mut self );
}

pub trait Clip<T> {
    /// Clip all values to the `{lower_limit, uppeer_limit}` range.
    fn clip( &mut self, lower_limit:T, uppeer_limit:T );
}

pub trait Minimum<T> {
    /// Constrain `self` to be  `>=lower_limit`.
    fn minimum( &mut self, lower_limit:T );
}

pub trait Maximum<T> {
    /// Constrain `self` to be  `>=lower_limit`.
    fn maximum( &mut self, upper_limit:T );
}

pub trait Fft {
    /// Compute the the FFT of `self`. Computed-in-place.
    fn fft( &mut self );
}

pub trait Ifft {
    /// Compute the the FFT of `self`. Computed-in-place.
    fn ifft( &mut self );
}

pub trait HilbertTransform<T>
{
    /// Compute the Discrete Hilbert Transform (DHT).
    fn hilbert(&self, output_buffer: &mut Vec<Cartesian<T>>);
}

pub trait Max<T> {
    // Return the value of the highest item in the vector 
    fn max( &self ) -> T;
}

pub trait Min<T> {
    // Return the value of the lowest item in the vector 
    fn min( &self ) -> T;
}

pub trait MinMax<T> {
    // Returns a touple with the value of the highest and lowest items in the vector. 
    fn min_max( &self ) -> (T,T);
}

pub trait ToRange<T> {
    // Returns the min_max as a `Range<T>`.
    fn to_range( &self ) -> core::ops::Range<T>;
}

pub trait Indices<T> {
    // Returns the indices of the vector on `0..N-1` for self.
    fn indices( &self ) -> Vec<T>;
}

pub trait Re<T> {
    /// Returns the real part of the vector as a real only vector.
    fn re(&self) -> Vec<T>;
}
pub trait Im<T> {
    /// Returns the real part of the vector as a real only vector.
    fn im(&self) -> Vec<T>;
}

pub trait AsComplexCartesian<T> {
    /// Returns the real part of the vector in a complex cartesian vector.
    fn as_complex_cartesian(&self) -> Vec<Cartesian<T>>;
}

pub trait AsComplexPolar<T> {
    /// Returns the real part of the vector in a complex cartesian vector.
    fn as_complex_polar(&self) -> Vec<Polar<T>>;
}

// Traits requiring std

/// The type of the items in a binary file file.
#[derive(Clone, Debug, PartialEq)]
pub enum ItemType {
    Complex32,
    Float32,
}

pub trait Decibel<T>{
    // Computed-in-place.
    fn mag2db( &mut self );
    // Computed-in-place.
    fn db2mag( &mut self );
    // Computed-in-place.
    fn pow2db( &mut self );
    // Computed-in-place.
    fn db2pow( &mut self );
}

pub trait Sum<T>{
    /// Sum of vector.
    fn sum( &self ) -> T;
}

pub trait Mean<T>{
    /// Mean of vector.
    fn mean( &self ) -> T;
}

pub trait Energy<T>{
    /// Energy of vector.
    fn energy( &self ) -> T;
}

pub trait Power<T>{
    /// Power of the vector
    fn power( &mut self );
}

pub trait Psd<T>{
    /// Calculate the Power Spectral Density (PSD) in linear scale of a signal.
    fn psd( &self ) -> Vec<T>;
}

pub trait FromBinary {
    // Load signal of type T in a binary file into vector. 
    fn from_binary( item_type: ItemType, path: &str ) -> Self;
}

pub trait ToBinary {
    // Load signal of type T in a binary file into vector. 
    fn to_binary( &self, path: &str );
}

pub trait ToTouples<T>{
    // Load signal of type T in a binary file into vector. 
    fn to_touples( &self ) -> alloc::vec::Vec<(T, T)>;
    // Load two vectors self, other into touple, for plotting.
    fn to_xy_touples( &self, other: &Self ) -> alloc::vec::Vec<(T, T)>;
}

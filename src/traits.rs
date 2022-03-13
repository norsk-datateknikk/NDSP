//----------------------//
// Norsk Datateknikk AS //
//----------------------//

extern crate alloc;

use num::Complex;
use crate::vec::Vec;

// structs to enable separation between real and complex implementations of the same trait.
pub struct R;
pub struct C;

// Generic vector operations.
pub trait Len {
    /// The number of items in the vector.
    fn len( &self ) -> usize;
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

pub trait Ones {
    /// Create a vector of ones.
    fn ones() -> Self;
}

pub trait Zeros {
    /// Create a vector of zeros.
    fn zeros() -> Self;
}

pub trait LinRange<T>{
    /// Generate a 1D vector of evenly spaced numbers of type T.
    fn lin_range( start:T, stop:T, num:usize ) -> Self;
} 

// Compute-In-place operations.
pub trait Powi {
    /// Rase the vector to an integer power. Computed-in-place.
    fn powi( &mut self, power:u32 );
}

pub trait Abs<T> {
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

pub trait WrapPhase {
    /// Wrapps `self` to the -π=<x<π range. Computed-in-place.
    fn wrap_phase( &mut self );
}

pub trait Fft {
    /// Compute the the FFT of `self`. Computed-in-place.
    fn fft( &mut self );
}

pub trait Ifft {
    /// Compute the the FFT of `self`. Computed-in-place.
    fn ifft( &mut self );
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
    fn minmax( &self ) -> (T,T);
}

pub trait ToRange<T> {
    // Returns the minmax as a `Range<T>`.
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


pub trait AsComplex<T> {
    /// Returns the real part of the vector in a complex vector.
    fn as_complex(&self) -> Vec<Complex<T>>;
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
}

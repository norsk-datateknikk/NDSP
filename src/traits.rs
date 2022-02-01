//-----------------------------------------------------------------//
// Copyright 2021 Norsk Datateknikk AS                             //
//-----------------------------------------------------------------//
// This file is subject to the terms and conditions defined in the //
// file 'LICENSE', which is part of this source code package.      //
//-----------------------------------------------------------------//


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
    fn ones()      -> Self;
}

pub trait Zeros {
    /// Create a vector of zeros.
    fn zeros()      -> Self;
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

/// The type of the items in a binary file file.
#[derive(Clone, Debug, PartialEq)]
pub enum ItemType {
    Complex32,
    Float32,
}
// Traits requiring std
pub trait FromFile {
    // Load signal of type T in a binary file into vector. 
    fn from_file( item_type: ItemType, path: &str ) -> Self;
}
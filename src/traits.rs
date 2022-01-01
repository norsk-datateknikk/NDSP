//--------------------------------------------------------------//
// Copyright 2021 Norsk Datateknikk AS                          //
//--------------------------------------------------------------//
// This file is subject to the terms and conditions defined in  //
// file 'LICENSE', which is part of this source code package.   //
//--------------------------------------------------------------//


// Generic vector operations.
pub trait Len {
    fn len( &self ) -> usize;
}

pub trait Cap {
    fn capacity( &self ) -> usize;
}

// Generic vector operations.
pub trait PushBack<T> {
    fn push_back( &mut self, value: T );
}

pub trait Ones {
    fn ones()      -> Self;
}

pub trait Zeros {
    fn zeros()      -> Self;
}

// Compute-In-place operations.
pub trait Powi {
    fn powi( &mut self, power:u32 );
}

pub trait Abs {
    fn abs( &mut self );
}

pub trait Sqrt {
    fn sqrt( &mut self );
}

pub trait Sin {
    fn sin( &mut self );
}

pub trait Cos {
    fn cos( &mut self );
}

pub trait Tan {
    fn tan( &mut self );
}

pub trait Atan {
    fn atan( &mut self );
}

pub trait WrapPhase {
    fn wrap_phase( &mut self );
}

pub trait Fft {
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
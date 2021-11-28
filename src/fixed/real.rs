//--------------------------------------------------------------//
// Copyright 2021 Norsk Datateknikk AS                          //
//--------------------------------------------------------------//
// This file is subject to the terms and conditions defined in  //
// file 'LICENSE', which is part of this source code package.   //
//--------------------------------------------------------------//

extern crate alloc;
extern crate num;
extern crate fixed_trigonometry; 

use crate::traits;
use crate::traits::*;

/// Numeric vector of real, fixed-point numbers.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Vec<T>
{
    vec: alloc::vec::Vec<T>
}

impl<T> Vec<T> {
    /// Allocate a memmory for a vector of a certain capacity.
    /// 
    /// ## Arguments
    /// 
    /// * `capacity` - The capacity of the new vector.
    ///
    #[allow(dead_code)]
    fn new_with_capacity( capacity: usize ) -> Vec<T>
    {
        let mut vec = alloc::vec::Vec::<T>::new(); 
        vec.reserve_exact(capacity);
        Vec {
            vec: vec,
        }
    }
}

impl <T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    /// Trait for returning an indexed value of the array.
    #[inline]
    fn index(&self, index: usize) -> &T {
        return &self.vec[index];
    }
}

impl <T> core::ops::IndexMut<usize> for Vec<T> {
    /// Trait for returning a mutable reference to indexed item.
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut T {
        return &mut self.vec[index];
    }
}

impl <T> traits::Len for Vec<T> {
    /// Function returning the size of the vector.
    /// 
    /// The size is the number of items with values.
    #[inline]
    fn len(&self) -> usize {
        return self.vec.len();
    }
}
impl <T> traits::Len for &Vec<T> {
    /// Function returning the size of the vector.
    /// 
    /// The size is the number of items with values.
    #[inline]
    fn len(&self) -> usize {
        return self.vec.len();
    }
}

impl <T> traits::Cap for Vec<T> {
    /// Function returning the capacity of the vector.
    /// 
    /// The capacity is the maximum number of items the vector can hold.
    #[inline]
    fn capacity(&self) -> usize {
        return self.vec.capacity();
    }
}
impl <T> traits::Cap for &crate::fixed::real::Vec<T> {
    /// Function returning the capacity of the vector.
    /// 
    /// The capacity is the maximum number of items the vector can hold.
    #[inline]
    fn capacity(&self) -> usize {
        return self.vec.capacity();
    }
}

impl <T: fixed::traits::FixedSigned> traits::Sin for Vec<T> {
    /// Take the elemtent-wise sine using a Taylor approximation of sin(x).
    /// Self must be wrapped to the -π=<x<π range.
    fn sin(&mut self) {
        for idx in 0..self.len() {
            self[idx] = fixed_trigonometry::sin(self[idx]);
        }
    }
}

impl <T: fixed::traits::FixedSigned> traits::Cos for Vec<T> {
    /// Take the elemtent-wise sine using a Taylor approximation of sin(x).
    /// Self must be wrapped to the -π=<x<π range.
    fn cos(&mut self) {
        for idx in 0..self.len() {
            self[idx] = fixed_trigonometry::cos(self[idx]);
        }
    }
}

impl <T: fixed::traits::FixedSigned> traits::Sqrt for Vec<T> {
    /// Take the elemtent-wise sine using a Taylor approximation of sin(x).
    /// Self must be wrapped to the -π=<x<π range.
    fn sqrt(&mut self) {
        for idx in 0..self.len() {
            self[idx] = fixed_trigonometry::sqrt::niirf( self[idx], 2);
        }
    }
}

impl <T: fixed::traits::FixedSigned> traits::Abs for Vec<T> {
    /// Take the elemtent-wise sine using a Taylor approximation of sin(x).
    /// Self must be wrapped to the -π=<x<π range.
    fn abs(&mut self) {
        for idx in 0..self.len() {
            if self[idx]<0
            {
                self[idx]=-self[idx];
            }
        }
    }
}
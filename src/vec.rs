//-----------------------------------------------------------------//
// Copyright 2021 Norsk Datateknikk AS                             //
//-----------------------------------------------------------------//
// This file is subject to the terms and conditions defined in the //
// file 'LICENSE', which is part of this source code package.      //
//-----------------------------------------------------------------//

use crate::*;

extern crate alloc;
extern crate num;

use crate::traits;
use crate::traits::*;

pub mod real;
pub mod complex;

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

impl <T> traits::Cap for Vec<T> {
    /// Function returning the capacity of the vector.
    /// 
    /// The capacity is the maximum number of items the vector can hold.
    #[inline]
    fn capacity(&self) -> usize {
        return self.vec.capacity();
    }
}

impl <T> traits::PushBack<T> for Vec<T> {
    /// Push a value to the back of the vector.
    #[inline]
    fn push_back(&mut self, value: T) {
        self.vec.push(value);
    }
}
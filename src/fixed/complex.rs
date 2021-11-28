//--------------------------------------------------------------//
// Copyright 2021 Norsk Datateknikk AS                          //
//--------------------------------------------------------------//
// This file is subject to the terms and conditions defined in  //
// file 'LICENSE', which is part of this source code package.   //
//--------------------------------------------------------------//


extern crate alloc;
extern crate num;
extern crate fixed_trigonometry;
use fixed_trigonometry as trig;

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

impl <T> traits::Cap for Vec<T> {
    /// Function returning the capacity of the vector.
    /// <>
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

impl<T:fixed::traits::Fixed> Vec<num::complex::Complex<T>> {
    /// Allocate a memmory for a vector of a certain capacity.
    ///
    #[allow(dead_code)]
    fn new_from_real( real_vec: crate::fixed::real::Vec<T> ) -> Vec<num::complex::Complex<T>>
    {
        let mut vec = Vec::<num::complex::Complex<T>>::new_with_capacity(real_vec.len());

        for idx in 0..real_vec.len()
        {
            vec.push_back(num::complex::Complex::new(T::from_num(real_vec[idx]), T::from_num(0)))
        }
        return vec;
    }
}

impl <T: fixed::traits::FixedSigned> traits::Abs for Vec<num::complex::Complex<T>> {
    /// Take the elemtent-wise absolute value.
    fn abs(&mut self) {
        for idx in 0..self.len() {
            self[idx].re = trig::complex::abs(self[idx]);
            self[idx].im = T::from_num(0);
        }
    }
}

/* TODO
impl<T> Vec<num::complex::Complex<T>> {
    /// Creates a rotating pahsor with a specific angular frequency.
    /// 
    /// ## Arguments
    /// 
    /// * `capacity` - The capacity of the new vector.
    ///
    #[allow(dead_code)]
    fn nco( angular_freq: T,  ) -> Vec<num::complex::Complex<T>>
    {
        
    }
}
*/

impl <T:fixed::traits::FixedSigned> traits::Fft for Vec<num::complex::Complex<T>> {
    /// Calculate the Raddix-2 FFT for fixed point vectors.
    /// Scaled for each butterfly computation.
    /// Requires input size to be a power of two.
    /// 
    /// Computed-in-place.
    /// Decimation-in-freqency.
    /// 
    /// The method utilizes fixed point approximations for square root, sine, cosine and atan calculations.
    /// 
    /// ## Arguments
    /// 
    /// * `vec` - A mutable reference to the vector to do the computation on, and store the result in.
    /// 
    fn fft(&mut self){
        fixed_trigonometry::fft::fft( &mut self.vec);
    }
}
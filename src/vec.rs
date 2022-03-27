//----------------------//
// Norsk Datateknikk AS //
//----------------------//

use crate::*;
use crate::traits;

use mixed_num::traits::*;

mod ops;
pub use ops::*;

pub mod math_impl;
pub mod complex;
#[cfg(feature = "std")]
pub mod plot;

extern crate alloc;
use alloc::string::ToString;

extern crate num;

use core::fmt;
use alloc::string::String;

/// Numeric vector of real, fixed-point numbers.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Vec<T>
{
    vec: alloc::vec::Vec<T>
}

impl <T> NewFromVec<T> for Vec<T> {
    /// Create a new vector from an std or alloc vector.
    fn new_from_vec( vec: alloc::vec::Vec<T> ) -> Vec<T>
    {
        return Vec{ vec:vec };
    }
}

impl<T> Vec<T> {
    /// Allocate a memmory for a vector of a certain capacity.
    /// 
    /// ## Arguments
    /// 
    /// * `capacity` - The capacity of the new vector.
    ///
    #[allow(dead_code)]
    pub fn new_with_capacity( capacity: usize ) -> Vec<T>
    {
        let mut vec = alloc::vec::Vec::<T>::new(); 
        vec.reserve_exact(capacity);
        Vec {
            vec: vec,
        }
    }

    /// Expose the alloc::vec::Vec containing the data contents.
    #[allow(dead_code)]
    pub fn to_alloc_vec(&self) -> &alloc::vec::Vec<T>
        where T: Clone
    {
        return &self.vec;
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

impl <T> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    /// Conversion into an [`Iterator`].
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let test_vec = Vec::lin_range(0f32, 3f32, 4);
    /// let mut iterator = test_vec.into_iter();
    /// assert_eq!(iterator.next().unwrap(), 0f32 )
    /// ```
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}

impl <T: fmt::Display> fmt::Display for Vec<T> {
    /// # Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let test_vec = Vec::lin_range(0f32, 3f32, 4);
    /// assert_eq!(test_vec.to_string(), "[ 0, 1, 2, 3 ]" )
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut temp_string = String::from("[ ");
        for i in 0..self.len()
        {
            if 0<i
            {
                temp_string.push_str(", ");
            }
            temp_string.push_str(self[i].to_string().as_str());
        }
        temp_string.push_str(" ]");
        write!(f, "{}", temp_string)
    }
}

impl <T: MixedReal + MixedNumConversion<T2>, T2: MixedReal> ToTouples<T2> for Vec<T> {
    /// Returns the vector as a vector of touples (x,y), where `outvec[1] = (n, in_vec[n])`.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let test_vec = Vec::lin_range(0f32, 3f32, 4);
    /// assert_eq!(test_vec.to_touples()[1], (1f32,1f32) );
    /// assert_eq!(test_vec.to_touples()[2], (2f32,2f32) );
    /// ```
    fn to_touples( &self ) -> alloc::vec::Vec<(T2, T2)>
    {
        let mut outvec = alloc::vec::Vec::<(T2, T2)>::new();
        for idx in 0..self.len() {
            let tuple = (T::mixed_from_num(idx as f32).mixed_to_num(), self[idx].mixed_to_num());
            outvec.push(tuple);
        }
        return outvec;
    }
}


// We prefer doctests, as they provide documentation additionally.
#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test() {
        assert_eq!(true, true )
    }
}
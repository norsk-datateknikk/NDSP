//----------------------//
// Norsk Datateknikk AS //
//----------------------//

use crate::*;
use crate::traits;

use mixed_num::traits::*;

pub mod ops;
pub use ops::*;

pub mod math_impl;

pub mod window;

pub mod complex;
pub use complex::*;


#[cfg(any(feature = "std", test))]
pub mod plot;

extern crate alloc;
use alloc::string::ToString;

extern crate num;

use core::fmt;
use alloc::string::String;

/// Numeric vector of real, complex, fixed or floating-point numbers.
/// 
/// ## Example
/// 
/// ```
/// use ndsp::*;
/// 
/// let signalf32 = Vec::lin_range(0f32, 9f32, 10);
/// let signalf64 = Vec::lin_range(2f64, 11f64, 10);
///
/// let result = signalf32.clone()*signalf64;
/// assert_eq!(result.to_string(), "[ 0, 3, 8, 15, 24, 35, 48, 63, 80, 99 ]" ); 
/// ```
/// 
/// This vec simulataneously support complex numbers, and operations with real ones.
/// 
/// ## Example
///
/// ```
/// use ndsp::*;
/// use mixed_num::*;
/// 
/// let omega = <f32>::mixed_pi()/f32::mixed_from_num(8i32);
/// let theta = 0f32; 
/// 
/// let mut signal = Vec::osc(omega, theta, 4);
/// signal = &signal*&signal;
/// assert_eq!(signal.to_string(), "[ 1+0i, 0.7071067+0.7071068i, 0+0.99999994i, -0.7071067+0.7071068i ]" );
/// 
/// signal /= 2f32;
/// assert_eq!(signal.to_string(), "[ 0.25-0.25i, 0.35355338+0.000000029802322i, 0.24999999+0.24999999i, 0.000000029802322+0.35355338i ]" )
/// ```

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
    /// ```
    /// use ndsp::*;
    /// use mixed_num::*;
    /// 
    /// // Allocate memory for vec.
    /// let mut vec = Vec::<f32>::new_with_capacity(4);
    /// 
    /// // Add values to the vec.
    /// vec.push_back(2f32);
    /// vec.push_back(1f32);
    /// vec.push_back(4f32);
    /// vec.push_back(4f32);
    /// 
    /// assert_eq!(vec.to_string(), "[ 2, 1, 4, 4 ]" );
    /// ```
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

impl <T: MixedReal + MixedNumConversion<T2>, T2: MixedReal + MixedNumConversion<usize>> ToTouples<T2> for Vec<T> {
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
            let tuple = (T2::mixed_from_num(idx), self[idx].mixed_to_num());
            outvec.push(tuple);
        }
        return outvec;
    }

    /// Load two vectors self, y_vec into touple, (x,y) for plotting.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let x_vec = Vec::lin_range(0f32, 3f32, 4);
    /// let y_vec = Vec::lin_range(0f32, 6f32, 4);
    /// let touple_vec = x_vec.to_xy_touples(&y_vec);
    /// assert_eq!(touple_vec[1], (1f32,2f32) );
    /// ```
    fn to_xy_touples( &self, y_vec: &Self ) -> alloc::vec::Vec<(T2, T2)>
    {
        let mut outvec = alloc::vec::Vec::<(T2, T2)>::new();
        for idx in 0..self.len() {
            let tuple = (self[idx].mixed_to_num(), y_vec[idx].mixed_to_num());
            outvec.push(tuple);
        }
        return outvec;
    }
}

impl <T1: MixedNum + MixedNumConversion<T2>, T2: MixedNum> VecNumConversion<T1, T2> for Vec<T1> 
{
    /// Create a `Vec<T1>` form `Vec<T2>`
    fn vec_from_num( in_vec: &Vec<T2>) -> Self
    {
        let mut out_vec = Vec::<T1>::new_with_capacity(in_vec.len());
        for idx in 0..in_vec.len()
        {
            out_vec.push_back(T1::mixed_from_num(in_vec[idx]));
        }
        return out_vec;
    }
    // Create a `Vec<T2>` form `Vec<T1>`
    fn vec_to_num(&self) -> Vec<T2>
    {
        let mut out_vec = Vec::<T2>::new_with_capacity(self.len());
        for idx in 0..self.len()
        {
            out_vec.push_back(self[idx].mixed_to_num());
        }
        return out_vec;
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
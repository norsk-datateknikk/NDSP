//----------------------//
// Norsk Datateknikk AS //
//----------------------//


extern crate alloc;

use num::Complex;
use mixed_num::traits::*;

use crate::traits;
use crate::traits::*;
use crate::vec::*;

/*
#[cfg(any(feature = "std"))]
use std::fs::File;
#[cfg(any(feature = "std"))]
use std::io::{BufReader, Read};
*/

impl <T: MixedNum> Vec<T> {
    
}

impl <T: MixedNum + MixedNumConversion<T2>, T2: MixedNum> ToTouples<T2> for Vec<T> {
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

impl <T: MixedNum> LinRange<T> for Vec<T>
{
    /// Returns a 1D vector of evenly spaced numbers of type T.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let test_vec = Vec::lin_range(0f32, 3f32, 4);
    /// assert_eq!(test_vec.to_string(), "[ 0, 1, 2, 3 ]" )
    /// ```
    #[allow(dead_code)]
    fn lin_range( start:T, stop:T, num:usize ) -> Vec<T> {
        let step = (stop-start)/T::mixed_from_num((num-1) as i32);

        let mut vector = Vec::<T>::new_with_capacity(num);
        
        let mut val = start;
        for _idx in 0..vector.capacity()
        {
            vector.push_back(val);
            val += step;
        }
        return vector;
    }
} 


impl <T: MixedTrigonometry> traits::Sin for Vec<T> {
    /// Take the elemtent-wise sine using a Taylor approximation of sin(x).
    /// Self must be wrapped to the -π=<x<π range.
    fn sin(&mut self) {
        for idx in 0..self.len() {
            self[idx] = self[idx].mixed_sin();
        }
    }
}

impl <T: MixedTrigonometry> traits::Cos for Vec<T> {
    /// Take the elemtent-wise cosine using a shifted Taylor approximation of sin(x).
    /// Self must be wrapped to the -π=<x<π range.
    fn cos(&mut self) {
        for idx in 0..self.len() {
            self[idx] = self[idx].mixed_cos();
        }
    }
}

impl <T: MixedWrapPhase> traits::WrapPhase for Vec<T> {
    /// Wrapps θ to the -π=<x<π range.
    /// 
    /// ## Arguments 
    ///
    /// * `phi` - The unwrapped phase in radians.
    /// 
    fn wrap_phase(&mut self) {
        for idx in 0..self.len() {
            self[idx] = self[idx].mixed_wrap_phase();
        }
    }
}

impl <T: MixedSqrt> traits::Sqrt for Vec<T> {
    /// Take the element-wise square root.
    fn sqrt(&mut self) {
        for idx in 0..self.len() {
            self[idx] = self[idx].mixed_sqrt();
        }
    }
}

impl <T: MixedNum + MixedNumSigned> traits::Abs<R> for Vec<T> {
    /// Take the elemtent-wise absolute value.
    fn abs(&mut self) {
        for idx in 0..self.len() {
            if self[idx]< T::mixed_from_num(0)
            {
                self[idx]=-self[idx];
            }
        }
    }
}

impl <T: MixedNum> traits::AsComplex<T> for Vec<T> {
    /// Returns the real part of the vector as a real only vector.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let test_vec = Vec::lin_range(0f32, 3f32, 4);
    /// assert_eq!(test_vec.as_complex().to_string(), "[ 0+0i, 1+0i, 2+0i, 3+0i ]" )
    /// ```
    fn as_complex(&self) -> Vec<Complex<T>>
    {
        let len = *&self.len();
        let mut r_vec = Vec::<Complex<T>>::new_with_capacity(len);
        for i in 0..len
        {
            r_vec.push_back( Complex::new( self[i], T::mixed_from_num(0)));
        }
        return r_vec;
    }
}

impl <T: MixedNum> traits::Max<T> for Vec<T> {
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let test_vec = Vec::lin_range(0f32, 3f32, 4);
    /// assert_eq!(test_vec.max(), 3f32 )
    /// ```
    fn max(&self) -> T
    {
        let len = *&self.len();
        let mut value = T::mixed_min_value();
        for i in 0..len
        {
            if value < self[i]
            {
                value = self[i]
            }
        }
        return value;
    }
}

impl <T: MixedNum> traits::Min<T> for Vec<T> {
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let test_vec = Vec::lin_range(0f32, 3f32, 4);
    /// assert_eq!(test_vec.min(), 0f32 )
    /// ```
    fn min(&self) -> T
    {
        let len = *&self.len();
        let mut value = T::mixed_max_value();
        for i in 0..len
        {
            if self[i] < value
            {
                value = self[i]
            }
        }
        return value;
    }
}

impl <T: MixedNum> traits::MinMax<T> for Vec<T> {
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let test_vec = Vec::lin_range(0f32, 3f32, 4);
    /// assert_eq!(test_vec.minmax(), (0f32,3f32) )
    /// ```
    fn minmax(&self) -> (T,T)
    {
        let len = *&self.len();
        let mut max_value = T::mixed_min_value();
        let mut min_value = T::mixed_max_value();
        for i in 0..len
        {
            if self[i] < min_value
            {
                min_value = self[i]
            }
            if max_value < self[i]
            {
                max_value = self[i]
            }
        }
        return (min_value, max_value);
    }
}

impl <T: MixedNum> ToRange<T> for Vec<T>{
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let test_vec = Vec::lin_range(2f32, 5f32, 4);
    /// assert_eq!(test_vec.to_range(), 2f32..5f32 )
    /// ```
    #[inline]
    fn to_range( &self ) -> core::ops::Range<T>
    {
        let (min_value, max_value) = self.minmax();
        return core::ops::Range{start: min_value, end: max_value};
    }
}

impl <T: MixedNum> Indices<T> for Vec<T> {
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let test_vec = Vec::lin_range(2f32, 5f32, 4);
    /// assert_eq!(test_vec.indices().to_string(), "[ 0, 1, 2, 3 ]" )
    /// ```
    #[inline(always)]
    fn indices( &self ) -> Vec<T>
    {
        return Self::lin_range(T::mixed_zero(), T::mixed_from_num((self.len()-1usize) as u32), self.len());
    }
}

/*
#[cfg(any(feature = "std"))]
impl <T> traits::FromBinary for Vec<T>
    where T: MixedNum
{
    /// Read a binary file from e.g. Gnu Radio Companion into a vector.
    /// Assuming a binary file containing complex32.
    fn from_binary( _item_type: ItemType, path: &str ) -> Self
    {

        let file = File::open(path).expect("file wasn't found.");
        let mut reader = BufReader::new(&file);

        let file_size_bytes = &file.metadata().unwrap().len();

        // Currently only float32 and complex32 is supported.
        const ITEM_SIZE_BYTES:usize = 4;
        
        let mut vec = Self::new_with_capacity(*file_size_bytes as usize/ITEM_SIZE_BYTES);
        
        // Counter to keep track of I/Q sample. Even = I, odd = Q.
        let mut counter:usize = 0;

        let mut temp_complex = Complex::new(T::mixed_from_num(0), T::mixed_from_num(0));

        loop {
            use std::io::ErrorKind;
            let mut buffer = [0u8; std::mem::size_of::<f32>()];
            
            let res = reader.read_exact(&mut buffer);
            match res {
                Err(error) if error.kind() == ErrorKind::UnexpectedEof => break,
                _ => {}
            }
            res.expect("Unexpected error during read");

            if &counter%2==0
            {
                // Use `from_be_bytes` if numbers in file is big-endian
                temp_complex.re = T::mixed_from_num(f32::from_le_bytes(buffer));
            }
            else {
                // Use `from_be_bytes` if numbers in file is big-endian
                temp_complex.im = T::mixed_from_num(f32::from_le_bytes(buffer));
                vec.push_back(temp_complex);
            }
            counter +=1;
        }

        return vec;
    }
}
*/

// We prefer doctests, as they provide documentation additionally.
#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test() {
        assert_eq!(true, true )
    }
}
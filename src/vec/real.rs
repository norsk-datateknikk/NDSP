//----------------------//
// Norsk Datateknikk AS //
//----------------------//


extern crate alloc;

use num::Complex;
use mixed_num::traits::*;

use crate::traits;
use crate::traits::*;
use crate::vec::*;

impl <T: MixedNum> Vec<T> {
    
}

impl <T: MixedNum> LinRange<T> for Vec<T>
{
    /// Returns a 1D vector of evenly spaced numbers of type T.
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
    fn max(&self) -> T
    {
        let len = *&self.len();
        let mut max_value = T::mixed_min_value();
        for i in 0..len
        {
            if max_value < self[i]
            {
                max_value = self[i]
            }
        }
        return max_value;
    }
}

impl <T: MixedNum> traits::Min<T> for Vec<T> {
    fn min(&self) -> T
    {
        let len = *&self.len();
        let mut max_value = T::mixed_max_value();
        for i in 0..len
        {
            if self[i] < max_value
            {
                max_value = self[i]
            }
        }
        return max_value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn linrange() {
        let test_vec = Vec::lin_range(0f32, 3f32, 4);
        assert_eq!(test_vec.to_string(), "[ 0, 1, 2, 3 ]" )
    }

    #[test]
    fn max() {
        let test_vec = Vec::lin_range(0f32, 3f32, 4);
        assert_eq!(test_vec.max(), 3f32 )
    }

    #[test]
    fn min() {
        let test_vec = Vec::lin_range(0f32, 3f32, 4);
        assert_eq!(test_vec.min(), 0f32 )
    }

    #[test]
    fn as_complex() {
        let test_vec = Vec::lin_range(0f32, 3f32, 4);
        assert_eq!(test_vec.as_complex().to_string(), "[ 0+0i, 1+0i, 2+0i, 3+0i ]" )
    }
}
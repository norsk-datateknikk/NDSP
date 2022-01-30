//-----------------------------------------------------------------//
// Copyright 2021 Norsk Datateknikk AS                             //
//-----------------------------------------------------------------//
// This file is subject to the terms and conditions defined in the //
// file 'LICENSE', which is part of this source code package.      //
//-----------------------------------------------------------------//


extern crate alloc;
use mixed_num::traits::*;

use crate::traits;
use crate::traits::*;
use crate::vec::*;

impl <T: MixedNum> Vec<T> {
    /// Returns a 1D vector of evenly spaced numbers of type T.
    #[allow(dead_code)]
    fn linspace( start:T, stop:T, num:usize, end_point: bool ) -> Vec<T> {
        let temp_num;
        if end_point
        {
            temp_num = num;
        }
        else
        {
            temp_num = num+1
        }

        let step = (stop-start)/T::mixed_from_num(temp_num as i32);

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
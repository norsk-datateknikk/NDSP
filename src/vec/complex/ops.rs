use crate::*;
use mixed_num::traits::*;
use num::Complex;

macro_rules! impl_mixed_num_for_primitive{
    ( $T:ty ) => {

        impl <T: MixedNum + MixedNumSigned + MixedTrigonometry + MixedSqrt + MixedWrapPhase> core::ops::Mul<&'a Vec<Complex<T>>> for Vec<Complex<T>> {
            type Output = Self;
            fn mul(self, rhs: &Self) -> Self {
        
                if rhs.len() != self.len()
                {
                    core::panic!("Vectors must be of equal size!");
                }
        
                let mut outvec = self.clone();
                for idx in 0..self.len() {
                    outvec[idx] = complex::mul_cartesian(outvec[idx], rhs[idx])
                }
                return outvec;
            }
        }

        impl <T: MixedNum + MixedNumSigned + MixedTrigonometry + MixedSqrt + MixedWrapPhase> core::ops::Mul<Complex<T>> for Vec<Complex<T>> {
            type Output = Self;
            fn mul(self, rhs: T) -> Self {
        
                if rhs.len() != self.len()
                {
                    core::panic!("Vectors must be of equal size!");
                }
        
                let mut outvec = self.clone();
                for idx in 0..self.len() {
                    outvec[idx] = complex::mul_cartesian(outvec[idx], rhs[idx])
                }
                return outvec;
            }
        }
        
        impl <T: MixedNum + MixedNumSigned + MixedTrigonometry + MixedSqrt + MixedWrapPhase> core::ops::MulAssign<&Vec<Complex<T>>> for Vec<Complex<T>> {
            fn mul_assign(&mut self, rhs: &Self){
        
                if rhs.len() != self.len()
                {
                    core::panic!("Vectors must be of equal size!");
                }
        
                for idx in 0..self.len() {
                    self[idx] = complex::mul_cartesian(self[idx], rhs[idx])
                }
            }
        }
        
        impl <T: MixedNum + MixedNumSigned + MixedTrigonometry + MixedSqrt + MixedWrapPhase> core::ops::Add<&Vec<Complex<T>>> for Vec<Complex<T>> {
            type Output = Self;
            fn add(self, rhs: &Self) -> Self {
        
                if rhs.len() != self.len()
                {
                    core::panic!("Vectors must be of equal size!");
                }
        
                let mut outvec = self.clone();
                for idx in 0..self.len() {
                    outvec[idx] = complex::add(outvec[idx], rhs[idx])
                }
                return outvec;
            }
        }
        
        impl <T: MixedNum + MixedNumSigned + MixedTrigonometry + MixedSqrt + MixedWrapPhase> core::ops::AddAssign<&Vec<Complex<T>>> for Vec<Complex<T>> {
            fn add_assign(&mut self, rhs: &Self){
        
                if rhs.len() != self.len()
                {
                    core::panic!("Vectors must be of equal size!");
                }
        
                for idx in 0..self.len() {
                    self[idx] = complex::add(self[idx], rhs[idx])
                }
            }
        }
    }
}

impl_mixed_num_for_primitive!(f32);
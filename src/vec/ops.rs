use crate::*;
use mixed_num::*;

use core::ops;

impl <T1: MixedNum + MixedNumConversion<T2>, T2: MixedNum + ops::Mul<Output = T2>> ops::Mul<Vec<T1>> for Vec<T2> {
    type Output = Vec<T2>;

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
    /// 
    /// let signal1f32 = Vec::lin_range(2f64, 11f64, 10);
    ///
    /// let result = signalf32*signal1f32;
    /// assert_eq!(result.to_string(), "[ 0, 3, 8, 15, 24, 35, 48, 63, 80, 99 ]" );
    /// ```
    /// 
    /// This implementation simulataneously support complex numbers.
    /// 
    /// ## Example
    /// ```
    /// use ndsp::*;
    /// use mixed_num::*;
    /// 
    /// let omega = <f32>::mixed_pi()/f32::mixed_from_num(8i32);
    /// let theta = 0f32; 
    /// 
    /// let mut signal = Vec::osc(omega, theta, 4);
    /// signal = signal.clone()*signal.clone();
    /// assert_eq!(signal.to_string(), "[ 1+0i, 0.7071067+0.7071068i, 0+0.99999994i, -0.7071067+0.7071068i ]" )
    /// ```
    fn mul(self, rhs: Vec<T1>) -> Vec<T2> {

        if rhs.len() != self.len()
        {
            core::panic!("Vectors must be of equal size!");
        }

        let mut outvec = Vec::<T2>::new_with_capacity(self.len());
        for idx in 0..self.len() {
            outvec.push_back(self[idx]* rhs[idx].mixed_to_num());
        }
        return outvec;
    }
}

impl <T1: MixedNum + MixedNumConversion<T2>, T2: MixedNum + ops::Mul<Output = T2>> ops::Mul<&Vec<T1>> for &Vec<T2> {
    type Output = Vec<T2>;

    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// 
    /// let signalf32 = Vec::lin_range(0f32, 9f32, 10);
    /// let signalf64 = Vec::lin_range(2f64, 11f64, 10);
    ///
    /// let result = &signalf32*&signalf64;
    /// assert_eq!(result.to_string(), "[ 0, 3, 8, 15, 24, 35, 48, 63, 80, 99 ]" );
    /// 
    /// let signal1f32 = Vec::lin_range(2f64, 11f64, 10);
    ///
    /// let result = &signalf32*&signal1f32;
    /// assert_eq!(result.to_string(), "[ 0, 3, 8, 15, 24, 35, 48, 63, 80, 99 ]" );
    /// ```
    /// 
    /// This implementation simulataneously support complex numbers.
    /// 
    /// ## Example
    /// ```
    /// use ndsp::*;
    /// use mixed_num::*;
    /// 
    /// let omega = <f32>::mixed_pi()/f32::mixed_from_num(8i32);
    /// let theta = 0f32; 
    /// 
    /// let mut signal = Vec::osc(omega, theta, 4);
    /// signal = &signal*&signal;
    /// assert_eq!(signal.to_string(), "[ 1+0i, 0.7071067+0.7071068i, 0+0.99999994i, -0.7071067+0.7071068i ]" )
    /// ```
    fn mul(self, rhs: &Vec<T1>) -> Vec<T2> {

        if rhs.len() != self.len()
        {
            core::panic!("Vectors must be of equal size!");
        }

        let mut outvec = Vec::<T2>::new_with_capacity(self.len());
        for idx in 0..self.len() {
            outvec.push_back(self[idx]* rhs[idx].mixed_to_num());
        }
        return outvec;
    }
}

impl <T1: MixedNum + MixedNumConversion<T2>, T2: MixedNum + ops::Mul<Output = T2>> ops::Mul<T1> for Vec<T2> {
    type Output = Vec<T2>;
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// 
    /// let signal0 = Vec::lin_range(0f32, 9f32, 10);
    ///
    /// let result = signal0*2f64;
    /// assert_eq!(result.to_string(), "[ 0, 2, 4, 6, 8, 10, 12, 14, 16, 18 ]" )
    /// 
    /// ```
    fn mul(self, rhs: T1) -> Self {
        let mut outvec = Vec::<T2>::new_with_capacity(self.len());
        let rhs = rhs.mixed_to_num();
        for idx in 0..self.len() {
            outvec.push_back(self[idx]* rhs);
        }
        return outvec;
    }
}

impl <T1: MixedNum + MixedNumConversion<T2>, T2: MixedNum + ops::Mul<Output = T2>> ops::MulAssign<Vec<T1>> for Vec<T2> {
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// 
    /// let mut signalf32 = Vec::lin_range(0f32, 9f32, 10);
    /// let     signalf64 = Vec::lin_range(2f64, 11f64, 10);
    ///
    /// signalf32 *= signalf64;
    /// assert_eq!(signalf32.to_string(), "[ 0, 3, 8, 15, 24, 35, 48, 63, 80, 99 ]" );
    /// ```
    fn mul_assign(&mut self, rhs: Vec<T1>){

        if rhs.len() != self.len()
        {
            core::panic!("Vectors must be of equal size!");
        }

        for idx in 0..self.len() {
            self[idx] = self[idx]* rhs[idx].mixed_to_num();
        }
    }
}

impl <T1: MixedNum + MixedNumConversion<T2>, T2: MixedNum + ops::Mul<Output = T2>> ops::MulAssign<&Vec<T1>> for Vec<T2> {
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// 
    /// let mut signalf32 = Vec::lin_range(0f32, 9f32, 10);
    /// let     signalf64 = Vec::lin_range(2f64, 11f64, 10);
    ///
    /// signalf32 *= &signalf64;
    /// assert_eq!(signalf32.to_string(), "[ 0, 3, 8, 15, 24, 35, 48, 63, 80, 99 ]" );
    /// ```
    fn mul_assign(&mut self, rhs: &Vec<T1>){

        if rhs.len() != self.len()
        {
            core::panic!("Vectors must be of equal size!");
        }

        for idx in 0..self.len() {
            self[idx] = self[idx]* rhs[idx].mixed_to_num();
        }
    }
}

impl <T1: MixedNum + MixedNumConversion<T2>, T2: MixedNum + ops::MulAssign> ops::MulAssign<T1> for Vec<T2> {
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// 
    /// let mut signalf32 = Vec::lin_range(0f32, 9f32, 10);
    ///
    /// signalf32 *= 2f64;
    /// assert_eq!(signalf32.to_string(), "[ 0, 2, 4, 6, 8, 10, 12, 14, 16, 18 ]" );
    /// ```
    fn mul_assign(&mut self, rhs: T1){
        let rhs = rhs.mixed_to_num();
        for idx in 0..self.len() {
            self[idx] *= rhs;
        }
    }
}

impl <T1: MixedNum + MixedNumConversion<T2>, T2: MixedNum + ops::Add<Output = T2>> ops::Add<Vec<T1>> for Vec<T2> {
    type Output = Self;
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// 
    /// let mut signalf32 = Vec::lin_range(0f32, 9f32, 10);
    /// let     signalf64 = Vec::lin_range(0f64, 9f64, 10);
    ///
    /// signalf32 = signalf32.clone() + signalf64;
    /// assert_eq!(signalf32.to_string(), "[ 0, 2, 4, 6, 8, 10, 12, 14, 16, 18 ]" );
    /// ```
    fn add(self, rhs: Vec<T1>) -> Self {

        if rhs.len() != self.len()
        {
            core::panic!("Vectors must be of equal size!");
        }

        let mut outvec = self.clone();
        for idx in 0..self.len() {
            outvec[idx] = outvec[idx]+ rhs[idx].mixed_to_num();
        }
        return outvec;
    }
}

impl <T1: MixedNum + MixedNumConversion<T2>, T2: MixedNum + ops::Add<Output = T2>> ops::Add<&Vec<T1>> for &Vec<T2> {
    type Output = Vec<T2>;
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// 
    /// let mut signalf32 = Vec::lin_range(0f32, 9f32, 10);
    /// let     signalf64 = Vec::lin_range(0f64, 9f64, 10);
    ///
    /// signalf32 = &signalf32 + &signalf64;
    /// assert_eq!(signalf32.to_string(), "[ 0, 2, 4, 6, 8, 10, 12, 14, 16, 18 ]" );
    /// ```
    fn add(self, rhs: &Vec<T1>) -> Vec<T2> {

        if rhs.len() != self.len()
        {
            core::panic!("Vectors must be of equal size!");
        }

        let mut outvec = Vec::<T2>::new_with_capacity(self.len());
        for idx in 0..self.len() {
            outvec.push_back(self[idx] + rhs[idx].mixed_to_num());
        }
        return outvec;
    }
}

impl <T1: MixedNum + MixedNumConversion<T2>, T2: MixedNum + ops::Add<Output = T2>> ops::Add<T1> for Vec<T2> {
    type Output = Self;
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// 
    /// let signalf32 = Vec::lin_range(0f32, 9f32, 10);
    ///
    /// let signalf32 = signalf32 + 2f64;
    /// assert_eq!(signalf32.to_string(), "[ 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 ]" );
    /// ```
    fn add(self, rhs: T1) -> Self {
        let rhs = rhs.mixed_to_num();
        let mut outvec = self.clone();
        for idx in 0..self.len() {
            outvec[idx] = outvec[idx]+ rhs;
        }
        return outvec;
    }
}


impl <T1: MixedNum + MixedNumConversion<T2>, T2: MixedNum + ops::Add<Output = T2>> ops::Add<T1> for &Vec<T2> {
    type Output = Vec<T2>;
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// 
    /// let signalf32 = Vec::lin_range(0f32, 9f32, 10);
    ///
    /// let signalf32 = &signalf32 + 2f64;
    /// assert_eq!(signalf32.to_string(), "[ 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 ]" );
    /// ```
    fn add(self, rhs: T1) -> Vec<T2> {
        let rhs = rhs.mixed_to_num();
        let mut outvec = self.clone();
        for idx in 0..self.len() {
            outvec[idx] = outvec[idx]+ rhs;
        }
        return outvec;
    }
}

impl <T1: MixedNum + MixedNumConversion<T2>, T2: MixedNum + ops::AddAssign> ops::AddAssign<Vec<T1>> for Vec<T2> {
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// 
    /// let mut signalf32 = Vec::lin_range(0f32, 9f32, 10);
    /// let mut signalf64 = Vec::lin_range(0f64, 9f64, 10);
    ///
    /// signalf32 += signalf64;
    /// assert_eq!(signalf32.to_string(), "[ 0, 2, 4, 6, 8, 10, 12, 14, 16, 18 ]" );
    /// ```
    fn add_assign(&mut self, rhs: Vec<T1>){
        if rhs.len() != self.len()
        {
            core::panic!("Vectors must be of equal size!");
        }

        for idx in 0..self.len() {
            self[idx] += rhs[idx].mixed_to_num();
        }
    }
}

impl <T1: MixedNum + MixedNumConversion<T2>, T2: MixedNum + ops::AddAssign> ops::AddAssign<&Vec<T1>> for Vec<T2> {
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// 
    /// let mut signalf32 = Vec::lin_range(0f32, 9f32, 10);
    /// let mut signalf64 = Vec::lin_range(0f64, 9f64, 10);
    ///
    /// signalf32 += &signalf64;
    /// assert_eq!(signalf32.to_string(), "[ 0, 2, 4, 6, 8, 10, 12, 14, 16, 18 ]" );
    /// ```
    fn add_assign(&mut self, rhs: &Vec<T1>){
        if rhs.len() != self.len()
        {
            core::panic!("Vectors must be of equal size!");
        }

        for idx in 0..self.len() {
            self[idx] += rhs[idx].mixed_to_num();
        }
    }
}

impl <T1: MixedNum + MixedNumConversion<T2>, T2: MixedNum + ops::AddAssign> ops::AddAssign<T1> for Vec<T2> {
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// 
    /// let mut signalf32 = Vec::lin_range(0f32, 9f32, 10);
    ///
    /// signalf32 += 2f64;
    /// assert_eq!(signalf32.to_string(), "[ 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 ]" );
    /// ```
    fn add_assign(&mut self, rhs: T1){
        let rhs = rhs.mixed_to_num();
        for idx in 0..self.len() {
            self[idx] += rhs;
        }
    }
}
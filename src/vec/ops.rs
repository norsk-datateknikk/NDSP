use crate::*;
use mixed_num::*;

use core::ops;

impl <T> ops::Index<usize> for Vec<T> {
    type Output = T;
    /// Trait for returning an indexed value of the array.
    #[inline]
    fn index(&self, index: usize) -> &T {
        return &self.vec[index];
    }
}

impl <T> ops::IndexMut<usize> for Vec<T> {
    /// Trait for returning a mutable reference to indexed item.
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut T {
        return &mut self.vec[index];
    }
}

macro_rules! impl_assign_ops_for_vec {
    ($trait:tt, $fn:tt, $symb:tt) => {
        impl <T1: MixedNum + MixedNumConversion<T2>, T2: MixedNum + ops::$trait<Output = T2>> ops::$trait<Vec<T1>> for Vec<T2> {
            type Output = Self;
            fn $fn(self, rhs: Vec<T1>) -> Self {
        
                if rhs.len() != self.len()
                {
                    core::panic!("Vectors must be of equal size!");
                }
        
                let mut outvec = self.clone();
                for idx in 0..self.len() {
                    outvec[idx] = outvec[idx] $symb rhs[idx].mixed_to_num();
                }
                return outvec;
            }
        }
        
        impl <T1: MixedNum + MixedNumConversion<T2>, T2: MixedNum + ops::$trait<Output = T2>> ops::$trait<&Vec<T1>> for &Vec<T2> {
            type Output = Vec<T2>;
            fn $fn(self, rhs: &Vec<T1>) -> Vec<T2> {
        
                if rhs.len() != self.len()
                {
                    core::panic!("Vectors must be of equal size!");
                }
        
                let mut outvec = Vec::<T2>::new_with_capacity(self.len());
                for idx in 0..self.len() {
                    outvec.push_back(self[idx] $symb rhs[idx].mixed_to_num());
                }
                return outvec;
            }
        }
        
        impl <T1: MixedNum + MixedNumConversion<T2>, T2: MixedNum + ops::$trait<Output = T2>> ops::$trait<T1> for Vec<T2> {
            type Output = Self;
            fn $fn(self, rhs: T1) -> Self {
                let rhs = rhs.mixed_to_num();
                let mut outvec = self.clone();
                for idx in 0..self.len() {
                    outvec[idx] = outvec[idx] $symb rhs;
                }
                return outvec;
            }
        }
        
        impl <T1: MixedNum + MixedNumConversion<T2>, T2: MixedNum + ops::$trait<Output = T2>> ops::$trait<T1> for &Vec<T2> {
            type Output = Vec<T2>;
            fn $fn(self, rhs: T1) -> Vec<T2> {
                let rhs = rhs.mixed_to_num();
                let mut outvec = self.clone();
                for idx in 0..self.len() {
                    outvec[idx] = outvec[idx] $symb rhs;
                }
                return outvec;
            }
        }
    }
}

impl_assign_ops_for_vec!(Mul, mul , *);
impl_assign_ops_for_vec!(Div, div , /);
impl_assign_ops_for_vec!(Add, add , +);
impl_assign_ops_for_vec!(Sub, sub , -);


macro_rules! impl_assign_ops_assign_for_vec {
    ($trait:tt, $fn:tt, $symb:tt) => {
        impl <T1: MixedNum + MixedNumConversion<T2>, T2: MixedNum + ops::$trait> ops::$trait<Vec<T1>> for Vec<T2> {
            fn $fn(&mut self, rhs: Vec<T1>){
                if rhs.len() != self.len()
                {
                    core::panic!("Vectors must be of equal size!");
                }
        
                for idx in 0..self.len() {
                    self[idx] $symb rhs[idx].mixed_to_num();
                }
            }
        }

        impl <T1: MixedNum + MixedNumConversion<T2>, T2: MixedNum + ops::$trait> ops::$trait<&Vec<T1>> for Vec<T2> {
            fn $fn(&mut self, rhs: &Vec<T1>){
                if rhs.len() != self.len()
                {
                    core::panic!("Vectors must be of equal size!");
                }
        
                for idx in 0..self.len() {
                    self[idx] $symb rhs[idx].mixed_to_num();
                }
            }
        }

        impl <T1: MixedNum + MixedNumConversion<T2>, T2: MixedNum + ops::$trait> ops::$trait<T1> for Vec<T2> {
            fn $fn(&mut self, rhs: T1){
                let rhs = rhs.mixed_to_num();
                for idx in 0..self.len() {
                    self[idx] $symb rhs;
                }
            }
        }

    }
}

impl_assign_ops_assign_for_vec!(MulAssign, mul_assign , *=);
impl_assign_ops_assign_for_vec!(DivAssign, div_assign , /=);
impl_assign_ops_assign_for_vec!(AddAssign, add_assign , +=);
impl_assign_ops_assign_for_vec!(SubAssign, sub_assign , -=);

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::vec::*;
     
    #[test]
    fn mul() {
        //____________________________________________________________________________
        let signal0 = Vec::lin_range(0f32, 9f32, 10);

        let result = signal0*2f64;
        assert_eq!(result.to_string(), "[ 0, 2, 4, 6, 8, 10, 12, 14, 16, 18 ]" );
        //____________________________________________________________________________
        let signalf32 = Vec::lin_range(0f32, 9f32, 10);
        let signalf64 = Vec::lin_range(2f64, 11f64, 10);

        let result = &signalf32*&signalf64;
        assert_eq!(result.to_string(), "[ 0, 3, 8, 15, 24, 35, 48, 63, 80, 99 ]" );
        
        let signal1f32 = Vec::lin_range(2f64, 11f64, 10);

        let result = &signalf32*&signal1f32;
        assert_eq!(result.to_string(), "[ 0, 3, 8, 15, 24, 35, 48, 63, 80, 99 ]" );

        let omega = <f32>::mixed_pi()/f32::mixed_from_num(8i32);
        let theta = 0f32; 
        
        let mut signal = Vec::osc(omega, theta, 4);
        signal = &signal*&signal;
        assert_eq!(signal.to_string(), "[ 1+0i, 0.7071067+0.7071068i, 0+0.99999994i, -0.7071067+0.7071068i ]" )
        //____________________________________________________________________________
    }

    #[test]
    fn add() {
        //____________________________________________________________________________
        let signalf32 = Vec::lin_range(0f32, 9f32, 10);
    
        let signalf32 = &signalf32 + 2f64;
        assert_eq!(signalf32.to_string(), "[ 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 ]" );

        //____________________________________________________________________________
        let signalf32 = Vec::lin_range(0f32, 9f32, 10);
    
        let signalf32 = signalf32 + 2f64;
        assert_eq!(signalf32.to_string(), "[ 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 ]" );

        //____________________________________________________________________________

        let mut signalf32 = Vec::lin_range(0f32, 9f32, 10);
        let     signalf64 = Vec::lin_range(0f64, 9f64, 10);
    
        signalf32 = &signalf32 + &signalf64;
        assert_eq!(signalf32.to_string(), "[ 0, 2, 4, 6, 8, 10, 12, 14, 16, 18 ]" );
        //____________________________________________________________________________
    }

    #[test]
    fn add_assign() {
        //____________________________________________________________________________

        let mut signalf32 = Vec::lin_range(0f32, 9f32, 10);
        let     signalf64 = Vec::lin_range(0f64, 9f64, 10);

        signalf32 += signalf64;
        assert_eq!(signalf32.to_string(), "[ 0, 2, 4, 6, 8, 10, 12, 14, 16, 18 ]" );
        //____________________________________________________________________________

        let mut signalf32 = Vec::lin_range(0f32, 9f32, 10);
        let     signalf64 = Vec::lin_range(0f64, 9f64, 10);

        signalf32 += &signalf64;
        assert_eq!(signalf32.to_string(), "[ 0, 2, 4, 6, 8, 10, 12, 14, 16, 18 ]" );
        //____________________________________________________________________________

        let mut signalf32 = Vec::lin_range(0f32, 9f32, 10);
    
        signalf32 += 2f64;
        assert_eq!(signalf32.to_string(), "[ 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 ]" );
    }

    #[test]
    fn mul_assign() {
        //____________________________________________________________________________
        let mut signalf32 = Vec::lin_range(0f32, 9f32, 10);
        let     signalf64 = Vec::lin_range(2f64, 11f64, 10);

        signalf32 *= &signalf64;
        assert_eq!(signalf32.to_string(), "[ 0, 3, 8, 15, 24, 35, 48, 63, 80, 99 ]" );
        //____________________________________________________________________________

        let mut signalf32 = Vec::lin_range(0f32, 9f32, 10);

        signalf32 *= 2f64;
        assert_eq!(signalf32.to_string(), "[ 0, 2, 4, 6, 8, 10, 12, 14, 16, 18 ]" );
        //____________________________________________________________________________

        let mut signalf32 = Vec::lin_range(0f32, 9f32, 10);
        let     signalf64 = Vec::lin_range(2f64, 11f64, 10);

        signalf32 *= signalf64;
        assert_eq!(signalf32.to_string(), "[ 0, 3, 8, 15, 24, 35, 48, 63, 80, 99 ]" );
        //____________________________________________________________________________
    }
}
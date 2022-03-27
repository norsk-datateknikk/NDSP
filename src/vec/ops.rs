use crate::*;
use mixed_num::traits::*;

impl <T: MixedNum + core::ops::Mul<Output = T>> core::ops::Mul<Vec<T>> for Vec<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {

        if rhs.len() != self.len()
        {
            core::panic!("Vectors must be of equal size!");
        }

        let mut outvec = self.clone();
        for idx in 0..self.len() {
            outvec[idx] = outvec[idx]* rhs[idx];
        }
        return outvec;
    }
}

impl <T: MixedNum + core::ops::Mul<Output = T>> core::ops::Mul<T> for Vec<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {

        let mut outvec = self.clone();
        for idx in 0..self.len() {
            outvec[idx] = outvec[idx]* rhs;
        }
        return outvec;
    }
}

impl <T: MixedNum + core::ops::Mul<Output = T>> core::ops::MulAssign<Vec<T>> for Vec<T> {
    fn mul_assign(&mut self, rhs: Self){

        if rhs.len() != self.len()
        {
            core::panic!("Vectors must be of equal size!");
        }

        for idx in 0..self.len() {
            self[idx] = self[idx]* rhs[idx];
        }
    }
}

impl <T: std::ops::MulAssign + Copy> core::ops::MulAssign<T> for Vec<T> {
    fn mul_assign(&mut self, rhs: T){

        for idx in 0..self.len() {
            self[idx] *= rhs;
        }
    }
}

impl <T: MixedNum + core::ops::Add<Output = T>> core::ops::Add<Vec<T>> for Vec<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {

        if rhs.len() != self.len()
        {
            core::panic!("Vectors must be of equal size!");
        }

        let mut outvec = self.clone();
        for idx in 0..self.len() {
            outvec[idx] = outvec[idx]+ rhs[idx];
        }
        return outvec;
    }
}

impl <T: MixedNum + core::ops::Add<Output = T>> core::ops::Add<T> for Vec<T> {
    type Output = Self;
    fn add(self, rhs: T) -> Self {

        let mut outvec = self.clone();
        for idx in 0..self.len() {
            outvec[idx] = outvec[idx]+ rhs;
        }
        return outvec;
    }
}

impl <T: MixedNum + core::ops::AddAssign> core::ops::AddAssign<Vec<T>> for Vec<T> {
    fn add_assign(&mut self, rhs: Self){

        if rhs.len() != self.len()
        {
            core::panic!("Vectors must be of equal size!");
        }

        for idx in 0..self.len() {
            self[idx] += rhs[idx];
        }
    }
}

impl <T: MixedNum + core::ops::AddAssign> core::ops::AddAssign<T> for Vec<T> {
    fn add_assign(&mut self, rhs: T){

        for idx in 0..self.len() {
            self[idx] += rhs;
        }
    }
}
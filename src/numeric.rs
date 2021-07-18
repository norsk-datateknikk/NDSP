//--------------------------------------------------------------//
// Copyright 2021 Norsk Datateknikk AS                          //
//--------------------------------------------------------------//
// This file is subject to the terms and conditions defined in  //
// file 'LICENSE', which is part of this source code package.   //
//--------------------------------------------------------------//

use num::traits::Num;
use num::traits::Float;
use num::traits::real::Real;
use num::Complex;

use crate::sfunc::*;
#[macro_use]
use crate::cvfunc::*;
#[macro_use]
use crate::impl_ops::*;

use std::ops;


#[derive(Clone, Debug, PartialEq)]
/// Numeric vector type
struct VecF32
{
    vec:Vec<f32>
}

#[derive(Clone, Debug, PartialEq)]
/// Numeric vector type
struct VecC32F
{
    vec:Vec<C32F>
}

pub trait BasicTraits {
    fn new( arg_vec:Vec<C32F> ) -> Self;
}

impl BasicTraits for VecC32F {
    fn new( arg_vec:Vec<C32F> ) -> VecC32F{
        VecC32F{
            vec: arg_vec,
        }
    }
}

impl<'a, 'b> std::ops::Add<&'b VecC32F> for &'a VecC32F {
    type Output = VecC32F;

    fn add(self, other: &'b VecC32F) -> VecC32F {
        VecC32F {
            vec: crate::cvfunc::add( self.vec.clone(), other.vec.clone() ),
        }
    }
}

impl std::ops::Index<usize> for VecC32F {
    type Output = C32F;

    #[inline]
    fn index(&self, index: usize) -> &C32F {
        return &self.vec[index];
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use super::*;

    #[test]
    fn cv_trait_add() {
        let vec = vec![ C32F!(2,0), C32F!(0,4), C32F!(-2,0) ];
        let vec_obj = VecC32F::new(vec);
        let sum = &vec_obj+&vec_obj;
        assert_eq!( VecC32F::new(vec![ C32F!(4,0), C32F!(0,8), C32F!(-4,0) ]), sum );
    }

    #[test]
    fn cv_trait_index() {
        let vec = vec![ C32F!(2,0), C32F!(0,4), C32F!(-2,0) ];
        let vec_obj = VecC32F::new(vec);
        assert_eq!( C32F!(0,4), vec_obj[1] );
    }
}
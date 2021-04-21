//--------------------------------------------------------------//
// Copyright 2021 Norsk Datateknikk AS                          //
//--------------------------------------------------------------//
// This file is subject to the terms and conditions defined in  //
// file 'LICENSE', which is part of this source code package.   //
//--------------------------------------------------------------//

use crate::sfunc::*;
#[macro_use]
use crate::cvfunc::*;
#[macro_use]
use crate::impl_ops::*;

use std::ops;


/// Numeric vector type
struct VectorF32
{
    obj:Vec<f32>,
}

/// Numeric vector type
pub struct VectorC32F
{
    pub obj:Vec<C32F>,
}


pub trait Numeric {
    fn new( arg_vec:Vec<C32F> ) -> Self;
}


impl<'a, 'b> ops::Add<&'b VectorC32F> for &'a VectorC32F {
    type Output = VectorC32F;

    fn add(self, other: &'b VectorC32F) -> VectorC32F {
        VectorC32F {
            obj: crate::cvfunc::add(  self.obj, other.obj),
        }
    }
}

impl Numeric for VectorC32F {
    fn new( arg_vec:Vec<C32F> ) -> VectorC32F{
        VectorC32F{
            obj: arg_vec,
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::*;
    use super::*;

    #[test]
    fn cfunc_Add() {
        let vec = vec![ C32F!(2,0), C32F!(0,4), C32F!(-2,0) ];
        let vec_obj = VectorC32F::new(vec);
        let sum = &vec_obj+&vec_obj;
        println!("{:?}", sum.obj);
        //assert_eq!( vec![ C32F!(2,0), C32F!(2,0), C32F!(2,0) ], sum  );
    }
}
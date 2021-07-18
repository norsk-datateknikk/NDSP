//--------------------------------------------------------------//
// Copyright 2021 Norsk Datateknikk AS                          //
//--------------------------------------------------------------//
// This file is subject to the terms and conditions defined in  //
// file 'LICENSE', which is part of this source code package.   //
//--------------------------------------------------------------//

use super::sfunc::*;
use super::vfunc::*;
use super::cvfunc::*;

#[derive(Clone)]
/// Numeric vector type
struct VecF32
{
    vec:Vec<f32>
}

#[derive(Clone)]
/// Numeric vector type
struct VecC32F
{
    vec:Vec<C32F>
}

/*
impl VecF32 {
    fn
}
*/

pub trait VectorOperations {
    fn min(&self) -> f32; //TODO bør ha derivativ returtype.
}

impl VectorOperations for VecF32 {
    fn min(&self) -> f32 {
        return super::vfunc::min( self.vec.clone() );
    }
}

/*
impl<'a, 'b> Add<&'b VecF32> for &'a VecF32 {
    type Output = VecF32;

    fn add(self, other: &'b VecF32) -> VecF32 {
        VecF32 {
            
            
            //x: self.x + other.x,
            //y: self.y + other.y,
        }
    }
}
*/
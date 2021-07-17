//--------------------------------------------------------------//
// Copyright 2021 Norsk Datateknikk AS                          //
//--------------------------------------------------------------//
// This file is subject to the terms and conditions defined in  //
// file 'LICENSE', which is part of this source code package.   //
//--------------------------------------------------------------//

use super::sfunc::*;
use super::vfunc::*;
use super::cvfunc::*;

/// Numeric vector type
struct VecF32
{
    obj:Vec<f32>
}

/// Numeric vector type
struct VecC32F
{
    obj:Vec<C32F>
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
        return super::vfunc::min(self.obj);
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
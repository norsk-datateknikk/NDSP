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


pub trait BasicTraits<T> {
    fn new( arg_vec:Vec<T> ) -> Self;

}

macro_rules! vector_struct {
    ( $Name:ident, $T:ty ) => {
        #[derive(Clone, Debug, PartialEq)]
        /// Numeric vector type
        struct $Name
        {
            vec:Vec<$T>
        }

        
        impl BasicTraits<$T> for $Name {
            fn new( arg_vec:Vec<$T> ) -> $Name {
                $Name{
                    vec: arg_vec,
                }
            }
        }

        impl std::ops::Index<usize> for $Name {
            type Output = $T;
        
            #[inline]
            fn index(&self, index: usize) -> &$T {
                return &self.vec[index];
            }
        }
    };
}

vector_struct!( VecF32, f32 );
vector_struct!( VecF64, f64 );

vector_struct!( VecC32F, C32F );
vector_struct!( VecC64F, C64F );

macro_rules! float_vector_traits{
    ( $Name:ident, $T:ty, $type:ident ) => {
        impl<'a, 'b> std::ops::Add<&'b $Name> for &'a $Name {
            type Output = $Name;
        
            fn add(self, other: &'b $Name) -> $Name {
                $Name {
                    vec: crate::$type::add( self.vec.clone(), other.vec.clone() ),
                }
            }
        }
        
        impl<'a, 'b> std::ops::Mul<&'b $Name> for &'a $Name {
            type Output = $Name;
        
            fn mul(self, other: &'b $Name) -> $Name {
                $Name {
                    vec: crate::$type::mul( self.vec.clone(), other.vec.clone() ),
                }
            }
        }

        impl std::ops::Neg for $Name {   
            type Output = $Name;

            #[inline]
            fn neg( self) -> $Name {
                $Name {
                    vec: crate::$type::neg( self.vec.clone() ),
                }
            }
        }

    };
}

float_vector_traits!( VecC32F, C32F, cvfunc );
float_vector_traits!( VecC64F, C64F, cvfunc );
float_vector_traits!( VecF32,  f32, vfunc );
float_vector_traits!( VecF64,  f64, vfunc );

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
    fn cv_trait_mul() {
        let vec = vec![ C32F!(2,0), C32F!(0,4), C32F!(-2,0) ];
        let vec_obj = VecC32F::new(vec);
        let sum = &vec_obj*&vec_obj;
        assert_eq!( VecC32F::new(vec![ C32F!(4,0), C32F!(-16,0), C32F!(4,0) ]), sum );
    }

    #[test]
    fn cv_trait_neg() {
        let vec = vec![ C32F!(2,0), C32F!(0,4), C32F!(-2,0) ];
        let vec_obj = VecC32F::new(vec);
        assert_eq!( VecC32F::new(vec![ C32F!(-2,0), C32F!(0,-4), C32F!(2,0) ]), -vec_obj );
    }

    #[test]
    fn cv_trait_index() {
        let vec = vec![ C32F!(2,0), C32F!(0,4), C32F!(-2,0) ];
        let vec_obj = VecC32F::new(vec);
        assert_eq!( C32F!(0,4), vec_obj[1] );
    }
}
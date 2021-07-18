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


pub trait VectorTraits<T> {
    fn new( arg_vec:Vec<T> ) -> Self;
    fn zeros( len: usize ) -> Self;
    fn ones( len: usize ) -> Self;
    fn front( &self ) -> &T;
    fn back( &self ) -> &T;
    fn len( &self ) -> usize;
}

macro_rules! vector_struct {
    ( $Name:ident, $T:ty, $type:ident ) => {
        #[derive(Clone, Debug, PartialEq)]
        /// Numeric vector type
        struct $Name
        {
            vec:Vec<$T>
        }
        
        impl VectorTraits<$T> for $Name {
            fn new( arg_vec:Vec<$T> ) -> $Name {
                $Name{
                    vec: arg_vec,
                }
            }
            fn zeros( len: usize ) -> $Name {
                $Name {
                    vec: crate::$type::zeros( len ),
                }
            }
            fn ones( len: usize ) -> $Name {
                $Name {
                    vec: crate::$type::ones( len ),
                }
            }
            fn front( &self ) -> &$T {
                return &self.vec[0];
            }
            fn back( &self ) -> &$T {
                return &self.vec[self.vec.len()-1];
            }
            fn len( &self ) -> usize {
                return self.vec.len();
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

vector_struct!( VecF32, f32, vfunc );
vector_struct!( VecF64, f64, vfunc );

vector_struct!( VecC32F, C32F, cvfunc );
vector_struct!( VecC64F, C64F, cvfunc );

/*
/// This macro generates element-wise traits on vectors of complex numbers.
/// The operations must have a function in the respective func files.
macro_rules! element_wise_operand {
    (   
        $(#[$comment:meta])*
        $operand:ident
    ) => {
        $(#[$comment])*
        /// Element-wise operation on vector of real type R.
        impl std::ops::$operand for $Name {   
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
*/

/// Trait overload
macro_rules! vector_trait_overload{
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

vector_trait_overload!( VecC32F, C32F, cvfunc );
vector_trait_overload!( VecC64F, C64F, cvfunc );
vector_trait_overload!( VecF32,  f32, vfunc );
vector_trait_overload!( VecF64,  f64, vfunc );


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
    fn vector_trait_neg() {
        let vec = vec![ C32F!(2,0), C32F!(0,4), C32F!(-2,0) ];
        let vec_obj = VecC32F::new(vec);
        assert_eq!( VecC32F::new(vec![ C32F!(-2,0), C32F!(0,-4), C32F!(2,0) ]), -vec_obj );
    }

    #[test]
    fn basic_trait_index() {
        let vec = vec![ C32F!(2,0), C32F!(0,4), C32F!(-2,0) ];
        let vec_obj = VecC32F::new(vec);
        assert_eq!( C32F!(0,4), vec_obj[1] );
    }

    #[test]
    fn basic_trait_len() {
        let vec = vec![ C32F!(2,0), C32F!(0,4), C32F!(-2,0) ];
        let vec_obj = VecC32F::new(vec);
        assert_eq!( 3, vec_obj.len() );
    }

    #[test]
    fn basic_trait_ones() {
        assert_eq!( VecF32::new(vec![ 1_f32, 1_f32, 1_f32]), VecF32::ones(3) );
    }
}
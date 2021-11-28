//--------------------------------------------------------------//
// Copyright 2021 Norsk Datateknikk AS                          //
//--------------------------------------------------------------//
// This file is subject to the terms and conditions defined in  //
// file 'LICENSE', which is part of this source code package.   //
//--------------------------------------------------------------//

extern crate alloc;
extern crate num;

/// Numeric vector of complex, fixed-point numbers.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct CVec<T>
    where T: fixed::traits::Fixed
{
    vec: alloc::vec::Vec<num::complex::Complex<T>>
}

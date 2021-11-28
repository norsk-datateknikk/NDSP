//--------------------------------------------------------------//
// Copyright 2021 Norsk Datateknikk AS                          //
//--------------------------------------------------------------//
// This file is subject to the terms and conditions defined in  //
// file 'LICENSE', which is part of this source code package.   //
//--------------------------------------------------------------//

#[macro_use]
pub mod fixed;

extern crate alloc;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Vec<T> {
    vec: alloc::vec::Vec<T>
}
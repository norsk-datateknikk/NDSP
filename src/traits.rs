//--------------------------------------------------------------//
// Copyright 2021 Norsk Datateknikk AS                          //
//--------------------------------------------------------------//
// This file is subject to the terms and conditions defined in  //
// file 'LICENSE', which is part of this source code package.   //
//--------------------------------------------------------------//


pub trait LinSpace {
    fn linspace<T>( start:T, step:T ) -> Self;
}

pub trait Ones {
    fn ones()      -> Self;
}

pub trait Zeros {
    fn ones()      -> Self;
}

pub trait Len {
    fn len( &self ) -> usize;
}

pub trait Powi {
    fn powi( &self, power:u32 ) -> Self;
}

pub trait Sin {
    fn sin( &self )  -> Self;
}

pub trait Cos {
    fn cos( &self )  -> Self;
}

pub trait Tan {
    fn tan( &self )  -> Self;
}

pub trait Atan {
    fn atan( &self )  -> Self;
}

pub trait WrapPhase {
    fn wrap_phase( &self )  -> Self;
}

pub trait Fft {
    fn fft( &self )  -> Self;
}

pub trait ComplexCartesian {
    fn real() -> Self;
    fn imag() -> Self;
}
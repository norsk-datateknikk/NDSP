//--------------------------------------------------------------//
// Norsk Datateknikk AS 2021                                    //
//--------------------------------------------------------------//
// This file is subject to the terms and conditions defined in  //
// file 'LICENSE', which is part of this source code package.   //
//--------------------------------------------------------------//

// use plotters::prelude::*;
use num_traits::Num;
use num_traits::Float;
//use num::complex::Complex;
use num_complex::Complex;

/// Complex 32-bit float
pub type C32F = Complex::<f32>;
/// Complex 64-bit float
pub type C64F = Complex::<f32>;
/// Complex 64-bit float
pub type C32I = Complex::<i32>;
/// Complex 64-bit float
pub type C64I = Complex::<i64>;


#[macro_export]
/// Calculate element-wise exponential.
/// Explicitly statin the input type is required.
macro_rules! exp {
    ( $vector:expr => $T:ty ) => {
        {   
            let mut r_vector: Vec<$T> = Vec::with_capacity( $vector.len() );
            for index in 0..$vector.len() {
                r_vector[index] = $vector[index].exp();
            }        
            r_vector
        }
    };
}

/// Returns the element-wise sine of a vector.
pub fn exp<F>( vector: Vec<F> )-> Vec<F>
    where F: Float
{
    let mut r_vector: Vec<F> = Vec::with_capacity( vector.len() );
    for index in 0..vector.len() {
        r_vector[index] = vector[index].exp();
    }        
            
    return r_vector;
}

/// Returns the complex element-wise sine of a vector.
pub fn c_exp<F>( vector: Vec<Complex<F>> )-> Vec<Complex<F>>
    where F: Float
{
    let mut r_vector: Vec<Complex<F>> = Vec::with_capacity( vector.len() );
    for index in 0..vector.len() {
        r_vector[index] = vector[index].exp();
    }
            
    return r_vector;
}


///Generate an unmodulated pulse, (sine).
/// Using floating point calculation.
/// TODO: Scale to N-bit resolution.
/// TODO: Use fixed point type.
/// Todo: Utilize SIMD.
pub fn gen_real_unmod_pulse( sample_rate_hz:i32, numb_samples:usize, frequency_hz:i32, phase_rad:f32  ) -> Vec<f32>
{
    let x = f32::sin(12f32);
    println!("{}", x);

    //let mut pulse = Vec::<i16>::with_capacity(numb_samples);
    
    let end_sample:f32 = 2_f32*std::f32::consts::PI*(frequency_hz as f32)*((numb_samples as f32)/(sample_rate_hz as f32))+phase_rad; // 2*pi'freq*time+phase

    let time = linspace::<f32>( phase_rad, end_sample, numb_samples );

    println!( "{:#?}", time );

    return time;
}

/// Returns the element-wise sine of a vector.
pub fn sin<F>( vector: Vec<F> )-> Vec<F>
    where F: Float
{
    let mut return_vector: Vec<F> = Vec::with_capacity(vector.len());
    for index in 0..vector.len() {
        return_vector.push( F::sin(vector[index]) );
    }

    return return_vector;
}

/// Returns a 1D vector of zeros of size numb_samples.
/// [0,0,...,0]
pub fn zeros<N>( numb_samples: usize ) -> Vec<N>
    where N: Num
{
    let mut vector: Vec<N> = Vec::with_capacity(numb_samples);
    for i in 0..numb_samples {
        vector.push(N::zero());
    }
    return vector;
}

/// Returns a 1D vector of ones of size numb_samples.
/// [1,1,...,1]
pub fn ones<N>( numb_samples: usize ) -> Vec<N>
    where N: Num
{
    let mut vector: Vec<N> = Vec::with_capacity(numb_samples);
    for i in 0..numb_samples {
        vector.push(N::one());
    }
    return vector;
}


/// Returns a 1D vector of evenly spaced numbers of type F.
/// [0,1,...,N-1]
pub fn linspace<F>( start: F, stop: F, numb_samples: usize ) -> Vec<F>
    where F: Float
{
    let delta = if numb_samples > 1
    {
        let nf: F = F::from(numb_samples).unwrap();
        (stop - start) / (nf - F::one())
    } else {
        F::zero()
    };

    let mut vector = Vec::<F>::with_capacity(numb_samples);

    for index in 0..vector.capacity()
    {
        let idx = F::from(index).unwrap();
        vector.push(start+(idx)*delta);
    }
    return vector;
}
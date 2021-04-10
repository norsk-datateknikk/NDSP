//--------------------------------------------------------------//
// Norsk Datateknikk AS 2021                                    //
//--------------------------------------------------------------//
// This file is subject to the terms and conditions defined in  //
// file 'LICENSE', which is part of this source code package.   //
//--------------------------------------------------------------//

// use plotters::prelude::*;
use num_traits::Num;
use num_traits::Float;
use num_traits::real::Real;
//use num::complex::Complex;
use num_complex::Complex;

/// Complex 32-bit float
pub type C32F = Complex::<f32>;
/// Complex 64-bit float
pub type C64F = Complex::<f64>;
/// Complex 64-bit float
pub type C32I = Complex::<i32>;
/// Complex 64-bit float
pub type C64I = Complex::<i64>;

#[macro_export]
macro_rules! C32F {
    ( $re:expr, $im:expr ) => {
        {
            num_complex::Complex::new( $re as f32, $im as f32 );
        }
    };
}

/// Returns the complex valu re+i*im.
pub fn c_value<N>( re:N, im:N)-> Complex<N>
    where N: Num
{
    let mut c_val:Complex<N>;
    c_val.re = re;
    c_val.im = im;
    return c_val;
}



/// Returns the index of the highest valued item.
pub fn arg_max<R>( vector: Vec<R> )-> usize
    where R: Real
{
    let mut max_val:R = R::min_value();
    let mut arg_max:usize = 0;
    for index in 0..vector.len() {
        if max_val < vector[index]
        {
            max_val = vector[index];
            arg_max = index;
        }
    } 
    return arg_max;
}

/// Returns the index of the lowest valued item.
pub fn arg_min<R>( vector: Vec<R> )-> usize
    where R: Real
{
    let mut min_val:R = R::max_value();
    let mut arg_min:usize = 0;
    for index in 0..vector.len() {
        if min_val < vector[index]
        {
            min_val = vector[index];
            arg_min = index;
        }
    } 
    return arg_min;
}

/// Returns the highest valued item.
pub fn max<R>( vector: Vec<R> )-> R
    where R: Real
{
    let mut max_val:R = R::min_value();
    for index in 0..vector.len() {
        if max_val < vector[index]
        {
            max_val = vector[index];
        }
    } 
    return max_val;
}

/// Returns the index of the lowest valued item.
pub fn min<R>( vector: Vec<R> )-> R
    where R: Real
{
    let mut min_val:R = R::max_value();
    for index in 0..vector.len() {
        if min_val < vector[index]
        {
            min_val = vector[index];
        }
    } 
    return min_val;
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

macro_rules! F2 {
    () => {
        F::from(2).unwrap();
    };
}

/// Returns the complex element-wise sine of a vector.
pub fn c_abs<F>( vector: Vec<Complex<F>> )-> Vec<F>
    where F: Float
{
    let mut r_vector: Vec<F> = Vec::with_capacity( vector.len() );
    for index in 0..vector.len() {

        r_vector[index] = F::sqrt( vector[index].re.powf( F2!() )+vector[index].im.powf( F2!() ) );
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
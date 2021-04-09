// use plotters::prelude::*;

struct Scan
{
    sample_rate_hz: i32,
    frame: Vec<i8>
}

use num_traits::Float;

//use itertools::Itertools;

/**
 * This library is intended for use on embedded 32-bit platforms.
 * Operations are separated into two classes, setup and continious.
 * - The setup functions are assumed to be ran before some continious operation, precision is preferred over efficiency.
 * - The continious functions are assumed to run real-time. Here efficiency is preferred.
 */

///Generate a 16-bit unmodulated pulse, (sine).
/// Using floating point calculation.
/// TODO: Scale to N-bit resolution.
/// TODO: Use fixed point type.
/// Todo: Utilize SIMD.
fn gen_real_unmod_pulse( sample_rate_hz:i32, numb_samples:usize, frequency_hz:i32, phase_rad:f32  ) -> Vec<f32>
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
fn sin<F>( vector: Vec<F> )-> Vec<F>
    where F: Float
{
    let mut return_vector: Vec<F> = Vec::with_capacity(vector.len());
    for index in 0..vector.len() {
        return_vector[index] = F::sin(vector[index]);
    }

    return return_vector;
}

/// Returns a 1D vector of floating point zeros of size numb_samples.
/// [0,0,...,0]
fn zeros<F>( numb_samples: usize ) -> Vec<F>
    where F: Float
{
    let mut vector: Vec<F> = Vec::with_capacity(numb_samples);
    for i in 0..numb_samples {
        vector.push(F::zero());
    }
    return vector;
}

/// Returns a 1D vector of floating point ones of size numb_samples.
/// [1,1,...,1]
fn ones<F>( numb_samples: usize ) -> Vec<F>
    where F: Float
{
    let mut vector: Vec<F> = Vec::with_capacity(numb_samples);
    for i in 0..numb_samples {
        vector.push(F::one());
    }
    return vector;
}


/// Returns a 1D vector of evenly spaced numbers of type F.
/// [0,1,...,N-1]
fn linspace<F>( start: F, stop: F, numb_samples: usize ) -> Vec<F>
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


fn main()
{
    let test_vec = linspace::<f32>(0_f32, 100_f32, 600);
    let vector2 = sin::<f32>( test_vec );
    println!( "{:?}", vector2 );
}

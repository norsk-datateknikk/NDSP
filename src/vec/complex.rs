//-----------------------------------------------------------------//
// Copyright 2021 Norsk Datateknikk AS                             //
//-----------------------------------------------------------------//
// This file is subject to the terms and conditions defined in the //
// file 'LICENSE', which is part of this source code package.      //
//-----------------------------------------------------------------//

extern crate alloc;
use num;
use num::complex::Complex;

use mixed_num::traits::*;

use crate::traits::*;
use crate::vec::*;
use Vec;

#[cfg(any(feature = "std"))]
use std::fs::File;
#[cfg(any(feature = "std"))]
use std::io::{BufReader, Read};

use crate::traits;


impl<T: MixedNum> Vec<Complex<T>> {
    /// Create a complex vector from a real one.
    #[allow(dead_code)]
    fn new_from_real( real_vec: Vec<T> ) -> Vec<Complex<T>>
    {
        let mut vec = Vec::<Complex<T>>::new_with_capacity(real_vec.len());

        for idx in 0..real_vec.len()
        {
            vec.push_back(Complex::new(real_vec[idx], T::mixed_from_num(0)))
        }
        return vec;
    }
}


impl <T: MixedNum + MixedNumSigned> traits::Abs<C> for Vec<Complex<T>> {
    /// Take the elemtent-wise absolute value.
    fn abs(&mut self) {
        for idx in 0..self.len() {
            self[idx].re = fixed_trigonometry::complex::abs(self[idx]);
            self[idx].im = T::mixed_zero();
        }
    }
}


impl<T: MixedOps + MixedTrigonometry + MixedWrapPhase>  Vec<Complex<T>> {
    /// Creates a rotating phasor with a specific angular frequency.
    /// 
    ///  `s[n] = cos(ωn+θ) + i sin(n+θ), n ∈ {0,1,...N-1}`.
    /// 
    /// ## Arguments
    /// 
    /// * `angular_freq_rad`- The angular frequency (ω).
    /// * `phase_rad`       - The start phase in rad (θ).
    /// * `numb`            - The number of samples (N).
    /// 
    #[allow(dead_code)]
    pub fn osc( angular_freq_rad: T, phase_rad: T, numb: usize ) -> Vec<Complex<T>>
    {
        let mut vec = Vec::new_with_capacity(numb);
        let mut phase_rad_inc= phase_rad;
        for _i in 0..numb
        {   
            // let (real,imag ) = cordic::sin_cos(phase_rad_inc); //! Requires cordic number trait.
            let real = phase_rad_inc.mixed_cos();
            let imag = phase_rad_inc.mixed_sin();

            vec.push_back( num::Complex::new( real, imag ) );

            phase_rad_inc = phase_rad_inc + angular_freq_rad;
            phase_rad_inc = phase_rad_inc.mixed_wrap_phase();
        }
        return vec;
    }
}


impl <T: MixedNumSigned + MixedNum + MixedTrigonometry> traits::Fft for Vec<Complex<T>> {
    /// Calculate the Raddix-2 FFT for fixed point vectors.
    /// Scaled for each butterfly computation.
    /// Requires input size to be a power of two.
    /// 
    /// Computed-in-place.
    /// Decimation-in-freqency.
    /// 
    /// The method utilizes fixed point approximations for square root, sine, cosine and atan calculations.
    /// 
    fn fft(&mut self){
        fixed_trigonometry::fft::fft( &mut self.vec);
    }
}

#[cfg(any(feature = "std"))]
impl <T> traits::FromFile for Vec<Complex<T>>
    where T: MixedNum
{
    /// Read a binary file from e.g. Gnu Radio Companion into a vector.
    /// Assuming a binary file containing complex32.
    fn from_file( _item_type: ItemType, path: &str ) -> Self
    {

        let file = File::open(path).expect("file wasn't found.");
        let mut reader = BufReader::new(&file);

        let file_size_bytes = &file.metadata().unwrap().len();

        // Currently only float32 and complex32 is supported.
        const ITEM_SIZE_BYTES:usize = 4;
        
        let mut vec = Self::new_with_capacity(*file_size_bytes as usize/ITEM_SIZE_BYTES);
        
        // Counter to keep track of I/Q sample. Even = I, odd = Q.
        let mut counter:usize = 0;

        let mut temp_complex = Complex::new(T::mixed_from_num(0), T::mixed_from_num(0));

        loop {
            use std::io::ErrorKind;
            let mut buffer = [0u8; std::mem::size_of::<f32>()];
            
            let res = reader.read_exact(&mut buffer);
            match res {
                Err(error) if error.kind() == ErrorKind::UnexpectedEof => break,
                _ => {}
            }
            res.expect("Unexpected error during read");

            if &counter%2==0
            {
                // Use `from_be_bytes` if numbers in file is big-endian
                temp_complex.re = T::mixed_from_num(f32::from_le_bytes(buffer));
            }
            else {
                // Use `from_be_bytes` if numbers in file is big-endian
                temp_complex.im = T::mixed_from_num(f32::from_le_bytes(buffer));
                vec.push_back(temp_complex);
            }
            counter +=1;
        }

        return vec;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        use fixed::{types::extra::U28, FixedI32};
        use std::println;

        let omega = 0.8f32;
        let theta = 0f32;
         
        let signal = super::vec::Vec::osc(omega, theta, 4);
         
        println!("Signal {}", signal);
        assert_eq!(signal.to_string(), "[ 1.0000035+0i, -0.902972+0.7173561i, 3.0477293+0.94117063i, 0.74554664-0.9825768i ]" )
    }
}
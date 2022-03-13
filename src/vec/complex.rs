//----------------------//
// Norsk Datateknikk AS //
//----------------------//

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

impl <T: MixedNum> traits::AsReal<T> for Vec<Complex<T>> {
    /// Returns the real part of the vector as a real only vector.
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// 
    /// let mut signal = Vec::new_with_capacity(2);
    /// signal.push_back( num::Complex::new( 0f32, 1f32 ) );
    /// signal.push_back( num::Complex::new( 1f32, 0f32 ) );
    /// assert_eq!(signal.as_real().to_string(), "[ 0, 1 ]" )
    /// ```
    fn as_real(&self) -> Vec<T>
    {
        let mut r_vec = Vec::<T>::new_with_capacity(*&self.len());
        for i in 0..*&self.len()
        {
            r_vec.push_back( self[i].re);
        }
        return r_vec;
    }
}


impl <T: MixedNum + MixedNumSigned> traits::Abs<C> for Vec<Complex<T>> {
    /// Take the elemtent-wise absolute value.
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// 
    /// let mut signal = Vec::new_with_capacity(2);
    /// signal.push_back( num::Complex::new( 0f32, 1f32 ) );
    /// signal.push_back( num::Complex::new( 1f32, 0f32 ) );
    /// signal.abs();
    /// assert_eq!(signal.to_string(), "[ 1.000752+0i, 1.000752+0i ]" )
    /// ```
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
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// use mixed_num::traits::*;
    /// 
    /// let omega = <f32>::mixed_pi()/f32::mixed_from_num(8i32);
    /// let theta = 0f32; 
    /// 
    /// let signal = Vec::osc(omega, theta, 4);
    /// assert_eq!(signal.to_string(), "[ 1.0000035+0i, 0.9238796+0.38268343i, 0.7071068+0.70710677i, 0.38268334+0.9238796i ]" )
    /// ```
    pub fn osc( angular_freq_rad: T, phase_rad: T, numb: usize ) -> Vec<Complex<T>>
    {
        let mut vec = Vec::new_with_capacity(numb);
        let mut sample_phase_rad= phase_rad;
        for _i in 0..numb
        {   
            sample_phase_rad = sample_phase_rad.mixed_wrap_phase();
            
            let real = sample_phase_rad.mixed_cos();
            let imag = sample_phase_rad.mixed_sin();

            vec.push_back( num::Complex::new( real, imag ) );

            sample_phase_rad += angular_freq_rad;
        }
        return vec;
    }
}


impl <T: MixedNumSigned + MixedNum + MixedTrigonometry> traits::Fft for Vec<Complex<T>> {
    /// Calculate the Raddix-2 FFT for self.
    /// Scaled for each butterfly computation.
    /// Requires input size to be a power of two.
    /// 
    /// Computed-in-place.
    /// Decimation-in-freqency.
    /// 
    /// The method utilizes fixed point approximations for square root, sine, cosine and atan calculations.
    fn fft(&mut self){
        fixed_trigonometry::fft::fft( &mut self.vec);
    }
}

impl <T: MixedNumSigned + MixedNum + MixedTrigonometry> traits::Ifft for Vec<Complex<T>> {
    /// Calculate the Raddix-2 IFFT for self.
    /// Scaled for each butterfly computation.
    /// Requires input size to be a power of two.
    /// 
    /// Computed-in-place.
    /// Decimation-in-freqency.
    /// 
    /// The method utilizes fixed point approximations for square root, sine, cosine and atan calculations.
    fn ifft(&mut self){
        fixed_trigonometry::fft::ifft( &mut self.vec);
    }
}
/*
#[cfg(any(feature = "std"))]
impl <T> traits::FromBinary for Vec<Complex<T>>
    where T: MixedNum
{
    /// Read a binary file from e.g. Gnu Radio Companion into a vector.
    /// Assuming a binary file containing complex32.
    fn from_binary( _item_type: ItemType, path: &str ) -> Self
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
 */

// We prefer doctests, as they provide documentation additionally.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        let mut signal = Vec::new_with_capacity(2);
        signal.push_back( num::Complex::new( 0f32, 1f32 ) );
        signal.push_back( num::Complex::new( 1f32, 0f32 ) );
        assert_eq!(signal.to_string(), "[ 0+1i, 1+0i ]" )
    }
}
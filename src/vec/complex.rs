//----------------------//
// Norsk Datateknikk AS //
//----------------------//

extern crate alloc;
use mixed_num::Cartesian;

use mixed_num::traits::*;
mod fft;
pub use fft::*;

use crate::traits::*;
use crate::traits::Mag; 
use crate::vec::*;
use Vec;

/*
#[cfg(any(feature = "std"))]
use std::fs::File;
#[cfg(any(feature = "std"))]
use std::io::{BufReader, Read};
*/
use crate::traits;


impl<T: MixedReal> Vec<Cartesian<T>> {
    /// Create a complex vector from a real one.
    #[allow(dead_code)]
    fn new_from_real( real_vec: Vec<T> ) -> Vec<Cartesian<T>>
    {
        let mut vec = Vec::<Cartesian<T>>::new_with_capacity(real_vec.len());

        for idx in 0..real_vec.len()
        {
            vec.push_back(Cartesian::new(real_vec[idx], T::mixed_from_num(0)))
        }
        return vec;
    }

    /// Create a complex vector from a real one.
    /// 
    /// Expects buffer to be at least as large as self. 
    #[allow(dead_code)]
    pub fn copy_from_buffer( &mut self, buffer: &Vec<T> )
    {
        for idx in 0..self.len()
        {
            self[idx].re = buffer[idx];
        }
    }
}

impl <T: MixedNum> traits::Re<T> for Vec<Cartesian<T>> {
    /// Returns the real part of the vector as a real only vector.
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// use mixed_num::Cartesian;
    /// 
    /// let mut signal = Vec::new_with_capacity(2);
    /// signal.push_back( Cartesian::new( 0f32, 1f32 ) );
    /// signal.push_back( Cartesian::new( 1f32, 0f32 ) );
    /// assert_eq!(signal.re().to_string(), "[ 0, 1 ]" )
    /// ```
    fn re(&self) -> Vec<T>
    {
        let mut r_vec = Vec::<T>::new_with_capacity(*&self.len());
        for i in 0..*&self.len()
        {
            r_vec.push_back( self[i].re);
        }
        return r_vec;
    }
}


impl <T: MixedNum> traits::Im<T> for Vec<Cartesian<T>> {
    /// Returns the real part of the vector as a real only vector.
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// use mixed_num::Cartesian;
    /// 
    /// let mut signal = Vec::new_with_capacity(2);
    /// signal.push_back( Cartesian::new( 0f32, 1f32 ) );
    /// signal.push_back( Cartesian::new( 1f32, 0f32 ) );
    /// assert_eq!(signal.im().to_string(), "[ 1, 0 ]" )
    /// ```
    fn im(&self) -> Vec<T>
    {
        let mut r_vec = Vec::<T>::new_with_capacity(*&self.len());
        for i in 0..*&self.len()
        {
            r_vec.push_back( self[i].im);
        }
        return r_vec;
    }
}


impl<T: MixedNum + MixedWrapPhase + MixedSin + MixedOps>  Vec<Cartesian<T>> {
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
    /// assert_eq!(signal.to_string(), "[ 1+0i, 0.9238795+0.38268346i, 0.70710677+0.70710677i, 0.38268346+0.9238795i ]" )
    /// ```
    pub fn osc( angular_freq_rad: T, phase_rad: T, numb: usize ) -> Vec<Cartesian<T>>
    {
        let mut vec = Vec::new_with_capacity(numb);
        let mut sample_phase_rad= phase_rad;
        for _i in 0..numb
        {   
            sample_phase_rad = sample_phase_rad.mixed_wrap_phase();
            
            let (imag, real) = sample_phase_rad.mixed_sincos();

            vec.push_back( Cartesian::new( real, imag ) );

            sample_phase_rad += angular_freq_rad;
        }
        return vec;
    }
}

impl <T: MixedNum + MixedAtan + MixedZero> Ang<T> for Vec<Cartesian<T>> {
    /// Element-wise angle of complex numbers.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// use mixed_num::*;
    /// 
    /// let omega = <f32>::mixed_pi()/f32::mixed_from_num(8i32);
    /// let theta = 0f32; 
    /// 
    /// let mut signal = Vec::osc(omega, theta, 4);
    /// signal.ang();
    /// assert_eq!(signal.re().to_string(), "[ 0, 0.39269912, 0.7853982, 1.1780972 ]" );
    ///
    /// 
    /// let omega = <f32>::mixed_pi()/f32::mixed_from_num(8i32);
    /// let theta = 0f32; 
    /// 
    /// let mut signal = Vec::osc(omega, theta, 64);
    /// signal.ang();
    /// signal.re().simple_plot("./figures/ang_documentation.png", "Angle"); 
    /// ```
    /// 
    /// ![Alt version]()
    /// 
    fn ang( &mut self ) {
        for i in 0..self.len()
        {
            self[i].re = self[i].im.mixed_atan2(self[i].re);
            self[i].im = T::mixed_zero();
        }
    }
}

impl <T: MixedAbs> crate::traits::Mag<T> for Vec<T> {
    /// Element-wise magnitude.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// use mixed_num::*;
    /// 
    /// let omega = <f32>::mixed_pi()/f32::mixed_from_num(2i32);
    /// let theta = 0f32; 
    /// 
    /// let mut signal = Vec::osc(omega, theta, 4);
    /// 
    /// signal.mag();
    /// assert_eq!(signal.to_string(), "[ 1+0i, 1+0i, 1+0i, 1+0i ]" );
    /// ```
    fn mag( &mut self ) {
        for i in 0..self.len()
        {
            self[i] = self[i].mixed_abs();
        }
    }
}

impl<T: MixedReal + MixedNumSigned + MixedNumConversion<T> + MixedTrigonometry + MixedSqrt + MixedWrapPhase + MixedOps + MixedPi + MixedZero + MixedPowi> Psd<T> for Vec<Cartesian<T>> {
    /// Calculate the Power Spectral Density (PSD) in linear scale of a signal.
    /// 
    /// Expects the signal length to be a power of two. If not, the signal is zero padded.
    ///  
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// use mixed_num::traits::*;
    /// 
    /// let phase_rad = 0f32;
    /// 
    /// let f_sample = 10e3;
    /// let tone_frequency = 2e3;
    /// let angular_frequency = tone_frequency/(2f32*f_sample)*f32::mixed_tau();
    /// 
    /// let signal = Vec::osc(angular_frequency, phase_rad, 8);
    /// let mut psd = signal.psd();
    /// 
    /// 
    /// assert_eq!(psd.to_string(), "[ 0.056531776, 0.87694174, 0.026191715, 0.009336119, 0.005968219, 0.0054317378, 0.006799792, 0.012798772 ]" );
    /// 
    /// let signal = Vec::osc(angular_frequency, phase_rad, 128);
    /// let mut psd = signal.psd();
    /// 
    /// 
    /// let step:f32 = 2f32*f_sample/(psd.len() as f32);
    /// let x_vec    = Vec::lin_range(0f32, 2f32*f_sample-step, psd.len());
    /// 
    /// psd.pow2db();
    /// 
    /// x_vec.plot(&psd, "./figures/osc_psd.png", "Power Spectral Density", "[Hz]", "dBW/Hz" );
    /// ```
    fn psd( &self ) -> Vec<T>
    {
        let padded_len = round_to_power_of_two(self.len());
        let mut buffer = Vec::<Cartesian<T>>::new_with_capacity(padded_len);
        for idx in 0..self.len() {
            buffer.push_back(self[idx]);
        }
        for _idx in 0..padded_len-self.len() {
            buffer.push_back(Cartesian::<T>::mixed_zero());
        }

        buffer.fft();
        buffer.mag();
        let mut real_buffer = buffer.re();
        real_buffer.powi(2);

        return real_buffer;
    }
}

impl <T: MixedReal + MixedNumSigned + MixedNumConversion<T> + MixedTrigonometry + MixedSqrt + MixedWrapPhase + MixedOps + MixedPi + MixedZero + MixedPowi> traits::Fft for Vec<Cartesian<T>> {
    /// Calculate the Raddix-2 FFT for self.
    /// Scaled for each butterfly computation.
    /// Requires input size to be a power of two.
    /// 
    /// Computed-in-place.
    /// Decimation-in-freqency.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// 
    /// let n = 512;
    /// let mut complex_vec = Vec::osc(1f32,0f32,n);
    ///
    /// complex_vec.fft();
    ///
    /// complex_vec.mag();
    /// 
    /// complex_vec.re().simple_plot("./figures/fft_demonstration.png", "FFT Demonstration");
    /// ```
    fn fft(&mut self){
        fft( &mut self.vec);
    }
}

impl <T: MixedReal + MixedNumSigned + MixedNumConversion<T> + MixedTrigonometry + MixedSqrt + MixedWrapPhase + MixedOps + MixedPi + MixedZero + MixedPowi> traits::Ifft for Vec<Cartesian<T>> {
    /// Calculate the Raddix-2 IFFT for self.
    /// Scaled for each butterfly computation.
    /// Requires input size to be a power of two.
    /// 
    /// Computed-in-place.
    /// Decimation-in-freqency.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// use mixed_num::*;
    /// 
    /// let n = 128;
    /// let mut buffer = Vec::<Cartesian<f32>>::zeros(128);
    ///
    /// buffer[4] = Cartesian::<f32>::mixed_one();
    /// 
    /// buffer.ifft();
    ///
    /// buffer.re().simple_plot("./figures/ifft_demonstration.png", "IFFT Demonstration");
    /// ```
    fn ifft(&mut self){
        ifft( &mut self.vec);
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
        signal.push_back( mixed_num::Cartesian::new( 0f32, 1f32 ) );
        signal.push_back( mixed_num::Cartesian::new( 1f32, 0f32 ) );
        assert_eq!(signal.to_string(), "[ 0+1i, 1+0i ]" )
    }
}

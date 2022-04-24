//----------------------//
// Norsk Datateknikk AS //
//----------------------//


extern crate alloc;

use mixed_num::*;

use crate::traits;
use crate::traits::*;
use crate::vec::*;

/*
#[cfg(any(feature = "std"))]
use std::fs::File;
#[cfg(any(feature = "std"))]
use std::io::{BufReader, Read};
*/

impl <T: MixedNum> Vec<T> {
    
}

impl <T: MixedNum + MixedOps> LinRange<T> for Vec<T>
{
    /// Returns a 1D vector of evenly spaced numbers of type T.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let test_vec = Vec::lin_range(0f32, 3f32, 4);
    /// assert_eq!(test_vec.to_string(), "[ 0, 1, 2, 3 ]" )
    /// ```
    #[allow(dead_code)]
    fn lin_range( start:T, stop:T, num:usize ) -> Vec<T> {
        let step = (stop-start)/T::mixed_from_num((num-1) as i32);

        let mut vector = Vec::<T>::new_with_capacity(num);
        
        let mut val = start;
        for _idx in 0..vector.capacity()
        {
            vector.push_back(val);
            val += step;
        }
        return vector;
    }
}

impl <T: MixedZero> Zeros<T> for Vec<T> {
    /// Create a vector of zeros.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let test_vec = Vec::<f32>::zeros(4);
    /// assert_eq!(test_vec.to_string(), "[ 0, 0, 0, 0 ]" )
    /// ```
    fn zeros( len: usize ) -> Vec<T>
    {
        let mut rvec = Vec::<T>::new_with_capacity(len);

        for _idx in 0..len
        {
            rvec.push_back(T::mixed_zero());
        }
        return rvec;
    }
}

impl <T: MixedOne> Ones<T> for Vec<T> {
    /// Create a vector of ones, 𝟙.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let test_vec = Vec::<f32>::ones(4);
    /// assert_eq!(test_vec.to_string(), "[ 1, 1, 1, 1 ]" )
    /// ```
    fn ones( len: usize ) -> Vec<T>
    {
        let mut rvec = Vec::<T>::new_with_capacity(len);

        for _idx in 0..len
        {
            rvec.push_back(T::mixed_one());
        }
        return rvec;
    }
}

impl <T: MixedTrigonometry> traits::Sin for Vec<T> {
    /// Take the elemtent-wise sin(x).
    /// 
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// use mixed_num::*;
    /// 
    /// let x_vec = Vec::lin_range(0f32, f32::mixed_tau(), 64);
    /// let mut y_vec = x_vec.clone();
    /// 
    /// y_vec.sin();
    /// 
    //
    /// x_vec.plot(&y_vec, "./figures/sin_test.png", "Sine", "x", "sin(x)");
    /// ```
    /// 
    /// The resulting plot is shown below.
    /// 
    /// ![Alt version](https://raw.githubusercontent.com/norsk-datateknikk/NDSP/main/figures/sin_test.png)
    fn sin(&mut self) {
        for idx in 0..self.len() {
            self[idx] = self[idx].mixed_sin();
        }
    }
}

impl <T: MixedTrigonometry> traits::Cos for Vec<T> {
    /// Take the elemtent-wise cos(x).
    /// 
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// use mixed_num::*;
    /// 
    /// let x_vec = Vec::lin_range(0f32, f32::mixed_tau(), 64);
    /// let mut y_vec = x_vec.clone();
    /// 
    /// y_vec.cos();
    /// 
    /// x_vec.plot(&y_vec, "./figures/cos_test.png", "Cosine", "x", "cos(x)");
    /// ```
    /// 
    /// The resulting plot is shown below.
    /// 
    /// ![Alt version](https://raw.githubusercontent.com/norsk-datateknikk/NDSP/main/figures/cos_test.png)
    fn cos(&mut self) {
        for idx in 0..self.len() {
            self[idx] = self[idx].mixed_cos();
        }
    }
}

impl <T: MixedWrapPhase> traits::WrapPhase for Vec<T> {
    /// Wrapps θ to the -π=<x<π range.
    /// 
    /// ## Arguments 
    ///
    /// * `phi` - The unwrapped phase in radians.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// use mixed_num::*;
    /// 
    /// let x_vec = Vec::lin_range(0f32, 4f32*f32::mixed_tau(), 256);
    /// let org_phase = x_vec.clone();
    /// let mut wrapped_phase = x_vec.clone();
    /// 
    /// wrapped_phase.wrap_phase();
    /// 
    /// x_vec.plot_multiple(&[&org_phase,&wrapped_phase], "./figures/wrap_phase_test.png", "Wrapped phase", "", "Phase", &["Original","Wrapped"]);
    /// ```
    /// 
    /// The resulting plot is shown below.
    /// 
    /// ![Alt version](https://raw.githubusercontent.com/norsk-datateknikk/NDSP/main/figures/wrap_phase_test.png)
    fn wrap_phase(&mut self) {
        for idx in 0..self.len() {
            self[idx] = self[idx].mixed_wrap_phase();
        }
    }
}

impl <T: MixedSqrt> traits::Sqrt for Vec<T> {
    /// Take the element-wise square root.
    fn sqrt(&mut self) {
        for idx in 0..self.len() {
            self[idx] = self[idx].mixed_sqrt();
        }
    }
}

impl <T: MixedReal + MixedAbs> traits::Abs for Vec<T> {
    /// Element-wise absolute value of `self`. Computed-in-place.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let mut test_vec = Vec::lin_range(-2f32, 2f32, 5);
    /// test_vec.abs();
    /// assert_eq!(test_vec.to_string(), "[ 2, 1, 0, 1, 2 ]" )
    /// ```
    fn abs(&mut self) {
        for idx in 0..self.len() {
            if self[idx]< T::mixed_from_num(0)
            {
                self[idx]=self[idx].mixed_abs();
            }
        }
    }
}

impl<T: MixedPowi> Powi for Vec<T> {
    /// Rase the vector to an integer power. Computed-in-place.
    fn powi( &mut self, power:i32 ){
        for idx in 0..self.len() {
                self[idx]=self[idx].mixed_powi(power);
        }
    }
}

impl<T: MixedNum + MixedPow> Pow<T> for Vec<T> {
    /// Rase the vector to an integer power. Computed-in-place.
    fn pow( &mut self, power:T ) {
        for idx in 0..self.len() {
            self[idx]=self[idx].mixed_pow(power);
        }
    }
}

impl <T: MixedReal> traits::AsComplexCartesian<T> for Vec<T> {
    /// Returns the real part of the vector as a real only vector.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let test_vec = Vec::lin_range(0f32, 3f32, 4);
    /// assert_eq!(test_vec.as_complex_cartesian().to_string(), "[ 0+0i, 1+0i, 2+0i, 3+0i ]" )
    /// ```
    fn as_complex_cartesian(&self) -> Vec<Cartesian<T>>
    {
        let len = *&self.len();
        let mut r_vec = Vec::<Cartesian<T>>::new_with_capacity(len);
        for i in 0..len
        {
            r_vec.push_back( Cartesian::new( self[i], T::mixed_from_num(0)));
        }
        return r_vec;
    }
}

impl <T: MixedReal> traits::AsComplexPolar<T> for Vec<T> {
    /// Returns the real part of the vector as a real only vector.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let test_vec = Vec::lin_range(0f32, 3f32, 4);
    /// assert_eq!(test_vec.as_complex_polar().to_string(), "[ 0∠0, 1∠0, 2∠0, 3∠0 ]" )
    /// ```
    fn as_complex_polar(&self) -> Vec<Polar<T>>
    {
        let len = *&self.len();
        let mut r_vec = Vec::<Polar<T>>::new_with_capacity(len);
        for i in 0..len
        {
            r_vec.push_back( Polar::new( self[i], T::mixed_from_num(0)));
        }
        return r_vec;
    }
}

impl <T: MixedReal> traits::Max<T> for Vec<T> {
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let test_vec = Vec::lin_range(0f32, 3f32, 4);
    /// assert_eq!(test_vec.max(), 3f32 )
    /// ```
    fn max(&self) -> T
    {
        let len = *&self.len();
        let mut value = T::mixed_min_value();
        for i in 0..len
        {
            if value < self[i]
            {
                value = self[i]
            }
        }
        return value;
    }
}

impl <T: MixedReal> traits::Min<T> for Vec<T> {
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let test_vec = Vec::lin_range(0f32, 3f32, 4);
    /// assert_eq!(test_vec.min(), 0f32 )
    /// ```
    fn min(&self) -> T
    {
        let len = *&self.len();
        let mut value = T::mixed_max_value();
        for i in 0..len
        {
            if self[i] < value
            {
                value = self[i]
            }
        }
        return value;
    }
}

impl <T: MixedReal> traits::MinMax<T> for Vec<T> {
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let test_vec = Vec::lin_range(0f32, 3f32, 4);
    /// assert_eq!(test_vec.min_max(), (0f32,3f32) )
    /// ```
    fn min_max(&self) -> (T,T)
    {
        let len = *&self.len();
        let mut max_value = T::mixed_min_value();
        let mut min_value = T::mixed_max_value();
        for i in 0..len
        {
            if self[i] < min_value
            {
                min_value = self[i]
            }
            if max_value < self[i]
            {
                max_value = self[i]
            }
        }
        return (min_value, max_value);
    }
}

impl <T: MixedReal> ToRange<T> for Vec<T>{
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let test_vec = Vec::lin_range(2f32, 5f32, 4);
    /// assert_eq!(test_vec.to_range(), 2f32..5f32 )
    /// ```
    #[inline]
    fn to_range( &self ) -> core::ops::Range<T>
    {
        let (min_value, max_value) = self.min_max();
        return core::ops::Range{start: min_value, end: max_value};
    }
}

impl <T: MixedReal + MixedOps + MixedZero> Indices<T> for Vec<T> {
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let test_vec = Vec::lin_range(2f32, 5f32, 4);
    /// assert_eq!(test_vec.indices().to_string(), "[ 0, 1, 2, 3 ]" )
    /// ```
    #[inline(always)]
    fn indices( &self ) -> Vec<T>
    {
        return Self::lin_range(T::mixed_zero(), T::mixed_from_num((self.len()-1usize) as u32), self.len());
    }
}

impl <T: DbMag + DbPow> Decibel<T> for Vec<T>{
    fn mag2db( &mut self )
    {
        for idx in 0..self.len() {
            self[idx]=self[idx].mixed_mag2db();
        }
    }
    fn db2mag( &mut self )
    {
        for idx in 0..self.len() {
            self[idx]=self[idx].mixed_db2mag();
        }
    }
    fn pow2db( &mut self )
    {
        for idx in 0..self.len() {
            self[idx]=self[idx].mixed_pow2db();
        }
    }
    fn db2pow( &mut self )
    {
        for idx in 0..self.len() {
            self[idx]=self[idx].mixed_db2pow();
        }
    }
}

impl <T: MixedNum + MixedZero + MixedOps> Sum<T> for Vec<T>{
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let test_vec = Vec::lin_range(2f32, 5f32, 4);
    /// assert_eq!(test_vec.sum(), 2f32+3f32+4f32+5f32 )
    /// ```
    fn sum( &self ) -> T
    {
        let mut r_val:T = <T>::mixed_zero();
        for idx in 0..self.len() {
            r_val = r_val+self[idx];
        }
        return r_val;
    }
}

impl <T: MixedNum + MixedZero + MixedOps> Mean<T> for Vec<T>{
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let test_vec = Vec::lin_range(2f32, 5f32, 4);
    /// assert_eq!(test_vec.mean(), 3.5f32 )
    /// ```
    fn mean( &self ) -> T
    {
        return self.sum()/T::mixed_from_num(self.len() as i32);
    }
}

impl <T: MixedNum + MixedZero + MixedOps + MixedPowi> Power<T> for Vec<T>{
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let mut test_vec = Vec::lin_range(2f32, 5f32, 4);
    /// test_vec.power();
    /// assert_eq!(test_vec.to_string(), "[ 4, 9, 16, 25 ]" )
    /// ```
    fn power( &mut self )
    {
        for idx in 0..self.len() {
            self[idx]=self[idx].mixed_powi(2);
        }
    }
}

impl <T: MixedNum + MixedZero + MixedOps + MixedPowi> Energy<T> for Vec<T>{
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let test_vec = Vec::lin_range(2f32, 5f32, 4);
    /// assert_eq!(test_vec.energy(), 54f32 )
    /// ```
    fn energy( &self ) -> T
    {
        let mut r_val:T = <T>::mixed_zero();
        for idx in 0..self.len() {
            r_val = r_val+self[idx].mixed_powi(2);
        }
        return r_val;
    }
}

impl <T1: MixedNum, T2: MixedNum + MixedNumConversion<T1> + core::cmp::PartialOrd> Minimum<T1> for Vec<T2>{
    /// Constrain `self` to be  `>= lower_limit`.
    /// 
    /// Computed-in-place.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let mut test_vec = Vec::lin_range(2f32, 5f32, 4);
    /// 
    /// test_vec.minimum(3i32);
    /// assert_eq!(test_vec.to_string(), "[ 3, 3, 4, 5 ]" )
    /// ```
    fn minimum( &mut self, lower_limit:T1 )
    {
        let lower_limit = T2::mixed_from_num(lower_limit);

        for idx in 0..self.len() {
            if self[idx] < lower_limit {
                self[idx]=lower_limit
            }
        }
    }
}

impl <T1: MixedNum, T2: MixedNum + MixedNumConversion<T1> + core::cmp::PartialOrd> Maximum<T1> for Vec<T2>{
    /// Constrain `self` to be `<= upper_limit`.
    /// 
    /// Computed-in-place.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let mut test_vec = Vec::lin_range(2f32, 5f32, 4);
    /// 
    /// test_vec.maximum(4i32);
    /// assert_eq!(test_vec.to_string(), "[ 2, 3, 4, 4 ]" )
    /// ```
    fn maximum( &mut self,uppper_limit:T1 )
    {
        let uppper_limit = T2::mixed_from_num(uppper_limit);

        for idx in 0..self.len() {
            if uppper_limit < self[idx] {
                self[idx]=uppper_limit
                
            }
        }
    }
}

impl <T1: MixedNum, T2: MixedNum + MixedNumConversion<T1> + core::cmp::PartialOrd> Clip<T1> for Vec<T2>{
    /// Clip all values to the `{lower_limit, uppeer_limit}` range.
    /// 
    /// Computed-in-place.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// use mixed_num::*;
    /// let mut test_vec = Vec::lin_range(2f32, 5f32, 4);
    /// 
    /// test_vec.clip(3i32,4i32);
    /// assert_eq!(test_vec.to_string(), "[ 3, 3, 4, 4 ]" );
    /// 
    /// 
    /// // Example plot
    /// let x_vec = Vec::lin_range(0f32, f32::mixed_tau(), 64);
    /// let mut y_vec = x_vec.clone();
    /// 
    /// y_vec.cos();
    /// 
    /// let mut z_vec = y_vec.clone();
    /// z_vec.clip(-0.2f32, 0.6f32);
    //
    /// x_vec.plot_multiple(&[&y_vec, &z_vec], "./figures/clip_test.png", "Clip","Phase [rad]", "y", &["y", "z"] );
    /// ```
    /// 
    /// The resulting plot is shown below.
    /// 
    /// ![Alt version](https://raw.githubusercontent.com/norsk-datateknikk/NDSP/main/figures/clip_test.png)
    fn clip( &mut self, lower_limit:T1, uppper_limit:T1 )
    {
        let lower_limit = T2::mixed_from_num(lower_limit);
        let uppper_limit = T2::mixed_from_num(uppper_limit);

        for idx in 0..self.len() {
            if self[idx] < lower_limit {
                self[idx]=lower_limit
            }
            else if uppper_limit < self[idx] {
                self[idx]=uppper_limit
                
            }
        }
    }
}

impl <T: MixedReal + MixedZero + MixedPowi + MixedNumSigned + MixedTrigonometry + MixedSqrt + MixedWrapPhase +
         MixedOps + MixedPi > HilbertTransform<T> for Vec<T>
    where T: MixedNumConversion<T>
{
    /// Compute the Discrete Hilbert Transform (DHT).
    /// 
    /// The transform computes the analytical signal from a real-only signal.
    /// 
    /// \[1\] [L. Marple, Computing the Discrete-Time “Analytic” Signal via FFT, IEEE, 1999](https://ieeexplore.ieee.org/document/782222)
    /// 
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// use mixed_num::*;
    /// 
    /// 
    /// let n = 128;
    /// let mut signal = Vec::lin_range(0f32, 2f32*f32::mixed_tau(), n);
    /// signal.cos();
    /// 
    /// let mut buffer = Vec::<Cartesian<f32>>::zeros(n);
    /// 
    /// signal.hilbert(&mut buffer);
    /// 
    /// buffer.plot_psd(1e3, "./figures/plot_hilbert.png", "PSD of Analytical Signal");
    /// 
    /// ```
    /// 
    /// The resulting plot is shown below.
    /// 
    /// ![Alt version](https://raw.githubusercontent.com/norsk-datateknikk/NDSP/main/figures/plot_hilbert.png) 
    fn hilbert(&self, output_buffer: &mut Vec<Cartesian<T>>)
    {        
        output_buffer.copy_from_buffer(self);

        output_buffer.fft();
        
        for idx in 1..(output_buffer.len()/2)
        {
            output_buffer[idx] = output_buffer[idx]*T::mixed_from_num(2);
        }
        for idx in output_buffer.len()/2+1..output_buffer.len()
        {
            output_buffer[idx] = Cartesian::mixed_zero();
        }

        output_buffer.ifft();
    }
}

/*
#[cfg(any(feature = "std"))]
impl <T> traits::FromBinary for Vec<T>
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

// We prefer doctests, as they also provide documentation.
#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test() {
        assert_eq!(true, true )
    }
}

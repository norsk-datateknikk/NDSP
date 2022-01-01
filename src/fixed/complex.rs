//--------------------------------------------------------------//
// Copyright 2021 Norsk Datateknikk AS                          //
//--------------------------------------------------------------//
// This file is subject to the terms and conditions defined in  //
// file 'LICENSE', which is part of this source code package.   //
//--------------------------------------------------------------//

extern crate alloc;
extern crate num;
extern crate fixed_trigonometry;
use fixed::traits::Fixed;
use fixed_trigonometry as trig;

#[cfg(any(feature = "std"))]
use std::fs::File;
#[cfg(any(feature = "std"))]
use std::io::{BufReader, Read};

use crate::traits;
use crate::traits::*;

/// Numeric vector of real, fixed-point numbers.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Vec<T>
{
    vec: alloc::vec::Vec<T>
}

impl<T> Vec<T> {
    /// Allocate a memmory for a vector of a certain capacity.
    /// 
    /// ## Arguments
    /// 
    /// * `capacity` - The capacity of the new vector.
    ///
    #[allow(dead_code)]
    fn new_with_capacity( capacity: usize ) -> Vec<T>
    {
        let mut vec = alloc::vec::Vec::<T>::new(); 
        vec.reserve_exact(capacity);
        Vec {
            vec: vec,
        }
    }
}


impl <T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    /// Trait for returning an indexed value of the array.
    #[inline]
    fn index(&self, index: usize) -> &T {
        return &self.vec[index];
    }
}

impl <T> core::ops::IndexMut<usize> for Vec<T> {
    /// Trait for returning a mutable reference to indexed item.
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut T {
        return &mut self.vec[index];
    }
}

impl <T> traits::Len for Vec<T> {
    /// Function returning the size of the vector.
    /// 
    /// The size is the number of items with values.
    #[inline]
    fn len(&self) -> usize {
        return self.vec.len();
    }
}

impl <T> traits::Cap for Vec<T> {
    /// Function returning the capacity of the vector.
    /// <>
    /// The capacity is the maximum number of items the vector can hold.
    #[inline]
    fn capacity(&self) -> usize {
        return self.vec.capacity();
    }
}

impl <T> traits::PushBack<T> for Vec<T> {
    /// Push a value to the back of the vector.
    #[inline]
    fn push_back(&mut self, value: T) {
        self.vec.push(value);
    }
}

impl<T:fixed::traits::Fixed> Vec<num::complex::Complex<T>> {
    /// Allocate a memmory for a vector of a certain capacity.
    ///
    #[allow(dead_code)]
    fn new_from_real( real_vec: crate::fixed::real::Vec<T> ) -> Vec<num::complex::Complex<T>>
    {
        let mut vec = Vec::<num::complex::Complex<T>>::new_with_capacity(real_vec.len());

        for idx in 0..real_vec.len()
        {
            vec.push_back(num::complex::Complex::new(T::from_num(real_vec[idx]), T::from_num(0)))
        }
        return vec;
    }
}

impl <T: fixed::traits::FixedSigned> traits::Abs for Vec<num::complex::Complex<T>> {
    /// Take the elemtent-wise absolute value.
    fn abs(&mut self) {
        for idx in 0..self.len() {
            self[idx].re = trig::complex::abs(self[idx]);
            self[idx].im = T::from_num(0);
        }
    }
}

/*
impl<T:fixed::traits::FixedSigned>  Vec<num::complex::Complex<T>> {
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
    /// use ndsp;
    /// use fixed::{types::extra::U28, FixedI32};
    /// 
    /// let omega =  FixedI32::<U28>::from_num(0.8);
    /// let theta =  FixedI32::<U28>::from_num(0);
    /// 
    /// let signal = ndsp::fixed::complex::Vec::osc(omega, theta, 4);
    /// 
    /// println!("Signal {:?}", signal);
    /// 
    /// assert_eq!{ -0.2831852, -0.2831853 };
    /// ``` 
    #[allow(dead_code)]
    pub fn osc( angular_freq_rad: T, phase_rad: T, numb: usize ) -> Vec<num::complex::Complex<T>>
    {
        let mut vec = Vec::new_with_capacity(numb);
        let mut phase_rad_inc= phase_rad;
        for _i in 0..numb
        {   
            // let (real,imag ) = cordic::sin_cos(phase_rad_inc); //! Requires cordic number trait.
            let real = trig::cos( phase_rad_inc );
            let imag = trig::sin( phase_rad_inc );

            vec.push_back( num::Complex::new( real, imag ) );

            phase_rad_inc = phase_rad_inc + angular_freq_rad;
            phase_rad_inc = trig::wrap_phase(phase_rad_inc);
        }
        return vec;
    }
}
*/

impl <T:fixed::traits::FixedSigned> traits::Fft for Vec<num::complex::Complex<T>> {
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
impl <T> traits::FromFile for Vec<num::complex::Complex<T>>
    where T: Fixed
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

        let mut temp_complex = num::complex::Complex::new(T::from_num(0), T::from_num(0));

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
                temp_complex.re = T::from_num(f32::from_le_bytes(buffer));
            }
            else {
                // Use `from_be_bytes` if numbers in file is big-endian
                temp_complex.im = T::from_num(f32::from_le_bytes(buffer));
                vec.push_back(temp_complex);
            }
            counter +=1;
        }

        return vec;
    }
}
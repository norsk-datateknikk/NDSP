extern crate alloc;

use alloc::vec::Vec;

use mixed_num::traits::*;
use mixed_num::complex::*;

use crate::*;

/// Check if x is a power of two.
/// 
/// ## Argument
/// 
/// * `x` - The number to check.
/// 
/// ## Example
/// 
/// ```
/// use ndsp::vec::complex::*;
/// 
/// assert_eq!( is_power_of_two(6), false  );
/// assert_eq!( is_power_of_two(4), true );
/// ```
pub fn is_power_of_two<T>( x: T) -> bool
    where T: num::traits::int::PrimInt
{
    if  T::zero()<x && (x & (x - T::one())) == T::zero()
    {
        return true
    }
    return false
}

/// Calculate the base 2 logarithm of x.
/// 
/// ## Argument
/// 
/// * `x` - The number on which to calculate the base 2 logarithm.
/// 
/// ## Example
/// 
/// ```
/// use ndsp::vec::complex::*;
/// 
/// assert_eq!( log2(8), 3 );
/// ```
pub fn log2( x: usize ) -> usize
{
  let mut k: usize = x;
  let mut i = 0;
  while k != 0
  {
    k >>= 1;
    i+=1;
  }
  return i - 1;
}

/// Bit-reverse the order of the input array. Computed in place.
/// 
/// ## Arguments
/// 
/// * `arr` - A mutable reference to the array to do the computation on, and store the result in.
/// 
fn bitreverse_order<T>( arr: &mut [Cartesian<T>] )
    where T: core::marker::Copy
{
    let n:usize = arr.len();
    let mut target_index:usize = 0;

    for index in 0..n
    {
        if index<target_index
        {
            let temp = arr[target_index];

            arr[target_index] = arr[index];
            arr[index] = temp;
        }

        let mut mask:usize = n;
        mask >>=1;
        
        // Bit reverse index
        while target_index & mask != 0
        {
	        target_index &= !mask;

            mask >>=1;
        }
        target_index |= mask;
    }
}

/// Calculate the Raddix-2 FFT for fixed point vectors.
/// - Scaled for each butterfly computation.
/// - Requires input size to be a power of two.
/// - Computed-in-place.
/// - Decimation-in-freqency.
/// 
/// The function computes the twiddle factor each time it is called, which is suboptimal for repeating computations.
/// 
/// ## Arguments
/// 
/// * `vec` - A mutable reference to the vector to do the computation on, and store the result in.
/// 
/// ## Example
/// 
/// ```
/// 
/// use fixed::FixedI32 as F;
/// use fixed::types::extra::U28 as U;
/// use mixed_num::Cartesian;
/// use ndsp::*;
/// use ndsp::complex::fft;
/// 
/// const N:usize = 4;
/// let mut arr  = vec![ Cartesian::new(1f32, 0f32 ); N  ];
///
/// arr[3].re = 0f32;
/// 
/// fft( &mut arr );
/// assert_eq!( arr, vec![  Cartesian::new(0.75, 0.0  ),
///                         Cartesian::new(0.0, -0.25 ),
///                         Cartesian::new(0.25, 0.0  ),
///                         Cartesian::new(0.0,  0.25 )] );
/// ```
pub fn fft<T>( array: &mut [Cartesian<T>] )
    where T: MixedReal + MixedNumSigned + MixedTrigonometry + MixedSqrt + MixedWrapPhase + MixedOps + MixedPi + MixedZero + MixedPowi
{
    // Process fft.
    fft_processor(array, T::mixed_from_num(1));

    // Decimation-in-freqency.
    bitreverse_order(array); // Bitreverse order
}

/// Calculate the Raddix-2 Inverse FFT for fixed point vectors.
/// - Scaled for each butterfly computation.
/// - Requires input size to be a power of two.
/// - Computed-in-place.
/// - Decimation-in-freqency.
/// 
/// The function computes the twiddle factor each time it is called, which is suboptimal for repeating computations.
/// 
/// ## Arguments
/// 
/// * `vec` - A mutable reference to the vector to do the computation on, and store the result in.
/// 
/// ## Example
/// 
/// ```
/// use mixed_num::Cartesian;
/// use ndsp::*;
/// use ndsp::complex::ifft;
///  
/// use fixed::FixedI32 as F;
/// use fixed::types::extra::U28 as U;
/// 
/// const N:usize = 4;
/// let mut arr  = vec![ Cartesian::new(1f32, 0f32 ); N  ];
///
/// arr[3].re = 0f32;
/// 
/// ifft( &mut arr );
/// assert_eq!( arr, vec![  Cartesian::new(0.75, 0.0  ),
///                         Cartesian::new(0.0,  0.25 ),
///                         Cartesian::new(0.25, 0.0  ),
///                         Cartesian::new(0.0, -0.25 )] );
/// ```
pub fn ifft<T>( vec: &mut Vec<Cartesian<T>> )
    where T: MixedReal + MixedNumSigned + MixedTrigonometry + MixedSqrt + MixedWrapPhase + MixedOps + MixedPi + MixedZero + MixedOps + MixedPowi
{
    // Process fft.
    fft_processor(vec, T::mixed_from_num(-1));
    // Decimation-in-freqency.
    bitreverse_order(vec); // Bitreverse order
}

/// Butterfly computation for decimate-in-frequeny.
/// 
/// ## Arguments
/// 
/// * `a` - input/output.
/// * `b` - input/output.
/// * `w` - twiddle factor.
/// 
fn butterfly_df<T>( a: &mut Cartesian<T>, b: &mut Cartesian<T>, w:Cartesian<T> )
    where T: MixedNum + MixedNumSigned + MixedTrigonometry + MixedSqrt + MixedWrapPhase + MixedOps + MixedPowi
{
    let temp_a  = mixed_num::add(*a,*b);
    let temp_b  = mixed_num::mul_cartesian(mixed_num::sub(*a, *b), w);
    
    *a = temp_a;
    *b = temp_b;
}

pub fn calculate_twiddle_factors<T>( n: usize, dir: T) -> crate::Vec<Cartesian<T>>
    where T: MixedNum + MixedNumSigned + MixedOps + MixedZero + MixedTrigonometry + MixedSqrt + MixedPi + MixedWrapPhase
{
    // Create heap-allocated vector
    let mut w = crate::Vec::<Cartesian<T>>::new_with_capacity(n/2);

    // Calculate Twiddle factor W.

    let mut angle:T = dir*-<T>::mixed_tau();
    for _i in 0..log2(n)
    {
        angle = angle / <T>::mixed_from_num(2);
    }

    let mut phase_inc = T::mixed_zero();
    for _i in 0..n/2
    {
        // Calculate twiddle factor for W_i.
        let (imag, real) = phase_inc.mixed_sincos();

        phase_inc += angle;

        w.push_back( Cartesian::new( real, imag ) );
    }
    return w;
}

/// Shared fft processor for fft and ifft.
/// Requires bit-reversion afterwards.
fn fft_processor<T>( array: &mut [Cartesian<T>], dir: T )
    where T: MixedReal + MixedNumSigned + MixedTrigonometry + MixedSqrt + MixedWrapPhase + MixedOps + MixedPi + MixedZero + MixedPowi
{
    let n = array.len();

    let w = calculate_twiddle_factors(n, dir);

    // Number of butterfly computations per block.
    let mut num_butt:   usize = n/2;
    // Number of blocks.
    let mut num_blocks: usize = 1;
    // Bumber of stages (Left-to-right movement).
    let stages = log2(n);
    let mut w_idx_step_size = 1;

    // Iterate over stages
    for _stage in 1..=stages
    {
        // Iterate over blocks.
        for block in 0..num_blocks
        {   
            // Calculate indexes
            let pa = (block*n)/num_blocks;
            let pb = (block*n)/num_blocks + num_butt;

            // Iterate over butterflies in current block.
            for butt in 0..num_butt
            {
                // Scale values to avoid overflow.
                let mut a = mixed_num::div_scalar_cartesian( array[pa+butt], T::mixed_from_num(2) );
                let mut b = mixed_num::div_scalar_cartesian( array[pb+butt], T::mixed_from_num(2) );

                let w_idx:usize = w_idx_step_size*(butt);
                let w_temp = w[ w_idx ];
                
                butterfly_df( &mut a, &mut b, w_temp );
                
                array[pa+butt] = a;
                array[pb+butt] = b;
            }
        }
        w_idx_step_size *= 2;
        num_blocks *= 2;
        num_butt   /= 2;
    }
}
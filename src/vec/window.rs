//----------------------//
// Norsk Datateknikk AS //
//----------------------//

use crate::*;
use mixed_num::*;

impl <T: MixedNum + MixedOps + MixedNumConversion<usize> + MixedConsts + MixedCos + MixedZero + MixedOne + MixedAbs> Vec<T>
{
    /// Generate a Hamming window funciton.
    /// 
    /// ```
    /// use ndsp::*;
    /// use mixed_num::Cartesian;
    /// 
    /// let mut vec = Vec::<f32>::hamming(512);
    /// 
    /// vec.simple_plot("./figures/hamming_test.png", "Hamming Window Function");
    /// 
    /// let c_vec = Vec::<Cartesian<f32>>::new_from_real(vec);
    /// c_vec.plot_psd( 1f32, -180f32, "./figures/hamming_psd_test.png", "Hamming Window Function" );
    /// ```
    /// 
    /// The resulitg plots are shown below.
    /// 
    /// ![Alt version](https://raw.githubusercontent.com/norsk-datateknikk/NDSP/main/figures/hamming_test.png)
    /// 
    /// ![Alt version](https://raw.githubusercontent.com/norsk-datateknikk/NDSP/main/figures/hamming_psd_test.png)
    pub fn hamming(len: usize) -> Self {
        let a0: T = T::mixed_from_num(25) / T::mixed_from_num(46);

        let mut r_vec = crate::Vec::<T>::new_with_capacity(len);
        let size:T = T::mixed_from_num(len) - T::mixed_one();

        for idx in 0..len {
            let n = T::mixed_from_num(idx);
            let w = a0 - (T::mixed_one() - a0) * (T::mixed_tau() * n / size).mixed_cos();
            r_vec.push_back(w);
        }
        return r_vec;
    }

    /// Generate a Bartlett window funciton.
    /// 
    /// ```
    /// use ndsp::*;
    /// use mixed_num::Cartesian;
    /// 
    /// let mut vec = Vec::<f32>::bartlett(512);
    /// 
    /// vec.simple_plot("./figures/bartlett_test.png", "Bartlett Window Function");
    /// 
    /// let c_vec = Vec::<Cartesian<f32>>::new_from_real(vec);
    /// c_vec.plot_psd( 1f32, -110f32, "./figures/bartlett_psd_test.png", "Bartlett Window Function" );
    /// ```
    /// 
    /// The resulitg plots are shown below.
    /// 
    /// ![Alt version](https://raw.githubusercontent.com/norsk-datateknikk/NDSP/main/figures/bartlett_test.png)
    /// 
    /// ![Alt version](https://raw.githubusercontent.com/norsk-datateknikk/NDSP/main/figures/bartlett_psd_test.png)
    pub fn bartlett(len: usize) -> Self {
        let mut r_vec = crate::Vec::<T>::new_with_capacity(len);
        
        let half_len = T::mixed_from_num(len.clone())/T::mixed_from_num(2usize);

        for idx in 0..len {
            let n = T::mixed_from_num(idx);
            let w = T::mixed_one()-((n-half_len)/half_len).mixed_abs();
            r_vec.push_back(w);
        }
        return r_vec;
    }
    /// Generate a Nuttal window funciton.
    /// 
    /// ```
    /// use ndsp::*;
    /// use mixed_num::Cartesian;
    /// 
    /// let mut vec = Vec::<f32>::nuttall(512);
    /// 
    /// vec.simple_plot("./figures/nuttall_test.png", "Nuttall Window Function");
    /// 
    /// let c_vec = Vec::<Cartesian<f32>>::new_from_real(vec);
    /// c_vec.plot_psd( 1f32, -200f32, "./figures/Nuttall_psd_test.png", "Nuttall Window Function" );
    /// ```
    /// 
    /// The resulitg plots are shown below.
    /// 
    /// ![Alt version](https://raw.githubusercontent.com/norsk-datateknikk/NDSP/main/figures/bartlett_test.png)
    /// 
    /// ![Alt version](https://raw.githubusercontent.com/norsk-datateknikk/NDSP/main/figures/bartlett_psd_test.png)
    pub fn nuttall(len: usize) -> Self {
        let mut r_vec = crate::Vec::<T>::new_with_capacity(len);

        let len = T::mixed_from_num(len);
        for idx in 0..len.mixed_to_num() {
            let n = T::mixed_from_num(idx);
            let w = T::mixed_from_num(0.355768)
                    - T::mixed_from_num(0.487396) * (T::mixed_from_num(1)*T::mixed_tau()*n/(len-T::mixed_one())).mixed_cos()
                    + T::mixed_from_num(0.144232) * (T::mixed_from_num(2)*T::mixed_tau()*n/(len-T::mixed_one())).mixed_cos()
                    - T::mixed_from_num(0.012604) * (T::mixed_from_num(3)*T::mixed_tau()*n/(len-T::mixed_one())).mixed_cos();
            r_vec.push_back(w);
        }
        return r_vec;
    }
    /// Generate a Blackman window funciton.
    /// 
    /// ```
    /// use ndsp::*;
    /// use mixed_num::Cartesian;
    /// 
    /// let mut vec = Vec::<f32>::blackman(512);
    /// 
    /// vec.simple_plot("./figures/blackman_test.png", "Blackman Window Function");
    /// 
    /// let c_vec = Vec::<Cartesian<f32>>::new_from_real(vec);
    /// c_vec.plot_psd( 1f32, -180f32, "./figures/blackman_psd_test.png", "Blackman Window Function" );
    /// ```
    /// 
    /// The resulitg plots are shown below.
    /// 
    /// ![Alt version](https://raw.githubusercontent.com/norsk-datateknikk/NDSP/main/figures/blackman_test.png)
    /// 
    /// ![Alt version](https://raw.githubusercontent.com/norsk-datateknikk/NDSP/main/figures/blackman_psd_test.png)
    pub fn blackman(len: usize) -> Self {
        let a0: T = T::mixed_from_num(7938) / T::mixed_from_num(18608);
        let a1: T = T::mixed_from_num(9240) / T::mixed_from_num(18608);
        let a2: T = T::mixed_from_num(1430) / T::mixed_from_num(18608);

        let mut r_vec = crate::Vec::<T>::new_with_capacity(len);
        let size:T = T::mixed_from_num(len) - T::mixed_one();

        for idx in 0..len {
            let n = T::mixed_from_num(idx);
            let w = a0 - a1 * (T::mixed_tau() * n / size).mixed_cos()
                      + a2 * (T::mixed_from_num(2) * T::mixed_tau() * n / size).mixed_cos();
            r_vec.push_back(w);
        }
        return r_vec;
    }
}

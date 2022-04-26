//----------------------//
// Norsk Datateknikk AS //
//----------------------//

use crate::*;
use mixed_num::*;

impl <T: MixedNum + MixedOps + MixedNumConversion<usize> + MixedConsts + MixedCos> Vec<T>
{
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
    /// c_vec.plot_psd( 1f32, "./figures/blackman_psd_test.png", "Blackman Window Function" );
    /// ```
    /// 
    /// The resulitg plot is shown below.
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
            let v = a0 - a1 * (T::mixed_tau() * n / size).mixed_cos()
                        + a2 * (T::mixed_from_num(2) * T::mixed_tau() * n / size).mixed_cos();
            r_vec.push_back(v);
        }
        return r_vec;
    }
}

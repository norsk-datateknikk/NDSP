//--------------------------------------------------------------//
// Norsk Datateknikk AS 2021                                    //
//--------------------------------------------------------------//
// This file is subject to the terms and conditions defined in  //
// file 'LICENSE', which is part of this source code package.   //
//--------------------------------------------------------------//

mod ndsp;
use num::complex::Complex;

fn main()
{
    /*
    struct Scan
    {
        sample_rate_hz: i32,
        frame: Vec<i8>
    }
    */

    let test_vec = ndsp::linspace::<f32>(0_f32, 100_f32, 600);
    let vector2 = ndsp::sin::<f32>( test_vec );
    //let vector3 = exp!(test_vec); //TODO
    println!( "{:?}", vector2 );

    let mut c_vector: Vec<ndsp::C32I>;

    //let tmp2 =  complex!;
    //println!( "{:?}", tmp );
    
}

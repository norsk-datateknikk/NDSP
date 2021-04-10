//--------------------------------------------------------------//
// Norsk Datateknikk AS 2021                                    //
//--------------------------------------------------------------//
// This file is subject to the terms and conditions defined in  //
// file 'LICENSE', which is part of this source code package.   //
//--------------------------------------------------------------//

mod ndsp;

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
    //let vector2 = ndsp::sin::<f32>( test_vec );

    let c_vec = vec![ C32F!(1,2), C32F!(3,6) ];
    
    println!( "{:?}", c_vec );        
}

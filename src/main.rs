//--------------------------------------------------------------//
// Norsk Datateknikk AS 2021                                    //
//--------------------------------------------------------------//
// This file is subject to the terms and conditions defined in  //
// file 'LICENSE', which is part of this source code package.   //
//--------------------------------------------------------------//

mod ndsp;

fn main()
{
    let test_vec = ndsp::linspace::<f32>(0_f32, 100_f32, 600);
    let vector2 = ndsp::sin::<f32>( test_vec );
    println!( "{:?}", vector2 );
}

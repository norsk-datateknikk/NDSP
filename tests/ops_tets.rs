use ndsp::*;
use mixed_num::*;

#[test]
fn test_vec_external_interface() {
    let signal0 = Vec::lin_range(0f32, 9f32, 10);
    
    let result = signal0*2f32;
    assert_eq!(result.to_string(), "[ 0, 2, 4, 6, 8, 10, 12, 14, 16, 18 ]" )
}


#[test]
fn test_cartesian_vec_ang_external_interface() {
    let omega = <f32>::mixed_pi()/f32::mixed_from_num(8i32);
    let theta = 0f32; 
    
    let signal = Vec::osc(omega, theta, 4);
    let ang = signal.ang();
    assert_eq!(ang.to_string(), "[ 0, 0.39269912, 0.7853982, 1.1780972 ]" )
}
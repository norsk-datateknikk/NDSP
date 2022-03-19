use plotters::prelude::*;
use ndsp::*;
use ndsp::Vec;



use rustfft::{FftPlanner};


/// Plots comparison between various sqrt implementations.
#[test]
fn fft_plot(  )
{
    fn plot( vec: &ndsp::Vec<f64>, path: &str, caption: &str, xlabel: &str, ylabel: &str ) -> Result<(), Box<dyn std::error::Error>>
    {
        let root = BitMapBackend::new(path, (1000, 500)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption(caption, ("sans-serif", 25).into_font())
            .margin(10i32)
            .x_label_area_size(40i32)
            .y_label_area_size(50i32)
            .build_cartesian_2d( vec.indices().to_range(), vec.to_range() )?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .bold_line_style(&WHITE.mix(0.3))
            .y_desc(ylabel)
            .x_desc(xlabel)
            .axis_desc_style(("sans-serif", 15))
            .draw()?;

        chart.configure_mesh().draw()?;

        // Draws a sinle line
        chart
            .draw_series( LineSeries::new(
                vec.to_touples(),
                &BLUE) )?;

        Ok(())
    }

    let N = 8;

    //___________________________________________________________________________
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(N);

    let complex_vec = Vec::osc(1f64,0f64,N);
    let mut buffer = complex_vec.to_alloc_vec().clone(); 
    fft.process(&mut buffer);

    let mut complex_vec =  Vec::new_from_vec(buffer);

    complex_vec.abs();
    let abs_vec = complex_vec.re();

    plot(&abs_vec, "./figures/rustfft_plot.png", "Rust FFT example", "idx", "Power [dB]").unwrap();

    //___________________________________________________________________________



    let complex_vec = Vec::osc(1f64,0f64,8);
    let real_component = complex_vec.re();

    let mut complex_vec = real_component.as_complex();
    
    plot(&complex_vec.re(), "./figures/osc_plot_real.png", "Osc real", "idx", "").unwrap();
    plot(&complex_vec.im(), "./figures/osc_plot_imag.png", "Osc imag", "idx", "").unwrap();

    complex_vec.fft();
    
    complex_vec.abs();
    let mut abs_vec = complex_vec.re();
    //abs_vec.mag2db();

    plot(&abs_vec, "./figures/fft_plot.png", "FFT example", "idx", "Power [dB]").unwrap();
}
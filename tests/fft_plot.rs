use plotters::prelude::*;
use ndsp::*;

/// Plots comparison between various sqrt implementations.
#[test]
fn fft_plot(  )
{
    fn plot( vec: &ndsp::Vec<f32>, path: &str, caption: &str ) -> Result<(), Box<dyn std::error::Error>>
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
            .y_desc("y")
            .x_desc("x [idx]")
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

    let mut complex_vec = Vec::osc(0.2f32,0f32,256);
    
    plot(&complex_vec.re(), "./figures/osc_plot_real.png", "Osc real").unwrap();
    plot(&complex_vec.im(), "./figures/osc_plot_imag.png", "Osc imag").unwrap();
    
    let mut absolute = complex_vec.clone();
    absolute.abs();
    plot(&absolute.re(), "./figures/osc_plot_abs.png", "Osc abs").unwrap();
    

    complex_vec.fft();
    complex_vec.abs();
    plot(&complex_vec.re(), "./figures/fft_plot.png", "FFT example").unwrap();
}
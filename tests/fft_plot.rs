use plotters::prelude::*;
use ndsp::*;
use ndsp::Vec;

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

    let n = 1024;
    let mut complex_vec = Vec::osc(1f64,0f64,n);

    complex_vec.fft();
    complex_vec.im();

    complex_vec.mag();

    plot(&complex_vec.re(), "./figures/fft_plot.png", "FFT example", "idx", "Magnitude").unwrap();


    //&complex_vec.re().simple_plot("./figures/fft_plot_2.png", "FFT example 2")
}

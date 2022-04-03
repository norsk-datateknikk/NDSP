use plotters::prelude::*;
use ndsp::*;

#[test]
fn test_vec_plot(  )
{
    fn plot( vec: &ndsp::Vec<f32> ) -> Result<(), Box<dyn std::error::Error>>
    {
        let root = BitMapBackend::new("./figures/test_vector_plot.png", (1000, 500)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption("Test Plot", ("sans-serif", 25).into_font())
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

    let test_vec = Vec::lin_range(0f32, 1f32, 64);
    plot(&test_vec).unwrap();
}
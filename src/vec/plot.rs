//----------------------//
// Norsk Datateknikk AS //
//----------------------//

use plotters::prelude::*;

use std::vec;
use std::string::String;
use std::boxed::Box;

#[macro_export]
macro_rules! sci_chart {
    ($path:item) => {
        let root = BitMapBackend::new( &*path, (1000, 500)).into_drawing_area();
        root.fill(&WHITE);
        let mut chart = ChartBuilder::on(&root)
            .margin(10i32)
            .x_label_area_size(40i32)
            .y_label_area_size(50i32)
            .build_cartesian_2d( 0f32..1f32, 0f32..1f32 ).unwrap();
        
        return chart;
    };
}

/// Plots comparison between various sqrt implementations.
/// 
/// ## Example
/// 
/// ```
/// use ndsp::plot::*;  
/// let path = "figures/polynomial_sine_comparison.png"
/// 
/// let mut chart = sci_chart!(path);
/// 
/// let root = BitMapBackend::new( &*path, (1000, 500)).into_drawing_area();
/// root.fill(&WHITE);
/// let mut chart = ChartBuilder::on(&root)
///     .margin(10i32)
///     .x_label_area_size(40i32)
///     .y_label_area_size(50i32)
///     .build_cartesian_2d( 0f32..1f32, 0f32..1f32 ).unwrap();
/// chart..caption("title", ("sans-serif", 25).into_font())
/// 
/// plot(&chart, [0,1], [2,4]) 
/// ```
fn plot<'a, 'b, T, DB, CT>( &chart: &ChartContext<DB, CT>, x: &[T], y: &[T] ) -> Result<(), Box<dyn std::error::Error>> 
    where DB: DrawingBackend, 
          CT: CoordTranslate,
{
    use mixed_num::trigonometry;
    use std::f32::consts::PI as PI;

    chart
        .configure_mesh()
        .x_label_area_size(40i32)
        .y_label_area_size(50i32)
        .build_cartesian_2d( -PI..PI, -1.1f32..1.1f32 )?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("sin(θ)")
        .x_desc("θ")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.configure_mesh().draw()?;

    let std_sin_series = LineSeries::new(
        (-500..=500).map(|x| x as f32 *PI / 500.0).map(|x| (x, x.sin() )),
        &BLUE);

    // Draws a sinle line
    chart
        .draw_series( std_sin_series )?
        .label("f32::sin")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    
    // Draws a sinle line
    chart
        .draw_series(LineSeries::new(
            (-500..=500).map(|x| x as f32 *PI / 500.0).map(|x| (x, 100000.0*f32::abs(x.sin()-trigonometry::sin(x)) )),
            &GREEN,
        ))?
        .label("error*100 000")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
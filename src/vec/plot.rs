//----------------------//
// Norsk Datateknikk AS //
//----------------------//

use plotters::prelude::*;
use crate::*;
use std::boxed::Box;


impl Vec<f32> {
    /// Plots self in its own figure.
    /// 
    /// ## Arguments
    /// 
    /// * `path` - The path and name of the file ot be generated.
    /// * `capion` - The plot caption.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let test_vec = Vec::lin_range(0f32, 1f32, 64);
    /// test_vec.simple_plot("./figures/plot_test.png", "Test Plot");
    /// ```
    #[inline]
    pub fn simple_plot( &self, path: &str, caption: &str ) -> Result<(), Box<dyn std::error::Error>>
    {
        self.plot(path, caption, "x [idx]", "y")
    }

    /// Plots self in its own figure.
    ///
    /// ## Arguments
    /// 
    /// * `path` - The path and name of the file ot be generated.
    /// * `capion` - The plot caption.
    /// * `x_label` - The label shown below the x axis.
    /// * `y_label` - The label shown next to y axis
    /// 
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// let test_vec = Vec::lin_range(0f32, 1f32, 64);
    /// test_vec.plot("./figures/plot_test.png", "Test Plot","x [idx]", "y" );
    /// ```
    #[allow(dead_code)]
    pub fn plot( &self, path: &str, caption: &str, x_label: &str, y_label: &str ) -> Result<(), Box<dyn std::error::Error>>
    {
        let root = BitMapBackend::new(path, (1000, 500)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption(caption, ("sans-serif", 25).into_font())
            .margin(10i32)
            .x_label_area_size(40i32)
            .y_label_area_size(50i32)
            .build_cartesian_2d( self.indices().to_range(), self.to_range() )?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .bold_line_style(&WHITE.mix(0.3))
            .y_desc(y_label)
            .x_desc(x_label)
            .axis_desc_style(("sans-serif", 15))
            .draw()?;

        chart.configure_mesh().draw()?;

        // Draws a sinle line
        chart
            .draw_series( LineSeries::new(
                self.to_touples(),
                &BLUE) )?;

        Ok(())
    }
}



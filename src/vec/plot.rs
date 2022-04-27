//----------------------//
// Norsk Datateknikk AS //
//----------------------//

use mixed_num::*;

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
    /// 
    /// The resulitg plot is shown below.
    /// 
    /// ![Alt version](https://raw.githubusercontent.com/norsk-datateknikk/NDSP/main/figures/plot_test.png) 
    pub fn simple_plot( &self, path: &str, caption: &str ) -> Result<(), Box<dyn std::error::Error>>
    {
        let x_label = "x [idx]";
        let y_label = "y";

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
    /// let y_vec = Vec::lin_range(0f32, 1f32, 64);
    /// let x_vec = Vec::lin_range(0f32, 10f32, 64);
    /// x_vec.plot(&y_vec, "./figures/plot_test.png", "Test Plot","x [idx]", "y" );
    /// ```
    /// 
    /// The resulting plot is shown below.
    /// 
    /// ![Alt version](https://raw.githubusercontent.com/norsk-datateknikk/NDSP/main/figures/plot_test.png) 
    pub fn plot( &self, y_vec: &Self, path: &str, caption: &str, x_label: &str, y_label: &str ) -> Result<(), Box<dyn std::error::Error>>
    {
        let root = BitMapBackend::new(path, (1000, 500)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption(caption, ("sans-serif", 25).into_font())
            .margin(10i32)
            .x_label_area_size(40i32)
            .y_label_area_size(50i32)
            .build_cartesian_2d( self.to_range(), y_vec.to_range() )?;

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
                self.to_xy_touples(y_vec),
                &BLUE) )?;

        Ok(())
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
    /// let y_vec = Vec::lin_range(0f32, 1f32, 64);
    /// let x_vec = Vec::lin_range(0f32, 10f32, 64);
    /// let z_vec = Vec::lin_range(1.5f32, 0f32, 64);
    /// 
    /// x_vec.plot_multiple(&[&y_vec, &z_vec], "./figures/plot_multiple_test.png", "MultiPlot","x [idx]", "y", &["y", "z"] );
    /// ```
    /// 
    /// The resulting plot is shown below.
    /// 
    /// ![Alt version](https://raw.githubusercontent.com/norsk-datateknikk/NDSP/main/figures/plot_multiple_test.png) 
    pub fn plot_multiple( &self, y_vectors: &[&Self], path: &str, caption: &str, x_label: &str, y_label: &str, line_labels: &[&str] ) -> Result<(), Box<dyn std::error::Error>>
    {
        let y_range; 
        {
            let mut min_max_vec = Vec::<f32>::new_with_capacity(2*y_vectors.len());
            for vec in y_vectors
            {
                let (min, max) = vec.min_max();
                min_max_vec.push_back(min);
                min_max_vec.push_back(max);
            }

            y_range = min_max_vec.to_range();
        }
        

        let root = BitMapBackend::new(path, (1000, 500)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption(caption, ("sans-serif", 25).into_font())
            .margin(10i32)
            .x_label_area_size(40i32)
            .y_label_area_size(50i32)
            .build_cartesian_2d( self.to_range(), y_range )?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .bold_line_style(&WHITE.mix(0.3))
            .y_desc(y_label)
            .x_desc(x_label)
            .axis_desc_style(("sans-serif", 15))
            .draw()?;

        chart.configure_mesh().draw()?;

        let colors = [&BLUE, &RED, &GREEN, &MAGENTA, &CYAN];

        for idx in 0..line_labels.len()
        {
            // Line color
            let color = colors[idx%y_vectors.len()];
            // Draws a sinle line.
            chart
                .draw_series( LineSeries::new(
                    self.to_xy_touples(y_vectors[idx]),
                    color.clone()) )?     // Loop through colors.
                .label(line_labels[idx])
                .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], color.clone()));
        }

        // Apply legend
        chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

        Ok(())
    }
}

impl <T:MixedNum + MixedReal + MixedNumSigned + MixedTrigonometry + MixedSqrt + mixed_num::MixedWrapPhase
      + MixedOps + MixedPi + MixedZero + MixedPowi + MixedNumConversion<usize> + MixedNumConversion<f32>> Vec<Cartesian<T>>
{
    /// Calculate the Power Spectral Density (PSD) in linear scale of a signal.
    /// 
    /// Expects the signal length to be a power of two. If not, the signal is zero padded.
    /// 
    /// For signals in the magnitude<1 range, the PSD plot is in dBFS.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use ndsp::*;
    /// use mixed_num::traits::*;
    /// 
    /// let phase_rad = 0f32;
    /// 
    /// let f_sample = 10e3;
    /// let tone_frequency = 2e3;
    /// let angular_frequency = tone_frequency/f_sample*f32::mixed_tau();
    /// 
    /// let signal = Vec::osc(angular_frequency, phase_rad, 256);
    /// 
    /// signal.plot_psd( f_sample, -180f32, "./figures/plot_psd.png", "Power Spectral Density" );
    /// ```
    /// 
    /// The resulting plot is shown below.
    /// 
    /// ![Alt version](https://raw.githubusercontent.com/norsk-datateknikk/NDSP/main/figures/plot_psd.png) 
    pub fn plot_psd( &self, sample_rate_hz: T, floor_db: T, path: &str, caption: &str ) -> Result<(), Box<dyn std::error::Error>>
        where Vec<T>: Decibel<T>,
              Vec<Cartesian<T>>: Psd<T>,
              f32: MixedNumConversion<T>
    {
        let psd = self.psd();
        let mut psd: Vec<f32> = psd.vec_to_num();

        let sample_rate_hz:f32 = sample_rate_hz.mixed_to_num();
        let step_hz: f32 = 2f32*sample_rate_hz/(psd.len() as f32);

        let x_vec = Vec::lin_range(0f32, sample_rate_hz-step_hz, psd.len());
        psd.pow2db();

        psd.minimum( floor_db);
    
        x_vec.plot(&psd, path, caption, "Frequency [Hz]", "dB" )
    }
}


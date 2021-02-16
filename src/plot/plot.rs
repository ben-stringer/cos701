// use plotters::prelude::*;
// use std::ops::Range;
// use plotters::coord::types::RangedCoordf64;
// use plotters::coord::Shift;

// /// One approach, pass in a closure that accepts the chart.
// /// e.g.
// /// ```
// /// let rng = crate::rng::uniform::Uniform701::new();
// /// draw("example.png", "", 0..1, 0..1,
// ///         |chart|
// ///             chart.draw_series(
// ///                 (0..1000)
// ///                     .map(|_| (rng.next(), rng.next()))
// ///                     .inspect(|coord| println!("{:?}", coord))
// ///                     .map(|coord| Circle::new(coord, 1, &BLUE))));
// /// ```
// pub fn draw(path : &'static str,
//             caption : &str,
//             x_range: Range<f64>,
//             y_range: Range<f64>,
//             fun : Box<dyn Fn(&mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordf64, RangedCoordf64>>) -> Result<(), Box<dyn std::error::Error>>>)
//             -> Result<(), Box<dyn std::error::Error>> {
//     let root = BitMapBackend::new(path, (640, 480))
//         .into_drawing_area();
//     root.fill(&WHITE)?;
//
//     let mut chart = ChartBuilder::on(&root)
//         .caption(caption, ("sans-serif", 50).into_font())
//         .margin(16)
//         .x_label_area_size(30)
//         .y_label_area_size(30)
//         .build_cartesian_2d(x_range, y_range)?;
//
//     fun(&mut chart);
//
//     Ok(())
// }

/*
/// The preferred approach.  Stash the chart in a struct and expose a simpler API based on my needs.
/// e.g.,
/// ```
/// let rng = crate::rng::uniform::Uniform701::new();
/// let plot = Plot::new();
/// plot.draw_series((0..1000)
///       .map(|_| (rng.next(), rng.next()))
///       .inspect(|coord| println!("{:?}", coord))
///       .map(|coord| Circle::new(coord, 1, &BLUE))));
/// ```
/// I don't care about root, but I have to keep it otherwise it goes out of scope.
/// Can't figure out how to make this work.  Can't move it while it's borrowed,
/// but can't create a Plot without also creating the chart.
/// I tried making chart an Option and assigning it later, but that also failed.
*/
// pub struct Plot<'a> {
//     root : DrawingArea<BitMapBackend<'a>, Shift>,
//     chart : ChartContext<'a, BitMapBackend<'a>, Cartesian2d<RangedCoordf64, RangedCoordf64>>
// }
//
// impl Plot<'_> {
//     /// Create a new plot
//     ///
//     /// # Arguments
//     /// * `path` - path to the output file
//     /// * `caption` - the text to caption the plot with
//     /// * `xRange` - the range of the x-axis
//     /// * `yRange` - the range of the y-axis
//     pub fn new(path: &'static str, caption: &str, xRange: Range<f64>, yRange: Range<f64>)
//                -> Result<Self, Box<dyn std::error::Error>> {
//         let root = BitMapBackend::new(path, (640, 480))
//                 .into_drawing_area();
//
//         // let root = BitMapBackend::new(path, (640, 480))
//         //     .into_drawing_area();
//         root.fill(&WHITE)?;
//
//         let mut chart = ChartBuilder::on(&root)
//             .caption(caption, ("sans-serif", 50).into_font())
//             .margin(16)
//             .x_label_area_size(30)
//             .y_label_area_size(30)
//             .build_cartesian_2d(xRange, yRange)?;
//         chart.configure_mesh().disable_mesh().draw()?;
//
//         Ok(Self { root, chart })
//     }
// }

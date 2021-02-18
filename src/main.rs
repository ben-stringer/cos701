// use plotters::prelude::*;

mod rng;
mod plot;
mod assignment;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    assignment::assignment1::do_assignment_1()?;

    // // The preferred approach, creating a plot object that keeps track of the chart
    // let mut rng = crate::rng::uniform::Uniform701::new();
    // let plot = plot::plot::Plot::new("example.png", "", 0f64..1f64, 0f64..1f64);
    // // use functions exposed on the plot, not yet shown

    // // An alternate option
    // let rng = crate::rng::uniform::Uniform701::new();
    // plot::plot::draw("example.png", "", 0f64..1f64, 0f64..1f64,
    //      |chart| {
    //          chart.draw_series(
    //              (0..1000)
    //                  .map(|_| (rng.next(),rng.next()))
    //                  .inspect(|coord| println!("{:?}", coord))
    //                  .map(|coord| Circle::new(coord, 1, &BLUE)))?;
    //         Ok(()
    //      });

    // let mut rng = rng::uniform::Uniform701::new();
    // let mut plot = plot::plot::Plot::new("test.png")?;
    // plot.build("foo", 0f64..1f64, 0f64..1f64)?;
    // plot.get_chart().draw_series((0..1000)
    //     .map(|_| (rng.next(), rng.next()))
    //     // .inspect(|coord| println!("{:?}", coord))
    //     .map(|coord| Circle::new(coord, 1, &BLUE)))?;


    Ok(())
}
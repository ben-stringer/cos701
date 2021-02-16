use plotters::prelude::*;

use crate::rng::uniform::Uniform701;
use crate::rng::boxmuller::BoxMullerGaussian701;

pub fn do_assignment_1() -> Result<(), Box<dyn std::error::Error>> {

    let uni = Uniform701::new();
    let mut norm = BoxMullerGaussian701::new(uni);

    let root = BitMapBackend::new("0.png", (640, 480))
        .into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Assignment 1", ("sans-serif", 50).into_font())
        .margin(16)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-5f64..5f64, 0f64..1f64)?;
    chart.configure_mesh().disable_mesh().draw()?;

    chart.draw_series(
        (0..1000)
            .map(|_| norm.next())
            // .inspect(|coord| println!("{:?}", coord))
            .map(|coord| Circle::new(coord, 1, &BLUE)))?;

    Ok(())
}
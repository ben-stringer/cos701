use crate::deranged::DerangedCoordf64;
use plotters::prelude::*;
use std::collections::HashMap;

use crate::rng::boxmuller::BoxMullerGaussian701;
use crate::rng::uniform::Uniform701;

pub fn do_assignment_1() -> Result<(), Box<dyn std::error::Error>> {
    let uni = Uniform701::new();
    let mut norm = BoxMullerGaussian701::new(uni);

    let mut bins: HashMap<i32, i32> = (0..10).map(|i| (i, 0)).collect();

    (0..1000)
        .map(|_| norm.next())
        .map(|v| (v.0 * 10f64).round() as i32)
        .for_each(|k| {
            bins.entry(k).and_modify(|v| {
                *v + 1;
            });
        });

    let root = BitMapBackend::new("0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_range = DerangedCoordf64::from(-5f64..5f64);

    let mut chart = ChartBuilder::on(&root)
        .caption("Assignment 1", ("sans-serif", 50).into_font())
        .margin(16)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x_range.into_segmented(), 0f64..1f64)?;
    chart.configure_mesh().disable_mesh().draw()?;

    chart.draw_series(bins.iter().map(|(k, v)| {
        let x = *k as f64 * 0.1f64;
        let x0 = SegmentValue::Exact(x);
        let x1 = SegmentValue::Exact(x + 0.1);
        Rectangle::new([(x0, 0f64), (x1, *v as f64)], RED.filled())
    }))?;

    Ok(())
}

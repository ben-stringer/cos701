use plotters::prelude::*;

use crate::rng::uniform::Uniform701;
use std::ops::Range;

const L: f64 = 20.0;

pub fn do_assignment_2() -> Result<(), Box<dyn std::error::Error>> {
    println!("Doing assignment 2");

    let mut uni = Uniform701::new();

    // scatter_2d(
    //     "output/assignment2/test.png",
    //     "Assignment 2a, L = 20, n = 500",
    //     0.0..L,
    //     0.0..L,
    //     (0..500).map(|_| (uni.next() * L, uni.next() * L)).collect(),
    // )?;

    // scatter_3d(
    animated_3d(
        "output/assignment2/test.png",
        "Assignment 2a, L = 20, n = 500",
        0.0..L,
        0.0..L,
        0.0..L,
        (0..500)
            .map(|_| (uni.next() * L, uni.next() * L, uni.next() * L))
            .collect(),
    )?;
    Ok(())
}

fn scatter_2d(
    path: &str,
    caption: &str,
    x_range: Range<f64>,
    y_range: Range<f64>,
    points: Vec<(f64, f64)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(path, (1440, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 50).into_font())
        .margin(32)
        .x_label_area_size(32)
        .y_label_area_size(32)
        .build_cartesian_2d(x_range.to_owned(), y_range.to_owned())?;
    chart.configure_mesh().disable_mesh().draw()?;

    chart.draw_series(
        points
            .into_iter()
            .map(|coord| Circle::new(coord, 2, RED.filled())),
    )?;

    Ok(())
}

fn scatter_3d(
    path: &str,
    caption: &str,
    x_range: Range<f64>,
    y_range: Range<f64>,
    z_range: Range<f64>,
    points: Vec<(f64, f64, f64)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(path, (1440, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 50).into_font())
        .margin(32)
        .x_label_area_size(32)
        .y_label_area_size(32)
        .build_cartesian_3d(x_range.to_owned(), y_range.to_owned(), z_range.to_owned())?;
    chart.configure_axes().draw()?;

    chart.draw_series(
        points
            .into_iter()
            .map(|coord| Circle::new(coord, 2, RED.filled())),
    )?;

    Ok(())
}

fn animated_3d(
    path: &str,
    caption: &str,
    x_range: Range<f64>,
    y_range: Range<f64>,
    z_range: Range<f64>,
    points: Vec<(f64, f64, f64)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::gif(path, (1440, 900), 1_000)?.into_drawing_area();

    for i in 0..10 {
        println!("Iteration {}", i);
        let yaw = i as f64 * 0.2;
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption(
                format!("{}, {:.1}", caption, yaw),
                ("sans-serif", 50).into_font(),
            )
            .margin(32)
            .x_label_area_size(32)
            .y_label_area_size(32)
            .build_cartesian_3d(x_range.to_owned(), y_range.to_owned(), z_range.to_owned())?;
        chart.with_projection(|mut pb| {
            pb.yaw = yaw;
            pb.into_matrix()
        });
        chart.configure_axes().draw()?;

        chart.draw_series(
            points
                .to_owned()
                .into_iter()
                .map(|coord| Circle::new(coord, 2, RED.filled())),
        )?;

        root.present()?;
    }

    Ok(())
}

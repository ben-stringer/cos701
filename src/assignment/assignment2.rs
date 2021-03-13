use plotters::prelude::*;

use crate::rand::uniform::Uniform701;
use crate::util::{distance_2d, distance_3d};
use std::error::Error;
use std::ops::Range;

const L: f64 = 20.0;

/// Entry-point for assignment 2
pub fn do_assignment_2() -> Result<(), Box<dyn Error>> {
    log::info!("Doing assignment 2");

    let mut uni = Uniform701::new();

    part_2a(&mut uni, 500)?;
    part_2b(&mut uni, 500, 0.7)?;
    part_2c(&mut uni, 500, 2.0)?;

    Ok(())
}

/// Generate a set of random numbers in a two-dimensional box of length L. Assume L = 20
/// and the number of points, n = 500.
fn part_2a(uni: &mut Uniform701, n: usize) -> Result<(), Box<dyn Error>> {
    log::info!("Doing part a");

    scatter_2d(
        "output/assignment2/part_2a.png",
        "Assignment 2a, L = 20, n = 500",
        0.0..L,
        0.0..L,
        (0..n).map(|_| (uni.next() * L, uni.next() * L)),
    )?;
    Ok(())
}

/// Modify the code in (2a) such that the minimum distance between any two random points is
/// greater or equal to rmin. Choose n and rmin as 500 and 1.0, respectively, in your program.
fn part_2b(uni: &mut Uniform701, n: usize, r_min: f64) -> Result<(), Box<dyn Error>> {
    log::info!("Doing part b");

    let mut accepted: Vec<(f64, f64)> = Vec::with_capacity(n);

    for _ in 0..n {
        let mut rejected = true;
        while rejected {
            let next = (uni.next() * L, uni.next() * L);
            if let None = (&accepted)
                .into_iter()
                .find(|point| distance_2d(**point, next) < r_min)
            {
                accepted.push(next);
                rejected = false;
            }
        }
    }
    scatter_2d(
        "output/assignment2/part_2b.png",
        &format!("Assignment 2b, L = 20, n = {}, r_min = {}", n, r_min),
        0.0..L,
        0.0..L,
        accepted,
    )?;
    Ok(())
}

/// Repeat the computation in (2b) in three dimensions by changing rmin from 1 to 2.
/// Show your results graphically.
fn part_2c(uni: &mut Uniform701, n: usize, r_min: f64) -> Result<(), Box<dyn Error>> {
    log::info!("Doing part c");

    let mut accepted: Vec<(f64, f64, f64)> = Vec::with_capacity(n);

    for _ in 0..n {
        let mut rejected = true;
        while rejected {
            let next = (uni.next() * L, uni.next() * L, uni.next() * L);
            if let None = (&accepted)
                .into_iter()
                .find(|point| distance_3d(**point, next) < r_min)
            {
                accepted.push(next);
                rejected = false;
            }
        }
    }
    // scatter_3d(
    animated_3d(
        "output/assignment2/part_2c.gif",
        &format!("Assignment 2c, L = 20, n = {}, r_min = {}", n, r_min),
        0.0..L,
        0.0..L,
        0.0..L,
        accepted,
    )?;
    Ok(())
}

/// Draw a scatter plot for the supplied 2-dimensional points
fn scatter_2d<'a>(
    path: &str,
    caption: &str,
    x_range: Range<f64>,
    y_range: Range<f64>,
    points: impl IntoIterator<Item = (f64, f64)>,
) -> Result<(), Box<dyn Error>> {
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

/// Draw a scatter plot for the supplied 3-dimensional points
fn scatter_3d(
    path: &str,
    caption: &str,
    x_range: Range<f64>,
    y_range: Range<f64>,
    z_range: Range<f64>,
    points: Vec<(f64, f64, f64)>,
) -> Result<(), Box<dyn Error>> {
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

/// Draw an animated scatter plot for the supplied 3-dimensional points.
/// The animation adjusts the matrix perspective's yaw, generating a gif instead of a png.
/// This function is slow.
fn animated_3d(
    path: &str,
    caption: &str,
    x_range: Range<f64>,
    y_range: Range<f64>,
    z_range: Range<f64>,
    points: Vec<(f64, f64, f64)>,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::gif(path, (1440, 900), 1_000)?.into_drawing_area();

    for i in 0..10 {
        let frame = format!("Frame {}", i);
        log::info!("{}", frame);
        let yaw = i as f64 * 0.2;
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption(
                format!("{}, {}", caption, frame),
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

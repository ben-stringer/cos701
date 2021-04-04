use plotters::prelude::*;

use crate::data::point::{Point2d, Point3d};
use crate::data::random_vec::RandomVec;
use crate::rand::boxmuller::BoxMullerGaussian701;
use crate::rand::uniform::Uniform701;
use crate::util;
use std::collections::BTreeMap;
use std::error::Error;
use std::f64::consts::PI;
use std::ops::Range;

pub fn do_assignment_3() -> Result<(), Box<dyn Error>> {
    log::info!("Doing assignment 3");

    let mut uni = Uniform701::new();
    let mut gau = BoxMullerGaussian701::new(Uniform701::new());

    draw_2d_and_3d_naive(&mut uni, 10_000)?;
    draw_2d_and_3d_efficient(&mut uni, 10_000)?;

    let mut accept_rates: Vec<(BTreeMap<usize, f64>, String, RGBColor)> = Vec::new();

    // Compute accept rates for the accept-reject method
    for (num_iter, color) in vec![(100, CYAN), (1000, GREEN), (10_000, RED)] {
        accept_rates.push((
            (2..=10)
                .map(|dim| (dim, part_3b_nd(&mut uni, dim, num_iter).unwrap()))
                .map(|(dim, num_accepted)| (dim, num_accepted as f64 / num_iter as f64))
                .collect(),
            format!("n = {}", num_iter),
            color,
        ));
    }

    // Compute accept rates for the efficient method; expecting 100% accept rate
    accept_rates.push((
        (2..=10)
            .map(|dim| (dim, part_3c(&mut uni, &mut gau, dim, 10_000).unwrap()))
            .map(|(dim, num_accepted)| (dim, num_accepted as f64 / 10_000.0))
            .collect(),
        "Efficient".to_owned(),
        BLUE,
    ));

    // Compute expected accept rates for the accept-reject method using the gamma function
    accept_rates.push((
        (2..=10)
            .map(|d| {
                (
                    d,
                    PI.powf(d as f64 / 2.0)
                        / (2_f64.powf(d as f64) * (d as f64 / 2.0) * util::gamma_half(d)),
                )
            })
            .collect(),
        "Exact".to_owned(),
        BLACK,
    ));

    plot_accept_rates(
        "output/assignment3/3b_3c_accept_rates.png",
        "Accept rates for d = 2..10",
        accept_rates,
    )?;

    Ok(())
}

fn draw_2d_and_3d_naive(uni: &mut Uniform701, n_iter: usize) -> Result<(), Box<dyn Error>> {
    log::info!("Doing part 3a for 2-dimensions, naively");
    scatter_2d(
        "output/assignment3/3a_2D_naive.png",
        "Naive, sphere r = 1, dimension = 2",
        -1.0..1.0,
        -1.0..1.0,
        (0..n_iter)
            .map(|_| RandomVec::naive_scaled(uni, 2, 2.0, -1.0))
            .filter(|v| v.is_in_sphere(1.0))
            .map(|v| {
                let points = v.get().to_owned();
                Point2d {
                    x: points.get(0).unwrap().to_owned(),
                    y: points.get(1).unwrap().to_owned(),
                }
            })
            .collect::<Vec<Point2d>>(),
    )?;

    log::info!("Doing part 3a for 3-dimensions, naively");
    animate_3d(
        "output/assignment3/3a_3D_naive.gif",
        "Naive sphere r = 1, dimension = 3",
        -1.0..1.0,
        -1.0..1.0,
        -1.0..1.0,
        (0..n_iter)
            .map(|_| RandomVec::naive_scaled(uni, 3, 2.0, -1.0))
            .filter(|v| v.is_in_sphere(1.0))
            .map(|v| {
                let points = v.get().to_owned();
                Point3d {
                    x: points.get(0).unwrap().to_owned(),
                    y: points.get(1).unwrap().to_owned(),
                    z: points.get(2).unwrap().to_owned(),
                }
            })
            .collect::<Vec<Point3d>>(),
    )?;

    Ok(())
}

fn draw_2d_and_3d_efficient(uni: &mut Uniform701, n_iter: usize) -> Result<(), Box<dyn Error>> {
    log::info!("Doing part 3a for 2-dimensions, efficiently");
    let mut gaussian = BoxMullerGaussian701::new(Uniform701::new());
    scatter_2d(
        "output/assignment3/3a_2D_efficient.png",
        "Efficient sphere r = 1, dimension = 2",
        -1.0..1.0,
        -1.0..1.0,
        (0..n_iter)
            .map(|_| RandomVec::efficient_scaled(uni, &mut gaussian, 2, 2.0, -1.0))
            .filter(|v| v.is_in_sphere(1.0))
            .map(|v| {
                let points = v.get().to_owned();
                Point2d {
                    x: points.get(0).unwrap().to_owned(),
                    y: points.get(1).unwrap().to_owned(),
                }
            })
            .collect::<Vec<Point2d>>(),
    )?;

    log::info!("Doing part 3a for 3-dimensions, efficiently");
    animate_3d(
        "output/assignment3/3a_3D_efficient.gif",
        "Efficient sphere r = 1, dimension = 3",
        -1.0..1.0,
        -1.0..1.0,
        -1.0..1.0,
        (0..n_iter)
            .map(|_| RandomVec::efficient_scaled(uni, &mut gaussian, 3, 2.0, -1.0))
            .filter(|v| v.is_in_sphere(1.0))
            .map(|v| {
                let points = v.get().to_owned();
                Point3d {
                    x: points.get(0).unwrap().to_owned(),
                    y: points.get(1).unwrap().to_owned(),
                    z: points.get(2).unwrap().to_owned(),
                }
            })
            .collect::<Vec<Point3d>>(),
    )?;

    Ok(())
}

fn part_3b_nd(uni: &mut Uniform701, dim: usize, n_iter: usize) -> Result<usize, Box<dyn Error>> {
    log::info!("Doing part 3b for {}-dimensions", dim);

    Ok((0..n_iter)
        .map(|_| RandomVec::naive_scaled(uni, dim, 2.0, -1.0))
        .filter(|v| v.is_in_sphere(1.0))
        .count())
}

fn part_3c(
    uni: &mut Uniform701,
    gaussian: &mut BoxMullerGaussian701,
    dim: usize,
    n_iter: usize,
) -> Result<usize, Box<dyn Error>> {
    log::info!("Doing part 3c for {}-dimensions", dim);

    Ok((2..n_iter)
        .map(|_| RandomVec::efficient_scaled(uni, gaussian, dim, 2.0, -1.0))
        .filter(|v| v.is_in_sphere(1.0))
        .count())
}

/// Draw a scatter plot for the supplied 2-dimensional points
fn scatter_2d<'a>(
    path: &str,
    caption: &str,
    x_range: Range<f64>,
    y_range: Range<f64>,
    points: impl IntoIterator<Item = Point2d>,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(path, (900, 900)).into_drawing_area();
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
            .map(|coord| Circle::new(coord.into(), 2, RED.filled())),
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
    points: Vec<Point3d>,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(path, (900, 900)).into_drawing_area();
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
            .map(|coord| Circle::new(coord.into(), 2, RED.filled())),
    )?;

    Ok(())
}

/// Draw an animated scatter plot for the supplied 3-dimensional points.
/// The animation adjusts the matrix perspective's yaw, generating a gif instead of a png.
/// This function is slow.
fn animate_3d(
    path: &str,
    caption: &str,
    x_range: Range<f64>,
    y_range: Range<f64>,
    z_range: Range<f64>,
    points: Vec<Point3d>,
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
                .map(|coord| Circle::new(coord.into(), 2, RED.filled())),
        )?;

        root.present()?;
    }

    Ok(())
}

fn plot_accept_rates(
    path: &str,
    caption: &str,
    to_plot: Vec<(BTreeMap<usize, f64>, String, RGBColor)>,
) -> Result<(), Box<dyn Error>> {
    log::info!("Plotting accept rates for parts 3a-3b.");

    let root = BitMapBackend::new(path, (1440, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 50).into_font())
        .margin(32)
        .x_label_area_size(32)
        .y_label_area_size(32)
        .build_cartesian_2d(1.9..10.0, 0.0..1.01)?;
    chart
        .configure_mesh()
        // .disable_mesh()
        .draw()?;

    for (accept_rate, curve_label, color) in to_plot {
        chart
            .draw_series(LineSeries::new(
                accept_rate
                    .iter()
                    .map(|entry| (*entry.0 as f64, (*entry.1 as f64))),
                ShapeStyle::from(&color),
            ))?
            .label(curve_label.to_owned())
            .legend(move |(x, y)| {
                PathElement::new(vec![(x, y), (x + 20, y)], ShapeStyle::from(&color))
            });
    }

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()?;

    Ok(())
}

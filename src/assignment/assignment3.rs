use plotters::prelude::*;

use crate::rand::random_vec::{NaiveRandomVec, EfficientRandomVec};
use crate::rand::uniform::Uniform701;
use std::collections::BTreeMap;
use std::error::Error;
use std::ops::Range;
use crate::rand::boxmuller::BoxMullerGaussian701;

const NUM_ITER: usize = 1_000_000;

pub fn do_assignment_3() -> Result<(), Box<dyn Error>> {
    log::info!("Doing assignment 3");

    let mut uni = Uniform701::new();
    let mut gau = BoxMullerGaussian701::new(Uniform701::new());

    // key is the number of dimensions, e.g., accept_rate.get(2) is the 2-D accept rate
    let mut accept_rate: BTreeMap<usize, usize> = BTreeMap::new();

    accept_rate.insert(2, part_3a_2d(&mut uni, NUM_ITER)?);
    accept_rate.insert(3, part_3a_3d(&mut uni, NUM_ITER)?);
    (4..11)
        .map(|dim| (dim, part_3b_nd(&mut uni, dim, NUM_ITER).unwrap()))
        .for_each(|(dim, num_accepted)| {
            accept_rate.insert(dim, num_accepted);
        });
    (2..11)
        .map(|dim| part_3c(&mut uni, &mut gau, dim, NUM_ITER).unwrap())
        .for_each(|x| log::info!("{} / {} = {}", x, NUM_ITER, x as f64 / NUM_ITER as f64));

    plot_accept_rates(
        "output/assignment3/3b_accept_rates.png",
        "Accept rates for d = 2..10",
        NUM_ITER,
        accept_rate,
    )?;

    Ok(())
}

fn part_3a_2d(uni: &mut Uniform701, n_iter: usize) -> Result<usize, Box<dyn Error>> {
    log::info!("Doing part 3a for 2-dimensions");

    let accepted: Vec<(f64, f64)> = (0..n_iter)
        .map(|_| NaiveRandomVec::scaled(uni, 2, 2.0, -1.0))
        .filter(|v| v.is_in_sphere(1.0))
        .map(|v| {
            let points = v.get().to_owned();
            (
                points.get(0).unwrap().to_owned(),
                points.get(1).unwrap().to_owned(),
            )
        })
        .collect();

    scatter_2d(
        "output/assignment3/3a_2d.png",
        "Points within sphere r = 1, dimension = 2",
        -1.0..1.0,
        -1.0..1.0,
        accepted.to_owned(),
    )?;

    Ok(accepted.len())
}

fn part_3a_3d(uni: &mut Uniform701, n_iter: usize) -> Result<usize, Box<dyn Error>> {
    log::info!("Doing part 3a for 3-dimensions");

    let accepted: Vec<(f64, f64, f64)> = (0..n_iter)
        .map(|_| NaiveRandomVec::scaled(uni, 3, 2.0, -1.0))
        .filter(|v| v.is_in_sphere(1.0))
        .map(|v| {
            let points = v.get().to_owned();
            (
                points.get(0).unwrap().to_owned(),
                points.get(1).unwrap().to_owned(),
                points.get(2).unwrap().to_owned(),
            )
        })
        .collect();

    scatter_3d(
        "output/assignment3/3a_3d.png",
        "Points within sphere r = 1, dimension = 3",
        -1.0..1.0,
        -1.0..1.0,
        -1.0..1.0,
        accepted.to_owned(),
    )?;

    Ok(accepted.len())
}

fn part_3b_nd(uni: &mut Uniform701, dim: usize, n_iter: usize) -> Result<usize, Box<dyn Error>> {
    log::info!("Doing part 3b for {}-dimensions", dim);

    Ok((0..n_iter)
        .map(|_| NaiveRandomVec::scaled(uni, dim, 2.0, -1.0))
        .filter(|v| v.is_in_sphere(1.0))
        .count())
}

fn part_3c(uni: &mut Uniform701, gaussian: &mut BoxMullerGaussian701, dim: usize, n_iter: usize) -> Result<usize, Box<dyn Error>> {
    log::info!("Doing part 3c for {}-dimensions", dim);

    Ok((0..n_iter)
        .map(|_| EfficientRandomVec::scaled(uni, gaussian, dim, 2.0, -1.0))
        .filter(|v| v.is_in_sphere(1.0))
        .count())
}

/// Draw a scatter plot for the supplied 2-dimensional points
fn scatter_2d<'a>(
    path: &str,
    caption: &str,
    x_range: Range<f64>,
    y_range: Range<f64>,
    points: impl IntoIterator<Item = (f64, f64)>,
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
            .map(|coord| Circle::new(coord, 2, RED.filled())),
    )?;

    Ok(())
}

fn plot_accept_rates(
    path: &str,
    caption: &str,
    n_iter: usize,
    accept_rate: BTreeMap<usize, usize>,
) -> Result<(), Box<dyn Error>> {
    log::info!("Plotting accept rates for parts 3a-3b.");

    let root = BitMapBackend::new(path, (1440, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 50).into_font())
        .margin(32)
        .x_label_area_size(32)
        .y_label_area_size(32)
        .build_cartesian_2d(1.9..10.0, 0.0..1.0)?;
    chart.configure_mesh().disable_mesh().draw()?;
    chart.draw_series(LineSeries::new(
        accept_rate
            .iter()
            .map(|entry| (*entry.0 as f64, (*entry.1 as f64 / n_iter as f64))),
        &BLUE,
    ))?;
    // .label(curve_label)
    // .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE)),
    // );
    // chart
    //     .configure_series_labels()
    //     .border_style(&BLACK)
    //     .background_style(&WHITE.mix(0.8))
    //     .draw()?;

    Ok(())
}

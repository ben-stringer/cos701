use plotters::prelude::*;

use crate::data::line::Line2d;
use crate::data::point::{Point2d, ORIGIN_2D};
use crate::rand::points_in_grid::gen_points_in_box;
use crate::rand::uniform::Uniform701;
use std::error::Error;
use std::time::Instant;

const BOX_LEN: f64 = 100.0;

pub fn do_assignment_7() -> Result<(), Box<dyn Error>> {
    log::info!("Doing assignment 7");

    let mut uni = Uniform701::new();

    let mut grids: Vec<Vec<Point2d>> = (500..=10_000)
        .step_by(500)
        .map(|n| gen_points_in_box(&mut uni, BOX_LEN, n))
        .collect();

    let hulls: Vec<(u128, Vec<Line2d>)> = grids.iter_mut().map(compute_convex_hull).collect();

    hulls.iter().enumerate().try_for_each(|(i, (time, hull))| {
        let n = 500 + i * 500;
        plot_hull(
            &*format!("output/assignment7/hull_{}.png", n),
            &*format!("Convex Hull for n={}, computed in {}ns.", n, time),
            &grids[i],
            hull,
        )
    })?;

    plot_runtimes(
        "output/assignment7/runtimes.png",
        "Convex Hull Runtimes",
        hulls.iter().map(|&(time, _)| time).collect::<Vec<u128>>(),
    )?;

    Ok(())
}

pub fn compute_convex_hull(grid: &mut Vec<Point2d>) -> (u128, Vec<Line2d>) {
    log::info!("Computing convex hull for n={}", grid.len());
    let now = Instant::now();

    // Sort by magnitude so that element 0 is in the bottom-left
    grid.sort_unstable_by(|l, r| l.x.partial_cmp(&r.x).unwrap());

    // Start with element 0 since we forced it to be in the bottom-left
    let starting_i = 0;
    // Assigned at the start of each iteration, represents the current index
    let mut i;
    // Assigned at the end of each iteration, represents the best line
    let mut k = 0;

    // Assigned at the end of each iteration.
    // For each iteration, represents the previous line against which we want a minimum angle.
    // For the first iteration, this is a line arbitrarily from the origin,
    // but thereafter, it will be a line included in the hull.
    let mut reference_line = Line2d {
        src: grid[0],
        dst: ORIGIN_2D,
    };

    let mut hull = vec![];

    loop {
        i = k;
        let src = grid[i];
        let next = grid
            .iter()
            .enumerate()
            .filter_map(|(j, &pt)| {
                if j == i || j == k {
                    None
                } else {
                    let line = Line2d { src, dst: grid[j] };
                    Some((j, reference_line.angle_between(&line)))
                }
            })
            .reduce(|l, r| if l.1 < r.1 { l } else { r })
            .unwrap();
        k = next.0;
        reference_line = Line2d {
            src: grid[i],
            dst: grid[k],
        };
        hull.push(reference_line);
        if k == starting_i {
            break;
        }
    }

    let elapsed = now.elapsed().as_nanos();
    log::info!(
        "Computing convex hull for n={} took {}ns and produced a hull with {} lines.",
        grid.len(),
        elapsed,
        hull.len()
    );
    (elapsed, hull)
}

pub fn plot_hull(
    path: &str,
    caption: &str,
    sites: &Vec<Point2d>,
    hull: &Vec<Line2d>,
) -> Result<(), Box<dyn Error>> {
    log::info!("Plotting '{}'", caption);

    let root = BitMapBackend::new(path, (1440, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 50).into_font())
        .margin(32)
        .x_label_area_size(32)
        .y_label_area_size(32)
        .build_cartesian_2d(-10.0..BOX_LEN + 10.0, -10.0..BOX_LEN + 10.0)?;
    chart.configure_mesh().disable_mesh().draw()?;

    chart.draw_series(
        sites
            .iter()
            .map(|&coord| Circle::new(coord.into(), 2, RED.filled())),
    )?;
    hull.iter()
        .map(|&line| (line.src, line.dst))
        .for_each(|(src, dst)| {
            if let Err(err) = chart.draw_series(LineSeries::new(
                vec![src.to_owned().into(), dst.to_owned().into()],
                &BLUE,
            )) {
                log::error!("Error occurred drawing a line!  Details: {:?}", err);
            }
        });
    Ok(())
}

fn plot_runtimes(path: &str, caption: &str, to_plot: Vec<u128>) -> Result<(), Box<dyn Error>> {
    log::info!("Plotting runtimes");

    let root = BitMapBackend::new(path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 50).into_font())
        .margin(32)
        .x_label_area_size(64)
        .y_label_area_size(64)
        .build_cartesian_2d(500_usize..10_000_usize, 0..*to_plot.iter().max().unwrap())?;
    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .x_desc("Number of points")
        .y_desc("Runtime in nanoseconds")
        .y_label_formatter(&|y| format!("{:E}", y))
        .draw()?;

    chart.draw_series(LineSeries::new(
        to_plot.iter().enumerate().map(|(i, &v)| (500 + i * 500, v)),
        &BLUE,
    ))?;

    Ok(())
}

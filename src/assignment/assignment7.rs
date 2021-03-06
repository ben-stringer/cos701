use plotters::prelude::*;

use crate::data::line::Line2d;
use crate::data::point::{Point2d, ORIGIN_2D};
use crate::data::points_in_grid::gen_points_in_box;
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

    let hulls: Vec<(u128, Vec<Line2d>)> = grids
        .iter_mut()
        .map(Vec::as_mut_slice)
        .map(compute_convex_hull)
        .collect();

    hulls.iter().enumerate().try_for_each(|(i, (time, hull))| {
        let n = 500 + i * 500;
        plot_hull(
            &*format!("output/assignment7/hull_{}.png", n),
            &*format!("Convex Hull for n={}, computed in {}ns.", n, time),
            &grids[i],
            hull,
        )
    })?;

    let from_lib: Vec<(u128, Vec<Line2d>)> = grids
        .iter()
        .map(Vec::as_slice)
        .map(compute_convex_hull_from_library)
        .collect();

    plot_runtimes(
        "output/assignment7/runtimes.png",
        "Convex Hull Runtimes",
        &hulls
            .iter()
            .map(|&(time, _)| time)
            .zip(from_lib)
            .map(|(time_me, (time_lib, _))| (time_me, time_lib))
            .collect::<Vec<(u128, u128)>>(),
    )?;

    Ok(())
}

pub fn compute_convex_hull_from_library(grid: &[Point2d]) -> (u128, Vec<Line2d>) {
    let now = Instant::now();

    let wrapped_points = &grid
        .iter()
        .map(|&v| delaunator::Point { x: v.x, y: v.y })
        .collect::<Vec<delaunator::Point>>();

    let hull = delaunator::triangulate(wrapped_points)
        .expect("No triangulation for the points exists!")
        .hull
        .windows(2)
        .map(|points| (grid[points[0]], grid[points[1]]).into())
        .collect::<Vec<Line2d>>();

    let elapsed = now.elapsed().as_nanos();
    (elapsed, hull)
}

pub fn compute_convex_hull(grid: &mut [Point2d]) -> (u128, Vec<Line2d>) {
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
    (elapsed, hull)
}

pub fn plot_hull(
    path: &str,
    caption: &str,
    sites: &[Point2d],
    hull: &[Line2d],
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

fn plot_runtimes(
    path: &str,
    caption: &str,
    to_plot: &[(u128, u128)],
) -> Result<(), Box<dyn Error>> {
    log::info!("Plotting runtimes");

    let root = BitMapBackend::new(path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let y_max = u128::max(
        to_plot.iter().map(|&(v, _)| v).max().unwrap(),
        to_plot.iter().map(|&(_, v)| v).max().unwrap(),
    );

    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 50).into_font())
        .margin(32)
        .x_label_area_size(64)
        .y_label_area_size(64)
        .build_cartesian_2d(500_usize..10_000_usize, 0..y_max)?;
    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .x_desc("Number of points")
        .y_desc("Runtime in nanoseconds")
        .y_label_formatter(&|y| format!("{:E}", y))
        .draw()?;

    chart
        .draw_series(LineSeries::new(
            to_plot
                .iter()
                .enumerate()
                .map(|(i, &(v, _))| (500 + i * 500, v)),
            BLUE.stroke_width(2),
        ))?
        .label("My implementation")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE.stroke_width(2)));

    chart
        .draw_series(LineSeries::new(
            to_plot
                .iter()
                .enumerate()
                .map(|(i, &(_, v))| (500 + i * 500, v)),
            RED.stroke_width(2),
        ))?
        .label("Library implementation")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED.stroke_width(2)));

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()?;

    Ok(())
}

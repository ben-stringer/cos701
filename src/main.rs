#![allow(dead_code)]
#![allow(unused_variables)]

mod assignment;
mod data;
mod rand;
mod util;

#[allow(unused_imports)]
use plotters::prelude::*;
use simple_logger::SimpleLogger;
use std::error::Error;

#[allow(unused_imports)]
use crate::assignment::assignment1;
#[allow(unused_imports)]
use crate::assignment::assignment2;
#[allow(unused_imports)]
use crate::assignment::assignment3;
#[allow(unused_imports)]
use crate::assignment::assignment4;
#[allow(unused_imports)]
use crate::assignment::assignment5;
#[allow(unused_imports)]
use crate::assignment::assignment6;
use crate::data::line::Line2d;
use crate::data::point::Point2d;
use log::LevelFilter::Warn;
use std::f64::consts::PI;

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new()
        .with_module_level("voronoi", Warn)
        .init()?;

    log::info!("Hello world");

    ensure_output_dirs_exist()?;

    // assignment1::do_assignment_1()?;
    // assignment2::do_assignment_2()?;
    // assignment3::do_assignment_3()?;
    // assignment4::do_assignment_4()?;
    // assignment5::do_assignment_5()?;
    assignment6::do_assignment_6()?;

    // scratch()?;

    Ok(())
}

fn ensure_output_dirs_exist() -> Result<(), Box<dyn Error>> {
    log::info!("Creating output directories");

    vec![
        "output/scratch",
        "output/assignment1",
        "output/assignment2",
        "output/assignment3",
        "output/assignment4",
        "output/assignment5",
        "output/assignment6",
    ]
    .into_iter()
    .try_for_each(std::fs::create_dir_all)?;

    Ok(())
}

fn scratch() -> Result<(), Box<dyn Error>> {
    let center: Point2d = (2.0, 2.0).into();
    let mut lines: Vec<Line2d> = vec![
        (center, (2.0, 3.0).into()).into(),
        (center, (3.0, 2.0).into()).into(),
        (center, (2.0, 1.0).into()).into(),
        (center, (1.0, 2.0).into()).into(),
        (center, (2.75, 2.75).into()).into(),
        (center, (0.25, 0.25).into()).into(),
        (center, (1.25, 2.75).into()).into(),
        (center, (2.5, 1.5).into()).into(),
    ];

    lines.sort_by(|&l, &r| l.angle().partial_cmp(&r.angle()).unwrap());

    let path = "output/scratch/foo.png";

    let root = BitMapBackend::new(path, (900, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(32)
        .x_label_area_size(32)
        .y_label_area_size(32)
        .build_cartesian_2d(-1.0..3.0, -1.0..3.0)?;
    chart.configure_mesh().disable_mesh().draw()?;

    (&lines)
        .into_iter()
        .map(|&line| (line.src, line.dst))
        .for_each(|(src, dst)| {
            if let Err(err) = chart.draw_series(LineSeries::new(
                vec![src.to_owned().into(), dst.to_owned().into()],
                &BLUE,
            )) {
                log::error!("Error occurred drawing a line!  Details: {:?}", err);
            }
        });
    (&lines).into_iter().enumerate().for_each(|(i, &line)| {
        if let Err(err) = chart.draw_series(PointSeries::of_element(
            vec![(i, line)],
            5,
            &RED,
            &|(i, l), s, st| {
                return EmptyElement::at(l.dst.into())    // We want to construct a composed element on-the-fly
                    + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
                    + Text::new(format!("{:}, {:.2}", i, l.angle() * 180.0 / PI), (10, 0), ("sans-serif", 10).into_font());
            })) {
            log::warn!("Something went wrong drawing a line; ignoring.");
        }
    });

    (&lines)
        .into_iter()
        .map(|&line| line.perpendicular_bisector())
        .for_each(|pb| {
            if let Err(err) = chart.draw_series(LineSeries::new(
                vec![pb.src.to_owned().into(), pb.dst.to_owned().into()],
                &GREEN,
            )) {
                log::error!("Error occurred drawing a PB!  Details: {:?}", err);
            }
        });

    chart.draw_series(
        (&lines)
            .into_iter()
            .map(|l| l.perpendicular_bisector())
            .collect::<Vec<Line2d>>()
            .windows(2)
            .map(|v| (v[0], v[1]))
            .filter_map(|(l, r)| l.intersection(&r))
            .map(|coord| Circle::new(coord.into(), 2, BLACK.filled())),
    )?;

    let mut bisectors = lines
        .into_iter()
        .map(|l| l.perpendicular_bisector())
        .collect::<Vec<Line2d>>();
    bisectors.push(bisectors[0]);
    let mut intersections = bisectors
        .windows(2)
        .map(|v| (v[0], v[1]))
        .filter_map(|(l, r)| l.intersection(&r))
        .collect::<Vec<Point2d>>();
    intersections.push(intersections[0]);
    intersections.windows(2).for_each(|pts| {
        let src = pts[0];
        let dst = pts[1];
        if let Err(err) = chart.draw_series(LineSeries::new(
            vec![src.to_owned().into(), dst.to_owned().into()],
            &RED,
        )) {
            log::error!(
                "Error occurred drawing intersecting lines!  Details: {:?}",
                err
            );
        }
    });

    Ok(())
}

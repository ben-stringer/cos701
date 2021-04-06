#![allow(dead_code)]
#![allow(unused_variables)]

mod assignment;
mod data;
mod rand;
mod util;

use log::LevelFilter::Warn;
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

use crate::data::point::Point2d;
use crate::rand::uniform::Uniform701;

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
    // assignment6::do_assignment_6()?;

    scratch()?;

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
    let mut uni = Uniform701::new();

    let mut r1_points = rand_plot(&mut uni);
    let mut r2_points = rand_plot(&mut uni);
    let mut r3_points = rand_plot(&mut uni);

    (90..110).for_each(|i| {
        r1_points[i] = {
            let mut p = r1_points[i];
            p.x = p.x + 1.0;
            p
        };
        r2_points[i] = {
            let mut p = r2_points[i];
            p.x = p.x + 1.0;
            p
        };
        r3_points[i] = {
            let mut p = r3_points[i];
            p.x = p.x + 1.0;
            p
        };
    });
    (130..150).for_each(|i| {
        r1_points[i] = {
            let mut p = r1_points[i];
            p.x = p.x + 1.0;
            p
        };
        r2_points[i + 20] = {
            let mut p = r2_points[i + 20];
            p.x = p.x + 1.0;
            p
        };
        r3_points[i + 40] = {
            let mut p = r3_points[i + 40];
            p.x = p.x + 1.0;
            p
        };
    });
    (10..30).for_each(|i| {
        r1_points[i + 40] = {
            let mut p = r1_points[i + 40];
            p.x = p.x + 1.0;
            p
        };
        r2_points[i + 20] = {
            let mut p = r2_points[i + 20];
            p.x = p.x + 1.0;
            p
        };
        r3_points[i] = {
            let mut p = r3_points[i];
            p.x = p.x + 1.0;
            p
        };
    });
    do_plot("output/scratch/r1.png", &r1_points)?;
    do_plot("output/scratch/r2.png", &r2_points)?;
    do_plot("output/scratch/r3.png", &r3_points)?;

    let nadir_sum: Vec<Point2d> = (&r1_points)
        .into_iter()
        .zip((&r2_points).into_iter().zip((&r3_points).into_iter()))
        .map(|(&p1, (&p2, &p3))| (p1.x + p2.x + p3.x, p2.y).into())
        .collect();

    let port_sum: Vec<Point2d> = (0..=200)
        .map(|i| {
            (
                r1_points[i],
                if i > 180 {
                    (0.0, 0.0).into()
                } else {
                    r2_points[i + 20]
                },
                if i > 160 {
                    (0.0, 0.0).into()
                } else {
                    r3_points[i + 40]
                },
            )
        })
        .map(|(p1, p2, p3)| (p1.x + p2.x + p3.x, p1.y).into())
        .collect();

    let stbd_sum: Vec<Point2d> = (0..=200)
        .map(|i| {
            (
                if i > 160 {
                    (0.0, 0.0).into()
                } else {
                    r1_points[i + 40]
                },
                if i > 180 {
                    (0.0, 0.0).into()
                } else {
                    r2_points[i + 20]
                },
                r3_points[i],
            )
        })
        .map(|(p1, p2, p3)| (p1.x + p2.x + p3.x, p3.y).into())
        .collect();

    do_plot("output/scratch/nadir_sum.png", &nadir_sum)?;
    do_plot("output/scratch/port_sum.png", &port_sum)?;
    do_plot("output/scratch/stbd_sum.png", &stbd_sum)?;

    do_plot(
        "output/scratch/r1_stbd.png",
        &(40..=200)
            .map(|i| (&r1_points[i]).clone())
            .map(|mut pt| {
                pt.y = pt.y - 4.0;
                pt
            })
            .collect(),
    )?;

    do_plot(
        "output/scratch/r2_both.png",
        &(20..=200)
            .map(|i| (&r2_points[i]).clone())
            .map(|mut pt| {
                pt.y = pt.y - 2.0;
                pt
            })
            .collect(),
    )?;

    do_plot(
        "output/scratch/r3_port.png",
        &(40..=200)
            .map(|i| (&r3_points[i]).clone())
            .map(|mut pt| {
                pt.y = pt.y - 4.0;
                pt
            })
            .collect(),
    )?;

    Ok(())
}

fn rand_plot(uni: &mut Uniform701) -> Vec<Point2d> {
    (0..=200)
        .map(|y| y as f64 * 0.1)
        .map(|y| (uni.next() * 0.5, y))
        .map(|coord| coord.into())
        .collect()
}

fn do_plot(path: &str, points: &Vec<Point2d>) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(path, (200, 450)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(32)
        .x_label_area_size(32)
        .y_label_area_size(32)
        .build_cartesian_2d(-0.5..5.5, -0.5..20.5)?;
    chart
        .configure_mesh()
        .disable_mesh()
        // .disable_y_axis()
        .y_labels(0)
        .x_labels(0)
        .y_desc("Time")
        .x_desc("Amplitude")
        .draw()?;

    chart.draw_series(LineSeries::new(
        points.into_iter().map(|&entry| entry.into()),
        &BLACK,
    ))?;
    chart.draw_series(LineSeries::new(
        (0..=200).map(|y| y as f64 * 0.1).map(|y| (3.2, y)),
        &RED,
    ))?;

    Ok(())
}

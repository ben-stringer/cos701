#![allow(dead_code)]
#![allow(unused_variables)]

mod assignment;
mod rng;

#[allow(unused_imports)]
use plotters::prelude::*;
use std::error::Error;

#[allow(unused_imports)]
use crate::assignment::assignment1;
#[allow(unused_imports)]
use crate::assignment::assignment2;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello world");

    ensure_output_dirs_exist()?;

    assignment1::do_assignment_1()?;

    // assignment2::do_assignment_2()?;

    // scratch()?;

    Ok(())
}

fn ensure_output_dirs_exist() -> Result<(), Box<dyn Error>> {
    println!("Creating output directories");

    vec!["output/scratch", "output/assignment1", "output/assignment2"]
        .into_iter()
        .try_for_each(std::fs::create_dir_all)?;

    Ok(())
}

// fn scratch() -> Result<(), Box<dyn Error>> {
//     let path = "output/scratch/scratch.png";
//     let caption = "Scratch";
//     // let x_range = -5_f64..5_f64;
//     let x_range = 0.0..10.0;
//     let y_range = -1.0..1.0;
//     let x_step = 0.1;
//     // let curve = (-5_f64..5_f64)
//     //     .step(0.1)
//     //     .key_points(1_000_000)
//     //     .into_iter()
//     //     .map(|x| x.powi(2));
//
//     let root = BitMapBackend::new(path, (1440, 900)).into_drawing_area();
//     root.fill(&WHITE)?;
//
//     let mut chart = ChartBuilder::on(&root)
//         .caption(caption, ("sans-serif", 50).into_font())
//         .margin(16)
//         .x_label_area_size(30)
//         .y_label_area_size(30)
//         // .build_cartesian_2d(x_range.step(x_step).into_segmented(), 0_f64..10_f64)?;
//         .build_cartesian_2d(x_range, y_range)?;
//     chart.configure_mesh().disable_mesh().draw()?;
//
//     chart.draw_series(LineSeries::new(
//         (0..100).map(|x| x as f64 / 10.0).map(|x| (x, x.sin())),
//         &BLACK,
//     ))?;
//     // chart.draw_series(LineSeries::new(
//     //     x_range
//     //         .step(x_step)
//     //         .key_points(1_000_000)
//     //         .into_iter()
//     //         .map(|x| (x, x.powi(2))),
//     //     &BLUE,
//     // ))?;
//     Ok(())
// }

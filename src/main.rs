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

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init()?;

    log::info!("Hello world");

    ensure_output_dirs_exist()?;

    // assignment1::do_assignment_1()?;
    // assignment2::do_assignment_2()?;
    // assignment3::do_assignment_3()?;
    // assignment4::do_assignment_4()?;
    assignment5::do_assignment_5()?;

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
    ]
    .into_iter()
    .try_for_each(std::fs::create_dir_all)?;

    Ok(())
}

// fn scratch() -> Result<(), Box<dyn Error>> {
//     let a = (1.0, 0.0);
//     let b = (0.0, 1.0);
//     let c = (-1.0, 0.0);
//
//     let (z, r) = util::circle_through(&a, &b, &c);
//
//     log::info!("Center point: ({}, {}), radius: {}", z.0, z.1, r);
//
//     Ok(())
// }

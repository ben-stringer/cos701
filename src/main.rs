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
#[allow(unused_imports)]
use crate::assignment::assignment7;
#[allow(unused_imports)]
use crate::assignment::project_d;
use log::LevelFilter::Warn;

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new()
        .with_module_level("voronoi", Warn)
        .init()?;

    log::info!("Hello world");

    ensure_output_dirs_exist()?;

    assignment1::do_assignment_1()?;
    assignment2::do_assignment_2()?;
    assignment3::do_assignment_3()?;
    assignment4::do_assignment_4()?;
    assignment5::do_assignment_5()?;
    assignment6::do_assignment_6()?;
    assignment7::do_assignment_7()?;
    project_d::do_project_d()?;

    // scratch()?;

    Ok(())
}

fn ensure_output_dirs_exist() -> Result<(), Box<dyn Error>> {
    log::info!("Creating output directories");

    vec![
        // "output/scratch",
        "output/assignment1",
        "output/assignment2",
        "output/assignment3",
        "output/assignment4",
        "output/assignment5",
        "output/assignment6",
        "output/assignment7",
        "output/projectD",
    ]
    .iter()
    .try_for_each(std::fs::create_dir_all)?;

    Ok(())
}

// fn scratch() -> Result<(), Box<dyn Error>> {
//     Ok(())
// }

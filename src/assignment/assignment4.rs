use crate::rand::points_in_grid::gen_points_in_cube;
use std::error::Error;
use crate::rand::uniform::Uniform701;

const L: f64 = 20.0;

pub fn do_assignment_4() -> Result<(), Box<dyn Error>> {
    log::info!("Doing assignment 4");

    part_4a()?;

    Ok(())
}

fn part_4a() -> Result<(), Box<dyn Error>> {
    log::info!("Doing part a");

    let mut uni = Uniform701::new();

    let points = gen_points_in_cube(&mut uni, L, 500, 2.0);

    Ok(())
}

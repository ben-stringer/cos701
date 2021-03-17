use crate::rand::points_in_grid::gen_points_in_cube;
use crate::rand::uniform::Uniform701;
use crate::data::nn_map::NearestNeighborMap;
use crate::util::{distance_3d,ORIGIN};
use std::error::Error;

const L: f64 = 20.0;

pub fn do_assignment_4() -> Result<(), Box<dyn Error>> {
    log::info!("Doing assignment 4");

    part_4a()?;

    Ok(())
}

fn part_4a() -> Result<(), Box<dyn Error>> {
    log::info!("Doing part a");

    let mut uni = Uniform701::new();

    let mut points = gen_points_in_cube(&mut uni, L, 500, 2.0);
    points.sort_by(|l, r| distance_3d(ORIGIN, *l).partial_cmp(&distance_3d(ORIGIN, *r)).unwrap());

    let nn_map = NearestNeighborMap::from(points, 3.0);
    nn_map.print_table("output/assignment4/first_neighbors.tex")?;

    Ok(())
}

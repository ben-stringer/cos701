use crate::data::neighbors::{AdjacencyMatrix, NearestNeighborMap};
use crate::rand::points_in_grid::gen_points_in_cube;
use crate::rand::uniform::Uniform701;
use crate::util::magnitude_3d;
use std::error::Error;

const L: f64 = 20.0;

pub fn do_assignment_4() -> Result<(), Box<dyn Error>> {
    log::info!("Doing assignment 4");

    let mut uni = Uniform701::new();

    let mut sites = gen_points_in_cube(&mut uni, L, 500, 2.0);
    sites.sort_by(|l, r| magnitude_3d(*l).partial_cmp(&magnitude_3d(*r)).unwrap());

    let nn_map = part_4a(&sites)?;
    part_4b(&nn_map)?;
    part_4c(&nn_map)?;

    Ok(())
}

fn part_4a(sites: &Vec<(f64, f64, f64)>) -> Result<NearestNeighborMap, Box<dyn Error>> {
    log::info!("Doing part a");

    let nn_map = NearestNeighborMap::from(sites, 3.0);
    nn_map.print_latex("output/assignment4/first_neighbors.tex")?;
    nn_map.print_csv("output/assignment4/first_neighbors.csv")?;

    Ok(nn_map)
}

fn part_4b(nn_map: &NearestNeighborMap) -> Result<AdjacencyMatrix, Box<dyn Error>> {
    log::info!("Doing part b");

    let adj_mat = AdjacencyMatrix::from(nn_map);
    adj_mat.print_csv("output/assignment4/adjacency_matrix.csv")?;

    Ok(adj_mat)
}

fn part_4c(nn_map: &NearestNeighborMap) -> Result<AdjacencyMatrix, Box<dyn Error>> {
    Err(From::from("Not implemented"))
}

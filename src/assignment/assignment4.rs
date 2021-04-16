use crate::data::neighbors::{AdjacencyMatrix, NearestNeighborMap};
use crate::data::point::Point3d;
use crate::rand::points_in_grid::gen_points_in_cube;
use crate::rand::uniform::Uniform701;
use std::error::Error;

const L: f64 = 20.0;

pub fn do_assignment_4() -> Result<(), Box<dyn Error>> {
    log::info!("Doing assignment 4");

    let mut uni = Uniform701::new();

    let mut sites = gen_points_in_cube(&mut uni, L, 500, 2.0);
    sites.sort_unstable_by(|l, r| l.magnitude().partial_cmp(&r.magnitude()).unwrap());

    let first_neighbors = part_4a(&sites)?;
    let first_adj = part_4b(&first_neighbors)?;
    let second_neighbors = part_4c(&first_neighbors)?;

    NearestNeighborMap::print_txt(
        &sites,
        &first_neighbors,
        &second_neighbors,
        "output/assignment4/sites_first_second.txt",
    )?;

    Ok(())
}

fn part_4a(sites: &Vec<Point3d>) -> Result<NearestNeighborMap, Box<dyn Error>> {
    log::info!("Doing part a");

    let nn_map = NearestNeighborMap::first_3d(sites, 3.0);
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

fn part_4c(first_neighbors: &NearestNeighborMap) -> Result<NearestNeighborMap, Box<dyn Error>> {
    log::info!("Doing part c");

    let second_neighbors = NearestNeighborMap::second(first_neighbors);
    second_neighbors.print_latex("output/assignment4/second_neighbors.tex")?;
    second_neighbors.print_csv("output/assignment4/second_neighbors.csv")?;

    Ok(second_neighbors)
}

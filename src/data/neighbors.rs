use crate::data::point::{Point2d, Point3d};
use crate::util::{TEX_PREFIX, TEX_SUFFIX};
use std::error::Error;
use std::fs::File;
use std::io::Write;

/// A nearest neighbor map for 3-D points.
pub(crate) struct NearestNeighborMap {
    pub neighbors: Vec<Vec<usize>>,
}

impl NearestNeighborMap {
    pub fn first_2d(sites: &[Point2d], r_cutoff: f64) -> Self {
        let n = sites.len();
        let mut neighbors = vec![vec![]; n];

        for i in 0..n - 1 {
            let site_i = sites[i];
            for j in i + 1..n {
                let site_j = sites[j];
                if site_i.distance_to(&site_j) < r_cutoff {
                    neighbors[i].push(j);
                    neighbors[j].push(i);
                }
            }
        }

        Self { neighbors }
    }

    pub fn first_3d(sites: &[Point3d], r_cutoff: f64) -> Self {
        let n = sites.len();
        let mut neighbors = vec![vec![]; n];

        for i in 0..n - 1 {
            let site_i = sites[i];
            for j in i + 1..n {
                let site_j = sites[j];
                if site_i.distance_to(&site_j) < r_cutoff {
                    neighbors[i].push(j);
                    neighbors[j].push(i);
                }
            }
        }

        Self { neighbors }
    }

    pub fn second(first_neighbors: &Self) -> Self {
        let n = first_neighbors.neighbors.len();
        let mut second_neighbors = vec![vec![]; n];

        for (i, first_neighbors_i) in first_neighbors.neighbors.iter().enumerate() {
            for j in first_neighbors_i {
                for k in &first_neighbors.neighbors[*j] {
                    if !(i == *k
                        || first_neighbors_i.contains(k)
                        || second_neighbors[i].contains(k))
                    {
                        second_neighbors[i].push(k.to_owned());
                    }
                }
            }
        }

        Self {
            neighbors: second_neighbors,
        }
    }

    pub fn print_latex(&self, to_file: &str) -> Result<(), Box<dyn Error>> {
        let mut of = File::create(to_file)?;
        of.write_all(TEX_PREFIX.as_ref())?;
        of.write_all("\\begin{tabular}{c c l}".as_ref())?;
        of.write_all("Site Index & Number of Neighbors & Neighbor List \\\\\n".as_ref())?;
        for i in 0..self.neighbors.len() {
            let site_neighbors: &Vec<usize> = &self.neighbors[i];
            of.write_all(
                format!(
                    "{} & {} & {} \\\\\n",
                    i,
                    site_neighbors.len(),
                    site_neighbors
                        .iter()
                        .map(|&v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(" "),
                )
                .as_ref(),
            )?;
        }
        of.write_all("\\end{tabular}".as_ref())?;
        of.write_all(TEX_SUFFIX.as_ref())?;
        Ok(())
    }

    pub fn print_csv(&self, to_file: &str) -> Result<(), Box<dyn Error>> {
        let mut of = File::create(to_file)?;
        of.write_all("Site Index, Number of Neighbors, Neighbor List\n".as_ref())?;
        for i in 0..self.neighbors.len() {
            let site_neighbors: &Vec<usize> = &self.neighbors[i];
            of.write_all(
                format!(
                    "{}, {}, {}\n",
                    i,
                    site_neighbors.len(),
                    site_neighbors
                        .iter()
                        .map(|&v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(" "),
                )
                .as_ref(),
            )?;
        }
        Ok(())
    }

    pub fn print_txt(
        sites: &[Point3d],
        first_neighbors: &NearestNeighborMap,
        second_neighbors: &NearestNeighborMap,
        to_file: &str,
    ) -> Result<(), Box<dyn Error>> {
        let mut of = File::create(to_file)?;

        for (i, site_i) in sites.iter().enumerate() {
            let first_i = &first_neighbors.neighbors[i];
            let second_i = &second_neighbors.neighbors[i];
            of.write_all(
                format!(
                    "{} {} {} {} {} {} {} {}\n",
                    i,
                    site_i.x,
                    site_i.y,
                    site_i.z,
                    first_i.len(),
                    first_i
                        .iter()
                        .map(|&v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(" "),
                    second_i.len(),
                    second_i
                        .iter()
                        .map(|&v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
                .as_ref(),
            )?;
        }

        Ok(())
    }
}

pub(crate) struct AdjacencyMatrix {
    m: Vec<Vec<u8>>,
}

impl AdjacencyMatrix {
    pub fn from(nn_map: &NearestNeighborMap) -> Self {
        let n = nn_map.neighbors.len();
        let mut m = vec![vec![0; n]; n];

        for (i, i_neighbors) in nn_map.neighbors.iter().enumerate() {
            for j in i_neighbors {
                m[i][*j] = 1;
            }
        }

        Self { m }
    }

    pub fn print_csv(&self, to_file: &str) -> Result<(), Box<dyn Error>> {
        let mut of = File::create(to_file)?;
        for i in 0..self.m.len() {
            let row = &self.m[i];
            of.write_all(
                row.iter()
                    .map(|&v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
                    .as_ref(),
            )?;
            of.write_all("\n".as_ref())?;
        }
        Ok(())
    }
}

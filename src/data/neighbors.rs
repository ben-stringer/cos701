use crate::util::{distance_3d, TEX_PREFIX, TEX_SUFFIX};
use std::error::Error;
use std::fs::File;
use std::io::Write;

/// A nearest neighbor map for 3-D points.
pub(crate) struct NearestNeighborMap {
    neighbors: Vec<Vec<usize>>,
}

impl NearestNeighborMap {
    pub fn first(sites: &Vec<(f64, f64, f64)>, r_cutoff: f64) -> Self {
        let n = sites.len();
        let mut neighbors = vec![vec![]; n];

        for i in 0..n - 1 {
            let site_i = sites[i];
            for j in i + 1..n {
                let site_j = sites[j];
                if distance_3d(site_i, site_j) < r_cutoff {
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

        for i in 0..n {
            let first_neighbors_i = &first_neighbors.neighbors[i];
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
        of.write(TEX_PREFIX.as_ref())?;
        of.write("\\begin{tabular}{c c l}".as_ref())?;
        of.write("Site Index & Number of Neighbors & Neighbor List \\\\\n".as_ref())?;
        for i in 0..self.neighbors.len() {
            let site_neighbors: &Vec<usize> = &self.neighbors[i];
            of.write(
                format!(
                    "{} & {} & {} \\\\\n",
                    i,
                    site_neighbors.len(),
                    site_neighbors
                        .into_iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(" "),
                )
                .as_ref(),
            )?;
        }
        of.write("\\end{tabular}".as_ref())?;
        of.write(TEX_SUFFIX.as_ref())?;
        Ok(())
    }

    pub fn print_csv(&self, to_file: &str) -> Result<(), Box<dyn Error>> {
        let mut of = File::create(to_file)?;
        of.write("Site Index, Number of Neighbors, Neighbor List\n".as_ref())?;
        for i in 0..self.neighbors.len() {
            let site_neighbors: &Vec<usize> = &self.neighbors[i];
            of.write(
                format!(
                    "{}, {}, {}\n",
                    i,
                    site_neighbors.len(),
                    site_neighbors
                        .into_iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(" "),
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

        for i in 0..n {
            let i_neighbors = &nn_map.neighbors[i];
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
            of.write(
                row.into_iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
                    .as_ref(),
            )?;
            of.write("\n".as_ref())?;
        }
        Ok(())
    }
}

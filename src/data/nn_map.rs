use crate::util;
use std::error::Error;
use std::fs::File;
use std::io::Write;

/// A nearest neighbor map for 3-D points.
pub(crate) struct NearestNeighborMap {
    sites: Vec<(f64, f64, f64)>,
    neighbors: Vec<Vec<usize>>,
}

impl NearestNeighborMap {
    pub fn from(sites: Vec<(f64, f64, f64)>, r_cutoff: f64) -> Self {
        let n = sites.len();
        let mut neighbors: Vec<Vec<usize>> = (0..n).map(|_| Vec::new()).collect();

        for i in 0..n - 1 {
            let site_i = sites[i];
            for j in i+1..n {
                let site_j = sites[j];
                if util::distance_3d(site_i, site_j) < r_cutoff {
                    neighbors[i].push(j);
                    neighbors[j].push(i);
                }
            }
        }

        Self { sites, neighbors }
    }

    pub fn print_table(&self, to_file: &str) -> Result<(), Box<dyn Error>> {
        let mut of = File::create(to_file)?;
        of.write("\\begin{tabular}{c c l}\n".as_ref())?;
        of.write("Site Index & Number of Neighbors & Neighbor List \\\\ \n".as_ref())?;
        for i in 0..self.sites.len() {
            let site_neighbors: &Vec<usize> = &self.neighbors[i];
            of.write(
                format!(
                    "{} & {} & {} \\\\ \n",
                    i,
                    site_neighbors.len(),
                    site_neighbors
                            .into_iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<String>>()
                            .join(", "),
                )
                .as_ref(),
            )?;
        }
        of.write("\\end{tabular}\n".as_ref())?;
        Ok(())
    }
}

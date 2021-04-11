use crate::data::neighbors::NearestNeighborMap;
use crate::data::point::Point2d;
use crate::util::{circle_through, point_in_circle};

/// Compute the Delaunay triangulation of the sites.  `r_cutoof` represents the distance of
/// neighbors to consider rather than considering the whole space.  Send `f64::MAX` if you
/// want all points considered.  When `bidirectional` is false, any two points will be connected
/// in one direction only, e.g., point i will contain j, but j will not contain i.
pub fn dealunay_701(sites: &Vec<Point2d>, r_cutoff: f64, bidirectional: bool) -> Vec<Vec<usize>> {
    let n_sites = sites.len();

    let mut lines = vec![vec![]; n_sites];

    let first_neighbors = NearestNeighborMap::first_2d(sites, r_cutoff);

    for i in 0..n_sites {
        let neighbors_i = &first_neighbors.neighbors[i];
        for &j in neighbors_i {
            for &k in neighbors_i {
                if k == j {
                    continue;
                }
                let (center, r) = circle_through(sites[i], sites[j], sites[k]);
                if let None = (0..n_sites)
                    .filter(|&v| !(v == i || v == j || v == k))
                    .find(|&v| point_in_circle(sites[v], center, r))
                {
                    lines[i].push(j);
                    lines[i].push(k);
                    lines[j].push(k);
                }
            }
        }
    }

    (0..n_sites).for_each(|i| {
        lines[i].sort_unstable();
        lines[i].dedup();
    });

    if bidirectional {
        let clone = lines.clone();
        for i in 0..n_sites {
            for &j in &clone[i] {
                lines[j].push(i);
            }
        }
    }
    lines
}

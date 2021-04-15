use crate::rand::uniform::Uniform701;

type Site = (usize, usize);

pub struct Lattice {
    pub grid: Vec<Vec<bool>>,
    pub box_len: usize,
}

impl Lattice {
    pub fn populate(p: f64, box_len: usize, uni: &mut Uniform701) -> Self {
        Self {
            grid: (0..box_len)
                .map(|_| (0..box_len).map(|_| uni.next() < p).collect())
                .collect(),
            box_len,
        }
    }

    pub fn create_clusters(&self) -> Clusters {
        let mut clusters = vec![];

        let mut world: Vec<Site> = (0..self.box_len)
            .flat_map(|i| (0..self.box_len).map(move |j| (i, j)))
            .collect();

        // Loop while there is some site to be visited
        while let Some(&(i, j)) = world.last() {
            // Check if this site is occupied
            if self.grid[i][j] {
                // The site is occupied.  Create a new cluster, currently empty.
                let mut new_cluster: Vec<Site> = vec![];

                // Create a stack of sites to visit, initialized to be the current site.
                // Other sites will be added as we visit each.
                let mut to_visit = vec![(i, j)];

                // Iterate over all the sites in the to_visit stack.
                while let Some((x, y)) = to_visit.pop() {
                    // See if we have visited this site before.  If we have, it will not be in `world`.
                    if let Some(ix) = world.iter().position(|&v| v == (x, y)) {
                        // We have not visited this site yet
                        // Remove it from the world since we're visiting it now
                        world.swap_remove(ix);
                        // Is it a part of this cluster?
                        if self.grid[x][y] {
                            // Add it to the cluster
                            new_cluster.push((x, y));
                            // Queue up all neighboring sites to be visited
                            if y < self.box_len - 1 {
                                to_visit.push((x, y + 1));
                            }
                            if x < self.box_len - 1 {
                                to_visit.push((x + 1, y));
                            }
                            if y > 0 {
                                to_visit.push((x, y - 1));
                            }
                            if x > 0 {
                                to_visit.push((x - 1, y));
                            }
                        }
                    }
                }
                clusters.push(new_cluster);
            } else {
                // The site is not occupied.
                // The .last() method returned a reference to the element but did not remove it.
                // .pop() will remove the last element.
                world.pop();
            }
        }

        Clusters { clusters }
    }
}

pub struct Clusters {
    pub clusters: Vec<Vec<Site>>,
}

impl Clusters {
    /// Get a new instance of this type where all the elements are percolating clusters
    /// A percolating cluster is a cluster with an element at the top and bottom
    pub fn get_percolating_clusters(&self, box_len: usize) -> Self {
        let mut pc: Vec<Vec<Site>> = vec![];
        for cluster in &self.clusters {
            if let Some(entry_element) = cluster.iter().find(|&&site| site.0 == box_len - 1) {
                // We found an element along the top row
                if let Some(exit_element) = cluster.iter().find(|&&site| site.0 == 0) {
                    // We also found an element along the bottom row
                    // Because this is a cluster, these must be connected
                    pc.push(cluster.clone());
                }
            }
        }
        Self { clusters: pc }
    }
}

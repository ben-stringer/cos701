use crate::data::lattice::Lattice;
use crate::rand::uniform::Uniform701;
use std::error::Error;

pub fn do_project_d() -> Result<(), Box<dyn Error>> {
    let mut uni = Uniform701::new();

    for &p in &[0.0, 0.5, 1.0] {
        let lattice = Lattice::populate(p, 50, &mut uni);
        let clusters = lattice.create_clusters();

        log::info!("Lattice with p={} has {} clusters.", p, clusters.len());

        for i in 0..clusters.len() {
            log::info!(
                "Lattice with p={} cluster {} has {} elements.",
                p,
                i,
                clusters[i].len()
            );
        }
    }
    Ok(())
}

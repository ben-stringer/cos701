use plotters::prelude::*;

use crate::data::lattice::{Clusters, Lattice};
use crate::rand::uniform::Uniform701;
use std::error::Error;
use std::ops::Range;

const BOX_LEN: usize = 50;
const N_ITER: usize = 20;
/// p is calculated as (i + 1)*5 to give a range of 0.05..0.95
const P_RANGE: Range<usize> = 0..19;

pub fn do_project_d() -> Result<(), Box<dyn Error>> {
    log::info!("Doing Project D");

    let mut uni = Uniform701::new();

    log::info!(
        "Generating clusters for p range {}-{}, with {} iterations each.",
        (P_RANGE.start + 1) * 5,
        P_RANGE.end * 5,
        N_ITER
    );

    let mut clusters = P_RANGE
        .map(|i| {
            let p = ((i + 1) * 5) as f64 * 0.01;
            (0..N_ITER)
                .map(|j| Lattice::populate(p, BOX_LEN, &mut uni).create_clusters())
                .collect::<Vec<Clusters>>()
        })
        .collect::<Vec<Vec<Clusters>>>();

    log::info!("Calculating cluster sizes for p=0.25, p=0.5, and p=7");
    let sizes = vec![
        (0.25, calculate_spread(&mut clusters[4])),
        (0.5, calculate_spread(&mut clusters[9])),
        (0.7, calculate_spread(&mut clusters[13])),
    ];

    plot_cluster_sizes("output/projectD/cluster_sizes.png", "Cluster Sizes", &sizes)?;

    plot_percolating_cluster_rates(
        "output/projectD/percolating_clusters.png",
        "Average percolating clusters",
        &clusters
            .iter()
            .map(|p_v| {
                p_v.iter()
                    .map(|v| v.get_percolating_clusters().clusters.len())
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>(),
    )?;
    Ok(())
}

fn calculate_spread(data: &mut Vec<Clusters>) -> (f64, f64, f64) {
    let n = data.len();
    let mut sizes: Vec<usize> = data
        .iter()
        .flat_map(|c| {
            c.clusters
                .iter()
                .map(|cluster| cluster.len())
                .collect::<Vec<usize>>()
        })
        .collect();
    sizes.sort_unstable();
    let min = sizes[0] as f64;
    let max = sizes[sizes.len() - 1] as f64;
    let avg = sizes.iter().sum::<usize>() as f64 / (sizes.len() - 1) as f64;
    (min, avg, max)
}

fn plot_cluster_sizes(
    path: &str,
    caption: &str,
    sizes: &Vec<(f64, (f64, f64, f64))>,
) -> Result<(), Box<dyn Error>> {
    log::info!("Plotting cluster size distribution");

    let root = BitMapBackend::new(path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_range = sizes.iter().map(|v| v.0).collect::<Vec<f64>>();

    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 50).into_font())
        .margin(32)
        .x_label_area_size(32)
        .y_label_area_size(32)
        .build_cartesian_2d(x_range.into_segmented(), (1.0..10000.0).log_scale())?;
    chart
        .configure_mesh()
        // .disable_mesh()
        .y_label_formatter(&|y| format!("{}", y))
        .draw()?;

    chart.draw_series(sizes.iter().map(|(p, q)| {
        ErrorBar::new_vertical(SegmentValue::CenterOf(p), q.0, q.1, q.2, BLUE.filled(), 10)
    }))?;

    Ok(())
}

fn plot_percolating_cluster_rates(
    path: &str,
    caption: &str,
    to_plot: &Vec<Vec<usize>>,
) -> Result<(), Box<dyn Error>> {
    log::info!("Plotting average number of percolating clusters");

    let root = BitMapBackend::new(path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 50).into_font())
        .margin(32)
        .x_label_area_size(32)
        .y_label_area_size(32)
        .build_cartesian_2d(-0.0..1.0, -0.0..1.0)?;
    chart
        .configure_mesh()
        // .disable_mesh()
        .draw()?;

    chart.draw_series(LineSeries::new(
        to_plot.iter().enumerate().map(|(usize, vals)| {
            let n_vals = vals.len() as f64;
            (
                ((usize + 1) * 5) as f64 * 0.01,
                vals.iter().sum::<usize>() as f64 / n_vals,
            )
        }),
        &BLUE,
    ))?;

    Ok(())
}

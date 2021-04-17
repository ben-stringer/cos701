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

    let lattices = P_RANGE
        .map(|i| {
            let p = ((i + 1) * 5) as f64 * 0.01;
            (0..N_ITER)
                .map(|j| Lattice::populate(p, BOX_LEN, &mut uni))
                .collect::<Vec<Lattice>>()
        })
        .collect::<Vec<Vec<Lattice>>>();

    let mut clusters = lattices
        .iter()
        .map(|lattices_for_p| {
            lattices_for_p
                .iter()
                .map(|lattice| lattice.create_clusters())
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

    draw_lattice("output/projectD/lattice_p_0.25.png",
    "Representative lattice for p=0.25",
    &lattices[4][0])?;
    draw_lattice("output/projectD/lattice_p_0.5.png",
                 "Representative lattice for p=0.5",
                 &lattices[9][0])?;
    draw_lattice("output/projectD/lattice_p_0.7.png",
                 "Representative lattice for p=0.7",
                 &lattices[13][0])?;

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
        ErrorBar::new_vertical(
            SegmentValue::CenterOf(p),
            q.0,
            q.1,
            q.2,
            BLUE.stroke_width(4),
            12,
        )
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
        BLUE.stroke_width(4),
    ))?;

    Ok(())
}

fn draw_lattice(path: &str, caption: &str, lattice: &Lattice) -> Result<(), Box<dyn Error>> {
    log::info!("Plotting lattice: '{}'", caption);

    let root = BitMapBackend::new(path, (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 50).into_font())
        .margin(32)
        .x_label_area_size(32)
        .y_label_area_size(32)
        .build_cartesian_2d(0..BOX_LEN, 0..BOX_LEN)?;
    chart
        .configure_mesh()
        // .disable_mesh()
        .draw()?;

    chart.draw_series(lattice.grid.iter().enumerate()
        .flat_map(|(i, row)| row.iter().enumerate()
            .map(move|(j, &occupied)| if occupied { Some((i, j)) } else { None }))
        .filter(Option::is_some).map(Option::unwrap)
        .map(|(i,j)| Rectangle::new([(i, j), (i+1,j+1)], BLACK.filled())
    ))?;
    Ok(())
}

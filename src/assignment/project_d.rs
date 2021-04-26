use plotters::prelude::*;

use crate::data::lattice::{Clusters, Lattice};
use crate::rand::uniform::Uniform701;
use std::error::Error;
use std::ops::RangeInclusive;
use std::time::Instant;

const BOX_LEN: usize = 50;
const N_ITER: usize = 20;
const P_RANGE: RangeInclusive<usize> = 20..=75;

pub fn do_project_d() -> Result<(), Box<dyn Error>> {
    log::info!("Doing Project D");

    let mut uni = Uniform701::new();

    log::info!(
        "Generating clusters for p range {}-{}, with {} iterations each.",
        *P_RANGE.start() as f64 * 0.01,
        *P_RANGE.end() as f64 * 0.01,
        N_ITER
    );

    let lattices = P_RANGE
        .map(|i| {
            let p = i as f64 * 0.01;
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
        (0.25, calculate_spread(&mut clusters[5])),
        (0.5, calculate_spread(&mut clusters[30])),
        (0.7, calculate_spread(&mut clusters[50])),
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

    draw_lattice(
        "output/projectD/lattice_p_0.25.png",
        "Representative lattice for p=0.25",
        &lattices[5][0],
    )?;
    draw_lattice(
        "output/projectD/lattice_p_0.5.png",
        "Representative lattice for p=0.5",
        &lattices[30][0],
    )?;
    draw_lattice(
        "output/projectD/lattice_p_0.7.png",
        "Representative lattice for p=0.7",
        &lattices[50][0],
    )?;

    calculate_runtimes(&mut uni)?;

    Ok(())
}

fn calculate_runtimes(uni: &mut Uniform701) -> Result<(), Box<dyn Error>> {
    log::info!("Calculating runtimes");

    let l_vals = (10..=50).step_by(5);

    let mut runtimes = vec![];

    for l in l_vals {
        let now = Instant::now();
        let average_pc = (0..10)
            .map(|_| {
                let lattice = Lattice::populate(0.6, l, uni);
                let clusters = lattice.create_clusters();
                let pc = clusters.get_percolating_clusters();
                pc.clusters.len()
            })
            .sum::<usize>() as f64
            / 10.0;
        log::trace!("Average num pc for l={} is {}", l, average_pc);
        let elapsed = now.elapsed().as_nanos();
        runtimes.push((l, elapsed));
    }

    log::info!("Plotting runtimes");

    let y_max = runtimes.iter().map(|&(_, v)| v).max().unwrap();

    let root = BitMapBackend::new("output/projectD/runtimes.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Runtimes for increasing values of L",
            ("sans-serif", 50).into_font(),
        )
        .margin(32)
        .x_label_area_size(32)
        .y_label_area_size(46)
        .build_cartesian_2d(10_usize..50_usize, 0..y_max)?;
    chart
        .configure_mesh()
        // .disable_mesh()
        .y_label_formatter(&|y| format!("{:E}", y))
        .x_desc("L value")
        .y_desc("Runtime in nanoseconds")
        .draw()?;

    chart.draw_series(LineSeries::new(
        runtimes.into_iter(),
        // .map(|&(l, t)| (SegmentValue::CenterOf(l), t)),
        BLUE.stroke_width(2),
    ))?;

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
        .y_label_area_size(42)
        .build_cartesian_2d(x_range.into_segmented(), (1.0..10000.0).log_scale())?;
    chart
        .configure_mesh()
        // .disable_mesh()
        .y_label_formatter(&|y| format!("{}", y))
        .x_desc("p-value")
        .y_desc("Cluster Size")
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
        .y_label_area_size(46)
        .build_cartesian_2d(-0.0..1.0, -0.0..1.0)?;
    chart
        .configure_mesh()
        // .disable_mesh()
        .x_desc("p-value")
        .y_desc("% lattices with percolating clusters")
        .draw()?;

    chart.draw_series(LineSeries::new(
        to_plot.iter().enumerate().map(|(i, vals)| {
            let n_vals = vals.len() as f64;
            (
                (i + 20) as f64 * 0.01,
                vals.iter().sum::<usize>() as f64 / n_vals,
            )
        }),
        BLUE.stroke_width(2),
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

    chart.draw_series(
        lattice
            .grid
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(j, &occupied)| if occupied { Some((i, j)) } else { None })
            })
            .filter(Option::is_some)
            .map(Option::unwrap)
            .map(|(i, j)| Rectangle::new([(i, j), (i + 1, j + 1)], BLACK.filled())),
    )?;
    Ok(())
}

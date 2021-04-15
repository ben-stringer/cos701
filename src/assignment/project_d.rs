use plotters::prelude::*;

use crate::data::lattice::Lattice;
use crate::rand::uniform::Uniform701;
use std::error::Error;
use std::ops::Range;

const BOX_LEN: usize = 50;
const N_ITER: usize = 20;
/// p is calculated as (i + 1)*5 to give a range of 0.05..0.95
const P_RANGE: Range<usize> = 0..19;

pub fn do_project_d() -> Result<(), Box<dyn Error>> {
    let mut uni = Uniform701::new();

    let mut results = vec![vec![0; N_ITER]; P_RANGE.end];

    for i in P_RANGE {
        let p = ((i + 1) * 5) as f64 * 0.01;
        for j in 0..N_ITER {
            let lattice = Lattice::populate(p, BOX_LEN, &mut uni);
            let clusters = lattice.create_clusters();
            let pc = clusters.get_percolating_clusters(BOX_LEN);
            results[i][j] = pc.clusters.len();
        }
    }

    plot_rates(
        "output/projectD/rates.png",
        "Average percolating clusters",
        &results,
    )?;
    Ok(())
}

fn plot_rates(path: &str, caption: &str, to_plot: &Vec<Vec<usize>>) -> Result<(), Box<dyn Error>> {
    log::info!("Plotting average percolating clusters");

    let root = BitMapBackend::new(path, (1440, 900)).into_drawing_area();
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
        to_plot.into_iter().enumerate().map(|(usize, vals)| {
            let n_vals = vals.len() as f64;
            (
                ((usize + 1) * 5) as f64 * 0.01,
                vals.into_iter().sum::<usize>() as f64 / n_vals,
            )
        }),
        &BLUE,
    ))?;

    // chart
    //     .configure_series_labels()
    //     .border_style(&BLACK)
    //     .background_style(&WHITE.mix(0.8))
    //     .draw()?;

    Ok(())
}

use plotters::prelude::*;
use std::collections::BTreeMap;

use crate::rng::boxmuller::BoxMullerGaussian701;
use crate::rng::clt_gaussian::CentralLimitTheoremGaussian701;
use crate::rng::uniform::Uniform701;
use std::ops::Range;

/// Entry-point for assignment 1
pub fn do_assignment_1() -> Result<(), Box<dyn std::error::Error>> {
    // println!("Generating a histogram using the uniform distribution");
    let mut uni = Uniform701::new();
    // generate_uniform_histogram(&mut uni)?;
    //
    // println!("Generating a histogram using the Box-Müller method");
    // let mut bm = BoxMullerGaussian701::new(&mut uni);
    // generate_boxmuller_histogram(&mut bm)?;

    println!("Generating a histogram using the central limit theorem method");
    let mut clt = CentralLimitTheoremGaussian701::new(&mut uni, 8);
    generate_clt_histogram(&mut clt)?;

    Ok(())
}

/// Sample from the supplied uniform random number generator, bin the results, and plot the bins
fn generate_uniform_histogram(uni: &mut Uniform701) -> Result<(), Box<dyn std::error::Error>> {
    let mut bins: BTreeMap<String, i32> = BTreeMap::new();

    (0..1_000_000)
        .map(|_| uni.next())
        .map(|v| format!("{:0.3}", v))
        .for_each(|k| {
            bins.entry(k).and_modify(|v| *v += 1).or_insert(1);
        });

    plot_histogram(
        "output/assignment1/uniform.png",
        "The Uniform Distribution, system supplied",
        0_f64..1_f64,
        0.001_f64,
        bins,
    )?;

    Ok(())
}

/// Sample from the supplied normal random number generator, bin the results, and plot the bins
fn generate_boxmuller_histogram(
    bm: &mut BoxMullerGaussian701,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut bins: BTreeMap<String, i32> = BTreeMap::new();

    (0..1_000_000)
        .map(|_| bm.next())
        .map(|v| format!("{:0.2}", v))
        .for_each(|k| {
            bins.entry(k).and_modify(|v| *v += 1).or_insert(1);
        });

    plot_histogram(
        "output/assignment1/box_muller.png",
        "The Box-Müller Method",
        -5_f64..5_f64,
        0.01_f64,
        bins,
    )?;

    Ok(())
}

/// Sample from the supplied normal random number generator, bin the results, and plot the bins
fn generate_clt_histogram(
    clt: &mut CentralLimitTheoremGaussian701,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut bins: BTreeMap<String, i32> = BTreeMap::new();

    (0..1_000_000)
        .map(|_| clt.next())
        .map(|v| format!("{:0.3}", v))
        .for_each(|k| {
            bins.entry(k).and_modify(|v| *v += 1).or_insert(1);
        });

    plot_histogram(
        "output/assignment1/clt_muller.png",
        "The Central Limit Theorem Method",
        0_f64..1_f64,
        0.001_f64,
        bins,
    )?;

    Ok(())
}

/// Plot a histogram for the supplied bins where the key represents the x-axis and the value
/// represents the y-axis.
fn plot_histogram(
    path: &str,
    caption: &str,
    x_range: Range<f64>,
    x_step: f64,
    bins: BTreeMap<String, i32>,
) -> Result<(), Box<dyn std::error::Error>> {
    let max_y = bins.values().max().unwrap();

    let root = BitMapBackend::new(path, (1440, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 50).into_font())
        .margin(16)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x_range.step(x_step).into_segmented(), 0..(*max_y))?;
    chart.configure_mesh().disable_mesh().draw()?;

    chart.draw_series(bins.iter().map(|(k, v)| {
        let x = k.parse::<f64>().unwrap();
        let y = *v;
        let x0 = SegmentValue::CenterOf(x);
        let x1 = SegmentValue::CenterOf(x + x_step);
        Rectangle::new([(x0, 0i32), (x1, y)], RED.filled())
    }))?;
    Ok(())
}

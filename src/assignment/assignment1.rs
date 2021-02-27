use plotters::prelude::*;
use std::collections::BTreeMap;

use crate::rng::boxmuller::BoxMullerGaussian701;
use crate::rng::clt_gaussian::CentralLimitTheoremGaussian701;
use crate::rng::uniform::Uniform701;
use std::f64::consts::PI;
use std::ops::Range;

const NUM_POINTS: i32 = 1_000_000;

/// Entry-point for assignment 1
pub fn do_assignment_1() -> Result<(), Box<dyn std::error::Error>> {
    let mut uni = Uniform701::new();

    println!("Generating a histogram using the uniform distribution");
    generate_uniform_histogram(&mut uni)?;

    // let mut bm = BoxMullerGaussian701::new(&mut uni);
    //
    // println!("Generating a histogram using the Box-Müller method");
    // generate_boxmuller_histogram(&mut bm)?;
    //
    // let mut clt = CentralLimitTheoremGaussian701::new(&mut uni, 8);
    //
    // println!("Generating a histogram using the central limit theorem method");
    // generate_clt_histogram(&mut clt)?;

    Ok(())
}

/// Sample from the supplied uniform random number generator, bin the results, and plot the bins
fn generate_uniform_histogram(uni: &mut Uniform701) -> Result<(), Box<dyn std::error::Error>> {
    let mut bins: BTreeMap<String, i32> = BTreeMap::new();

    (0..NUM_POINTS)
        .map(|_| uni.next())
        .map(|v| format!("{:0.3}", v))
        .for_each(|k| {
            bins.entry(k).and_modify(|v| *v += 1).or_insert(1);
        });

    plot_histogram(
        "output/assignment1/uniform.png",
        "The Uniform Distribution, system supplied",
        0.0..1.0,
        0.001,
        bins,
        Some(Box::new(|x| (x, 1000.0))),
    )?;

    Ok(())
}

/// Sample from the supplied normal random number generator, bin the results, and plot the bins
fn generate_boxmuller_histogram(
    bm: &mut BoxMullerGaussian701,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut bins: BTreeMap<String, i32> = BTreeMap::new();

    (0..NUM_POINTS)
        .map(|_| bm.next())
        .map(|v| format!("{:0.2}", v))
        .for_each(|k| {
            bins.entry(k).and_modify(|v| *v += 1).or_insert(1);
        });

    plot_histogram(
        "output/assignment1/box_muller.png",
        "The Box-Müller Method",
        -5.0..5.0,
        0.01,
        bins,
        Some(Box::new(|x| {
            let sigma: f64 = 1.0;
            let a = 1.0 / (2.0 * PI * sigma.powf(2.0)).sqrt();
            let b = -x.powf(2.0) / 2.0 * sigma.powf(2.0);
            (x, a * b.exp() * 10_000.0)
        })),
    )?;

    Ok(())
}

/// Sample from the supplied normal random number generator, bin the results, and plot the bins
fn generate_clt_histogram(
    clt: &mut CentralLimitTheoremGaussian701,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut bins: BTreeMap<String, i32> = BTreeMap::new();

    (0..NUM_POINTS)
        .map(|_| clt.next())
        .map(|v| format!("{:0.3}", v))
        .for_each(|k| {
            bins.entry(k).and_modify(|v| *v += 1).or_insert(1);
        });

    plot_histogram(
        "output/assignment1/clt_muller.png",
        "The Central Limit Theorem Method",
        0.0..1.0,
        0.001,
        bins,
        Some(Box::new(|x| {
            let sigma: f64 = 10.0;
            let a = 1.0 / (2.0 * PI * sigma.powf(2.0)).sqrt();
            let b = -(x - 0.5).powf(2.0) / 2.0 * sigma.powf(2.0);
            (x, a * b.exp() * 100_000.0)
        })),
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
    o_fun: Option<Box<dyn Fn(f64) -> (f64, f64)>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let max_y = *bins.values().max().unwrap() as f64;

    let root = BitMapBackend::new(path, (1440, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    {
        let mut chart = ChartBuilder::on(&root)
            .caption(caption, ("sans-serif", 50).into_font())
            .margin(32)
            .x_label_area_size(32)
            .y_label_area_size(32)
            .build_cartesian_2d(x_range.to_owned().step(x_step).into_segmented(), 0.0..max_y)?;
        chart.configure_mesh().disable_mesh().draw()?;

        chart.draw_series(bins.iter().map(|(k, v)| {
            let x = k.parse::<f64>().unwrap();
            let y = *v as f64;
            let x0 = SegmentValue::CenterOf(x);
            let x1 = SegmentValue::CenterOf(x + x_step);
            Rectangle::new([(x0, 0.0), (x1, y)], RED.filled())
        }))?;
    }

    if let Some(fun) = o_fun {
        let mut chart = ChartBuilder::on(&root)
            .caption(caption, ("sans-serif", 50).into_font())
            .margin(32)
            .x_label_area_size(32)
            .y_label_area_size(32)
            .build_cartesian_2d(x_range.to_owned(), 0.0..max_y)?;
        chart.configure_mesh().disable_mesh().draw()?;
        chart.draw_series(LineSeries::new(
            x_range
                .to_owned()
                .step(x_step)
                .key_points(1_000_000)
                .into_iter()
                .map(fun),
            &BLUE,
        ))?;
    }
    Ok(())
}

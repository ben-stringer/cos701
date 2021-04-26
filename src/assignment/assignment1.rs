use plotters::prelude::*;
use std::collections::BTreeMap;

use crate::rand::boxmuller::BoxMullerGaussian701;
use crate::rand::clt_gaussian::CentralLimitTheoremGaussian701;
use crate::rand::exponential::Exponential701;
use crate::rand::inverse_transform::InverseTransform701;
use crate::rand::uniform::Uniform701;
use std::error::Error;
use std::f64::consts::PI;
use std::ops::Range;

const NUM_POINTS: i32 = 1_000_000;

/// Entry-point for assignment 1
pub fn do_assignment_1() -> Result<(), Box<dyn Error>> {
    log::info!("Doing assignment 1");

    generate_uniform_histogram(Uniform701::new())?;
    part_1a(&mut Exponential701::new(Uniform701::new(), 1.0, 1.0))?;
    part_1b(&mut BoxMullerGaussian701::new(Uniform701::new()))?;
    part_1c(&mut CentralLimitTheoremGaussian701::new(
        Uniform701::new(),
        8,
    ))?;
    part_1d(&mut InverseTransform701::new(Uniform701::new(), 1.0))?;

    Ok(())
}

/// Sample from the supplied uniform random number generator, bin the results, and plot the bins
fn generate_uniform_histogram(mut uni: Uniform701) -> Result<(), Box<dyn Error>> {
    log::info!("Generating a histogram using the uniform distribution");

    let mut bins: BTreeMap<String, i32> = BTreeMap::new();

    (0..NUM_POINTS)
        .map(|_| uni.next())
        .map(|v| format!("{:0.3}", v))
        .for_each(|k| {
            bins.entry(k).and_modify(|v| *v += 1).or_insert(1);
        });

    let max_y = bins.values().max().unwrap().to_owned() as f64;

    plot_histogram(
        "output/assignment1/uniform.png",
        "The Uniform Distribution, system supplied",
        0.0..1.0,
        0.001,
        max_y,
        bins,
        Some((Box::new(|&x| (x, 1000.0)), "y = μ")),
    )?;

    Ok(())
}

/// Sample from the supplied exponential random number generator, bin the results, and plot the bins
fn part_1a(exp: &mut Exponential701) -> Result<(), Box<dyn Error>> {
    log::info!("Generating a histogram using the exponential distribution");

    let mut bins: BTreeMap<String, i32> = BTreeMap::new();
    (0..1_000_000)
        .map(|_| exp.next())
        .map(|v| format!("{:0.3}", v))
        .for_each(|k| {
            bins.entry(k).and_modify(|v| *v += 1).or_insert(1);
        });

    let max_y = bins.values().max().unwrap().to_owned() as f64;

    plot_histogram(
        "output/assignment1/exponential.png",
        &format!("Exponential distribution: a = {}, b = {}", exp.a(), exp.b()),
        0.0..1.0,
        0.001,
        max_y,
        bins,
        Some((
            Box::new(|&x| (x, exp.a() * (-exp.b() * x).exp() * 1_000.0)),
            "ae^(-bx)",
        )),
    )?;

    Ok(())
}

/// Sample from the supplied box muller random number generator, bin the results, and plot the bins
fn part_1b(bm: &mut BoxMullerGaussian701) -> Result<(), Box<dyn Error>> {
    log::info!("Generating a histogram using the Box-Müller method");

    let mut bins: BTreeMap<String, i32> = BTreeMap::new();

    (0..NUM_POINTS)
        .map(|_| bm.next())
        .map(|v| format!("{:0.3}", v))
        .for_each(|k| {
            bins.entry(k).and_modify(|v| *v += 1).or_insert(1);
        });

    let max_y = bins.values().max().unwrap().to_owned() as f64;

    plot_histogram(
        "output/assignment1/box_muller.png",
        "The Box-Müller Method",
        0.0..1.0,
        0.001,
        max_y,
        bins,
        Some((
            Box::new(|&x| {
                let sigma: f64 = 10.0;
                let a = 1.0 / (2.0 * PI * sigma.powf(2.0)).sqrt();
                let b = -(x - 0.5).powf(2.0) / 2.0 * sigma.powf(2.0);
                let y = a * b.exp() * 100_000.0;
                (x, y)
            }),
            "y = (1 / √(2πσ^2)) * e^( x^2 / 2σ^2 )",
        )),
    )?;

    Ok(())
}

/// Sample from the supplied central limit theorem gaussian random number generator,
/// bin the results, and plot the bins
fn part_1c(clt: &mut CentralLimitTheoremGaussian701) -> Result<(), Box<dyn Error>> {
    log::info!("Generating a histogram using the central limit theorem method");

    let mut bins: BTreeMap<String, i32> = BTreeMap::new();

    (0..NUM_POINTS)
        .map(|_| clt.next())
        .map(|v| format!("{:0.3}", v))
        .for_each(|k| {
            bins.entry(k).and_modify(|v| *v += 1).or_insert(1);
        });

    let max_y = bins.values().max().unwrap().to_owned() as f64;

    plot_histogram(
        "output/assignment1/clt_muller.png",
        "The Central Limit Theorem Method",
        0.0..1.0,
        0.001,
        max_y,
        bins,
        Some((
            Box::new(|&x| {
                let sigma: f64 = 10.25;
                let a = 1.0 / (2.0 * PI * sigma.powf(2.0)).sqrt();
                let b = -(x - 0.5).powf(2.0) / 2.0 * sigma.powf(2.0);
                let y = a * b.exp() * 100_000.0;
                (x, y)
            }),
            "y = 1 / √(2πσ^2) * e^( (x-0.5)^2 / 2σ^2 )",
        )),
    )?;

    Ok(())
}

/// Sample from the supplied inverse transform random number generator,
/// bin the results, and plot the bins
fn part_1d(inv: &mut InverseTransform701) -> Result<(), Box<dyn Error>> {
    log::info!("Generating a histogram using the inverse transform method");

    let mut bins: BTreeMap<String, i32> = BTreeMap::new();

    (0..NUM_POINTS)
        .map(|_| inv.next())
        .map(|v| format!("{:0.3}", v))
        .for_each(|k| {
            bins.entry(k).and_modify(|v| *v += 1).or_insert(1);
        });

    let max_y = bins.values().max().unwrap().to_owned() as f64;

    plot_histogram(
        "output/assignment1/inv_trans.png",
        &format!("The Inverse Transform Method, σ = {}", inv.sigma()),
        0.0..5.0,
        0.005,
        max_y,
        bins,
        Some((
            Box::new(|&x| {
                let y = (x / inv.sigma().powf(2.0))
                    * (-1.0 * x.powf(2.0) / (2.0 * inv.sigma().powf(2.0))).exp()
                    * 1_050.0;
                (x, y)
            }),
            "y = (x/σ^2) * e^( -(x^2)/(2σ^2) )",
        )),
    )?;

    Ok(())
}

/// Plot a histogram for the supplied bins where the key represents the x-axis and the value
/// represents the y-axis.  Optionally take a function to plot a curve.
fn plot_histogram<'a>(
    path: &str,
    caption: &str,
    x_range: Range<f64>,
    x_step: f64,
    max_y: f64,
    bins: BTreeMap<String, i32>,
    optional_curve: Option<(Box<dyn Fn(&f64) -> (f64, f64) + 'a>, &str)>,
) -> Result<(), Box<dyn Error>> {
    log::info!("Plotting '{}'.", caption);

    let root = BitMapBackend::new(path, (1440, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 50).into_font())
        .margin(32)
        .x_label_area_size(32)
        .y_label_area_size(32)
        .build_cartesian_2d(x_range.to_owned().step(x_step).into_segmented(), 0.0..1.0)?
        .set_secondary_coord(x_range.to_owned(), 0.0..1.0);
    chart.configure_mesh().disable_mesh().draw()?;

    chart.draw_series(bins.iter().map(|(k, v)| {
        let x = k.parse::<f64>().unwrap();
        let y = *v as f64 / max_y;
        let x0 = SegmentValue::CenterOf(x);
        let x1 = SegmentValue::CenterOf(x + x_step);
        Rectangle::new([(x0, 0.0), (x1, y)], RED.filled())
    }))?;

    if let Some((curve_fn, curve_label)) = optional_curve {
        chart
            .draw_secondary_series(LineSeries::new(
                x_range
                    .to_owned()
                    .step(x_step)
                    .key_points(1_000_000)
                    .iter()
                    .map(curve_fn)
                    .map(|(x, y)| (x, y / max_y)),
                BLUE.stroke_width(4),
            ))?
            .label(curve_label)
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE.stroke_width(4)));
        chart
            .configure_series_labels()
            .border_style(&BLACK)
            .background_style(&WHITE.mix(0.8))
            .draw()?;
    }

    log::info!("Done drawing '{}'", caption);
    Ok(())
}

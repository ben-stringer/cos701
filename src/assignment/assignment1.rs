use plotters::prelude::*;
use std::collections::BTreeMap;

use crate::rng::boxmuller::BoxMullerGaussian701;
use crate::rng::clt_gaussian::CentralLimitTheoremGaussian701;
use crate::rng::exponential::Exponential701;
use crate::rng::inverse_transform::InverseTransform701;
use crate::rng::uniform::Uniform701;
use std::error::Error;
use std::f64::consts::PI;
use std::ops::Range;

const NUM_POINTS: i32 = 1_000_000;

/// Entry-point for assignment 1
pub fn do_assignment_1() -> Result<(), Box<dyn Error>> {
    println!("Doing assignment 1");

    let mut uni = Uniform701::new();

    println!("Generating a histogram using the uniform distribution");
    generate_uniform_histogram(&mut uni)?;

    println!("Generating a histogram using the exponential distribution");
    let mut exp = Exponential701::new(&mut uni, 1.0, 1.0);
    generate_exponential_histogram(&mut exp)?;

    println!("Generating a histogram using the Box-Müller method");
    let mut bm = BoxMullerGaussian701::new(&mut uni);
    generate_boxmuller_histogram(&mut bm)?;

    println!("Generating a histogram using the central limit theorem method");
    let mut clt = CentralLimitTheoremGaussian701::new(&mut uni, 8);
    generate_clt_histogram(&mut clt)?;

    println!("Generating a histogram using the inverse transform method");
    let mut inv = InverseTransform701::new(&mut uni, 1.0);
    generate_inv_trans(&mut inv)?;

    Ok(())
}

/// Sample from the supplied uniform random number generator, bin the results, and plot the bins
fn generate_uniform_histogram(uni: &mut Uniform701) -> Result<(), Box<dyn Error>> {
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
        Some((Box::new(|x| Some((x, 1000.0))), "y = 1000")),
    )?;

    Ok(())
}

/// Sample from the supplied exponential random number generator, bin the results, and plot the bins
fn generate_exponential_histogram(exp: &mut Exponential701) -> Result<(), Box<dyn Error>> {
    let mut bins: BTreeMap<String, i32> = BTreeMap::new();

    (0..1_000_000)
        .map(|_| exp.next())
        // .map(|v| (v - 0.368) * (1.0 / 0.632))
        .map(|v| format!("{:0.3}", v))
        .for_each(|k| {
            bins.entry(k).and_modify(|v| *v += 1).or_insert(1);
        });

    let max_y = bins.values().max().unwrap().to_owned() as f64;

    plot_histogram(
        "output/assignment1/exponential.png",
        "Exponential distribution",
        0.0..1.0,
        0.001,
        max_y,
        bins,
        Some((
            Box::new(|x| {
                let sigma: f64 = 1.0;
                let a = 1.0 / (2.0 * PI * sigma.powf(2.0)).sqrt();
                let b = -x.powf(2.0) / 2.0 * sigma.powf(2.0);
                Some((x, a * b.exp() * 10_000.0))
            }),
            "",
        )),
    )?;

    Ok(())
}

/// Sample from the supplied box muller random number generator, bin the results, and plot the bins
fn generate_boxmuller_histogram(bm: &mut BoxMullerGaussian701) -> Result<(), Box<dyn Error>> {
    let mut bins: BTreeMap<String, i32> = BTreeMap::new();

    (0..NUM_POINTS)
        .map(|_| bm.next())
        .map(|v| format!("{:0.2}", v))
        .for_each(|k| {
            bins.entry(k).and_modify(|v| *v += 1).or_insert(1);
        });

    let max_y = bins.values().max().unwrap().to_owned() as f64;

    plot_histogram(
        "output/assignment1/box_muller.png",
        "The Box-Müller Method",
        -5.0..5.0,
        0.01,
        max_y,
        bins,
        Some((
            Box::new(|x| {
                let sigma: f64 = 1.0;
                let a = 1.0 / (2.0 * PI * sigma.powf(2.0)).sqrt();
                let b = -x.powf(2.0) / 2.0 * sigma.powf(2.0);
                Some((x, a * b.exp() * 10_000.0))
            }),
            "",
        )),
    )?;

    Ok(())
}

/// Sample from the supplied central limit theorem gaussian random number generator,
/// bin the results, and plot the bins
fn generate_clt_histogram(clt: &mut CentralLimitTheoremGaussian701) -> Result<(), Box<dyn Error>> {
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
            Box::new(|x| {
                let sigma: f64 = 10.0;
                let a = 1.0 / (2.0 * PI * sigma.powf(2.0)).sqrt();
                let b = -(x - 0.5).powf(2.0) / 2.0 * sigma.powf(2.0);
                let y = a * b.exp() * 100_000.0;
                if y > max_y {
                    None
                } else {
                    Some((x, y))
                }
            }),
            "y = 1 / √(2πσ^2) * e^( (-0.5x)^2 / 2σ^2 )",
        )),
    )?;

    Ok(())
}

/// Sample from the supplied inverse transform random number generator,
/// bin the results, and plot the bins
fn generate_inv_trans(clt: &mut InverseTransform701) -> Result<(), Box<dyn Error>> {
    let mut bins: BTreeMap<String, i32> = BTreeMap::new();

    (0..NUM_POINTS)
        .map(|_| clt.next())
        .map(|v| format!("{:0.3}", v))
        .for_each(|k| {
            bins.entry(k).and_modify(|v| *v += 1).or_insert(1);
        });

    let max_y = bins.values().max().unwrap().to_owned() as f64;

    plot_histogram(
        "output/assignment1/inv_trans.png",
        "The Inverse Transform Method",
        0.0..5.0,
        0.005,
        max_y,
        bins,
        Some((
            Box::new(|x| {
                let y = (x / clt.sigma().powf(2.0))
                    * (-1.0 * x.powf(2.0) / (2.0 * clt.sigma().powf(2.0))).exp()
                    * 1_050.0;
                if y > max_y {
                    None
                } else {
                    Some((x, y))
                }
            }),
            "",
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
    optional_curve: Option<(Box<dyn Fn(f64) -> Option<(f64, f64)> + 'a>, &str)>,
) -> Result<(), Box<dyn Error>> {
    println!("Plotting '{}'.", caption);

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

    if let Some((curve_fn, curve_label)) = optional_curve {
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
                .map(curve_fn)
                .filter(|o| o.is_some())
                .map(|o| o.unwrap()),
            &BLUE,
        ))?;
        // .label(curve_label)
        // .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
        // chart
        //     .configure_series_labels()
        //     .border_style(&BLACK)
        //     .background_style(&WHITE.mix(0.8))
        //     .draw()?;
    }

    println!("Done drawing '{}'", caption);
    Ok(())
}

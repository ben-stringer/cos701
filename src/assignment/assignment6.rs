use ordered_float::OrderedFloat;
use plotters::prelude::*;

use crate::data::delaunay::dealunay_2d;
use crate::data::line::Line2d;
use crate::data::point::Point2d;
use crate::data::points_in_grid::gen_spaced_points_in_box;
use crate::data::voronoi::voronoi_701;
use crate::rand::uniform::Uniform701;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn do_assignment_6() -> Result<(), Box<dyn Error>> {
    log::info!("Doing assignment 6");

    // let sites = use_saved_sites();

    let mut uni = Uniform701::new();

    let sites = gen_spaced_points_in_box(&mut uni, 20.0, 500, 0.7);

    do_part_a(&sites)?;
    do_part_b(&sites)?;

    Ok(())
}

fn do_part_a(sites: &[Point2d]) -> Result<(), Box<dyn Error>> {
    log::info!("Doing part a");

    plot_voronoi_diagram(
        "output/assignment6/part_6a.png",
        "Using voronoi library",
        sites,
        &voronoi::make_polygons(&voronoi::voronoi(
            sites
                .iter()
                .map(|&v| voronoi::Point::new(v.x, v.y))
                .collect::<Vec<voronoi::Point>>(),
            800.0,
        ))
        .iter()
        .map(|polygon| {
            polygon
                .windows(2)
                .map(|line| [(line[0].x, line[0].y), (line[1].x, line[1].y)])
                .collect::<Vec<[(OrderedFloat<f64>, OrderedFloat<f64>); 2]>>()
        })
        .flatten()
        .map(|line| {
            let src = line[0];
            let dst = line[1];
            let sx = src.0;
            let sy = src.1;
            let dx = dst.0;
            let dy = dst.1;
            Line2d::from(((sx.0, sy.0), (dx.0, dy.0)))
        })
        .collect::<Vec<Line2d>>(),
    )?;

    Ok(())
}

fn do_part_b(sites: &[Point2d]) -> Result<(), Box<dyn Error>> {
    log::info!("Doing part b");

    let triangulation = dealunay_2d(sites, 4.0, false);

    let lines = voronoi_701(&sites, &triangulation);

    plot_voronoi_diagram(
        "output/assignment6/part_6b.png",
        "Homemade Voronoi Diagram",
        sites,
        &lines,
    )?;
    Ok(())
}

fn plot_voronoi_diagram(
    path: &str,
    caption: &str,
    sites: &[Point2d],
    lines: &[Line2d],
) -> Result<(), Box<dyn Error>> {
    log::info!("Plotting {}", &caption);

    let root = BitMapBackend::new(path, (1440, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 50).into_font())
        .margin(32)
        .x_label_area_size(32)
        .y_label_area_size(32)
        .build_cartesian_2d(-0.5..20.5, -0.5..20.5)?;
    chart.configure_mesh().draw()?;

    if let Err(err) = chart.draw_series(
        sites
            .iter()
            .map(|&coord| Circle::new(coord.into(), 2, BLACK.filled())),
    ) {
        log::error!("Error occurred drawing sites!  Details: {:?}", err);
    }

    lines
        .iter()
        .map(|&line| (line.src, line.dst))
        .for_each(|(src, dst)| {
            if let Err(err) = chart.draw_series(LineSeries::new(
                vec![src.to_owned().into(), dst.to_owned().into()],
                &BLUE,
            )) {
                log::error!("Error occurred drawing a line!  Details: {:?}", err);
            }
        });

    Ok(())
}

fn use_saved_sites() -> Vec<Point2d> {
    let fin = BufReader::new(File::open("output/scratch/a6_pts.txt").unwrap());

    fin.lines()
        .filter_map(|maybe_line| match maybe_line {
            Ok(line) => {
                let parts = line.split(' ').collect::<Vec<&str>>();
                Some(
                    (
                        f64::from_str(parts[0]).unwrap(),
                        f64::from_str(parts[1]).unwrap(),
                    )
                        .into(),
                )
            }
            Err(_) => None,
        })
        .collect::<Vec<Point2d>>()
}

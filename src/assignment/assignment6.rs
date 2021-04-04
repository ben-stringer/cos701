use ordered_float::OrderedFloat;
use plotters::prelude::*;
use voronoi::{make_polygons, voronoi, Point};

use crate::data::line::Line2d;
use crate::data::neighbors::NearestNeighborMap;
use crate::data::point::Point2d;
use crate::rand::points_in_grid::gen_points_in_box;
use crate::rand::uniform::Uniform701;
use std::error::Error;

pub fn do_assignment_6() -> Result<(), Box<dyn Error>> {
    log::info!("Doing assignment 6");

    let mut uni = Uniform701::new();

    let sites = gen_points_in_box(&mut uni, 20.0, 500, 0.7);
    let nn_map = NearestNeighborMap::first_2d(&sites, 3.0);

    do_part_a(&sites)?;
    do_part_b(&sites)?;

    Ok(())
}

fn do_part_a(sites: &Vec<Point2d>) -> Result<(), Box<dyn Error>> {
    log::info!("Doing part a");

    plot_voronoi_diagram(
        "output/assignment6/part_6a.png",
        "Using voronoi library",
        sites,
        &make_polygons(&voronoi(
            sites
                .into_iter()
                .map(|&v| Point::new(v.x, v.y))
                .collect::<Vec<Point>>(),
            800.0,
        ))
        .into_iter()
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
        .collect(),
    )?;

    Ok(())
}

fn do_part_b(sites: &Vec<Point2d>) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn plot_voronoi_diagram(
    path: &str,
    caption: &str,
    sites: &Vec<Point2d>,
    lines: &Vec<Line2d>,
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
        (&sites)
            .into_iter()
            .map(|&coord| Circle::new(coord.into(), 2, BLACK.filled())),
    ) {
        log::error!("Error occurred drawing sites!  Details: {:?}", err);
    }

    lines
        .into_iter()
        .map(|line| (line.src, line.dst))
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

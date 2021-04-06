use ordered_float::OrderedFloat;
use plotters::prelude::*;
use voronoi::{make_polygons, voronoi, Point};

use crate::data::line::Line2d;
use crate::data::neighbors::NearestNeighborMap;
use crate::data::point::Point2d;
use crate::rand::points_in_grid::gen_points_in_box;
use crate::rand::uniform::Uniform701;
use std::error::Error;
use std::f64::consts::PI;

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
    log::info!("Doing part b");

    let nn_map = NearestNeighborMap::first_2d(sites, 2.0);
    let bounding_lines: Vec<Line2d> = (100..101)//sites.len())
        .map(|src_i| {
            // for each site
            let src = sites[src_i];
            let neighbors = &nn_map.neighbors[src_i];
            // create a line to each neighbor
            let mut spokes: Vec<Line2d> = neighbors
                .into_iter()
                .map(|&site_j| sites[site_j])
                .map(|dst| (src, dst).into())
                .collect();
            // Sort the lines by their angle
            spokes.sort_by(|&l, &r| l.angle().partial_cmp(&r.angle()).unwrap());
            // Convert the lines into their perpendicular bisectors
            let mut bisectors: Vec<Line2d> = spokes
                .into_iter()
                .map(|spoke| spoke.perpendicular_bisector())
                .collect();
            // Close the shape
            bisectors.push(bisectors[0].clone());


            // Calculate the intersection point between each PB
            let mut intersections: Vec<Option<Point2d>> = bisectors
                .windows(2)
                .map(|b| b[0].intersection(&b[1]))
                .collect();
            // Close the shape
            intersections.push(intersections[0].clone());
            // Convert the intersections into lines

            let mut bounding_lines = intersections
                .windows(2)
                .filter_map(|pts| {
                    let src = pts[0];
                    let dst = pts[1];
                    if src.is_none() || dst.is_none() { None } else {
                        Some(Line2d {src: src.unwrap(), dst:dst.unwrap()})
                    }
                })
                .collect::<Vec<Line2d>>();

            let mut i = 0;
            while i < bounding_lines.len() - 1 {
                let mut j = i+1;
                let mut ln_i = bounding_lines[i];
                let mut next = None;
                while j < bounding_lines.len() && next.is_none() {
                    let ln_j = bounding_lines[j];
                    let angle = ln_i.angle_between(&ln_j);
                    if angle < PI {
                        next = Some(i);
                    } else {
                        bounding_lines.remove(j);
                        j+= 1;
                        let mut ln_k = if j < bounding_lines.len() {
                            bounding_lines[j]
                        } else {
                            bounding_lines[0]
                        };
                        let new_intersection = ln_i.intersection(&ln_k).unwrap();
                        ln_i.dst = new_intersection.clone();
                        ln_k.src = new_intersection.clone();
                    }
                }
                i+=j;
            }
            bounding_lines
        })
        .flatten()
        .collect();

    plot_voronoi_diagram(
        "output/assignment6/part_6b.png",
        "Homemade Voronoi Diagram",
        sites,
        &bounding_lines,
    )?;
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

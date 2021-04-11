use ordered_float::OrderedFloat;
use plotters::prelude::*;
use voronoi;

use crate::data::delaunay::dealunay_701;
use crate::data::line::Line2d;
use crate::data::point::Point2d;
use crate::data::voronoi::voronoi_701;
use crate::rand::points_in_grid::gen_points_in_box;
use crate::rand::uniform::Uniform701;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn do_assignment_6() -> Result<(), Box<dyn Error>> {
    log::info!("Doing assignment 6");

    let sites = read_points();

    // let mut uni = Uniform701::new();
    //
    // let sites = gen_points_in_box(&mut uni, 20.0, 500, 0.7);
    // let nn_map = NearestNeighborMap::first_2d(&sites, 3.0);

    do_part_a(&sites)?;
    do_part_b(&sites)?;

    Ok(())
}

fn read_points() -> Vec<Point2d> {
    let fin = BufReader::new(File::open("output/scratch/a6_pts.txt").unwrap());

    fin.lines()
        .map(|maybe_line| {
            let line = maybe_line.unwrap();
            let parts = line.split(" ").collect::<Vec<&str>>();
            (
                f64::from_str(parts[0]).unwrap(),
                f64::from_str(parts[1]).unwrap(),
            )
                .into()
        })
        .collect::<Vec<Point2d>>()
}

fn do_part_a(sites: &Vec<Point2d>) -> Result<(), Box<dyn Error>> {
    log::info!("Doing part a");

    plot_voronoi_diagram(
        "output/assignment6/part_6a.png",
        "Using voronoi library",
        sites,
        &voronoi::make_polygons(&voronoi::voronoi(
            sites
                .into_iter()
                .map(|&v| voronoi::Point::new(v.x, v.y))
                .collect::<Vec<voronoi::Point>>(),
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

    let triangulation = dealunay_701(sites, 4.0, false);

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

// fn worst_code_ever() {
//
//     let nn_map = NearestNeighborMap::first_2d(sites, 2.5);
//     let bounding_lines: Vec<Line2d> = //(100..101)
//         (0..sites.len())
//             .map(|src_i| {
//                 // for each site
//                 let src = sites[src_i];
//                 let neighbors = &nn_map.neighbors[src_i];
//                 // create a line to each neighbor
//                 let mut spokes: Vec<Line2d> = neighbors
//                     .into_iter()
//                     .map(|&site_j| sites[site_j])
//                     .map(|dst| (src, dst).into())
//                     .collect();
//                 let shortest = (0..spokes.len())
//                     .reduce(|i, j| if spokes[i].length() < spokes[j].length() { i } else { j })
//                     .unwrap();
//                 let removed = spokes.remove(shortest);
//                 spokes.insert(0, removed);
//                 // Sort the lines by their angle
//                 spokes.sort_by(|&l, &r| l.angle().partial_cmp(&r.angle()).unwrap());
//
//                 let mut i = 0;
//                 let mut j = 1;
//                 let mut has_mods = true;
//
//                 while i != 0 || has_mods {
//                     if i ==0 {has_mods = false}
//                     let spoke_i = spokes[i];
//                     let spoke_j = spokes[j];
//                     let pb_i = spoke_i.perpendicular_bisector();
//                     let pb_j = spoke_j.perpendicular_bisector();
//
//                     let the_angle = pb_i.angle_between(&pb_j);
//                     log::info!("At site {}, considering i = {}, j = {}, angle = {:.2}, # spokes = {}", src_i, i, j, the_angle, spokes.len());
//
//                     if the_angle >= PI {
//                         has_mods = true;
//                         spokes.remove(j);
//                         let len = spokes.len();
//                         i = i % len;
//                         j = j % len;
//                         if i == j { j = j + 1; }
//                     } else {
//                         let len = spokes.len();
//                         i = (i + 1) % len;
//                         j = (j + 1) % len;
//                     }
//                 }
//
//                 // Convert the lines into their perpendicular bisectors
//                 let bisectors: Vec<Line2d> = spokes
//                     .into_iter()
//                     .map(|spoke| spoke.perpendicular_bisector())
//                     .collect();
//
//                 // // let mut k = 2;
//                 // let mut has_mods = true;
//                 // while i != k && i != 0 || has_mods {
//                 //     if i == 0 {
//                 //         has_mods = false;
//                 //     }
//                 //     let bisector_i = bisectors[i];
//                 //     let bisector_j = bisectors[j];
//                 //     let bisector_k = bisectors[k];
//                 //
//                 //     let intr_ij = bisector_i.intersection(&bisector_j);
//                 //     let intr_ik = bisector_i.intersection(&bisector_j);
//                 //     let line_ij = Line2d { src : bisector_i.src, dst : intr_ij.unwrap() };
//                 //     let line_ik = Line2d { src : bisector_i.src, dst : intr_ik.unwrap() };
//                 //     if line_ij.length() > line_ik.length() {
//                 //         has_mods = true;
//                 //         bisectors.remove(j);
//                 //         let len = bisectors.len() - 1;
//                 //         j = j % len;
//                 //         k = k % len;
//                 //     } else {
//                 //         let len = bisectors.len() - 1;
//                 //         i = (i + 1) % len;
//                 //         j = (j + 1) % len;
//                 //         k = (k + 1) % len;
//                 //     }
//                 // }
//
//                 // if intersections.len() < 3 {
//                 //     log::warn!("Not enough intersecting points to bound the point; bailing.");
//                 //     vec![]
//                 // } else {
//                 //     // Have to remove lines that are outside the inner bounding region
//                 //     let mut i = 0;
//                 //     let mut j = 1;
//                 //     let mut k = 2;
//                 //     let mut has_mods = true;
//                 //     while i != k && i != 0 || has_mods {
//                 //         if i == 0 {
//                 //             has_mods = false;
//                 //         }
//                 //         let pt_i = intersections[i];
//                 //         let pt_j = intersections[j];
//                 //         let pt_k = intersections[k];
//                 //         let src = Line2d {
//                 //             src: pt_i,
//                 //             dst: pt_j,
//                 //         };
//                 //         let dst = Line2d {
//                 //             src: pt_j,
//                 //             dst: pt_k,
//                 //         };
//                 //         let angle = src.angle_between(&dst);
//                 //         // log::info!("Angle between {}->{} is {}", src, dst, angle);
//                 //         if angle > PI {
//                 //             // Including pt_j will create a concave hull, drop it
//                 //             intersections.remove(j);
//                 //             has_mods = true;
//                 //             let len = intersections.len()-1;
//                 //             i = i % len;
//                 //             j = j % len;
//                 //             k = k % len;
//                 //             log::info!("Dropped an intersection i, j, k = {}, {}, {}", i, j, k);
//                 //         } else {
//                 //             // i, j, k are all good points, advance
//                 //             let len = intersections.len()-1;
//                 //             i = (i + 1) % len;
//                 //             j = (j + 1) % len;
//                 //             k = (k + 1) % len;
//                 //         }
//                 //     }
//                 // }
//                 // Calculate the intersection point between each PB
//                 let mut intersections: Vec<Point2d> = bisectors
//                     .windows(2)
//                     .filter_map(|b| b[0].intersection(&b[1]))
//                     .collect();
//                 intersections.push(intersections[0]);
//                 intersections
//                     .windows(2)
//                     .map(|pts| Line2d {
//                         src: pts[0],
//                         dst: pts[1],
//                     })
//                     .collect::<Vec<Line2d>>()
//             })
//             .flatten()
//             .collect();
// }

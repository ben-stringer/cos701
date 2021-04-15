use plotters::prelude::*;

use crate::data::delaunay::{dealunay_2d, to_lines_2d};
use crate::data::point::Point2d;
use crate::rand::points_in_grid::gen_points_in_box;
use crate::rand::uniform::Uniform701;
use std::error::Error;

pub fn do_assignment_5() -> Result<(), Box<dyn Error>> {
    log::info!("Doing assignment 5");

    let mut uni = Uniform701::new();

    let sites = gen_points_in_box(&mut uni, 20.0, 500, 0.7);

    do_part_a(&sites)?;
    do_part_b(&sites)?;

    Ok(())
}

fn do_part_a(sites: &Vec<Point2d>) -> Result<(), Box<dyn Error>> {
    log::info!("Doing part a");

    let wrapped_points = &sites
        .iter()
        .map(|&v| delaunator::Point { x: v.x, y: v.y })
        .collect::<Vec<delaunator::Point>>();

    let mut lines = delaunator::triangulate(wrapped_points)
        .expect("No triangulation for the points exists!")
        .triangles
        .chunks(3)
        .flat_map(|t| vec![(t[0], t[1]), (t[1], t[2]), (t[2], t[0])])
        .map(|(i, j)| if i > j { (j, i) } else { (i, j) })
        .collect::<Vec<(usize, usize)>>();

    lines.sort_unstable();
    lines.dedup();

    plot_triangulation(
        "output/assignment5/part_5a.png",
        "Using Delaunay triangulation library",
        sites,
        &lines,
    )?;

    Ok(())
}

fn do_part_b(sites: &Vec<Point2d>) -> Result<(), Box<dyn Error>> {
    log::info!("Doing part b");

    let lines = to_lines_2d(dealunay_2d(sites, 4.0, false));

    plot_triangulation(
        "output/assignment5/part_5b.png",
        "Homemade Delaunay Triangulation",
        sites,
        &lines,
    )?;

    Ok(())
}

fn plot_triangulation(
    path: &str,
    caption: &str,
    sites: &Vec<Point2d>,
    lines: &Vec<(usize, usize)>,
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
        .map(|&(src, dst)| (sites[src], sites[dst]))
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

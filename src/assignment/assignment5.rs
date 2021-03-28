use plotters::prelude::*;

use crate::data::neighbors::NearestNeighborMap;
use crate::rand::points_in_grid::gen_points_in_box;
use crate::rand::uniform::Uniform701;
use crate::util::{circle_through, point_in_circle};
use std::error::Error;

pub fn do_assignment_5() -> Result<(), Box<dyn Error>> {
    log::info!("Doing assignment 5");

    let mut uni = Uniform701::new();

    let sites = gen_points_in_box(&mut uni, 20.0, 500, 0.7);

    do_part_a(&sites)?;
    do_part_b(&sites)?;

    Ok(())
}

fn do_part_a(sites: &Vec<(f64, f64)>) -> Result<(), Box<dyn Error>> {
    log::info!("Doing part a");

    let wrapped_points = &sites
        .into_iter()
        .map(|(x, y)| delaunator::Point { x: *x, y: *y })
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
        "Using built-in triangulation",
        sites,
        &lines,
    )?;

    Ok(())
}

fn do_part_b(sites: &Vec<(f64, f64)>) -> Result<(), Box<dyn Error>> {
    log::info!("Doing part b");

    let first_neighbors = NearestNeighborMap::first_2d(sites, 4.0);

    let mut lines: Vec<(usize, usize)> = Vec::with_capacity(sites.len());

    for i in 0..sites.len() {
        let neighbors_i = &first_neighbors.neighbors[i];
        for &j in neighbors_i {
            for &k in neighbors_i {
                if k == j {
                    continue;
                }
                let (center, r) = circle_through(sites[i], sites[j], sites[k]);
                if let None = (0..sites.len())
                    .filter(|&v| !(v == i || v == j || v == k))
                    .find(|&v| point_in_circle(sites[v], center, r))
                {
                    lines.push((i, j));
                    lines.push((j, k));
                    lines.push((k, i));
                }
            }
        }
    }

    lines.sort_unstable();
    lines.dedup();

    log::info!("Num lines: {}", lines.len());

    plot_triangulation(
        "output/assignment5/part_5b.png",
        "Homemade Delauney Triangulation",
        sites,
        &lines,
    )?;

    Ok(())
}

fn plot_triangulation(
    path: &str,
    caption: &str,
    sites: &Vec<(f64, f64)>,
    lines: &Vec<(usize, usize)>,
) -> Result<(), Box<dyn Error>> {
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
            .map(|coord| Circle::new(*coord, 2, BLACK.filled())),
    ) {
        log::error!("Error occurred drawing sites!  Details: {:?}", err);
    }

    lines
        .into_iter()
        .map(|(src, dst)| (sites[*src], sites[*dst]))
        .for_each(|(src, dst)| {
            if let Err(err) =
                chart.draw_series(LineSeries::new(vec![src.to_owned(), dst.to_owned()], &BLUE))
            {
                log::error!("Error occurred drawing a line!  Details: {:?}", err);
            }
        });

    Ok(())
}

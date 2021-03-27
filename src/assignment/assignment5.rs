use plotters::prelude::*;

use crate::rand::points_in_grid::gen_points_in_box;
use crate::rand::uniform::Uniform701;
use std::error::Error;

pub fn do_assignment_5() -> Result<(), Box<dyn Error>> {
    log::info!("Doing assignment 5");

    let mut uni = Uniform701::new();

    let sites = gen_points_in_box(&mut uni, 20.0, 500, 0.7);

    do_part_a(&sites)?;

    Ok(())
}

fn do_part_a(sites: &Vec<(f64, f64)>) -> Result<(), Box<dyn Error>> {
    log::info!("Doing part a");

    plot_triangulation(
        "output/assignment5/part_5a.png",
        "Using built-in triangulation",
        sites,
        &delaunator::triangulate(&sites
                .into_iter()
                .map(|(x, y)| delaunator::Point { x: *x, y: *y })
                .collect::<Vec<delaunator::Point>>(),
        )
        .expect("No triangulation for the points exists!")
        .triangles
        .chunks(3)
        .flat_map(|t| vec![(t[0], t[1]), (t[1], t[2]), (t[2], t[0])])
        .collect::<Vec<(usize, usize)>>(),
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

    chart.draw_series(
        (&sites)
            .into_iter()
            .map(|coord| Circle::new(*coord, 2, BLACK.filled())),
    )?;

    lines
        .into_iter()
        .map(|(src, dst)| (sites[*src], sites[*dst]))
        .for_each(|(src, dst)| {
            chart
                .draw_series(LineSeries::new(vec![src.to_owned(), dst.to_owned()], &BLUE))
                .unwrap();
        });

    Ok(())
}

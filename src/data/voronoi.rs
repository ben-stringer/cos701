use crate::data::line::Line2d;
use crate::data::point::Point2d;

pub fn voronoi_701(sites: &Vec<Point2d>, delaunay: &Vec<Vec<usize>>) -> Vec<Line2d> {
    let n_sites = sites.len();

    let mut ret = vec![];

    for i in 0..n_sites {
        let mut bounding_lines = (&delaunay[i])
            .into_iter()
            .map(|&j| {
                let src = sites[i];
                let dst = sites[j];
                let line = Line2d { src, dst };
                line.perpendicular_bisector()
            })
            .collect::<Vec<Line2d>>();
        bounding_lines.sort_by(|&l, &r| l.angle().partial_cmp(&r.angle()).unwrap());

        // Play connect the dots
        for j in 0..bounding_lines.len() {
            let k = if j + 1 == bounding_lines.len() {
                0
            } else {
                j + 1
            };
            let intersection = bounding_lines[j].intersection(&bounding_lines[k]).unwrap();
            log::info!(
                "Lines {} and {} intersect at {:.2}",
                bounding_lines[j],
                bounding_lines[k],
                intersection
            );
            bounding_lines[j].dst = intersection;
            bounding_lines[k].src = intersection;
        }

        ret.append(&mut bounding_lines);
    }

    ret
}

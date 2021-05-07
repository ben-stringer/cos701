use crate::data::line::Line2d;
use crate::data::point::Point2d;

pub fn voronoi_701(sites: &[Point2d], delaunay: &[Vec<usize>]) -> Vec<Line2d> {
    let n_sites = sites.len();

    let mut ret = vec![];

    for i in 0..n_sites {
        let mut spokes = delaunay[i]
            .iter()
            .map(|&j| {
                let src = sites[i];
                let dst = sites[j];
                Line2d { src, dst }
            })
            .collect::<Vec<Line2d>>();
        spokes.sort_unstable_by(|&l, &r| l.angle().partial_cmp(&r.angle()).unwrap());

        let mut bounding_lines = spokes
            .iter()
            .map(|&line| line.perpendicular_bisector())
            .collect::<Vec<Line2d>>();

        let mut to_remove = vec![];
        for j in 0..bounding_lines.len() {
            let k = if j + 1 == bounding_lines.len() {
                0
            } else {
                j + 1
            };
            let intersection = bounding_lines[j].intersection(&bounding_lines[k]).unwrap();
            bounding_lines[j].dst = intersection;
            bounding_lines[k].src = intersection;
            let len_j = bounding_lines[j].length();
            let len_k = bounding_lines[k].length();
            if len_j > 2.0 {
                to_remove.push(j);
            }
            if len_k > 2.0 {
                to_remove.push(k);
            }
        }
        to_remove.sort_unstable();
        to_remove.dedup();
        to_remove.reverse();
        for j in to_remove {
            bounding_lines.remove(j);
        }

        ret.append(&mut bounding_lines);
    }

    ret
}

use crate::data::line::Line2d;
use crate::data::point::{Point2d, Point3d};
use crate::rand::uniform::Uniform701;

/// Generate `n` points in a box of size `box_len`x`box_len`, with no constraints.
pub fn gen_points_in_box(uni: &mut Uniform701, box_len: f64, n: usize) -> Vec<Point2d> {
    (0..n)
        .map(|_| Point2d {
            x: uni.next() * box_len,
            y: uni.next() * box_len,
        })
        .collect::<Vec<Point2d>>()
}

/// Generate `n` points in a box of size `box_len`x`box_len`, with the constraint that
/// no point is closer than `r_min` to any other point.
pub fn gen_spaced_points_in_box(
    uni: &mut Uniform701,
    box_len: f64,
    n: usize,
    r_min: f64,
) -> Vec<Point2d> {
    let mut accepted: Vec<Point2d> = Vec::with_capacity(n);

    for _ in 0..n {
        let mut rejected = true;
        while rejected {
            let next = Point2d {
                x: uni.next() * box_len,
                y: uni.next() * box_len,
            };
            if let None = accepted
                .iter()
                .find(|&&point| point.distance_to(&next) < r_min)
            {
                accepted.push(next);
                rejected = false;
            }
        }
    }
    accepted
}

/// Generate `n` points in a cube of size `box_len`x`box_len`x`box_len`, with the constraint that
/// no point is closer than `r_min` to any other point.
pub fn gen_points_in_cube(
    uni: &mut Uniform701,
    cube_len: f64,
    n: usize,
    r_min: f64,
) -> Vec<Point3d> {
    let mut accepted: Vec<Point3d> = Vec::with_capacity(n);

    for _ in 0..n {
        let mut rejected = true;
        while rejected {
            let next = Point3d {
                x: uni.next() * cube_len,
                y: uni.next() * cube_len,
                z: uni.next() * cube_len,
            };
            if let None = accepted
                .iter()
                .find(|&&point| point.distance_to(&next) < r_min)
            {
                accepted.push(next);
                rejected = false;
            }
        }
    }

    accepted
}

/// Generate lines from the point at index `i` to all other points.
/// The returned Vec will be in the same order as the supplied `points`.
/// For index `i`, the line will have the same start/end point.  This will be meaningless
/// and will cause issues if used (e.g., calculating an angle), but is included to
/// keep the indexes aligned.
pub fn lines_from(i: usize, points: &Vec<Point2d>) -> Vec<Line2d> {
    let src = points[i];
    points
        .iter()
        .enumerate()
        .filter_map(|(j, &dst)| {
            if i == j {
                None
            } else {
                Some(Line2d { src, dst })
            }
        })
        .collect::<Vec<Line2d>>()
}

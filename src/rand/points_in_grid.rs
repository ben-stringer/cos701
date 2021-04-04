use crate::data::point::{Point2d, Point3d};
use crate::rand::uniform::Uniform701;

pub fn gen_points_in_box(uni: &mut Uniform701, box_len: f64, n: usize, r_min: f64) -> Vec<Point2d> {
    let mut accepted: Vec<Point2d> = Vec::with_capacity(n);

    for _ in 0..n {
        let mut rejected = true;
        while rejected {
            let next = Point2d {
                x: uni.next() * box_len,
                y: uni.next() * box_len,
            };
            if let None = (&accepted)
                .into_iter()
                .find(|&&point| point.distance_to(&next) < r_min)
            {
                accepted.push(next);
                rejected = false;
            }
        }
    }
    accepted
}

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
            if let None = (&accepted)
                .into_iter()
                .find(|&&point| point.distance_to(&next) < r_min)
            {
                accepted.push(next);
                rejected = false;
            }
        }
    }

    accepted
}

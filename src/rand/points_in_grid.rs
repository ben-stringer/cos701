use crate::rand::uniform::Uniform701;
use crate::util::{distance_2d, distance_3d};

pub fn gen_points_in_box(
    uni: &mut Uniform701,
    box_len: f64,
    n: usize,
    r_min: f64,
) -> Vec<(f64, f64)> {
    let mut accepted: Vec<(f64, f64)> = Vec::with_capacity(n);

    for _ in 0..n {
        let mut rejected = true;
        while rejected {
            let next = (uni.next() * box_len, uni.next() * box_len);
            if let None = (&accepted)
                .into_iter()
                .find(|point| distance_2d(**point, next) < r_min)
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
) -> Vec<(f64, f64, f64)> {
    let mut accepted: Vec<(f64, f64, f64)> = Vec::with_capacity(n);

    for _ in 0..n {
        let mut rejected = true;
        while rejected {
            let next = (
                uni.next() * cube_len,
                uni.next() * cube_len,
                uni.next() * cube_len,
            );
            if let None = (&accepted)
                .into_iter()
                .find(|point| distance_3d(**point, next) < r_min)
            {
                accepted.push(next);
                rejected = false;
            }
        }
    }

    accepted
}
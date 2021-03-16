//! Various utility functions

use std::f64::consts::PI;

/// Calculate the 2-dimensional distance between points a and b
pub fn distance_2d(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((a.0 - b.0).powf(2.0) + (a.1 - b.1).powf(2.0)).sqrt()
}

/// Calculate the 3-dimensional distance between points a and b
pub fn distance_3d(a: (f64, f64, f64), b: (f64, f64, f64)) -> f64 {
    ((a.0 - b.0).powf(2.0) + (a.1 - b.1).powf(2.0) + (a.2 - b.2).powf(2.0)).sqrt()
}

pub fn gamma_half(n: usize) -> f64 {
    if n == 1 {
        1.0
    } else if n % 2 == 1 {
        gamma_half(n - 1) * PI.sqrt()
    } else {
        let n_half = (n as f64 / 2.0) as usize;
        n_half as f64 * gamma_half(n_half)
    }
}

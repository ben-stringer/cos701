//! Various utility functions

// use std::f64::consts::PI;

/// Calculate the 2-dimensional distance between points a and b
pub fn distance_2d(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((a.0 - b.0).powf(2.0) + (a.1 - b.1).powf(2.0)).sqrt()
}

/// Calculate the 3-dimensional distance between points a and b
pub fn distance_3d(a: (f64, f64, f64), b: (f64, f64, f64)) -> f64 {
    ((a.0 - b.0).powf(2.0) + (a.1 - b.1).powf(2.0) + (a.2 - b.2).powf(2.0)).sqrt()
}

// pub fn gamma(n: usize) -> f64 {
//     let a = n as f64 / 2;
//     (PI.powf(a)) / factorial(a)
// }
//
// pub fn factorial(n: f64) -> f64 {
//     if n <= 0.0 {
//         1.0
//     } else {
//         n * factorial(n - 1)
//     }
// }

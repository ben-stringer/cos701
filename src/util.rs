//! Various utility functions

use std::f64::consts::PI;

pub const ORIGIN: (f64, f64, f64) = (0.0,0.0,0.0);

/// Calculate the 2-dimensional distance between points a and b
pub fn distance_2d(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((a.0 - b.0).powf(2.0) + (a.1 - b.1).powf(2.0)).sqrt()
}

/// Calculate the 3-dimensional distance between points a and b
pub fn distance_3d(a: (f64, f64, f64), b: (f64, f64, f64)) -> f64 {
    ((a.0 - b.0).powf(2.0) + (a.1 - b.1).powf(2.0) + (a.2 - b.2).powf(2.0)).sqrt()
}

/// Calculate the gamma for half of the supplied positive whole number.
/// If n is 1, i.e., gamma(1/2), return PI.sqrt().
/// If n is 2, i.e., gamma(2/2), return 1.  Otherwise,
/// if n is even, this function calls gamma with half of n, which is guaranteed to be a whole number.
/// If n is odd, this function calls this function with n-2.
pub fn gamma_half(n: usize) -> f64 {
    if n == 1 {
        // i.e., gamma(1/2)
        PI.sqrt()
    } else if n == 2 {
        // i.e., gamma(2/2) or gamma(1)
        1.0
    } else if n % 2 == 1 {
        // i.e., gamma(3/2), gamma(5/2), ...
        let numerator_minus_two_halves = n - 2;
        let n_half = numerator_minus_two_halves as f64 / 2.0;
        n_half * gamma_half(numerator_minus_two_halves)
    } else {
        // i.e., gamma(4/2), gamma(6/2), ...
        let n_half = (n as f64 / 2.0).round() as usize;
        gamma(n_half)
    }
}

/// Calculate the gamma for the supplied positive whole number.
/// return (n-1) * gamma(n-1)
pub fn gamma(n: usize) -> f64 {
    if n == 1 {
        1.0
    } else {
        let m = n - 1;
        m as f64 * gamma(m)
    }
}

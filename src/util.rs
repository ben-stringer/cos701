//! Various utility functions

use std::f64::consts::PI;

/// Constant string to begin a latex document in standalone mode
pub const TEX_PREFIX: &str = "\\documentclass{standalone}
%!TEX encoding =  UTF-16
\\usepackage{fontspec}
\\setmainfont{Arial}
\\setmonofont[Scale=MatchLowercase]{Andale Mono}
\\begin{document}
";

/// Constant string to end a latex document
pub const TEX_SUFFIX: &str = "\\end{document}";

/// Calculate the 2-dimensional distance between points a and b
pub fn distance_2d(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((a.0 - b.0).powf(2.0) + (a.1 - b.1).powf(2.0)).sqrt()
}

/// Calculate the 3-dimensional distance between points a and b
pub fn distance_3d(a: (f64, f64, f64), b: (f64, f64, f64)) -> f64 {
    ((a.0 - b.0).powf(2.0) + (a.1 - b.1).powf(2.0) + (a.2 - b.2).powf(2.0)).sqrt()
}

/// Calculate the magnitude of the supplied vector, equivalent to returning distance_2d(origin, v)
pub fn magnitude_2d(v: (f64, f64)) -> f64 {
    (v.0.powf(2.0) + v.1.powf(2.0)).sqrt()
}

/// Calculate the magnitude of the supplied vector, equivalent to returning distance_3d(origin, v)
pub fn magnitude_3d(v: (f64, f64, f64)) -> f64 {
    (v.0.powf(2.0) + v.1.powf(2.0) + v.2.powf(2.0)).sqrt()
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

/// Calculate a circle passing through points a, b, and c.
/// Return the center of the circle and the radius.
pub fn circle_through(p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) -> ((f64, f64), f64) {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let (x3, y3) = p3;

    let x12 = x1.powf(2.0);
    let y12 = y1.powf(2.0);
    let x22 = x2.powf(2.0);
    let y22 = y2.powf(2.0);
    let x32 = x3.powf(2.0);
    let y32 = y3.powf(2.0);

    let a = x1 * (y2 - y3) - y1 * (x2 - x3) + x2 * y3 - x3 * y2;
    let b = (x12 + y12) * (y3 - y2) + (x22 + y22) * (y1 - y3) + (x32 + y32) * (y2 - y1);
    let c = (x12 + y12) * (x2 - x3) + (x22 + y22) * (x3 - x1) + (x32 + y32) * (x1 - x2);
    let d = (x12 + y12) * (x3 * y2)
        + (x22 + y22) * (x1 * y3 - x3 * y1)
        + (x32 + y32) * (x2 * y1 - x1 * y2);

    let zx = -b / (2.0 * a);
    let zy = -c / (2.0 * a);
    let r = ((zx - x1).powf(2.0) + (zy - y1).powf(2.0)).sqrt();

    //(b.powf(2.0) + c.powf(2.0) - 4.0 * a * d) / (4.0 * a.powf(2.0)).sqrt();

    ((zx, zy), r)
}

/// Determine whether the supplied point is within the circle centered at center and with radius r
pub fn point_in_circle(point: (f64, f64), center: (f64, f64), r: f64) -> bool {
    let result = distance_2d(point, center) < r;

    // log::trace!(
    //     "Point ({},{}) in circle with center ({},{}) and radius {}? {}",
    //     point.0,
    //     point.1,
    //     center.0,
    //     center.1,
    //     r,
    //     result
    // );

    result
}

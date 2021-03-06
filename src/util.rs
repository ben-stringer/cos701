//! Various utility functions

use crate::data::point::Point2d;
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
#[allow(clippy::many_single_char_names)]
pub fn circle_through(p1: Point2d, p2: Point2d, p3: Point2d) -> (Point2d, f64) {
    let (x1, y1) = p1.into();
    let (x2, y2) = p2.into();
    let (x3, y3) = p3.into();

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

    ((zx, zy).into(), r)
}

/// Determine whether the supplied point is within the circle centered at center and with radius r
pub fn point_in_circle(point: Point2d, center: Point2d, r: f64) -> bool {
    point.distance_to(&center) < r
}

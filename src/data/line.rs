use crate::data::point::Point2d;
use std::f64::consts::PI;

#[derive(Copy, Clone)]
pub struct Line2d {
    pub src: Point2d,
    pub dst: Point2d,
}

impl Line2d {
    /// Calculate a perpendicular bisector for this line.
    /// The resulting line will have a length of `len` and will be orthogonal to this line
    pub fn perpendicular_bisector(&self) -> Line2d {
        let midpoint = self.midpoint();
        // let slope = self.orthogonal_slope();
        let slope = -1.0 / self.slope();
        let b = midpoint.y - (slope * midpoint.x);

        let l = Self {
            src: midpoint,
            dst: (
                midpoint.x + 0.2 * (1.0 / (1.0 + slope.powf(2.0)).sqrt()),
                midpoint.y + (0.2 * (slope / (1.0 + slope.powf(2.0)).sqrt())),
            )
                .into(),
        };
        let r = Self {
            src: midpoint,
            dst: (
                midpoint.x - 0.2 * (1.0 / (1.0 + slope.powf(2.0)).sqrt()),
                midpoint.y - (0.2 * (slope / (1.0 + slope.powf(2.0)).sqrt())),
            )
                .into(),
        };

        Self {
            src: l.dst,
            dst: r.dst,
        }
    }

    /// Calculate the midpoint of this line
    pub fn midpoint(&self) -> Point2d {
        Point2d {
            x: (self.src.x + self.dst.x) / 2.0,
            y: (self.src.y + self.dst.y) / 2.0,
        }
    }

    /// Calculate the slope of this line
    pub fn slope(&self) -> f64 {
        (self.dst.y - self.src.y) / (self.dst.x - self.src.x)
    }

    /// Calculate the slope of a line orthogonal to this line
    pub fn orthogonal_slope(&self) -> f64 {
        -(self.dst.x - self.src.x) / (self.dst.y - self.src.y)
    }

    /// Get the length of this line.
    pub fn length(&self) -> f64 {
        self.src.distance_to(&self.dst)
    }

    /// Determine the point at which the two lines intersect, or if they are parallel,
    /// return None.
    pub fn intersection(&self, that: &Self) -> Option<Point2d> {
        // fn line(src: &Point2d, dst: &Point2d) -> (f64, f64, f64) {
        //     let a = src.y - dst.y;
        //     let b = dst.x - src.x;
        //     let c = -(src.x * dst.y - dst.x * src.y);
        //     (a, b, c)
        // }
        //
        // let l1 = line(&self.src, &self.dst);
        // let l2 = line(&that.src, &that.dst);
        //
        // let d = l1.0 * l2.1 - l1.1 * l2.0;
        // if d == 0.0 {
        //     None
        // } else {
        //     let dx = l1.2 * l2.1 - l1.1 * l2.2;
        //     let dy = l1.0 * l2.2 - l1.2 * l2.0;
        //     Some((dx / d, dy / d).into())
        // }
        let a = self.slope();
        let b = that.slope();
        if a == b {
            None
        } else {
            let c = self.src.y - (a * self.src.x);
            let d = that.src.y - (b * that.src.x);
            if c == d {
                None
            } else if a.is_infinite() {
                Some(Point2d {
                    x: self.src.x,
                    y: b * self.src.x + d,
                })
            } else if b.is_infinite() {
                Some(Point2d {
                    x: that.src.x,
                    y: a * that.src.x + c,
                })
            } else {
                let x = (d - c) / (a - b);
                let y = b * x + d;
                Some(Point2d { x, y })
            }
        }
    }

    /// Return the angle formed between this line and a line starting at src and continuing in
    /// the positive x direction
    pub fn angle(&self) -> f64 {
        let r = self.length();
        let x = self.dst.x - self.src.x;
        let y = self.dst.y - self.src.y;

        if x < 0.0 && y < 0.0 {
            // In quadrant 3
            PI + (-y / r).asin()
        } else if y < 0.0 {
            // In quadrant 4
            2.0 * PI - (-y / r).asin()
        } else if x < 0.0 {
            // In quadrant 2
            PI - (y / r).asin()
        } else {
            // In quadrant 1
            (y / r).asin()
        }
    }

    /// Return the angle formed between this line and a line formed from
    /// this line's destination and the supplied point, counterclockwise.
    pub fn angle_between(&self, that: &Line2d) -> f64 {
        let angle_self = self.angle();
        let mut angle_that = that.angle();
        if angle_that < angle_self {
            angle_that += 2.0 * PI;
        }
        angle_that - angle_self
    }

    pub fn normalize(mut self) -> Self {
        let mut v = Line2d {
            src: (0.0, 0.0).into(),
            dst: (self.dst.x - self.src.x, self.dst.y - self.src.y).into(),
        };
        let len = v.length();
        v.dst.x = v.dst.x / len;
        v.dst.y = v.dst.y / len;
        self.dst = (v.dst.x + self.src.x, v.dst.y + self.src.y).into();
        self
    }
}

impl From<(Point2d, Point2d)> for Line2d {
    fn from(line: (Point2d, Point2d)) -> Self {
        Self {
            src: line.0,
            dst: line.1,
        }
    }
}

impl From<((f64, f64), (f64, f64))> for Line2d {
    fn from(line: ((f64, f64), (f64, f64))) -> Self {
        Self {
            src: line.0.into(),
            dst: line.1.into(),
        }
    }
}

impl std::fmt::Display for Line2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} -> {}]", self.src, self.dst)
    }
}

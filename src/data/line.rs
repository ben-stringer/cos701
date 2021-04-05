use crate::data::point::Point2d;

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
        let slope = self.orthogonal_slope();
        let b = midpoint.y - (slope * midpoint.x);

        Self {
            src: Point2d {
                x: midpoint.x + 1.0,
                y: (midpoint.y + 1.0) * slope + b,
            },
            dst: Point2d {
                x: midpoint.x - 1.0,
                y: (midpoint.y - 1.0) * slope + b,
            },
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
        (self.src.y - self.dst.y) / (self.src.x - self.dst.x)
    }

    /// Calculate the slope of a line orthogonal to this line
    pub fn orthogonal_slope(&self) -> f64 {
        -(self.src.x - self.dst.x) / (self.src.y - self.dst.y)
    }

    /// Get the length of this line.
    pub fn length(&self) -> f64 {
        self.src.distance_to(&self.dst)
    }

    /// Determine the point at which the two lines intersect, or if they are parallel,
    /// return None.
    pub fn intersection(&self, that: &Self) -> Option<Point2d> {
        let a = self.slope();
        let b = that.slope();
        if a == b {
            None
        } else {
            let c = self.src.y - (a * self.src.x);
            let d = that.src.y - (b * that.src.x);
            let x = (d - c) / (a - b);
            let y = a * x + c;
            Some(Point2d { x, y })
        }
    }

    /// Return the angle formed between this line and a line starting at src and continuing in
    /// the positive x direction
    pub fn angle(&self) -> f64 {
        let r = self.length();
        let y = self.dst.y - self.src.y;

        (y / r).asin()
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

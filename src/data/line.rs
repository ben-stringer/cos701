use crate::data::point::Point2d;

pub struct Line2d {
    pub src: Point2d,
    pub dst: Point2d,
}

impl Line2d {
    pub fn perpendicular_bisector(&self) -> Line2d {
        unimplemented!()
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

impl From< ((f64, f64), (f64, f64)) > for Line2d {
    fn from(line: ((f64, f64), (f64, f64))) -> Self {
        Self {
            src: Point2d::from(line.0),
            dst: Point2d::from(line.1),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

#[derive(Copy, Clone)]
pub struct Point3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<(f64, f64)> for Point2d {
    fn from(v: (f64, f64)) -> Self {
        Self { x: v.0, y: v.1 }
    }
}

impl From<Point2d> for (f64, f64) {
    fn from(p: Point2d) -> Self {
        (p.x, p.y)
    }
}

impl From<(f64, f64, f64)> for Point3d {
    fn from(v: (f64, f64, f64)) -> Self {
        Self {
            x: v.0,
            y: v.1,
            z: v.2,
        }
    }
}

impl From<Point3d> for (f64, f64, f64) {
    fn from(p: Point3d) -> Self {
        (p.x, p.y, p.z)
    }
}

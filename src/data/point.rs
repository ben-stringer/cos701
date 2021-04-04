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

impl Point2d {
    /// Calculate the magnitude of this point
    pub fn magnitude(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    /// Calculate the 2-dimensional distance between points a and b
    pub fn distance_to(&self, that: &Self) -> f64 {
        ((self.x - that.x).powf(2.0) + (self.y - that.y).powf(2.0)).sqrt()
    }
}

impl Point3d {
    /// Calculate the magnitude of this point
    pub fn magnitude(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    /// Calculate the 3-dimensional distance between points a and b
    pub fn distance_to(&self, that: &Self) -> f64 {
        ((self.x - that.x).powf(2.0) + (self.y - that.y).powf(2.0) + (self.z - that.z).powf(2.0))
            .sqrt()
    }
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

impl std::fmt::Display for Point2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
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

impl std::fmt::Display for Point3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

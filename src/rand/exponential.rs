use crate::rand::uniform::Uniform701;

pub struct Exponential701 {
    uniform: Uniform701,
    a: f64,
    b: f64,
}

impl Exponential701 {
    pub fn new(uniform: Uniform701, a: f64, b: f64) -> Self {
        Self { uniform, a, b }
    }

    pub fn next(&mut self) -> f64 {
        let x = self.uniform.next();
        let a = self.a;
        let b = self.b;
        -(-(b / a) * x + 1.0).ln() / b
    }

    pub fn a(&self) -> f64 {
        self.a
    }
    pub fn b(&self) -> f64 {
        self.b
    }
}

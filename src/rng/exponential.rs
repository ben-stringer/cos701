use crate::rng::uniform::Uniform701;

pub struct Exponential701<'a> {
    uniform: &'a mut Uniform701,
    a: f64,
    b: f64,
}

impl<'a> Exponential701<'a> {
    pub fn new(uniform: &'a mut Uniform701, a: f64, b: f64) -> Self {
        Self { uniform, a, b }
    }

    pub fn next(&mut self) -> f64 {
        let x = self.uniform.next();
        (-1.0 / self.b) * (1.0 - (self.b / self.a) * x).log10()
    }
}

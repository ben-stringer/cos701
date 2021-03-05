use crate::rng::uniform::Uniform701;

pub struct InverseTransform701<'a> {
    uniform: &'a mut Uniform701,
    sigma: f64,
}

impl<'a> InverseTransform701<'a> {
    pub fn new(uniform: &'a mut Uniform701, sigma: f64) -> Self {
        Self { uniform, sigma }
    }

    pub fn next(&mut self) -> f64 {
        let y = self.uniform.next();
        (-2.0 * self.sigma.powf(2.0) * (1.0 - y).ln()).sqrt()
    }

    pub fn sigma(&self) -> f64 {
        self.sigma
    }
}

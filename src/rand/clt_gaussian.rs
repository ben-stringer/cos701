use crate::rand::uniform::Uniform701;

pub struct CentralLimitTheoremGaussian701 {
    /// The uniform random source
    uniform: Uniform701,
    n: usize,
}

impl CentralLimitTheoremGaussian701 {
    /// Construct a new CentralLimitTheoremGaussian701
    pub fn new(uniform: Uniform701, n: usize) -> Self {
        Self { uniform, n }
    }

    /// Get the next random number normally distributed
    pub fn next(&mut self) -> f64 {
        let n = self.n;
        (0..n).map(|_| self.uniform.next()).sum::<f64>() / n as f64
    }
}

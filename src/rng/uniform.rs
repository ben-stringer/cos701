use rand::distributions::Distribution;

pub struct Uniform701 {
    /// The system random number generator
    rng : rand::rngs::ThreadRng,
    /// The system Uniform random distribution
    dst : rand::distributions::Uniform<f64>
}

/// The Uniform random data source for all of the COS701 project
impl Uniform701 {

    /// Construct a new Uniform701
    pub fn new() -> Self {
        Self {
            rng : rand::thread_rng(),
            dst : rand::distributions::Uniform::from(0f64..1f64)
        }
    }

    /// Get the next random number uniformly distributed between 0..1
    pub fn next(&mut self) -> f64 {
        let v = self.dst.sample(&mut self.rng);
        assert!(v > 0f64, "Generated random was less than 0: {}", v);
        assert!(v < 1f64, "Generated random was greater than 1: {}", v);
        v
    }
}

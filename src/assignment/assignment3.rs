use crate::rng::uniform::Uniform701;
use std::error::Error;
use std::f64::consts::PI;

const NUM_ITER: usize = 10_000;

struct RandomVec {
    v: Vec<f64>,
    d: usize,
}
impl RandomVec {
    pub fn new(uniform: &mut Uniform701, d: usize) -> Self {
        Self {
            v: (0..d).map(|_| uniform.next()).collect(),
            d,
        }
    }

    pub fn get(&mut self) -> &Vec<f64> {
        &self.v
    }

    pub fn accept(&self) -> bool {
        (&self.v)
            .into_iter()
            .map(|i| i.powf(2.))
            .sum::<f64>()
            .powf(1.0 / self.d as f64)
            <= 1.0
    }
}

pub fn do_assignment_3() -> Result<(), Box<dyn Error>> {
    let mut uni = Uniform701::new();
    let d2_accept = (0..NUM_ITER)
        .map(|_| RandomVec::new(&mut uni, 2))
        .filter(RandomVec::accept)
        .count();
    println!(
        "Accept rate, expected {:.4}, actual {}/{} = {:.4}",
        PI / 4.0,
        d2_accept,
        NUM_ITER,
        d2_accept as f64 / NUM_ITER as f64
    );

    Ok(())
}

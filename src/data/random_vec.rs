use crate::rand::boxmuller::BoxMullerGaussian701;
use crate::rand::uniform::Uniform701;

pub(crate) struct RandomVec {
    v: Vec<f64>,
}

impl RandomVec {
    pub fn naive(uniform: &mut Uniform701, dim: usize) -> Self {
        Self::naive_scaled(uniform, dim, 1.0, 0.0)
    }

    pub fn naive_scaled(
        uniform: &mut Uniform701,
        dim: usize,
        scale_by: f64,
        shift_by: f64,
    ) -> Self {
        Self {
            v: (0..dim)
                .map(|_| uniform.next() * scale_by + shift_by)
                .collect(),
        }
    }

    pub fn efficient(
        uniform: &mut Uniform701,
        gaussian: &mut BoxMullerGaussian701,
        dim: usize,
    ) -> Self {
        Self::efficient_scaled(uniform, gaussian, dim, 1.0, 0.0)
    }

    pub fn efficient_scaled(
        uniform: &mut Uniform701,
        gaussian: &mut BoxMullerGaussian701,
        dim: usize,
        scale_by: f64,
        shift_by: f64,
    ) -> Self {
        Self {
            v: Self::gen_efficient_vec(uniform, gaussian, dim, scale_by, shift_by),
        }
    }

    fn gen_efficient_vec(
        uniform: &mut Uniform701,
        gaussian: &mut BoxMullerGaussian701,
        dim: usize,
        scale_by: f64,
        shift_by: f64,
    ) -> Vec<f64> {
        let x: Vec<f64> = (0..dim)
            .map(|_| gaussian.next() * scale_by + shift_by)
            .collect();
        let mag = x.iter().map(|&xi| xi.powi(2)).sum::<f64>().sqrt();
        let alpha = uniform.next().powf(1.0 / dim as f64);
        x.iter()
            .map(|&xi| xi / mag) // scale point onto surface of unit sphere
            .map(|xi| xi * alpha) // scale point to point within unit sphere
            .collect()
    }

    pub fn get(&self) -> &Vec<f64> {
        &self.v
    }

    pub fn is_in_sphere(&self, radius: f64) -> bool {
        self.v
            .iter()
            .map(|&i| i.powf(2.0))
            .sum::<f64>()
            .powf(1.0 / self.v.len() as f64)
            <= radius
    }
}

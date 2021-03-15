use crate::rand::boxmuller::BoxMullerGaussian701;
use crate::rand::uniform::Uniform701;

pub(crate) struct RandomVec {
    v: Vec<f64>,
    dim: usize,
}

impl RandomVec {
    pub fn naive(uniform: &mut Uniform701, dim: usize) -> Self {
        Self {
            v: (0..dim).map(|_| uniform.next()).collect(),
            dim,
        }
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
            dim,
        }
    }

    pub fn efficient(
        uniform: &mut Uniform701,
        gaussian: &mut BoxMullerGaussian701,
        dim: usize,
    ) -> Self {
        Self {
            v: Self::gen_efficient_vec(uniform, gaussian, dim),
            dim,
        }
    }

    pub fn efficient_scaled(
        uniform: &mut Uniform701,
        gaussian: &mut BoxMullerGaussian701,
        dim: usize,
        scale_by: f64,
        shift_by: f64,
    ) -> Self {
        Self {
            v: Self::gen_efficient_vec(uniform, gaussian, dim)
                .into_iter()
                .map(|x| x * scale_by + shift_by)
                .collect(),
            dim,
        }
    }

    fn gen_efficient_vec(
        uniform: &mut Uniform701,
        gaussian: &mut BoxMullerGaussian701,
        dim: usize,
    ) -> Vec<f64> {
        let x: Vec<f64> = (0..dim).map(|_| gaussian.next()).collect();
        let sum_x2_sqrt = (&x).into_iter().map(|xi| xi.powf(2.0)).sum::<f64>().sqrt();
        let gamma = uniform.next().powf(1.0 / dim as f64);
        x.into_iter().map(|xi| gamma * (xi / sum_x2_sqrt)).collect()
    }

    pub fn get(&self) -> &Vec<f64> {
        &self.v
    }

    pub fn is_in_sphere(&self, radius: f64) -> bool {
        (&self.v)
            .into_iter()
            .map(|i| i.powf(2.0))
            .sum::<f64>()
            .powf(1.0 / self.dim as f64)
            <= radius
    }
}

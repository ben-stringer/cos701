use crate::rand::uniform::Uniform701;
use crate::rand::boxmuller::BoxMullerGaussian701;

pub(crate) struct NaiveRandomVec {
    v: Vec<f64>,
    d: usize,
}

impl NaiveRandomVec {
    pub fn new(uniform: &mut Uniform701, d: usize) -> Self {
        Self {
            v: (0..d).map(|_| uniform.next()).collect(),
            d,
        }
    }

    pub fn scaled(uniform: &mut Uniform701, d: usize, scale_by: f64, shift_by: f64) -> Self {
        Self {
            v: (0..d)
                .map(|_| uniform.next() * scale_by + shift_by)
                .collect(),
            d,
        }
    }

    pub fn get(&self) -> &Vec<f64> {
        &self.v
    }

    pub fn is_in_sphere(&self, radius: f64) -> bool {
        (&self.v)
            .into_iter()
            .map(|i| i.powf(2.))
            .sum::<f64>()
            .powf(1.0 / self.d as f64)
            <= radius
    }
}

pub(crate) struct EfficientRandomVec {
    v: Vec<f64>,
    d: usize,
}

impl EfficientRandomVec {
    pub fn new(uniform: &mut Uniform701, gaussian : &mut BoxMullerGaussian701, d: usize) -> Self {
        Self {
            v: Self::gen_vec(uniform, gaussian, d),
            d,
        }
    }

    pub fn scaled(uniform: &mut Uniform701, gaussian : &mut BoxMullerGaussian701, d: usize, scale_by: f64, shift_by: f64) -> Self {
        Self {
            v: Self::gen_vec(uniform, gaussian, d).into_iter().map(|x| x * scale_by + shift_by).collect(),
            d,
        }
    }

    fn gen_vec(uniform: &mut Uniform701, gaussian : &mut BoxMullerGaussian701, d: usize) -> Vec<f64> {
        let x: Vec<f64> = (0..d).map(|_| gaussian.next()).collect();
        let sum_x: f64 = (&x).into_iter().sum();
        let r = uniform.next().powf(1.0/d as f64);
        x.into_iter().map(|x| r * x / sum_x).collect()
    }

    pub fn get(&self) -> &Vec<f64> {
        &self.v
    }

    pub fn is_in_sphere(&self, radius: f64) -> bool {
        (&self.v)
            .into_iter()
            .map(|i| i.powf(2.))
            .sum::<f64>()
            .powf(1.0 / self.d as f64)
            <= radius
    }
}
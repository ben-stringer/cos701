use conv::prelude::*;
use plotters::coord::ranged1d::AsRangedCoord;
use plotters::coord::ranged1d::KeyPointHint;
use plotters::coord::ranged1d::NoDefaultFormatting;
use plotters::coord::ranged1d::ValueFormatter;
use plotters::prelude::*;
use std::ops::Range;

pub struct DerangedCoordf64(f64, f64);

impl From<Range<f64>> for DerangedCoordf64 {
    fn from(range: Range<f64>) -> Self {
        return DerangedCoordf64(range.start, range.end);
    }
}

impl From<Deranged> for DerangedCoordf64 {
    fn from(deranged: Deranged) -> Self {
        return Self(deranged.start, deranged.end);
    }
}

impl Ranged for DerangedCoordf64 {
    type FormatOption = NoDefaultFormatting;
    type ValueType = f64;
    #[allow(clippy::float_cmp)]
    fn map(&self, v: &f64, limit: (i32, i32)) -> i32 {
        if self.1 == self.0 {
            return (limit.1 - limit.0) / 2;
        }
        let logic_length = (*v as f64 - self.0 as f64) / (self.1 as f64 - self.0 as f64);
        let actual_length = limit.1 - limit.0;
        if actual_length == 0 {
            return limit.1;
        }
        return limit.0 + (actual_length as f64 * logic_length + 1e-3).floor() as i32;
    }
    fn key_points<Hint: KeyPointHint>(&self, hint: Hint) -> Vec<f64> {
        compute_f64_key_points((self.0, self.1), hint.max_num_points())
    }
    fn range(&self) -> Range<f64> {
        return self.0..self.1;
    }
}

fn compute_f64_key_points(range: (f64, f64), max_points: usize) -> Vec<f64> {
    if max_points == 0 {
        return Vec::new();
    }
    let range = (range.0 as f64, range.1 as f64);
    let mut scale = (10f64).powf((range.1 - range.0).log(10.0).floor());
    let mut digits = -(range.1 - range.0).log(10.0).floor() as i32 + 1;
    fn rem_euclid(a: f64, b: f64) -> f64 {
        if b > 0.0 {
            a - (a / b).floor() * b
        } else {
            a - (a / b).ceil() * b
        }
    }
    if 1 + ((range.1 - range.0) / scale).floor() as usize > max_points {
        scale *= 10.0;
    }
    'outer: loop {
        let old_scale = scale;
        for nxt in [2.0, 5.0, 10.0].iter() {
            let new_left = range.0 + scale / nxt - rem_euclid(range.0, scale / nxt);
            let new_right = range.1 - rem_euclid(range.1, scale / nxt);
            let npoints = 1 + ((new_right - new_left) / old_scale * nxt) as usize;
            if npoints > max_points {
                break 'outer;
            }
            scale = old_scale / nxt;
        }
        scale = old_scale / 10.0;
        if scale < 1.0 {
            digits += 1;
        }
    }
    let mut ret = Vec::new();
    let mut left = range.0 + scale - rem_euclid(range.0, scale);
    let right = range.1 - rem_euclid(range.1, scale);
    while left <= right {
        let size = (10f64).powf(digits as f64 + 1.0);
        let new_left = (left * size).abs() + 1e-3;
        if left < 0.0 {
            left = -new_left.round() / size;
        } else {
            left = new_left.round() / size;
        }
        ret.push(left as f64);
        left += scale;
    }
    return ret;
}

impl DiscreteRanged for DerangedCoordf64 {
    fn size(&self) -> usize {
        if &self.1 < &self.0 {
            return 0;
        }
        let values = self.1 - self.0;
        (values + 1_f64) as usize
    }
    fn index_of(&self, value: &Self::ValueType) -> Option<usize> {
        if value < &self.0 {
            return None;
        }
        let ret = value - self.0;
        Some(ret as usize)
    }
    fn from_index(&self, index: usize) -> Option<Self::ValueType> {
        if let Ok(index) = index.value_as::<f64>() {
            return Some(self.0 + index);
        }
        None
    }
}

impl ValueFormatter<f64> for DerangedCoordf64 {
    fn format(value: &f64) -> String {
        plotters::data::float::FloatPrettyPrinter {
            allow_scientific: false,
            min_decimal: 1,
            max_decimal: 5,
        }
        .print(*value)
    }
}

pub struct Deranged {
    pub start: f64,
    pub end: f64,
}

impl AsRangedCoord for Deranged {
    type CoordDescType = DerangedCoordf64;
    type Value = f64;
}

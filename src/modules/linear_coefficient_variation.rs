use super::{arifmetic_mean::arifmetic_mean, linear_deviation::linear_deviation};

pub fn linear_coefficient_variations(vec: &Vec<f64>) -> f64 {
    linear_deviation(vec) / arifmetic_mean(vec)
}

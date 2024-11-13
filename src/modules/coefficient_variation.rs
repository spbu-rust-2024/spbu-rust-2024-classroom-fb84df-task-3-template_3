use super::{arifmetic_mean::arifmetic_mean, standart_deviation::standart_deviation};

pub fn coefficient_variations(vec: &Vec<f64>) -> f64 {
    standart_deviation(vec) / arifmetic_mean(vec)
}

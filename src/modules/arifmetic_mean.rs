pub fn arifmetic_mean(vec: &Vec<f64>) -> f64 {
    let sum: f64 = vec.iter().sum();
    return sum / vec.len() as f64;
}
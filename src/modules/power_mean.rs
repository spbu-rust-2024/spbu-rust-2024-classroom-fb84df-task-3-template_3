pub fn power_mean(vec: &Vec<f64>, n: &i16) -> f64 {
    let powered: f64 = vec.iter().map(|&x| x.powf(*n as f64)).sum();
    return (powered / vec.len() as f64).sqrt();
}

pub fn geometric_mean(vec: &Vec<f64>) -> f64 {
    let product: f64 = vec.iter().product();
    return product.sqrt();
}

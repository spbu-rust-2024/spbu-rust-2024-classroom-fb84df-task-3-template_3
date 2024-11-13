use std::f64::consts::E;

fn f(x: f64) -> f64 {
    E.powf(x) + 2.0 * x * x + x + 5.0
}

fn df(x: f64) -> f64 {
    E.powf(x) + 4.0 * x + 1.0
}

fn newton_method(y: f64, eps: f64) -> f64 {
    let mut it = 0;
    let mut start_x = 1.0;
    let mut next_x = start_x - (f(start_x) - y) / df(start_x);

    while (next_x - y).abs() > eps && it < 100 {
        let f_val = f(start_x);
        let df_val = df(start_x);

        if f_val.is_nan() || df_val.is_nan() || next_x.is_nan() {
            return f64::NAN;
        }

        if f_val.is_infinite() || df_val.is_infinite() || next_x.is_infinite() {
            return f64::INFINITY;
        }

        next_x = start_x - (f_val - y) / df_val;
        start_x = next_x;
        it += 1;
    }

    next_x
}

pub fn kolmogorov_smirnov(vec: &Vec<f64>, eps: f64) -> f64 {
    let x = vec
        .iter()
        .map(|&x| E.powf(x) + 2.0 * x * x + x + 5.0)
        .sum::<f64>()
        / vec.len() as f64;
    newton_method(x, eps)
}

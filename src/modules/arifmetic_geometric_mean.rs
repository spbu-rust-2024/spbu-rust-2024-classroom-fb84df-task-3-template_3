use super::{arifmetic_mean::arifmetic_mean, geometric_mean::geometric_mean};

fn recursive_ag(eps: &f64, a_mean: &f64, g_mean: &f64, i: i32) -> [f64; 2] {
    if (a_mean - g_mean).abs() < *eps {
        return [*a_mean, *g_mean];
    }
    let new_a_mean = (a_mean + g_mean) / 2.0;
    let new_g_mean = (a_mean + g_mean).sqrt();
    let new_i = i + 1;
    println!("A{}: {}, B{}: {}", new_i, a_mean, new_i, g_mean);

    recursive_ag(eps, &new_a_mean, &new_g_mean, new_i)
}

pub fn arifmetic_geometric_mean(vec: &Vec<f64>, eps: &f64) -> [f64; 2] {
    let a_mean = arifmetic_mean(&vec);
    let g_mean = geometric_mean(&vec);
    let new_a;
    let new_b;
    let i = 0;
    [new_a, new_b] = recursive_ag(eps, &a_mean, &g_mean, i);
    return [new_a, new_b];
}

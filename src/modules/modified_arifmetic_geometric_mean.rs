use super::{arifmetic_mean::arifmetic_mean, geometric_mean::geometric_mean};

fn recursive_ag1(eps: &f64, a_mean: &f64, g_mean: &f64, z_mean: &f64, i: i32) -> [f64; 3] {
    if (a_mean - g_mean).abs() < *eps {
        return [*a_mean, *g_mean, *z_mean];
    }
    let new_a_mean = (a_mean + g_mean) / 2.0;
    let new_g_mean = z_mean + ((a_mean - z_mean) * (g_mean - z_mean)).sqrt();
    let new_z_mean = z_mean - ((a_mean - z_mean) * (g_mean - z_mean)).sqrt();
    let new_i = i + 1;
    println!(
        "A{}: {}, B{}: {}, Z{}: {}",
        new_i, a_mean, new_i, g_mean, new_i, z_mean
    );
    recursive_ag1(eps, &new_a_mean, &new_g_mean, &new_z_mean, new_i)
}

pub fn modified_arifmetic_geometric_mean(vec: &Vec<f64>, eps: &f64) -> [f64; 3] {
    let a_mean = arifmetic_mean(&vec);
    let g_mean = geometric_mean(&vec);
    let z_mean = 0.0;
    let new_a;
    let new_g;
    let new_z;
    let i: i32 = 0;
    [new_a, new_g, new_z] = recursive_ag1(eps, &a_mean, &g_mean, &z_mean, i);
    return [new_a, new_g, new_z];
}

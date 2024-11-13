use super::arifmetic_mean::arifmetic_mean;

fn replace(vec: &mut Vec<f64>, i: usize, k: u8, value: f64) {
    if i + k as usize <= vec.len() {
        vec[i..i + k as usize].iter_mut().for_each(|x| *x = value);
    }
}

pub fn winsorized_mean(vec: &Vec<f64>, k: u8) -> f64 {
    let mut replace_vec = vec.clone();
    replace_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let elements_to_replace = replace_vec.len() * k as usize / 100;

    let start_val = replace_vec[elements_to_replace];
    let end_val = replace_vec[replace_vec.len() - elements_to_replace - 1];

    replace(&mut replace_vec, 0, elements_to_replace as u8, start_val);

    let start_val = replace_vec.len() - elements_to_replace;
    replace(
        &mut replace_vec,
        start_val,
        elements_to_replace as u8,
        end_val,
    );
    arifmetic_mean(&replace_vec)
}

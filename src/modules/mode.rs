use ordered_float::OrderedFloat;
use std::collections::HashMap;

pub fn mode(vec: &Vec<f64>) -> f64 {
    let mut map = HashMap::new();

    for &num in vec.iter() {
        *map.entry(OrderedFloat(num)).or_insert(0) += 1;
    }
    let mut mode = 0.0;
    let mut max_count = 0;

    for (&num, &count) in map.iter() {
        if count > max_count {
            max_count = count;
            mode = *num;
        }
    }

    mode
}

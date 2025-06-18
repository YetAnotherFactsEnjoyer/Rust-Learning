pub fn comp_sq_root(nb: i32) -> i32 {
    let root = (n as f64).sqrt();

    if root.fract() == 0.0 { root as i32 } else { 0 }
}

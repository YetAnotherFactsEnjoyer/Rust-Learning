pub fn comp_pow_rec(nb: i32, power: u32) -> i32 {
    if power == 0 {
        1
    } else {
        nb * comp_pow_rec(nb, power - 1)
    }
}

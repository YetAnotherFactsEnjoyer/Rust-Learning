pub fn comp_fac_rec(nb: i32) -> i32 {
    if nb == 0 { 1 } else { nb * comp_fac_it(nb - 1) }
}

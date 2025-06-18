pub fn my_swap(a: &mut i16, b: &mut i16) {
    let c = *a;
    *a = *b;
    *b = c;
}

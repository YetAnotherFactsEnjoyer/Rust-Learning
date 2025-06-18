pub fn sort_arr(arr: &mut [i32], size: usize) {
    if arr.len() != size {
        panic!("Size mismatch");
    }
    arr.sort()
}

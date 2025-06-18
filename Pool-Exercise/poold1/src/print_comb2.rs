pub fn my_print_comb2() {
    for i in 0..=99 {
        for j in 0..=99 {
            if j > i {
                print!("{:02} {:02}, ", i, j);
            }
        }
    }
}

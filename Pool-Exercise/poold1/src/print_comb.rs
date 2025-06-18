pub fn my_print_comb() {
    for i in 0..=999 {
        if i / 100 >= i / 10 % 10 || i / 10 % 10 >= i % 10 {
            continue;
        }
        print!("{:03}, ", i);
    }
    println!();
}

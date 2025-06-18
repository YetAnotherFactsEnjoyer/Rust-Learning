fn loop_nb(i: u32) -> bool {
    let mut numb = i;
    while numb >= 10 {
        if numb % 10 <= numb % 100 / 10 {
            return false;
        }
        numb /= 10;
    }
    true
}

pub fn my_print_combn(n: u32) {
    for i in 0..(u32::pow(10, n)) {
        if loop_nb(i) {
            print!("{} ,", i);
        }
    }
    println!();
}

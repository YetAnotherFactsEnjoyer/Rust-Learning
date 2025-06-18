pub fn my_putstr(str: &str) {
    for char in str.chars() {
        print!("{char}");
    }
    println!("");
}

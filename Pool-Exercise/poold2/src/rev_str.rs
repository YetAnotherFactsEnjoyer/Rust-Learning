pub fn rev_str(str: &str) -> String {
    let mut rev_str = String::new();

    for char in str.chars().rev() {
        rev_str.push(char);
    }
    rev_str
}

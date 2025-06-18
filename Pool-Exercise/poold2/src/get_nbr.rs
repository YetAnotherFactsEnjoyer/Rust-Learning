pub fn get_nbr(str: &str) -> i32 {
    let mut nb = String::new();
    for c in str.chars().filter(|c| c.is_ascii_digit()) {
        nb.push(c);
    }
    nb.parse().unwrap_or_default()
}

pub fn strcpy(mut dest: String, src: &str) -> String {
    for char in src.chars() {
        dest.push(char);
    }
    dest
}

pub fn strncpy(mut dest: String, src: &str, size: usize) -> String {
    for (i, char) in src.chars().enumerate() {
        if i <= size {
            dest.push(char);
        }
    }
    dest
}

fn to_ascii(nb: u8) -> String {
    match nb {
        x @ 97..123 => format!("{}", x as char),
        _ => "".into(),
    }
}

pub fn my_rev_alpha() {
    for i in (97..123).rev() {
        print!("{}", to_ascii(i));
    }
}

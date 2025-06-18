pub fn is_prime(nb: i32) -> i32 {
    for i in 2..nb {
        if nb % i == 0 {
            return 0;
        }
    }
    1
}

pub fn find_prime_sup(nb: i32) -> i32 {
    let mut i = nb;

    while is_prime(i) != 1 {
        i += 1;
    }
    i
}
